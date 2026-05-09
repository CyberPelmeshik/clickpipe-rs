use clickpipe_rs::model::RawEvent;
use clickpipe_rs::validate::normalize_event;
use clickpipe_rs::error::AppError;

#[test]
fn test_raw_event_deserialization() {
    let json = r#"{
        "event_id": "e1",
        "ts": "2024-01-01T00:00:00Z",
        "source": "api",
        "event_name": "login",
        "payload": {"user_agent": "curl"}
    }"#;

    let raw: RawEvent = serde_json::from_str(json).unwrap();

    assert_eq!(raw.event_id, Some("e1".to_string()));
    assert_eq!(raw.ts, Some("2024-01-01T00:00:00Z".to_string()));
    assert!(raw.payload.is_some());
}


#[test]
fn test_invalid_event_handling() {
    let json = r#"{
        "event_id": "",
        "ts": "2024-01-01T00:00:00Z",
        "source": "api",
        "event_name": "login",
        "payload": {"user_agent": "curl"}
    }"#;

    let raw: RawEvent = serde_json::from_str(json).unwrap();

    let result = normalize_event(raw);

    match result {
        Err(AppError::Validation(message)) => {
            assert_eq!(message, "event_id is missing");
        }
        Ok(event) => {
            panic!("Expected validation error, but got Ok: {:?}", event);
        }
        Err(error) => {
            panic!("Expected AppError::Validation, but got: {:?}", error);
        }
    }


    // Доработать нормальную проверку Timestamp
    let json = r#"{
        "event_id": "e1",
        "ts": "",
        "source": "api",
        "event_name": "login",
        "payload": {"user_agent": "curl"}
    }"#;

    let raw: RawEvent = serde_json::from_str(json).unwrap();

    let result = normalize_event(raw);

    match result {
        Err(AppError::Validation(message)) => {
            assert_eq!(message, "ts is missing");
        }
        Ok(event) => {
            panic!("Expected validation error, but got Ok: {:?}", event);
        }
        Err(error) => {
            panic!("Expected AppError::Validation, but got: {:?}", error);
        }
    }


    let json = r#"{
        "event_id": "e1",
        "ts": "2024-01-01T00:00:00Z",
        "source": "",
        "event_name": "login",
        "payload": {"user_agent": "curl"}
    }"#;

    let raw: RawEvent = serde_json::from_str(json).unwrap();

    let result = normalize_event(raw);

    match result {
        Err(AppError::Validation(message)) => {
            assert_eq!(message, "source is missing");
        }
        Ok(event) => {
            panic!("Expected validation error, but got Ok: {:?}", event);
        }
        Err(error) => {
            panic!("Expected AppError::Validation, but got: {:?}", error);
        }
    }

    let json = r#"{
        "event_id": "e1",
        "ts": "2024-01-01T00:00:00Z",
        "source": "api",
        "event_name": "",
        "payload": {"user_agent": "curl"}
    }"#;

    let raw: RawEvent = serde_json::from_str(json).unwrap();

    let result = normalize_event(raw);

    match result {
        Err(AppError::Validation(message)) => {
            assert_eq!(message, "event_name is missing");
        }
        Ok(event) => {
            panic!("Expected validation error, but got Ok: {:?}", event);
        }
        Err(error) => {
            panic!("Expected AppError::Validation, but got: {:?}", error);
        }
    }

}