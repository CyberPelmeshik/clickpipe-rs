use crate::error::AppError;
use crate::model::{NormalizedEvent, RawEvent};

pub fn normalize_event(raw: RawEvent) -> Result<NormalizedEvent, AppError> {
    let event_id = raw
        .event_id
        .filter(|s| !s.trim().is_empty())
        .ok_or_else(|| AppError::Validation("event_id is missing".to_string()))?;

    let ts = raw
        .ts
        .filter(|s| !s.trim().is_empty())
        .ok_or_else(|| AppError::Validation("ts is missing".to_string()))?;

    let source = raw
        .source
        .filter(|s| !s.trim().is_empty())
        .ok_or_else(|| AppError::Validation("source is missing".to_string()))?;

    let event_name = raw
        .event_name
        .filter(|s| !s.trim().is_empty())
        .ok_or_else(|| AppError::Validation("event_name is missing".to_string()))?;

    let payload = raw.payload.unwrap_or(serde_json::Value::Null);

    let hash_input = format!("{event_id}|{ts}|{source}|{event_name}|{:?}", raw.user_id);
    let event_hash = simple_hash(&hash_input);

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

fn simple_hash(s: &str) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish()
}