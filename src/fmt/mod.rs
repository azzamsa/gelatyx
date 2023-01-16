pub mod lua;

use std::ffi::OsStr;
use std::io::{self, Write};
use std::path::Path;
use std::{fs, path::PathBuf};

use miette::{NamedSource, SourceOffset};
use owo_colors::{
    OwoColorize,
    Stream::{Stderr, Stdout},
};

use crate::{
    cli::Language,
    config::{Config, Mode},
    fmt::lua::format_lua,
    Error,
};

/// The status containing the result of desired formatter
pub enum FormatResult {
    /// Contains a formatted content
    Formatted(String),
    Unchanged,
    /// code block , and the error message
    /// (line, column), code block, error
    InvalidSyntax(SyntaxError),
}

pub struct SyntaxError {
    /// Contains (line, column) pair of the position of the error
    position: Option<(usize, usize)>,
    /// Code block where the error occured
    code_block: String,
    /// Full error message
    message: String,
    /// Short summary of the error message
    summary: String,
}

/// Enum helper to count the final status and
/// deciced what exit code to emit
#[derive(PartialEq)]
pub enum FormatStatus {
    Formatted,
    Unchanged,
    Failed,
}

pub fn format_files<P>(config: &Config, filename: P) -> Result<FormatStatus, Error>
where
    P: AsRef<Path> + AsRef<OsStr>,
{
    let mut format_status = FormatStatus::Formatted;

    let unformatted_content = fs::read_to_string(&filename).map_err(|_| Error::FileNotFound {
        path: PathBuf::from(&filename),
    })?;
    let file_str = format!("{}", Path::new(&filename).display());

    let format_result = match config.language {
        Language::Lua => format_lua(&unformatted_content, config)?,
    };

    match config.mode {
        Mode::Format => match format_result {
            FormatResult::Formatted(ref formatted_content) => {
                writeln!(
                    io::stdout(),
                    "{}",
                    format!(
                        "{} {}",
                        "Formatted",
                        &file_str.if_supports_color(Stdout, |text| text.green()),
                    )
                    .if_supports_color(Stdout, |text| text.bold()),
                )
                .ok();
                fs::write(file_str, formatted_content)?;
            }
            FormatResult::InvalidSyntax(SyntaxError {
                position,
                code_block,
                message,
                summary,
            }) => {
                writeln!(
                    io::stderr(),
                    "{}",
                    format!(
                        "{} {}",
                        "Can't format",
                        &file_str.if_supports_color(Stderr, |text| text.red())
                    )
                    .if_supports_color(Stdout, |text| text.bold()),
                )
                .ok();

                let bad_bit = if let Some(position) = position {
                    let (line, col) = position;
                    Some(SourceOffset::from_location(&code_block, line, col))
                } else {
                    None
                };
                Err(Error::InvalidSyntax {
                    src: NamedSource::new(file_str, code_block),
                    bad_bit,
                    summary,
                    message,
                })?;
                format_status = FormatStatus::Failed;
            }
            FormatResult::Unchanged => {
                writeln!(
                    io::stdout(),
                    "{}",
                    format!(
                        "{} {}",
                        "Unchanged",
                        &file_str.if_supports_color(Stdout, |text| text.green())
                    )
                    .if_supports_color(Stdout, |text| text.bold()),
                )
                .ok();
                format_status = FormatStatus::Unchanged;
            }
        },
        Mode::Check => match format_result {
            FormatResult::Formatted(ref _formatted_content) => {
                writeln!(
                    io::stderr(),
                    "{}",
                    format!(
                        "{} {}",
                        &file_str.if_supports_color(Stderr, |text| text.red()),
                        "is unformatted",
                    )
                    .if_supports_color(Stdout, |text| text.bold()),
                )
                .ok();
            }
            FormatResult::Unchanged => {
                writeln!(
                    io::stdout(),
                    "{}",
                    format!(
                        "{} {}",
                        &file_str.if_supports_color(Stdout, |text| text.green()),
                        "is formatted",
                    )
                    .if_supports_color(Stdout, |text| text.bold()),
                )
                .ok();
                format_status = FormatStatus::Unchanged;
            }
            FormatResult::InvalidSyntax(SyntaxError {
                position,
                code_block,
                message,
                summary,
            }) => {
                writeln!(
                    io::stderr(),
                    "{}",
                    format!(
                        "{} {}",
                        "Can't check",
                        &file_str.if_supports_color(Stdout, |text| text.red())
                    )
                    .if_supports_color(Stdout, |text| text.bold()),
                )
                .ok();

                let bad_bit = if let Some(position) = position {
                    let (line, col) = position;
                    Some(SourceOffset::from_location(&code_block, line, col))
                } else {
                    None
                };
                Err(Error::InvalidSyntax {
                    src: NamedSource::new(file_str, code_block),
                    bad_bit,
                    summary,
                    message,
                })?;
                format_status = FormatStatus::Failed;
            }
        },
    }

    Ok(format_status)
}
