//! Formatting utilities for durations, tokens, and time differences.

use std::time::Duration;

/// Format a duration in milliseconds to a human-readable string.
///
/// # Examples
///
/// ```
/// use claude_starship_rs::formatters::format_duration_ms;
///
/// assert_eq!(format_duration_ms(30_000), "0m");
/// assert_eq!(format_duration_ms(60_000), "1m");
/// assert_eq!(format_duration_ms(3_660_000), "1h 1m");
/// ```
#[must_use]
pub fn format_duration_ms(ms: u64) -> String {
    let total_min = ms / 60_000;
    format_minutes(total_min)
}

/// Format minutes to a human-readable string.
///
/// # Examples
///
/// ```
/// use claude_starship_rs::formatters::format_minutes;
///
/// assert_eq!(format_minutes(0), "0m");
/// assert_eq!(format_minutes(45), "45m");
/// assert_eq!(format_minutes(90), "1h 30m");
/// ```
#[must_use]
pub fn format_minutes(total_min: u64) -> String {
    let hours = total_min / 60;
    let mins = total_min % 60;

    if hours > 0 {
        format!("{hours}h {mins}m")
    } else {
        format!("{mins}m")
    }
}

/// Format a token count with K/M suffixes.
///
/// # Examples
///
/// ```
/// use claude_starship_rs::formatters::format_tokens;
///
/// assert_eq!(format_tokens(500), "500");
/// assert_eq!(format_tokens(1_500), "1K");
/// assert_eq!(format_tokens(1_500_000), "1M");
/// ```
#[must_use]
pub fn format_tokens(tokens: u64) -> String {
    if tokens >= 1_000_000 {
        let m = tokens / 1_000_000;
        format!("{m}M")
    } else if tokens >= 1_000 {
        let k = tokens / 1_000;
        format!("{k}K")
    } else {
        format!("{tokens}")
    }
}

/// Format a time difference in seconds to a human-readable relative time.
///
/// # Examples
///
/// ```
/// use claude_starship_rs::formatters::format_relative_time;
///
/// assert_eq!(format_relative_time(30), "now");
/// assert_eq!(format_relative_time(120), "2m");
/// assert_eq!(format_relative_time(7200), "2h");
/// assert_eq!(format_relative_time(172800), "2d");
/// ```
#[must_use]
pub fn format_relative_time(seconds: i64) -> String {
    let seconds = seconds.unsigned_abs();

    if seconds < 60 {
        "now".to_string()
    } else if seconds < 3600 {
        let m = seconds / 60;
        format!("{m}m")
    } else if seconds < 86400 {
        let h = seconds / 3600;
        format!("{h}h")
    } else if seconds < 604_800 {
        let d = seconds / 86400;
        format!("{d}d")
    } else if seconds < 2_592_000 {
        let w = seconds / 604_800;
        format!("{w}w")
    } else {
        let mo = seconds / 2_592_000;
        format!("{mo}mo")
    }
}

/// Format a `Duration` to a human-readable string.
#[must_use]
#[allow(clippy::cast_possible_truncation)] // u64::MAX ms = ~584 million years
pub fn format_duration(duration: Duration) -> String {
    format_duration_ms(duration.as_millis() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_duration_ms() {
        assert_eq!(format_duration_ms(0), "0m");
        assert_eq!(format_duration_ms(59_999), "0m");
        assert_eq!(format_duration_ms(60_000), "1m");
        assert_eq!(format_duration_ms(3_600_000), "1h 0m");
        assert_eq!(format_duration_ms(3_660_000), "1h 1m");
    }

    #[test]
    fn test_format_minutes() {
        assert_eq!(format_minutes(0), "0m");
        assert_eq!(format_minutes(59), "59m");
        assert_eq!(format_minutes(60), "1h 0m");
        assert_eq!(format_minutes(90), "1h 30m");
    }

    #[test]
    fn test_format_tokens() {
        assert_eq!(format_tokens(0), "0");
        assert_eq!(format_tokens(999), "999");
        assert_eq!(format_tokens(1_000), "1K");
        assert_eq!(format_tokens(999_999), "999K");
        assert_eq!(format_tokens(1_000_000), "1M");
        assert_eq!(format_tokens(5_500_000), "5M");
    }

    #[test]
    fn test_format_relative_time() {
        assert_eq!(format_relative_time(0), "now");
        assert_eq!(format_relative_time(59), "now");
        assert_eq!(format_relative_time(60), "1m");
        assert_eq!(format_relative_time(3599), "59m");
        assert_eq!(format_relative_time(3600), "1h");
        assert_eq!(format_relative_time(86399), "23h");
        assert_eq!(format_relative_time(86400), "1d");
        assert_eq!(format_relative_time(604_799), "6d");
        assert_eq!(format_relative_time(604_800), "1w");
        assert_eq!(format_relative_time(2_592_000), "1mo");
    }
}
