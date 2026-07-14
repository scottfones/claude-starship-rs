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

        let content = ctx
            .input
            .effort
            .level
            .as_deref()
            .filter(|s| !s.is_empty())
            .map_or_else(
                || format!("{} {}", icons::SETTINGS, model_name),
                |level| format!("{} {} {}", icons::SETTINGS, model_name, level),
            );

        let output = PowerlineBuilder::new()
            .start_rounded(Color::Yellow, Color::Bg1)
            .content(&content)
            .end_plain()
            .build();

        Some(format!("{}{}", output, Ansi::reset()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::ClaudeInput;
    use std::path::Path;

    fn render_json(json: &str) -> String {
        let input: ClaudeInput = serde_json::from_str(json).unwrap();
        let ctx = RenderContext::new(Path::new("."), &input, None);
        ModelSegment.render(&ctx).unwrap()
    }

    #[test]
    fn test_effort_level_appended() {
        let output =
            render_json(r#"{"model":{"display_name":"Opus 4.8"},"effort":{"level":"high"}}"#);
        assert!(output.contains("Opus 4.8 high"));
    }

    #[test]
    fn test_effort_level_absent() {
        let output = render_json(r#"{"model":{"display_name":"Opus 4.8"}}"#);
        // The model name is immediately followed by the wrapper space and an
        // ANSI escape (reset), not an appended level word.
        assert!(output.contains("Opus 4.8 \u{1b}"));
    }
}
