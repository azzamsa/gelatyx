pub mod lua;

use std::ffi::OsStr;
use std::path::Path;
use std::{fs, path::PathBuf};

use miette::{NamedSource, SourceOffset};
use owo_colors::{
    OwoColorize,
    Stream::{Stderr, Stdout},
    Style,
};

use crate::{
    cli::Language,
    config::{Config, Mode},
    epaint,
    fmt::lua::format_lua,
    output::{stderr, stdout},
    paint, Error,
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
    let style = Style::new();
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
                stdout(&paint!(
                    &format!("{} {}", "Formatted", paint!(&file_str, style.green())),
                    style.bold()
                ));
                fs::write(file_str, formatted_content)?;
            }
            FormatResult::InvalidSyntax(SyntaxError {
                position,
                code_block,
                message,
                summary,
            }) => {
                stderr(&epaint!(
                    &format!("{} {}", "Can't Format", epaint!(&file_str, style.red())),
                    style.bold()
                ));

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
                if config.is_verbose {
                    stdout(&paint!(
                        &format!("{} {}", "Unchanged", paint!(&file_str, style.green())),
                        style.bold()
                    ));
                }
                format_status = FormatStatus::Unchanged;
            }
        },
        Mode::Check => match format_result {
            FormatResult::Formatted(ref _formatted_content) => {
                // String after `paint` loses its style.
                // It must be multiple calls to `paint`
                stderr(&format!(
                    "{} {}",
                    epaint!(&file_str, style.red().bold()),
                    epaint!("is unformatted", style.bold())
                ));
            }
            FormatResult::Unchanged => {
                if config.is_verbose {
                    stdout(&format!(
                        "{} {}",
                        paint!(&file_str, style.green().bold()),
                        epaint!("is formatted", style.bold())
                    ));
                }
                format_status = FormatStatus::Unchanged;
            }
            FormatResult::InvalidSyntax(SyntaxError {
                position,
                code_block,
                message,
                summary,
            }) => {
                stderr(&epaint!(
                    &format!("{} {}", "Can't check", epaint!(&file_str, style.red())),
                    style.bold()
                ));

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
