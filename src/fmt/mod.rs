pub mod lua;

use std::{fs, path::PathBuf};

use ansi_term::Colour::{Blue, Green, Red};

use crate::{
    cli::Language,
    config::{Config, Mode},
    exit_codes::ExitCode,
    fmt::lua::format_lua,
    Error,
};

pub fn format_files(config: &Config, files: Vec<PathBuf>) -> Result<ExitCode, Error> {
    let mut exit_code = ExitCode::Success;
    let colored_output = config.colored_output;

    for file in &files {
        let file_str = format!("{}", file.display());
        let content =
            fs::read_to_string(file).map_err(|e| format!("'{}': {}", file.display(), e))?;

        let new_content = match config.language {
            Language::Lua => format_lua(&content, config)?,
        };

        match config.mode {
            Mode::Format => {
                if content != new_content {
                    println!(
                        "Formatting {}",
                        if colored_output {
                            format!("{}", Green.paint(file_str))
                        } else {
                            file_str
                        }
                    );
                    fs::write(file, new_content)?;
                } else {
                    println!(
                        "Skipping {}",
                        if colored_output {
                            format!("{}", Blue.paint(file_str))
                        } else {
                            file_str
                        }
                    );
                }
            }
            Mode::Check => {
                if content != new_content {
                    eprintln!(
                        "{} is unformatted",
                        if colored_output {
                            format!("{}", Red.paint(file_str))
                        } else {
                            file_str
                        }
                    );
                    exit_code = ExitCode::GeneralError;
                } else {
                    println!(
                        "{} is formatted",
                        if colored_output {
                            format!("{}", Green.paint(file_str))
                        } else {
                            file_str
                        }
                    );
                }
            }
        }
    }

    Ok(exit_code)
}
