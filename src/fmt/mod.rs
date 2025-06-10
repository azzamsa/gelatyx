pub mod lua;

use std::ffi::OsStr;
use std::path::Path;
use std::{fs, path::PathBuf};

use full_moon::tokenizer;

use miette::{NamedSource, SourceOffset, SourceSpan};
use owo_colors::{
    OwoColorize,
    Stream::{Stderr, Stdout},
    Style,
};

use crate::{
    Error,
    cli::Language,
    config::{Config, Mode},
    epaint,
    fmt::lua::format_lua,
    output::{stderr, stdout},
    paint,
};

/// The status containing the result of desired formatter
pub enum FormatResult {
    /// Contains a formatted content
    Formatted(String),
    Unchanged,
    /// code block , and the error message
    /// (line, column), code block, error
    InvalidSyntax(Vec<SyntaxError>),
}

pub struct SyntaxError {
    /// Contains (line, column) pair of the position of the error
    position: (tokenizer::Position, tokenizer::Position),
    /// Code block where the error occurred
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
            FormatResult::InvalidSyntax(errors) => {
                stderr(&epaint!(
                    &format!("{} {}", "Can't Format", epaint!(&file_str, style.red())),
                    style.bold()
                ));
                parse_invalid_syntax(file_str, errors)?;
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
            FormatResult::InvalidSyntax(errors) => {
                stderr(&epaint!(
                    &format!("{} {}", "Can't check", epaint!(&file_str, style.red())),
                    style.bold()
                ));
                parse_invalid_syntax(file_str, errors)?;
                format_status = FormatStatus::Failed;
            }
        },
    }

    Ok(format_status)
}

fn parse_invalid_syntax(file_str: String, errors: Vec<SyntaxError>) -> Result<(), Error> {
    if let Some(e) = errors.into_iter().next() {
        let length = e.position.1.bytes() - e.position.0.bytes();
        let bad_bit = SourceSpan::new(
            SourceOffset::from_location(
                e.code_block.clone(),
                e.position.0.line(),
                e.position.0.character(),
            ),
            length,
        );
        Err(Error::InvalidSyntax {
            src: NamedSource::new(&file_str, e.code_block),
            bad_bit,
            summary: e.summary,
            message: e.message,
        })?;
    }
    Ok(())
}
