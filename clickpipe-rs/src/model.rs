use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct RawEvent {
    pub event_id: Option<String>,
    pub ts: Option<String>,
    pub source: Option<String>,
    pub event_name: Option<String>,
    pub user_id: Option<u64>,
    pub payload: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct NormalizedEvent {
    pub event_id: String,
    pub ts: String,
    pub source: String,
    pub event_name: String,
    pub user_id: Option<u64>,
    pub payload: serde_json::Value,
    pub event_hash: u64,
}