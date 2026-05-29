//! Repository name segment - displays the Git repository name.

use crate::colors::{Ansi, Color, PowerlineBuilder};
use crate::segments::{RenderContext, Segment};

/// Repository name segment showing the Git repository name.
///
/// - Displays with aqua background
/// - First segment on line 3 (starts with rounded left cap)
pub struct RepoNameSegment;

impl Segment for RepoNameSegment {
    fn render(&self, ctx: &RenderContext<'_>) -> Option<String> {
        let repo = ctx.git_repo?;

        let workdir = repo.workdir()?;
        let repo_name = workdir.file_name()?.to_string_lossy();

        let output = PowerlineBuilder::new()
            .start_rounded(Color::Aqua, Color::Fg0)
            .content(&repo_name)
            .end_plain()
            .build();

        Some(format!("{}{}", output, Ansi::reset()))
    }
}
