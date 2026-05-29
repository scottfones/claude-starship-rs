//! Claude Code Statusline for Starship
//!
//! A Rust implementation of the Claude Code statusline, displaying:
//! - Line 1: Current directory + detected programming languages
//! - Line 2: Claude model, reset timer, session time, context usage
//! - Line 3: Git repository info (if in a git repo)

use claude_starship_rs::input::ClaudeInput;
use claude_starship_rs::renderer;

fn main() {
    // Parse JSON input from stdin
    let input = ClaudeInput::from_stdin().unwrap_or_default();

    // Render and print the statusline
    print!("{}", renderer::render(&input));
}
