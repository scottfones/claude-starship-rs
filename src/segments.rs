//! Segment trait and implementations for statusline segments.
//!
//! Each segment is responsible for rendering a portion of the statusline
//! with appropriate colors and powerline transitions.

mod context;
mod directory;
mod language;
mod model;
mod reset_time;
mod session_time;

pub use context::ContextSegment;
pub use directory::DirectorySegment;
pub use language::LanguageSegment;
pub use model::ModelSegment;
pub use reset_time::ResetTimeSegment;
pub use session_time::SessionTimeSegment;

use crate::input::ClaudeInput;
use git2::Repository;
use std::path::Path;

/// Rendering context passed to all segments.
pub struct RenderContext<'a> {
    /// Current working directory.
    pub current_dir: &'a Path,

    /// Git repository (if in a git repo).
    pub git_repo: Option<&'a Repository>,

    /// Parsed Claude Code input.
    pub input: &'a ClaudeInput,
}

impl<'a> RenderContext<'a> {
    #[must_use]
    pub const fn new(
        current_dir: &'a Path,
        input: &'a ClaudeInput,
        git_repo: Option<&'a Repository>,
    ) -> Self {
        Self {
            current_dir,
            git_repo,
            input,
        }
    }
}

/// A segment that can render itself to a string with ANSI escape codes.
pub trait Segment {
    /// Render the segment.
    ///
    /// Returns `None` if the segment should not be displayed (e.g., no data available).
    fn render(&self, ctx: &RenderContext<'_>) -> Option<String>;
}
