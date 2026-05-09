use crate::Cli;

#[derive(Debug)]
pub struct Config {
    pub clickhouse_url: String,
    pub clickhouse_db: String,
    pub clickhouse_table: String,
    pub clickhouse_user: String,
    pub clickhouse_password: String,
    pub input_file: String,
}

impl Config {
    /// 1. Defaults — жёстко в коде
    pub fn defaults() -> Self {
        Self {
            clickhouse_url: "http://localhost:8123".into(),
            clickhouse_db: "app".into(),
            clickhouse_table: "events".into(),
            clickhouse_user: "app".into(),
            clickhouse_password: "app_password".into(),
            input_file: "sample/events.jsonl".into(),
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
    /*
    pub fn merge_file(&mut self, file: &FileConfig) {
        if let Some(url) = &file.clickhouse.url {
            self.clickhouse_url = url.clone();
        }
        if let Some(db) = &file.clickhouse.db {
            self.clickhouse_db = db.clone();
        }
        // ... и так далее
    }
    */

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
}
