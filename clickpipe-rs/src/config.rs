use crate::Cli;
use serde::Deserialize;

// ──────────────────────────────────────────────
// 1. FileConfig — только для десериализации TOML
// ──────────────────────────────────────────────
#[derive(Debug, Deserialize)]
pub struct FileConfig {
    pub input: Option<InputFileConfig>,
    pub clickhouse: Option<ClickHouseFileConfig>,
    pub batch: Option<BatchFileConfig>,
}

#[derive(Debug, Deserialize)]
pub struct InputFileConfig {
    pub file: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ClickHouseFileConfig {
    pub url: Option<String>,
    pub db: Option<String>,
    pub table: Option<String>,
    pub user: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BatchFileConfig {
    pub size: Option<usize>,
    pub flush_interval_secs: Option<u64>,
}

// ──────────────────────────────────────────────
// 2. Config — финальный рабочий конфиг
// ──────────────────────────────────────────────
#[derive(Debug)]
pub struct Config {
    pub input_file: String,
    pub clickhouse_url: String,
    pub clickhouse_db: String,
    pub clickhouse_table: String,
    pub clickhouse_user: String,
    pub clickhouse_password: String,
    pub batch_size: usize,
    pub flush_interval_secs: u64,
}

impl Config {
    /// 1. Defaults — жёстко в коде
    pub fn defaults() -> Self {
        Self {
            input_file: "sample/events.jsonl".into(),
            clickhouse_url: "http://localhost:8123".into(),
            clickhouse_db: "app".into(),
            clickhouse_table: "events".into(),
            clickhouse_user: "app".into(),
            clickhouse_password: "app_password".into(),
            batch_size: 1000,
            flush_interval_secs: 1,
        }
    }

    /// 2. Переменные окружения — перебивают defaults, если есть
    pub fn apply_env(&mut self) {
        if let Ok(val) = std::env::var("CLICKHOUSE_URL") {
            self.clickhouse_url = val;
        }
        if let Ok(val) = std::env::var("CLICKHOUSE_DB") {
            self.clickhouse_db = val;
        }
        if let Ok(val) = std::env::var("CLICKHOUSE_TABLE") {
            self.clickhouse_table = val;
        }
        if let Ok(val) = std::env::var("CLICKHOUSE_USER") {
            self.clickhouse_user = val;
        }
        if let Ok(val) = std::env::var("CLICKHOUSE_PASSWORD") {
            self.clickhouse_password = val;
        }
        if let Ok(val) = std::env::var("INPUT_FILE") {
            self.input_file = val;
        }
    }

    /// 3. Конфиг-файл — перебивает env, если поле есть в файле
    pub fn merge_file(&mut self, file: &FileConfig) {
        if let Some(input) = &file.input {
            if let Some(val) = &input.file {
                self.input_file = val.clone();
            }
        }
        if let Some(ch) = &file.clickhouse {
            if let Some(val) = &ch.url {
                self.clickhouse_url = val.clone();
            }
            if let Some(val) = &ch.db {
                self.clickhouse_db = val.clone();
            }
            if let Some(val) = &ch.table {
                self.clickhouse_table = val.clone();
            }
            if let Some(val) = &ch.user {
                self.clickhouse_user = val.clone();
            }
            if let Some(val) = &ch.password {
                self.clickhouse_password = val.clone();
            }
        }
        if let Some(batch) = &file.batch {
            if let Some(val) = batch.size {
                self.batch_size = val;
            }
            if let Some(val) = batch.flush_interval_secs {
                self.flush_interval_secs = val;
            }
        }
    }

    /// 4. CLI — перебивает всё, но только если пользователь реально передал флаг
    pub fn merge_cli(&mut self, cli: &Cli) {
        // Если Some — пользователь явно написал, берём
        // Если None — пользователь не указывал, оставляем что было
        if let Some(url) = &cli.clickhouse_url {
            self.clickhouse_url = url.clone();
        }
        if let Some(db) = &cli.clickhouse_db {
            self.clickhouse_db = db.clone();
        }
        if let Some(table) = &cli.clickhouse_table {
            self.clickhouse_table = table.clone();
        }
        if let Some(user) = &cli.clickhouse_user {
            self.clickhouse_user = user.clone();
        }
        if let Some(password) = &cli.clickhouse_password {
            self.clickhouse_password = password.clone();
        }
        if let Some(file) = &cli.input_file {
            self.input_file = file.clone();
        }
    }

    pub fn build(config_file: Option<&str>, cli: &Cli) -> Self {
        // 1. Дефолты
        let mut cfg = Self::defaults();

        // 2. Переменные окружения
        cfg.apply_env();

        // 3. TOML-файл (если указан --config)
        if let Some(path) = config_file {
            match std::fs::read_to_string(path) {
                Ok(content) => match toml::from_str::<FileConfig>(&content) {
                    Ok(file_cfg) => cfg.merge_file(&file_cfg),
                    Err(e) => eprintln!("Warning: bad config {path}: {e}"),
                },
                Err(e) => eprintln!("Warning: can't read config {path}: {e}"),
            }
        }

        // 4. CLI — высший приоритет (ВОТ ТУТ merge_cli!)
        cfg.merge_cli(cli);

        cfg
    }
}
