use serde::Serialize;

use crate::error::AppError;
use crate::model::NormalizedEvent;

#[derive(Serialize)]
struct ClickHouseEventRow<'a> {
    event_id: &'a str,
    ts: &'a str,
    source: &'a str,
    event_name: &'a str,
    user_id: Option<u64>,
    payload: String,
    event_hash: u64,
}

pub fn insert_events(
    events: &[NormalizedEvent],
    clickhouse_url: &str,
    clickhouse_db: &str,
    clickhouse_table: &str,
    clickhouse_user: &str,
    clickhouse_password: &str,
) -> Result<(), AppError> {
    if events.is_empty() {
        return Ok(());
    }

    let mut lines = Vec::with_capacity(events.len());

    for event in events {
        let row = ClickHouseEventRow {
            event_id: &event.event_id,
            ts: &event.ts,
            source: &event.source,
            event_name: &event.event_name,
            user_id: event.user_id,
            payload: event.payload.to_string(),
            event_hash: event.event_hash,
        };

        lines.push(serde_json::to_string(&row)?);
    }

    let body = lines.join("\n");
    // print!("{clickhouse_url}/?query=INSERT%20INTO%20{clickhouse_db}.{clickhouse_table}%20FORMAT%20JSONEachRow");
    ureq::post(format!("{clickhouse_url}/?query=INSERT%20INTO%20{clickhouse_db}.{clickhouse_table}%20FORMAT%20JSONEachRow"))
        .header("Content-Type", "text/plain")
        .header("X-ClickHouse-User", clickhouse_user)
        .header("X-ClickHouse-Key", clickhouse_password)
        .send(&body)?;

    Ok(())
}
