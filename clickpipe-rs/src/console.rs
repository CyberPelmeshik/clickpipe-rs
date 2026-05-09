use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "clickpipe")]
#[command(version = "0.1")]
#[command(about = "Fault-tolerant event ingester for ClickHouse in Rust with local disk buffering, batching, retry, and deduplication.", long_about = None)]
pub struct Cli {
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
