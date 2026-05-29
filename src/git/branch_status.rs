//! Branch status segment - displays branch name and status indicators.

use crate::colors::{Ansi, Color, PowerlineBuilder};
use crate::icons;
use crate::segments::{RenderContext, Segment};
use git2::{Repository, StatusOptions};
use std::fmt::Write;

/// Branch status segment showing the current branch and status indicators.
///
/// - Displays with blue background
/// - Shows branch icon, name, and status indicators:
///   - `*` = modified files
///   - `+` = staged files
///   - `?` = untracked files
/// - Shows ahead/behind counts if tracking a remote
pub struct BranchStatusSegment;

impl BranchStatusSegment {
    /// Get the current branch name.
    fn get_branch_name(repo: &Repository) -> Option<String> {
        let head = repo.head().ok()?;

        if head.is_branch() {
            head.shorthand().map(str::to_string)
        } else {
            // Detached HEAD - show short commit hash
            head.target().map(|oid| format!("{:.7}", oid.to_string()))
        }
    }

    /// Get status indicators for the repository.
    fn get_status_indicators(repo: &Repository) -> String {
        let mut status = String::new();

        let mut opts = StatusOptions::new();
        opts.include_untracked(true).recurse_untracked_dirs(false);

        if let Ok(statuses) = repo.statuses(Some(&mut opts)) {
            let mut has_modified = false;
            let mut has_staged = false;
            let mut has_untracked = false;

            for entry in statuses.iter() {
                let s = entry.status();

                if s.is_wt_modified()
                    || s.is_wt_deleted()
                    || s.is_wt_renamed()
                    || s.is_wt_typechange()
                {
                    has_modified = true;
                }

                if s.is_index_new()
                    || s.is_index_modified()
                    || s.is_index_deleted()
                    || s.is_index_renamed()
                    || s.is_index_typechange()
                {
                    has_staged = true;
                }

                if s.is_wt_new() {
                    has_untracked = true;
                }
            }

            if has_modified {
                status.push('*');
            }
            if has_staged {
                status.push('+');
            }
            if has_untracked {
                status.push('?');
            }
        }

        status
    }

    /// Get ahead/behind counts relative to upstream.
    fn get_ahead_behind(repo: &Repository) -> Option<(usize, usize)> {
        let head = repo.head().ok()?;
        let local_oid = head.target()?;

        // Get upstream branch
        let branch = repo
            .find_branch(head.shorthand()?, git2::BranchType::Local)
            .ok()?;
        let upstream = branch.upstream().ok()?;
        let upstream_oid = upstream.get().target()?;

        repo.graph_ahead_behind(local_oid, upstream_oid).ok()
    }
}

impl Segment for BranchStatusSegment {
    fn render(&self, ctx: &RenderContext<'_>) -> Option<String> {
        let repo = ctx.git_repo?;

        let branch = Self::get_branch_name(repo)?;
        let status = Self::get_status_indicators(repo);

        let mut ahead_behind = String::new();
        if let Some((ahead, behind)) = Self::get_ahead_behind(repo) {
            if ahead > 0 {
                let _ = write!(ahead_behind, "+{ahead}");
            }
            if behind > 0 {
                let _ = write!(ahead_behind, "-{behind}");
            }
        }

        // Transition from aqua (repo name) to blue
        let output = format!(
            "{}{}{}{}{}{} {} {}{}{} {}",
            Ansi::fg(Color::Aqua),
            Ansi::bg(Color::Blue),
            PowerlineBuilder::ARROW,
            Ansi::reset(),
            Ansi::bg(Color::Blue),
            Ansi::fg(Color::Fg0),
            icons::BRANCH,
            branch,
            status,
            ahead_behind,
            Ansi::reset(),
        );

        Some(output)
    }
}
