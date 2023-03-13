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
    /// Format the docs or check only
    pub mode: Mode,
    /// Config file for the formatter
    pub language_config: Option<PathBuf>,
    /// Vebosity option
    pub is_verbose: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            language: Language::Lua,
            mode: Mode::Format,
            language_config: None,
            is_verbose: false,
        }
    }
}
