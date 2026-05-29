//! Gruvbox Dark color palette with ANSI escape sequence generation.
//!
//! Provides type-safe color handling for terminal output with powerline-style
//! segment transitions.

use std::fmt::{self, Display, Write};

/// RGB color value.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rgb {
    pub b: u8,
    pub g: u8,
    pub r: u8,
}

impl Rgb {
    #[must_use]
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { b, g, r }
    }
}

/// Gruvbox Dark color palette.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Color {
    /// #d65d0e - Orange
    Aqua,
    /// #3c3836 - Dark background
    Bg1,
    /// #665c54 - Medium gray
    Bg3,
    /// #458588 - Blue
    Blue,
    /// #fbf1c7 - Light cream (default foreground)
    Fg0,
    Orange,
    /// #d79921 - Yellow
    Yellow,
}

impl Color {
    /// Get the RGB values for this color.
    #[must_use]
    pub const fn rgb(self) -> Rgb {
        match self {
            Self::Aqua => Rgb::new(104, 157, 106),  // #689d6a
            Self::Bg1 => Rgb::new(60, 56, 54),      // #3c3836
            Self::Bg3 => Rgb::new(102, 92, 84),     // #665c54
            Self::Blue => Rgb::new(69, 133, 136),   // #458588
            Self::Fg0 => Rgb::new(251, 241, 199),   // #fbf1c7
            Self::Orange => Rgb::new(214, 93, 14),  // #d65d0e
            Self::Yellow => Rgb::new(215, 153, 33), // #d79921
        }
    }
}

/// ANSI escape sequence generator.
pub struct Ansi;

impl Ansi {
    /// Generate background color escape sequence.
    #[must_use]
    pub fn bg(color: Color) -> String {
        let rgb = color.rgb();
        format!("\x1b[48;2;{};{};{}m", rgb.r, rgb.g, rgb.b)
    }

    /// Generate foreground color escape sequence.
    #[must_use]
    pub fn fg(color: Color) -> String {
        let rgb = color.rgb();
        format!("\x1b[38;2;{};{};{}m", rgb.r, rgb.g, rgb.b)
    }

    /// Reset all formatting.
    #[must_use]
    pub const fn reset() -> &'static str {
        "\x1b[0m"
    }
}

/// A styled segment with foreground and background colors.
#[derive(Clone, Debug)]
pub struct Style {
    pub bg: Option<Color>,
    pub fg: Option<Color>,
}

impl Style {
    #[must_use]
    pub const fn new() -> Self {
        Self { bg: None, fg: None }
    }

    #[must_use]
    pub const fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }

    #[must_use]
    pub const fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }

    /// Generate ANSI escape sequence for this style.
    #[must_use]
    pub fn to_ansi(&self) -> String {
        let mut result = String::new();
        if let Some(fg) = self.fg {
            result.push_str(&Ansi::fg(fg));
        }
        if let Some(bg) = self.bg {
            result.push_str(&Ansi::bg(bg));
        }
        result
    }
}

impl Default for Style {
    fn default() -> Self {
        Self::new()
    }
}

/// Powerline segment builder for clean segment transitions.
pub struct PowerlineBuilder {
    output: String,
    current_bg: Option<Color>,
}

impl PowerlineBuilder {
    /// Powerline arrow (right-pointing).
    pub const ARROW: char = '\u{e0b0}';
    /// Rounded left cap.
    pub const ARROW_LEFT: char = '\u{e0b6}';

    #[must_use]
    pub const fn new() -> Self {
        Self {
            current_bg: None,
            output: String::new(),
        }
    }

    /// Start a new segment with a rounded left cap (for first segment of a line).
    #[must_use]
    pub fn start_rounded(mut self, bg: Color, fg: Color) -> Self {
        // Left cap: foreground = segment bg color, no background
        write!(
            self.output,
            "{}{}{}{}",
            Ansi::reset(),
            Ansi::fg(bg),
            Self::ARROW_LEFT,
            Ansi::reset()
        )
        .unwrap();
        // Start the segment
        write!(self.output, "{}{}", Ansi::bg(bg), Ansi::fg(fg)).unwrap();
        self.current_bg = Some(bg);
        self
    }

    /// Add content to the current segment.
    #[must_use]
    pub fn content(mut self, text: &str) -> Self {
        write!(self.output, " {text} ").unwrap();
        self
    }

    /// Transition to a new segment with powerline arrow.
    #[must_use]
    #[allow(clippy::similar_names)]
    pub fn transition(mut self, new_bg: Color, new_fg: Color) -> Self {
        if let Some(prev_bg) = self.current_bg {
            // Arrow: fg = previous bg, bg = new bg
            write!(
                self.output,
                "{}{}{}{}",
                Ansi::reset(),
                Ansi::fg(prev_bg),
                Ansi::bg(new_bg),
                Self::ARROW
            )
            .unwrap();
        }
        // Continue with new colors
        write!(
            self.output,
            "{}{}{}",
            Ansi::reset(),
            Ansi::bg(new_bg),
            Ansi::fg(new_fg)
        )
        .unwrap();
        self.current_bg = Some(new_bg);
        self
    }

    /// End the current line with a trailing arrow.
    #[must_use]
    pub fn end(mut self) -> Self {
        if let Some(prev_bg) = self.current_bg {
            write!(
                self.output,
                "{}{}{}",
                Ansi::reset(),
                Ansi::fg(prev_bg),
                Self::ARROW
            )
            .unwrap();
        }
        write!(self.output, "{}", Ansi::reset()).unwrap();
        self.current_bg = None;
        self
    }

    /// End the current segment without a trailing arrow (for plain transitions).
    #[must_use]
    pub fn end_plain(mut self) -> Self {
        write!(self.output, "{}", Ansi::reset()).unwrap();
        self.current_bg = None;
        self
    }

    /// Get the final output string.
    #[must_use]
    pub fn build(self) -> String {
        self.output
    }
}

impl Default for PowerlineBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for PowerlineBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_rgb() {
        let orange = Color::Orange.rgb();
        assert_eq!(orange.r, 214);
        assert_eq!(orange.g, 93);
        assert_eq!(orange.b, 14);
    }

    #[test]
    fn test_ansi_sequences() {
        let fg = Ansi::fg(Color::Orange);
        assert!(fg.contains("38;2;214;93;14"));

        let bg = Ansi::bg(Color::Orange);
        assert!(bg.contains("48;2;214;93;14"));
    }

    #[test]
    fn test_powerline_builder() {
        let output = PowerlineBuilder::new()
            .start_rounded(Color::Orange, Color::Fg0)
            .content("test")
            .end()
            .build();

        assert!(output.contains('\u{e0b6}')); // Left cap
        assert!(output.contains('\u{e0b0}')); // Right arrow
        assert!(output.contains("test"));
    }
}
