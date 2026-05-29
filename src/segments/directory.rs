//! Directory segment - displays the current working directory.

use crate::colors::{Ansi, Color, PowerlineBuilder};
use crate::segments::{RenderContext, Segment};
use std::path::Path;

/// Maximum number of path components to display.
const MAX_COMPONENTS: usize = 3;

/// Directory segment showing the current working directory.
///
/// - Replaces home directory with `~`
/// - Truncates to last 3 components with `...` prefix
/// - Displays with orange background
pub struct DirectorySegment;

impl DirectorySegment {
    /// Truncate a path to the last N components.
    fn truncate_path(path: &Path) -> String {
        // Convert to string and replace home with ~
        let path_str = path.to_string_lossy();
        let home = dirs::home_dir()
            .map(|h| h.to_string_lossy().to_string())
            .unwrap_or_default();

        let display_path = if path_str.starts_with(&home) {
            format!("~{}", &path_str[home.len()..])
        } else {
            path_str.to_string()
        };

        // Split and truncate
        let components: Vec<&str> = display_path.split('/').filter(|s| !s.is_empty()).collect();

        if components.len() > MAX_COMPONENTS {
            format!(
                ".../{}",
                components[components.len() - MAX_COMPONENTS..].join("/")
            )
        } else {
            display_path
        }
    }
}

impl Segment for DirectorySegment {
    fn render(&self, ctx: &RenderContext<'_>) -> Option<String> {
        let path = Self::truncate_path(ctx.current_dir);

        let output = PowerlineBuilder::new()
            .start_rounded(Color::Orange, Color::Fg0)
            .content(&path)
            .end_plain()
            .build();

        Some(format!("{}{}", output, Ansi::reset()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truncate_short_path() {
        let path = Path::new("/usr/bin");
        let result = DirectorySegment::truncate_path(path);
        assert_eq!(result, "/usr/bin");
    }

    #[test]
    fn test_truncate_long_path() {
        let path = Path::new("/home/user/projects/deep/nested/path");
        let result = DirectorySegment::truncate_path(path);
        assert!(result.starts_with("..."));
        assert!(result.contains("nested/path"));
    }
}
