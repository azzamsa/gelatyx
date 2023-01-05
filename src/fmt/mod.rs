pub mod lua;

use std::ffi::OsStr;
use std::io::{self, Write};
use std::path::Path;
use std::{fs, path::PathBuf};

use miette::{NamedSource, SourceOffset};
use owo_colors::AnsiColors::{Green, Red};
use owo_colors::{AnsiColors, OwoColorize};

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
    let colored_output = config.colored_output;

    let unformatted_content = fs::read_to_string(&filename).map_err(|_| Error::FileNotFound {
        path: PathBuf::from(&filename),
    })?;
    let file_str = format!("{}", Path::new(&filename).display());

    let format_result = match config.language {
        Language::Lua => format_lua(&unformatted_content, config)?,
    };

    let red_filename = paint(&file_str, Red, colored_output);
    let green_filename = paint(&file_str, Green, colored_output);

    match config.mode {
        Mode::Format => match format_result {
            FormatResult::Formatted(ref formatted_content) => {
                let msg = format!("Formatted {}", green_filename).bold().to_string();
                writeln!(io::stdout(), "{}", msg).ok();
                fs::write(file_str, formatted_content)?;
            }
            FormatResult::InvalidSyntax(SyntaxError {
                position,
                code_block,
                message,
                summary,
            }) => {
                let msg = format!("Can't format {}", red_filename).bold().to_string();
                writeln!(io::stderr(), "{}", msg).ok();

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
                let msg = format!("Unchanged {}", green_filename).bold().to_string();
                writeln!(io::stdout(), "{}", msg).ok();
                format_status = FormatStatus::Unchanged;
            }
        },
        Mode::Check => match format_result {
            FormatResult::Formatted(ref _formatted_content) => {
                let msg = format!("{} is unformatted", red_filename)
                    .bold()
                    .to_string();
                writeln!(io::stderr(), "{}", msg).ok();
            }
            FormatResult::Unchanged => {
                let msg = format!("{} is formatted", green_filename)
                    .bold()
                    .to_string();
                writeln!(io::stdout(), "{}", msg).ok();
                format_status = FormatStatus::Unchanged;
            }
            FormatResult::InvalidSyntax(SyntaxError {
                position,
                code_block,
                message,
                summary,
            }) => {
                let msg = format!("Can't check {}", red_filename).bold().to_string();
                writeln!(io::stderr(), "{}", msg).ok();

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

/// Colorize filename output
fn paint(input: &str, color: AnsiColors, is_colored: bool) -> String {
    if !is_colored {
        return input.to_string();
    };

    format!("{}", input.color(color))
}
