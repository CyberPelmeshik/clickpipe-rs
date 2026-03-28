use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::error::AppError;
use crate::model::RawEvent;

pub fn read_raw_events(path: &str) -> Result<Vec<RawEvent>, AppError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut events = Vec::new();

    for line_result in reader.lines() {
        let line = line_result?;

        if line.trim().is_empty() {
            continue;
        }

        let event: RawEvent = serde_json::from_str(&line)?;
        events.push(event);
    }

    Ok(events)
}