//! Tracking segment - displays the upstream branch being tracked.

use crate::colors::{Ansi, Color, PowerlineBuilder};
use crate::segments::{RenderContext, Segment};
use git2::Repository;

/// Tracking segment showing the upstream branch name.
///
/// - Displays with bg3 background
/// - Shows the remote/branch being tracked (e.g., "origin/main")
pub struct TrackingSegment;

impl TrackingSegment {
    /// Get the upstream branch name.
    fn get_upstream(repo: &Repository) -> Option<String> {
        let head = repo.head().ok()?;
        let branch_name = head.shorthand()?;

        let branch = repo
            .find_branch(branch_name, git2::BranchType::Local)
            .ok()?;
        let upstream = branch.upstream().ok()?;

        upstream.name().ok().flatten().map(str::to_string)
    }
}

impl Segment for TrackingSegment {
    fn render(&self, ctx: &RenderContext<'_>) -> Option<String> {
        let repo = ctx.git_repo?;

        // Transition from blue to bg3
        let transition = format!(
            "{}{}{}{}",
            Ansi::fg(Color::Blue),
            Ansi::bg(Color::Bg3),
            PowerlineBuilder::ARROW,
            Ansi::reset(),
        );

        if let Some(upstream) = Self::get_upstream(repo) {
            Some(format!(
                "{}{}{} {} {}",
                transition,
                Ansi::bg(Color::Bg3),
                Ansi::fg(Color::Fg0),
                upstream,
                Ansi::reset(),
            ))
        } else {
            // No upstream, just show transition
            Some(transition)
        }
    }
}
