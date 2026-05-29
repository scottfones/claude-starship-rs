//! JSON input parsing for Claude Code statusline data.
//!
//! Parses the JSON structure provided by Claude Code via stdin.

use serde::Deserialize;
use std::io::{self, BufRead};

/// Top-level input structure from Claude Code.
#[derive(Clone, Debug, Default, Deserialize)]
pub struct ClaudeInput {
    /// Context window information.
    #[serde(default)]
    pub context_window: ContextWindow,

    /// Session cost and duration information.
    #[serde(default)]
    pub cost: Cost,

    /// Current working directory (fallback).
    #[serde(default)]
    pub cwd: Option<String>,

    /// Model information.
    #[serde(default)]
    pub model: Model,

    /// Workspace information.
    #[serde(default)]
    pub workspace: Workspace,
}

/// Model information.
#[derive(Clone, Debug, Default, Deserialize)]
pub struct Model {
    /// Display name of the model (e.g., "Opus 4.5").
    pub display_name: Option<String>,
}

/// Workspace information.
#[derive(Clone, Debug, Default, Deserialize)]
pub struct Workspace {
    /// Current working directory.
    pub current_dir: Option<String>,
}

/// Context window usage information.
#[derive(Clone, Debug, Default, Deserialize)]
pub struct ContextWindow {
    /// Total context window size in tokens.
    #[serde(default = "default_context_size")]
    pub context_window_size: u64,

    /// Current usage statistics.
    #[serde(default)]
    pub current_usage: ContextUsage,
}

/// Context usage statistics.
#[derive(Clone, Debug, Default, Deserialize)]
pub struct ContextUsage {
    /// Tokens read from cache (matches terminal display).
    #[serde(default)]
    pub cache_read_input_tokens: u64,

    /// Total input tokens.
    #[serde(default)]
    pub input_tokens: u64,

    /// Total output tokens.
    #[serde(default)]
    pub output_tokens: u64,
}

/// Cost and duration information.
#[derive(Clone, Debug, Default, Deserialize)]
pub struct Cost {
    /// Total session duration in milliseconds.
    #[serde(default)]
    pub total_duration_ms: u64,
}

const fn default_context_size() -> u64 {
    200_000
}

impl ClaudeInput {
    /// Parse input from stdin.
    ///
    /// # Errors
    ///
    /// Returns `io::Error` if reading from stdin fails or the input
    /// contains invalid JSON.
    pub fn from_stdin() -> io::Result<Self> {
        let stdin = io::stdin();
        let mut input = String::new();

        for line in stdin.lock().lines() {
            input.push_str(&line?);
        }

        if input.is_empty() {
            return Ok(Self::default());
        }

        serde_json::from_str(&input).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }

    /// Get the current working directory from workspace or cwd fallback.
    #[must_use]
    pub fn current_dir(&self) -> Option<&str> {
        self.workspace
            .current_dir
            .as_deref()
            .or(self.cwd.as_deref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_minimal() {
        let json = r"{}";
        let input: ClaudeInput = serde_json::from_str(json).unwrap();
        assert!(input.model.display_name.is_none());
    }

    #[test]
    fn test_parse_model() {
        let json = r#"{"model":{"display_name":"Opus 4.5"}}"#;
        let input: ClaudeInput = serde_json::from_str(json).unwrap();
        assert_eq!(input.model.display_name.as_deref(), Some("Opus 4.5"));
    }

    #[test]
    fn test_parse_workspace() {
        let json = r#"{"workspace":{"current_dir":"/home/user/project"}}"#;
        let input: ClaudeInput = serde_json::from_str(json).unwrap();
        assert_eq!(input.current_dir(), Some("/home/user/project"));
    }

    #[test]
    fn test_parse_cwd_fallback() {
        let json = r#"{"cwd":"/home/user/fallback"}"#;
        let input: ClaudeInput = serde_json::from_str(json).unwrap();
        assert_eq!(input.current_dir(), Some("/home/user/fallback"));
    }

    #[test]
    fn test_parse_context_window() {
        let json = r#"{
            "context_window": {
                "context_window_size": 200000,
                "current_usage": {
                    "cache_read_input_tokens": 50000,
                    "input_tokens": 60000,
                    "output_tokens": 10000
                }
            }
        }"#;
        let input: ClaudeInput = serde_json::from_str(json).unwrap();
        assert_eq!(input.context_window.context_window_size, 200_000);
        assert_eq!(
            input.context_window.current_usage.cache_read_input_tokens,
            50_000
        );
    }

    #[test]
    fn test_parse_cost() {
        let json = r#"{"cost":{"total_duration_ms":300000}}"#;
        let input: ClaudeInput = serde_json::from_str(json).unwrap();
        assert_eq!(input.cost.total_duration_ms, 300_000);
    }
}
