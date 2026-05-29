//! Language detection segment - displays detected programming languages.

use crate::colors::{Ansi, Color, PowerlineBuilder};
use crate::icons;
use crate::segments::{RenderContext, Segment};
use std::fmt::Write;
use std::fs;
use std::path::Path;
use std::process::Command;

/// A detected language with its icon and optional version.
#[derive(Debug)]
struct DetectedLanguage {
    icon: char,
    version: Option<String>,
}

/// Language segment showing detected programming languages and their versions.
///
/// Detection priority:
/// 1. Marker files (Cargo.toml, package.json, etc.)
/// 2. File extensions (*.rs, *.py, etc.)
pub struct LanguageSegment;

impl LanguageSegment {
    /// Check if a file exists in the directory.
    fn has_file(dir: &Path, name: &str) -> bool {
        dir.join(name).exists()
    }

    /// Check if a directory exists.
    fn has_dir(dir: &Path, name: &str) -> bool {
        dir.join(name).is_dir()
    }

    /// Check if any file with the given extension exists in the directory.
    fn has_extension(dir: &Path, extensions: &[&str]) -> bool {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                if let Some(ext) = entry.path().extension() {
                    let ext_str = ext.to_string_lossy();
                    if extensions.iter().any(|e| ext_str.eq_ignore_ascii_case(e)) {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Check if any file matches a glob pattern (e.g., "*.cabal").
    fn has_glob(dir: &Path, pattern: &str) -> bool {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let name = entry.file_name();
                let name = name.to_string_lossy();
                if let Some(suffix) = pattern.strip_prefix('*')
                    && name.ends_with(suffix)
                {
                    return true;
                }
            }
        }
        false
    }

    /// Get version from a command (e.g., "rustc --version").
    fn get_version(cmd: &str, args: &[&str], pattern: Option<&str>) -> Option<String> {
        let output = Command::new(cmd).args(args).output().ok()?;

        if !output.status.success() {
            return None;
        }

        let stdout = String::from_utf8_lossy(&output.stdout);

        // Extract version number using simple parsing
        pattern.map_or_else(
            || Self::extract_version(&stdout),
            |pat| {
                // Remove pattern prefix if present (e.g., "go" from "go1.21.0")
                let text = stdout.replace(pat, "");
                Self::extract_version(&text)
            },
        )
    }

    /// Extract a version number (X.Y.Z or X.Y) from text.
    fn extract_version(text: &str) -> Option<String> {
        for word in text.split_whitespace() {
            // Look for X.Y.Z or X.Y pattern
            let parts: Vec<&str> = word.trim_start_matches('v').split('.').collect();
            if parts.len() >= 2
                && parts[0].chars().all(|c| c.is_ascii_digit())
                && parts[1].chars().all(|c| c.is_ascii_digit())
            {
                if parts.len() >= 3 && parts[2].chars().take(3).all(|c| c.is_ascii_digit()) {
                    return Some(format!(
                        "{}.{}.{}",
                        parts[0],
                        parts[1],
                        parts[2]
                            .chars()
                            .take_while(char::is_ascii_digit)
                            .collect::<String>()
                    ));
                }
                return Some(format!("{}.{}", parts[0], parts[1]));
            }
        }
        None
    }

    /// Detect all languages in the given directory.
    #[allow(clippy::too_many_lines)]
    fn detect_languages(dir: &Path) -> Vec<DetectedLanguage> {
        let mut langs = Vec::new();

        // C (Makefile or .c/.h files)
        if Self::has_file(dir, "Makefile") || Self::has_extension(dir, &["c", "h"]) {
            langs.push(DetectedLanguage {
                icon: icons::C,
                version: None,
            });
        }

        // CMake
        if Self::has_file(dir, "CMakeLists.txt") || Self::has_extension(dir, &["cmake"]) {
            langs.push(DetectedLanguage {
                icon: icons::CMAKE,
                version: Self::get_version("cmake", &["--version"], None),
            });
        }

        // C++
        if Self::has_extension(dir, &["cpp", "hpp", "cc", "cxx"]) {
            langs.push(DetectedLanguage {
                icon: icons::CPP,
                version: None,
            });
        }

        // Deno
        if Self::has_file(dir, "deno.json") || Self::has_file(dir, "deno.jsonc") {
            langs.push(DetectedLanguage {
                icon: icons::DENO,
                version: Self::get_version("deno", &["--version"], None),
            });
        }

        // Elixir
        if Self::has_file(dir, "mix.exs") || Self::has_extension(dir, &["ex", "exs"]) {
            langs.push(DetectedLanguage {
                icon: icons::ELIXIR,
                version: Self::get_version("elixir", &["--version"], None),
            });
        }

        // Go
        if Self::has_file(dir, "go.mod") || Self::has_extension(dir, &["go"]) {
            langs.push(DetectedLanguage {
                icon: icons::GO,
                version: Self::get_version("go", &["version"], Some("go")),
            });
        }

        // Gradle
        if Self::has_file(dir, "build.gradle") || Self::has_file(dir, "build.gradle.kts") {
            langs.push(DetectedLanguage {
                icon: icons::GRADLE,
                version: Self::get_version("gradle", &["--version"], None),
            });
        }

        // Haskell
        if Self::has_glob(dir, "*.cabal")
            || Self::has_file(dir, "stack.yaml")
            || Self::has_extension(dir, &["hs"])
        {
            langs.push(DetectedLanguage {
                icon: icons::HASKELL,
                version: Self::get_version("ghc", &["--version"], None),
            });
        }

        // Java
        if Self::has_file(dir, "pom.xml") || Self::has_extension(dir, &["java"]) {
            langs.push(DetectedLanguage {
                icon: icons::JAVA,
                version: Self::get_version("java", &["--version"], None),
            });
        }

        // Kotlin
        if Self::has_extension(dir, &["kt", "kts"]) {
            langs.push(DetectedLanguage {
                icon: icons::KOTLIN,
                version: Self::get_version("kotlin", &["-version"], None),
            });
        }

        // Lua
        if Self::has_extension(dir, &["lua"]) {
            langs.push(DetectedLanguage {
                icon: icons::LUA,
                version: Self::get_version("lua", &["-v"], None),
            });
        }

        // Nix
        if Self::has_file(dir, "flake.nix")
            || Self::has_file(dir, "shell.nix")
            || Self::has_extension(dir, &["nix"])
        {
            langs.push(DetectedLanguage {
                icon: icons::NIX,
                version: None,
            });
        }

        // Node.js
        if Self::has_file(dir, "package.json") || Self::has_dir(dir, "node_modules") {
            langs.push(DetectedLanguage {
                icon: icons::NODEJS,
                version: Self::get_version("node", &["--version"], None),
            });
        }

        // OCaml
        if Self::has_extension(dir, &["ml", "mli"]) {
            langs.push(DetectedLanguage {
                icon: icons::OCAML,
                version: Self::get_version("ocaml", &["-version"], None),
            });
        }

        // Perl
        if Self::has_extension(dir, &["pl", "pm"]) {
            langs.push(DetectedLanguage {
                icon: icons::PERL,
                version: Self::get_version("perl", &["--version"], None),
            });
        }

        // PHP
        if Self::has_file(dir, "composer.json") || Self::has_extension(dir, &["php"]) {
            langs.push(DetectedLanguage {
                icon: icons::PHP,
                version: Self::get_version("php", &["--version"], None),
            });
        }

        // Python
        if Self::has_file(dir, "pyproject.toml")
            || Self::has_file(dir, "requirements.txt")
            || Self::has_extension(dir, &["py"])
        {
            langs.push(DetectedLanguage {
                icon: icons::PYTHON,
                version: Self::get_version("python", &["--version"], None),
            });
        }

        // Ruby
        if Self::has_file(dir, "Gemfile") || Self::has_extension(dir, &["rb"]) {
            langs.push(DetectedLanguage {
                icon: icons::RUBY,
                version: Self::get_version("ruby", &["--version"], None),
            });
        }

        // Rust
        if Self::has_file(dir, "Cargo.toml") || Self::has_extension(dir, &["rs"]) {
            langs.push(DetectedLanguage {
                icon: icons::RUST,
                version: Self::get_version("rustc", &["--version"], None),
            });
        }

        // Scala
        if Self::has_file(dir, "build.sbt") || Self::has_extension(dir, &["scala"]) {
            langs.push(DetectedLanguage {
                icon: icons::SCALA,
                version: Self::get_version("scala", &["--version"], None),
            });
        }

        // Swift
        if Self::has_file(dir, "Package.swift") || Self::has_extension(dir, &["swift"]) {
            langs.push(DetectedLanguage {
                icon: icons::SWIFT,
                version: Self::get_version("swift", &["--version"], None),
            });
        }

        // Zig
        if Self::has_extension(dir, &["zig"]) {
            langs.push(DetectedLanguage {
                icon: icons::ZIG,
                version: Self::get_version("zig", &["version"], None),
            });
        }

        langs
    }
}

impl Segment for LanguageSegment {
    fn render(&self, ctx: &RenderContext<'_>) -> Option<String> {
        let langs = Self::detect_languages(ctx.current_dir);

        if langs.is_empty() {
            // Just output the transition arrow from orange to nothing
            return Some(format!(
                "{}{}{}",
                Ansi::fg(Color::Orange),
                PowerlineBuilder::ARROW,
                Ansi::reset()
            ));
        }

        // Build content: icon + version for each language
        let mut content = String::new();
        for lang in &langs {
            if let Some(ref version) = lang.version {
                let _ = write!(content, " {} {}", lang.icon, version);
            } else {
                let _ = write!(content, " {}", lang.icon);
            }
        }

        // Transition from orange (directory) to bg1 (language), then end
        let output = format!(
            "{}{}{}{}{}{}{}{}{}",
            Ansi::fg(Color::Orange),
            Ansi::bg(Color::Bg1),
            PowerlineBuilder::ARROW,
            Ansi::reset(),
            Ansi::bg(Color::Bg1),
            Ansi::fg(Color::Fg0),
            content,
            " ",
            Ansi::reset(),
        );

        Some(format!(
            "{}{}{}{}",
            output,
            Ansi::fg(Color::Bg1),
            PowerlineBuilder::ARROW,
            Ansi::reset()
        ))
    }
}
