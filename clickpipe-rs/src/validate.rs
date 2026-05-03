use crate::error::AppError;
use crate::model::{NormalizedEvent, RawEvent};

// normalize_event превращает RawEvent в NormalizedEvent.
//
// Это важная граница в программе:
// до этой функции данные считаются внешними и ненадежными,
// после нее событие уже можно безопасно отправлять дальше по pipeline.
pub fn normalize_event(raw: RawEvent) -> Result<NormalizedEvent, AppError> {
    // event_id обязателен.
    //
    // raw.event_id имеет тип Option<String>.
    // filter оставляет Some(value), только если строка не пустая после trim().
    // ok_or_else превращает None в ошибку валидации.
    let event_id = raw
        .event_id
        .filter(|s| !s.trim().is_empty())
        .ok_or_else(|| AppError::Validation("event_id is missing".to_string()))?;

    // ts тоже обязателен, но пока проверяется только наличие непустой строки.
    // Формат даты здесь еще не проверяется.
    let ts = raw
        .ts
        .filter(|s| !s.trim().is_empty())
        .ok_or_else(|| AppError::Validation("ts is missing".to_string()))?;

    // source обязателен: без него будет сложно понимать,
    // откуда пришло событие и как его потом анализировать.
    let source = raw
        .source
        .filter(|s| !s.trim().is_empty())
        .ok_or_else(|| AppError::Validation("source is missing".to_string()))?;

    // event_name обязателен: это основное смысловое имя события.
    let event_name = raw
        .event_name
        .filter(|s| !s.trim().is_empty())
        .ok_or_else(|| AppError::Validation("event_name is missing".to_string()))?;

    // payload не обязателен.
    // Если он отсутствует, подставляем JSON null, чтобы в NormalizedEvent поле всегда было заполнено.
    let payload = raw.payload.unwrap_or(serde_json::Value::Null);

    // Формируем строку, из которой считаем hash.
    // Сейчас hash учитывает основные поля события, но не payload.
    // Это сознательно простой вариант для первого прототипа.
    let hash_input = format!("{event_id}|{ts}|{source}|{event_name}|{:?}", raw.user_id);

    // Считаем u64 hash для нормализованного события.
    let event_hash = simple_hash(&hash_input);

    // Собираем итоговое событие.
    // На этом этапе все обязательные поля уже имеют обычный String, а не Option<String>.
    Ok(NormalizedEvent {
        event_id,
        ts,
        source,
        event_name,
        user_id: raw.user_id,
        payload,
        event_hash,
    })
}

// simple_hash - маленькая вспомогательная функция для получения u64 из строки.
//
// DefaultHasher удобен для прототипа, но важный нюанс:
// его не стоит считать стабильным форматом для долговременной дедупликации между версиями Rust.
// Для production-дедупликации лучше выбрать явный алгоритм, например xxhash/blake3/sha256.
fn simple_hash(s: &str) -> u64 {
    // DefaultHasher живет в стандартной библиотеке.
    use std::collections::hash_map::DefaultHasher;

    // Hash нужен, чтобы строка умела "записать себя" в hasher.
    // Hasher нужен, чтобы потом получить итоговое число через finish().
    use std::hash::{Hash, Hasher};

    // Создаем новый hasher для одной строки.
    let mut hasher = DefaultHasher::new();

    // Передаем содержимое строки в hasher.
    s.hash(&mut hasher);

    // Забираем итоговое значение.
    hasher.finish()
}
