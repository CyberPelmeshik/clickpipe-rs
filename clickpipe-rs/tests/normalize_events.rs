use clickpipe_rs::{model::RawEvent, validate::normalize_event};
use serde_json;

#[test]
fn test_valid_login_event() {
    let json = r#"{
        "event_id": "e1",
        "ts": "2024-01-01T12:00:00Z",
        "source": "api",
        "event_name": "login",
        "user_id": 123,
        "payload": {"ip": "192.168.1.1"}
    }"#;

    let raw: RawEvent = serde_json::from_str(json).unwrap();
    let normalized = normalize_event(raw).unwrap();

    assert_eq!(normalized.event_id, "e1");
    assert_eq!(normalized.ts, "2024-01-01T12:00:00Z");
    assert_eq!(normalized.user_id, Some(123u64));
}

#[test]
fn test_missing_event_id_fails() {
    let json = r#"{"event_name": "login", "ts": "2024-01-01"}"#;
    let raw: RawEvent = serde_json::from_str(json).unwrap();

    assert!(matches!(
        normalize_event(raw),
        Err(clickpipe_rs::error::AppError::Validation(_))
    ));
}

#[test]
fn test_empty_string_event_id_fails() {
    let json = r#"{"event_id": "", "ts": "2024-01-01"}"#;
    let raw: RawEvent = serde_json::from_str(json).unwrap();

    assert!(matches!(
        normalize_event(raw),
        Err(clickpipe_rs::error::AppError::Validation(_))
    ));
}

#[test]
fn test_missing_payload_substitutes_null() {
    let json = r#"{"event_id": "e1", "ts": "2024-01-01", "source": "api"}"#;
    let raw: RawEvent = serde_json::from_str(json).unwrap();

    let normalized = normalize_event(raw).unwrap();

    // payload всегда должен быть Some(null) для отсутствующего поля
    assert!(normalized.payload.is_null());
}

#[test]
fn test_payload_without_missing_field() {
    let json = r#"{
        "event_id": "e2",
        "ts": "2024-01-01T12:00:00Z",
        "source": "api",
        "event_name": "click",
        "payload": {"button": "submit"}
    }"#;

    let raw: RawEvent = serde_json::from_str(json).unwrap();
    let normalized = normalize_event(raw).unwrap();

    assert!(matches!(normalized.payload, serde_json::Value::Object(_)));
}
