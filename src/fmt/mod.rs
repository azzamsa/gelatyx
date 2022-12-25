pub mod lua;

use std::{fs, path::PathBuf};

use owo_colors::AnsiColors::{Green, Red};
use owo_colors::{AnsiColors, OwoColorize};

use crate::{
    cli::Language,
    config::{Config, Mode},
    exit_codes::ExitCode,
    fmt::lua::format_lua,
    Error,
};

pub enum FormatResult {
    Success(String),
    InvalidSyntax(String),
}

pub fn format_files(config: &Config, files: Vec<PathBuf>) -> Result<ExitCode, Error> {
    let mut exit_code = ExitCode::Success;
    let colored_output = config.colored_output;

    for file in &files {
        let unformatted_content = fs::read_to_string(file).map_err(|_| Error::FileNotFound {
            path: file.to_path_buf(),
        })?;
        let file = format!("{}", file.display());

        let format_result = match config.language {
            Language::Lua => format_lua(&unformatted_content, config)?,
        };

        let red_filename = paint(&file, Red, colored_output);
        let green_filename = paint(&file, Green, colored_output);

        match config.mode {
            Mode::Format => match format_result {
                FormatResult::Success(formatted_content) => {
                    println!("Formatting {}", green_filename);
                    fs::write(file, formatted_content)?;
                }
                FormatResult::InvalidSyntax(error) => {
                    eprintln!("Can't format {}", red_filename);
                    eprintln!("{}\n", error);
                    exit_code = ExitCode::GeneralError;
                }
            },
            Mode::Check => match format_result {
                FormatResult::Success(formatted_content) => {
                    if unformatted_content != formatted_content {
                        eprintln!("{} is unformatted", red_filename);
                        exit_code = ExitCode::GeneralError;
                    } else {
                        println!("{} is formatted", green_filename);
                    }
                }
                FormatResult::InvalidSyntax(error) => {
                    eprintln!("Can't check {}", red_filename);
                    eprintln!("{}\n", error);
                    exit_code = ExitCode::GeneralError;
                }
            },
        }
    }

    Ok(exit_code)
}

/// Colorize filename output
fn paint(input: &str, color: AnsiColors, is_colored: bool) -> String {
    if !is_colored {
        return input.to_string();
    };

    format!("{}", input.color(color))
}
