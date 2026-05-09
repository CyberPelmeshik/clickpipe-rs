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
    let args = Cli::parse();
    let mut config = Config::defaults();

    // Пока путь к файлу с событиями жестко задан в коде.
    // Программа ожидает, что запуск будет из корня Rust-проекта clickpipe-rs,
    // где существует папка sample/events.jsonl.
    config.merge_cli(&args);
    //config.merge_file(&args.config_file);
    config.apply_env();

    // main сам не содержит бизнес-логику: он только запускает run()
    // и красиво обрабатывает итоговый Result.
    match run(
        &config.input_file,
        &config.clickhouse_url,
        &config.clickhouse_db,
        &config.clickhouse_table,
        &config.clickhouse_user,
        &config.clickhouse_password,
    ) {
        // Ok(()) означает, что вся обработка завершилась успешно.
        Ok(()) => println!("Done"),
        // Err(e) означает, что случилась фатальная ошибка:
        // например, файл не найден или строка не распарсилась как JSON.
        Err(e) => eprintln!("Fatal error: {e}"),
    }
}

// run содержит основной сценарий работы приложения:
// 1. прочитать сырые события из файла;
// 2. попробовать нормализовать каждое событие;
// 3. посчитать валидные и невалидные события;
// 4. вывести результат в консоль.
fn run(
    path: &str,
    clickhouse_url: &str,
    clickhouse_db: &str,
    clickhouse_table: &str,
    clickhouse_user: &str,
    clickhouse_password: &str,
) -> Result<(), error::AppError> {
    // source::read_raw_events читает JSONL-файл и возвращает Vec<RawEvent>.
    // Оператор ? означает: если вернулась ошибка, сразу выйти из run с этой ошибкой.
    let raw_events = source::read_raw_events(path)?;

    // Сохраняем общее количество событий до того, как начнем их фильтровать.
    let total = raw_events.len();

    // Сюда будем складывать только те события, которые прошли валидацию.
    let mut valid_events: Vec<NormalizedEvent> = Vec::new();

    // А здесь считаем события, которые пришлось пропустить из-за ошибки валидации.
    let mut invalid_count = 0usize;

    // raw_events передается в цикл по значению.
    // Это нормально: после нормализации исходный RawEvent больше не нужен.
    for raw in raw_events {
        // normalize_event превращает RawEvent в NormalizedEvent
        // или возвращает AppError::Validation с объяснением проблемы.
        match validate::normalize_event(raw) {
            // Валидное событие сохраняем для дальнейшей обработки.
            // Сейчас дальнейшая обработка - это просто печать в консоль.
            Ok(event) => {
                valid_events.push(event);
            }
            Err(err) => {
                // Ошибка валидации одного события не останавливает весь процесс.
                // Для ingester это важное поведение: плохая запись не должна ломать всю пачку.
                invalid_count += 1;
                eprintln!("Skipped event: {err}");
            }
        }
    }

    sink::insert_events(
        &valid_events,
        clickhouse_url,
        clickhouse_db,
        clickhouse_table,
        clickhouse_user,
        clickhouse_password,
    )?;

    // Печатаем простую статистику по запуску.
    println!("Total events: {total}");
    println!("Valid events: {}", valid_events.len());
    println!("Invalid events: {invalid_count}");

    // Пока ClickHouse sink не реализован, валидные события просто выводятся.
    // Следующий логичный шаг - заменить этот блок на batch insert в ClickHouse.
    for event in &valid_events {
        println!("{event:?}");
    }

    // Возвращаем успешный результат без полезного значения.
    Ok(())
}
