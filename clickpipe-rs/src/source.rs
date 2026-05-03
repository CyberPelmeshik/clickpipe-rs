// File нужен, чтобы открыть файл на диске.
use std::fs::File;

// BufReader читает файл эффективно, не загружая все содержимое сразу.
// BufRead дает метод lines(), который возвращает итератор по строкам.
use std::io::{BufRead, BufReader};

use crate::error::AppError;
use crate::model::RawEvent;

// read_raw_events читает файл формата JSON Lines.
//
// JSON Lines означает:
// - каждая строка файла является отдельным JSON-объектом;
// - весь файл целиком не является одним большим JSON-массивом.
//
// Пример:
// {"event_id":"e1","event_name":"login"}
// {"event_id":"e2","event_name":"logout"}
pub fn read_raw_events(path: &str) -> Result<Vec<RawEvent>, AppError> {
    // Открываем файл.
    // Если файл не существует или его нельзя прочитать, File::open вернет ошибку.
    // Оператор ? автоматически превратит std::io::Error в AppError::Io.
    let file = File::open(path)?;

    // Оборачиваем файл в BufReader, чтобы читать его построчно.
    let reader = BufReader::new(file);

    // Здесь накапливаем все успешно распарсенные события.
    // Для будущего надежного ingester это место, вероятно, изменится:
    // лучше будет обрабатывать поток событий пачками, а не держать все в памяти.
    let mut events = Vec::new();

    // Проходим по файлу строка за строкой.
    for line_result in reader.lines() {
        // lines() возвращает Result<String, std::io::Error>,
        // потому что ошибка может случиться прямо во время чтения очередной строки.
        let line = line_result?;

        // Пустые строки пропускаем.
        // Это делает sample-файл чуть более терпимым к лишним переводам строк.
        if line.trim().is_empty() {
            continue;
        }

        // Парсим одну строку JSON в RawEvent.
        // Если JSON битый или поле имеет неправильный тип, вернется AppError::Json.
        let event: RawEvent = serde_json::from_str(&line)?;

        // Сохраняем сырое событие для следующего этапа - validate::normalize_event.
        events.push(event);
    }

    // Возвращаем все прочитанные события.
    Ok(events)
}
