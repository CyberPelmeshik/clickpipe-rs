// thiserror позволяет удобно описывать собственные типы ошибок.
// Без него пришлось бы вручную реализовывать std::error::Error и Display.
use thiserror::Error;

// AppError - общий тип ошибок приложения.
//
// Такой enum удобен тем, что разные слои программы могут возвращать один тип:
// - чтение файла возвращает std::io::Error;
// - парсинг JSON возвращает serde_json::Error;
// - бизнес-валидация возвращает текстовое объяснение.
//
// Благодаря #[from] ниже оператор ? сам превращает исходные ошибки в AppError.
#[derive(Debug, Error)]
pub enum AppError {
    // Ошибки ввода-вывода:
    // файл не найден, нет прав на чтение, ошибка чтения строки и т.д.
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    // Ошибки JSON:
    // строка в events.jsonl не является корректным JSON
    // или тип поля не совпал с ожидаемым типом в RawEvent.
    #[error("json parse error: {0}")]
    Json(#[from] serde_json::Error),

    // Ошибки валидации доменных правил:
    // например, event_id пустой или отсутствует event_name.
    // Здесь храним String, чтобы можно было вернуть понятное сообщение.
    #[error("validation error: {0}")]
    Validation(String),

    #[error("http error: {0}")]
    Http(#[from] ureq::Error),
}
