//! Model segment - displays the Claude model name.

use crate::colors::{Ansi, Color, PowerlineBuilder};
use crate::icons;
use crate::segments::{RenderContext, Segment};

/// Model segment showing the Claude model display name.
///
/// - Displays with yellow background
/// - Shows star icon followed by model name
/// - Starts with rounded left cap (first segment on line 2)
pub struct ModelSegment;

impl Segment for ModelSegment {
    fn render(&self, ctx: &RenderContext<'_>) -> Option<String> {
        let model_name = ctx
            .input
            .model
            .display_name
            .as_deref()
            .filter(|s| !s.is_empty())
            .unwrap_or("Claude");

        let content = format!("{} {}", icons::STAR, model_name);

        let output = PowerlineBuilder::new()
            .start_rounded(Color::Yellow, Color::Bg1)
            .content(&content)
            .end_plain()
            .build();

        Some(format!("{}{}", output, Ansi::reset()))
    }
}
