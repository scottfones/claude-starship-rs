//! History file parsing for reset time calculation.
//!
//! Parses `~/.claude/history.jsonl` to calculate time until the 5-hour usage
//! window resets.

use crate::formatters::format_minutes;
use serde::Deserialize;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

/// Five hours in milliseconds.
const FIVE_HOURS_MS: u64 = 5 * 60 * 60 * 1000;

/// One hour in milliseconds.
const ONE_HOUR_MS: u64 = 60 * 60 * 1000;

/// History entry from the JSONL file.
#[derive(Debug, Deserialize)]
struct HistoryEntry {
    /// Timestamp in milliseconds since epoch.
    timestamp: u64,
}

/// Get the path to the history file.
fn history_path() -> Option<PathBuf> {
    dirs::home_dir().map(|home| home.join(".claude").join("history.jsonl"))
}

/// Calculate the remaining time until the 5-hour window resets.
///
/// Returns `None` if the history file doesn't exist or has no recent entries.
/// Returns `Some("reset")` if the window has already reset.
/// Returns `Some("Xh Ym")` for the remaining time.
#[must_use]
pub fn calculate_reset_time() -> Option<String> {
    let path = history_path()?;
    let file = File::open(&path).ok()?;
    let reader = BufReader::new(file);

    #[allow(clippy::cast_possible_truncation)] // u64::MAX ms = ~584 million years
    let now_ms = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .ok()?
        .as_millis() as u64;

    let window_start = now_ms.saturating_sub(FIVE_HOURS_MS);

    // Find the earliest timestamp within the 5-hour window
    let mut earliest: Option<u64> = None;

    for line in reader.lines() {
        let line = line.ok()?;
        if line.trim().is_empty() {
            continue;
        }

        if let Ok(entry) = serde_json::from_str::<HistoryEntry>(&line)
            && entry.timestamp >= window_start
        {
            earliest = Some(earliest.map_or(entry.timestamp, |e: u64| e.min(entry.timestamp)));
        }
    }

    let earliest = earliest?;

    // Floor to the hour (matches bash script behavior)
    let floored = (earliest / ONE_HOUR_MS) * ONE_HOUR_MS;
    let window_end = floored + FIVE_HOURS_MS;

    if now_ms >= window_end {
        return Some("reset".to_string());
    }

    let remaining_ms = window_end.saturating_sub(now_ms);
    let remaining_min = remaining_ms / 60_000;

    Some(format_minutes(remaining_min))
}

/// Get the default reset time string when no history is available.
#[must_use]
pub fn default_reset_time() -> String {
    "5h".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_history_path() {
        let path = history_path();
        assert!(path.is_some());
        let path = path.unwrap();
        assert!(path.ends_with(".claude/history.jsonl"));
    }

    #[test]
    fn test_default_reset_time() {
        assert_eq!(default_reset_time(), "5h");
    }
}
