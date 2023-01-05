use std::path::PathBuf;

use crate::cli::Language;

#[derive(Debug, Clone, PartialEq)]
pub enum Mode {
    Format,
    Check,
}

#[derive(Debug, Clone)]
pub struct Config {
    /// The language to use
    pub language: Language,
    /// Whether or not the output should be colorized
    pub colored_output: bool,
    /// Format the docs or check only
    pub mode: Mode,
    /// Config file for the formatter
    pub language_config: Option<PathBuf>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            language: Language::Lua,
            colored_output: true,
            mode: Mode::Format,
            language_config: None,
        }
    }
}
