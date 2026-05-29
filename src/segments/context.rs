//! Context segment - displays context window usage with warning colors.

use crate::colors::{Ansi, Color, PowerlineBuilder};
use crate::formatters::format_tokens;
use crate::segments::{RenderContext, Segment};

/// Effective context percentage: auto-compact and other features reserve ~6.0% of
/// the 1M context window. This value should match what terminal shows as
/// "Context left until auto-compaction".
const EFFECTIVE_CONTEXT_PCT: u64 = 94;

/// Context segment showing context window usage.
///
/// - Calculates % used based on effective context limit
/// - Changes color based on usage level:
///   - Normal (bg3): < 75%
///   - Warning (yellow): 75-89%
///   - Critical (orange): 90%+
/// - Last segment on line 2, outputs trailing arrow
pub struct ContextSegment;

impl ContextSegment {
    /// Determine the background color based on usage percentage.
    const fn get_color(used_pct: u64) -> Color {
        if used_pct >= 95 {
            Color::Orange
        } else if used_pct >= 85 {
            Color::Yellow
        } else {
            Color::Bg3
        }
    }
}

impl Segment for ContextSegment {
    fn render(&self, ctx: &RenderContext<'_>) -> Option<String> {
        let cache_tokens = ctx
            .input
            .context_window
            .current_usage
            .cache_read_input_tokens;
        let context_size = ctx.input.context_window.context_window_size;

        let formatted_tokens = format_tokens(cache_tokens);

        // Calculate effective limit and used percentage
        let effective_limit = context_size * EFFECTIVE_CONTEXT_PCT / 100;
        let used_pct = if effective_limit > 0 && cache_tokens > 0 {
            (cache_tokens * 100 / effective_limit).min(100)
        } else {
            0
        };

        let ctx_bg = Self::get_color(used_pct);

        // Transition from blue to context color
        let output = format!(
            "{}{}{}{}{}{} {} {}% {}{}{}{}",
            Ansi::fg(Color::Blue),
            Ansi::bg(ctx_bg),
            PowerlineBuilder::ARROW,
            Ansi::reset(),
            Ansi::bg(ctx_bg),
            Ansi::fg(Color::Fg0),
            formatted_tokens,
            used_pct,
            Ansi::reset(),
            Ansi::fg(ctx_bg),
            PowerlineBuilder::ARROW,
            Ansi::reset(),
        );

        Some(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_thresholds() {
        assert_eq!(ContextSegment::get_color(0), Color::Bg3);
        assert_eq!(ContextSegment::get_color(74), Color::Bg3);
        assert_eq!(ContextSegment::get_color(75), Color::Yellow);
        assert_eq!(ContextSegment::get_color(89), Color::Yellow);
        assert_eq!(ContextSegment::get_color(90), Color::Orange);
        assert_eq!(ContextSegment::get_color(100), Color::Orange);
    }
}
