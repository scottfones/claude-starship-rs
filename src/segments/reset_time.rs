//! Reset time segment - displays time until 5-hour usage window resets.

use crate::colors::{Ansi, Color, PowerlineBuilder};
use crate::history::{calculate_reset_time, default_reset_time};
use crate::icons;
use crate::segments::{RenderContext, Segment};

/// Reset time segment showing time until the 5-hour usage window resets.
///
/// - Displays with aqua background
/// - Shows hourglass icon followed by remaining time
/// - Transitions from yellow (model segment)
pub struct ResetTimeSegment;

impl Segment for ResetTimeSegment {
    fn render(&self, _ctx: &RenderContext<'_>) -> Option<String> {
        let remaining = calculate_reset_time().unwrap_or_else(default_reset_time);

        // Transition from yellow to aqua
        let output = format!(
            "{}{}{}{}{}{} {} {} {}",
            Ansi::fg(Color::Yellow),
            Ansi::bg(Color::Aqua),
            PowerlineBuilder::ARROW,
            Ansi::reset(),
            Ansi::bg(Color::Aqua),
            Ansi::fg(Color::Fg0),
            icons::HOURGLASS,
            remaining,
            Ansi::reset(),
        );

        Some(output)
    }
}
