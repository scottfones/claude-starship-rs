//! Claude Code Statusline Library
//!
//! A Rust implementation of the Claude Code statusline for use with starship prompt.
//!
//! # Architecture
//!
//! The statusline is built from modular segments:
//!
//! - **Line 1**: Directory + detected programming languages
//! - **Line 2**: Claude model, reset timer, session time, context usage
//! - **Line 3**: Git repository info (only shown in git repos)
//!
//! Each segment implements the [`segments::Segment`] trait and is responsible
//! for rendering itself with appropriate ANSI escape codes and powerline transitions.
//!
//! # Example
//!
//! ```no_run
//! use claude_starship_rs::input::ClaudeInput;
//! use claude_starship_rs::renderer;
//!
//! let input = ClaudeInput::from_stdin().unwrap_or_default();
//! print!("{}", renderer::render(&input));
//! ```

pub mod colors;
pub mod formatters;
pub mod git;
pub mod history;
pub mod icons;
pub mod input;
pub mod renderer;
pub mod segments;
