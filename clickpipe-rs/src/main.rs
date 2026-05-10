/*
Подключаем соседние файлы из папки src как модули текущего crate.
После этого к их содержимому можно обращаться через error::..., model::... и т.д.
*/
mod config;
mod error;
mod model;
mod sink;
mod source;
mod validate;

// Берем тип NormalizedEvent из модуля model.
// Это уже проверенное и приведенное к удобному виду событие.
use crate::config::Config;
use crate::model::NormalizedEvent;
use clap::Parser;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[derive(Parser, Debug)]
#[command(name = "clickpipe")]
#[command(version = "0.1")]
#[command(about = "Fault-tolerant event ingester for ClickHouse in Rust with local disk buffering, batching, retry, and deduplication.", long_about = None)]
struct Cli {
    /// ClickHouse HTTP URL
    #[arg(short = 'c', long = "clickhouse-url")]
    clickhouse_url: Option<String>, // ← None = пользователь не передавал

    /// ClickHouse database
    #[arg(long = "db")]
    clickhouse_db: Option<String>,

    /// ClickHouse table
    #[arg(long = "table")]
    clickhouse_table: Option<String>,

    /// ClickHouse user
    #[arg(long = "user")]
    clickhouse_user: Option<String>,

    /// ClickHouse password
    #[arg(long = "password")]
    clickhouse_password: Option<String>,

    /// Batch size
    #[arg(short = 'b', long = "batch-size")]
    batch_size: Option<usize>,

    /// Flush interval in seconds
    #[arg(long = "flush-interval")]
    flush_interval_secs: Option<u64>,

    /// Input file
    #[arg(short = 'i', long = "input-file")]
    input_file: Option<String>,

    /// Config file path
    #[arg(short = 'f', long = "config")]
    config_file: Option<String>,
}

fn main() {
    // Флаг: "поступил сигнал на завершение?"
    let shutdown_flag = Arc::new(AtomicBool::new(false));
    let flag_clone = shutdown_flag.clone();

    // Устанавливаем обработчик Ctrl+C
    ctrlc::set_handler(move || {
        println!("\n⏳ Graceful shutdown...");
        flag_clone.store(true, Ordering::SeqCst); // сигнал: пора выключаться
    })
    .expect("Error setting Ctrl+C handler");

    let args = Cli::parse();
    let config = Config::build(args.config_file.as_deref(), &args);

    match run(&config, &shutdown_flag) {
        Ok(()) => println!("Done"),
        Err(e) => eprintln!("Fatal error: {e}"),
    }
}

// run содержит основной сценарий работы приложения:
// 1. прочитать сырые события из файла;
// 2. попробовать нормализовать каждое событие;
// 3. посчитать валидные и невалидные события;
// 4. вывести результат в консоль.
fn run(config: &Config, shutdown: &AtomicBool) -> Result<(), error::AppError> {
    let raw_events = source::read_raw_events(&config.input_file)?;

    // Если сигнал пришёл во время чтения — выходим
    if shutdown.load(Ordering::SeqCst) {
        println!("Shutdown before processing");
        return Ok(());
    }

    let total = raw_events.len();
    let mut valid_events: Vec<NormalizedEvent> = Vec::new();
    let mut invalid_count = 0usize;

    for raw in raw_events {
        // Проверяем флаг на каждом событии — быстрое реагирование
        if shutdown.load(Ordering::SeqCst) {
            println!(
                "Shutdown during validation — saved {} events",
                valid_events.len()
            );
            break; // не бросаем то, что уже навалидировали
        }

        match validate::normalize_event(raw) {
            Ok(event) => valid_events.push(event),
            Err(err) => {
                invalid_count += 1;
                eprintln!("Skipped event: {err}");
            }
        }
    }

    // Отправляем то, что успели накопить (даже если нас прервали)
    if !valid_events.is_empty() {
        println!("Flushing {} events...", valid_events.len());
        sink::insert_events(
            &valid_events,
            &config.clickhouse_url,
            &config.clickhouse_db,
            &config.clickhouse_table,
            &config.clickhouse_user,
            &config.clickhouse_password,
        )?;
    }

    println!("Total events: {total}");
    println!("Valid events: {}", valid_events.len());
    println!("Invalid events: {invalid_count}");

    // Если нас прервали — не выводим оставшиеся события
    if !shutdown.load(Ordering::SeqCst) {
        for event in &valid_events {
            println!("{event:?}");
        }
    }

    Ok(())
}
