//! Session time segment - displays the current session duration.

use crate::colors::{Ansi, Color, PowerlineBuilder};
use crate::formatters::format_duration_ms;
use crate::icons;
use crate::segments::{RenderContext, Segment};

/// Session time segment showing the current session duration.
///
/// - Displays with blue background
/// - Shows clock icon followed by duration
/// - Transitions from aqua (reset time segment)
pub struct SessionTimeSegment;

impl Segment for SessionTimeSegment {
    fn render(&self, ctx: &RenderContext<'_>) -> Option<String> {
        let duration_ms = ctx.input.cost.total_duration_ms;

        // Transition from aqua to blue
        let transition = format!(
            "{}{}{}{}",
            Ansi::fg(Color::Aqua),
            Ansi::bg(Color::Blue),
            PowerlineBuilder::ARROW,
            Ansi::reset(),
        );

        if duration_ms > 0 {
            let formatted = format_duration_ms(duration_ms);

            Some(format!(
                "{}{}{} {} {} {}",
                transition,
                Ansi::bg(Color::Blue),
                Ansi::fg(Color::Fg0),
                icons::CLOCK,
                formatted,
                Ansi::reset(),
            ))
        } else {
            // Just show the transition
            Some(transition)
        }
    }
}
