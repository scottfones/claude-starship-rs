//! Renderer that assembles all segments into the final statusline output.

use crate::colors::Ansi;
use crate::git::{self, BranchStatusSegment, LastCommitSegment, RepoNameSegment, TrackingSegment};
use crate::input::ClaudeInput;
use crate::segments::{
    ContextSegment, DirectorySegment, LanguageSegment, ModelSegment, RenderContext,
    ResetTimeSegment, Segment, SessionTimeSegment,
};
use std::path::Path;

/// Render the complete statusline.
pub fn render(input: &ClaudeInput) -> String {
    let current_dir = input
        .current_dir()
        .map_or_else(|| Path::new("."), Path::new);

    // Use absolute path
    let current_dir =
        std::fs::canonicalize(current_dir).unwrap_or_else(|_| current_dir.to_path_buf());

    // Try to open git repo
    let git_repo = git::open_repo(&current_dir);
    let git_repo_ref = git_repo.as_ref();

    let ctx = RenderContext::new(&current_dir, input, git_repo_ref);

    let mut output = String::new();

    // Reset any previous formatting
    output.push_str(Ansi::reset());

    // Line 1: Directory + Language
    output.push_str(&render_line1(&ctx));

    // Line 2: Claude info (model, reset time, session time, context)
    output.push('\n');
    output.push_str(&render_line2(&ctx));

    // Line 3: Git info (only if in a git repo)
    if git_repo_ref.is_some() {
        output.push('\n');
        output.push_str(&render_line3(&ctx));
    }

    output
}

/// Render Line 1: Directory + Language
fn render_line1(ctx: &RenderContext<'_>) -> String {
    let mut line = String::new();

    // Directory segment
    if let Some(segment) = DirectorySegment.render(ctx) {
        line.push_str(&segment);
    }

    // Language segment
    if let Some(segment) = LanguageSegment.render(ctx) {
        line.push_str(&segment);
    }

    line
}

/// Render Line 2: Model + Reset Time + Session Time + Context
fn render_line2(ctx: &RenderContext<'_>) -> String {
    let mut line = String::new();

    // Model segment (starts the line)
    if let Some(segment) = ModelSegment.render(ctx) {
        line.push_str(&segment);
    }

    // Reset time segment
    if let Some(segment) = ResetTimeSegment.render(ctx) {
        line.push_str(&segment);
    }

    // Session time segment
    if let Some(segment) = SessionTimeSegment.render(ctx) {
        line.push_str(&segment);
    }

    // Context segment (ends the line)
    if let Some(segment) = ContextSegment.render(ctx) {
        line.push_str(&segment);
    }

    line
}

/// Render Line 3: Git info (repo name + branch/status + tracking + last commit)
fn render_line3(ctx: &RenderContext<'_>) -> String {
    let mut line = String::new();

    // Repo name segment (starts the line)
    if let Some(segment) = RepoNameSegment.render(ctx) {
        line.push_str(&segment);
    }

    // Branch status segment
    if let Some(segment) = BranchStatusSegment.render(ctx) {
        line.push_str(&segment);
    }

    // Tracking segment
    if let Some(segment) = TrackingSegment.render(ctx) {
        line.push_str(&segment);
    }

    // Last commit segment (ends the line)
    if let Some(segment) = LastCommitSegment.render(ctx) {
        line.push_str(&segment);
    }

    line
}
