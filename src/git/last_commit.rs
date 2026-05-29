//! Last commit segment - displays time since last commit.

use crate::colors::{Ansi, Color, PowerlineBuilder};
use crate::formatters::format_relative_time;
use crate::segments::{RenderContext, Segment};
use git2::{Repository, StatusOptions};

/// Last commit segment showing time since the last commit.
///
/// - Displays with bg1 background
/// - Shows relative time (e.g., "2h", "3d")
/// - Adds "ok" indicator if working tree is clean
/// - Final segment on line 3, outputs trailing arrow
pub struct LastCommitSegment;

impl LastCommitSegment {
    /// Get the timestamp of the last commit.
    fn get_last_commit_time(repo: &Repository) -> Option<i64> {
        let head = repo.head().ok()?;
        let commit = head.peel_to_commit().ok()?;
        Some(commit.time().seconds())
    }

    /// Check if the working tree is clean.
    fn is_clean(repo: &Repository) -> bool {
        let mut opts = StatusOptions::new();
        opts.include_untracked(true).recurse_untracked_dirs(false);

        repo.statuses(Some(&mut opts))
            .is_ok_and(|statuses| statuses.is_empty())
    }
}

impl Segment for LastCommitSegment {
    #[allow(clippy::option_if_let_else)]
    fn render(&self, ctx: &RenderContext<'_>) -> Option<String> {
        let repo = ctx.git_repo?;

        // Transition from bg3 to bg1
        let transition = format!(
            "{}{}{}{}",
            Ansi::fg(Color::Bg3),
            Ansi::bg(Color::Bg1),
            PowerlineBuilder::ARROW,
            Ansi::reset(),
        );

        if let Some(commit_time) = Self::get_last_commit_time(repo) {
            let now = chrono::Utc::now().timestamp();
            let diff = now - commit_time;
            let time_str = format_relative_time(diff);

            let clean_indicator = if Self::is_clean(repo) { " ok" } else { "" };

            Some(format!(
                "{}{}{} @ {}{} {}{}{}{}",
                transition,
                Ansi::bg(Color::Bg1),
                Ansi::fg(Color::Fg0),
                time_str,
                clean_indicator,
                Ansi::reset(),
                Ansi::fg(Color::Bg1),
                PowerlineBuilder::ARROW,
                Ansi::reset(),
            ))
        } else {
            // No commits - just show transition and trailing arrow
            Some(format!(
                "{}{}{}{}",
                transition,
                Ansi::fg(Color::Bg1),
                PowerlineBuilder::ARROW,
                Ansi::reset(),
            ))
        }
    }
}
