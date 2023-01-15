use std::{fs, path::PathBuf};

use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(
    name = "gelatyx",
    version,
    about = "Gelatyx 🦤. \nFormat codebease inside the docs",
    after_long_help = "Bugs can be reported on GitHub: https://github.com/azzamsa/gelatyx/issues"
)]
pub struct Opts {
    /// File(s) to format
    pub file: Vec<PathBuf>,

    /// Specify a file containings file(s) to format
    #[arg(long, conflicts_with("file"))]
    pub file_list: Option<PathBuf>,

    /// Language used in code block
    #[arg(short, long, value_enum)]
    pub language: Language,

    /// Specify an alternate configuration file
    #[arg(long)]
    pub language_config: Option<PathBuf>,

    /// Check if the docs has been formatted
    #[arg(long)]
    pub check: bool,

    /// Declare wnhen to use colors
    #[arg(
        short,
        long,
        value_enum,
        default_value_t = Color::Auto,
        help = "When to use colors",
        long_help,
    )]
    pub color: Color,
}

impl Opts {
    pub fn files(&self) -> miette::Result<Vec<PathBuf>> {
        let files = match &self.file_list {
            Some(file_list) => {
                let content = fs::read_to_string(file_list).unwrap();
                content.lines().map(PathBuf::from).collect::<Vec<PathBuf>>()
            }
            None => self.file.to_owned(),
        };
        Ok(files)
    }
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Language {
    Lua,
}

#[derive(Clone, ValueEnum)]
pub enum Color {
    /// show colors if the output goes to an interactive console (default)
    Auto,
    /// always use colorized output
    Always,
    /// do not use colorized output
    Never,
}

impl Color {
    pub fn as_str(&self) -> &'static str {
        match self {
            Color::Auto => "auto",
            Color::Never => "never",
            Color::Always => "always",
        }
    }
}
