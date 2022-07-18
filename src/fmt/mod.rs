#[cfg(feature = "lua")]
pub mod lua;

use std::{fs, str::FromStr};

use ansi_term::Colour::{Blue, Green, Red};

#[cfg(feature = "lua")]
use crate::fmt::lua::format_lua;
use crate::{
    config::{Config, Mode},
    error::Result,
};

/// Language choices
#[derive(Debug)]
pub enum Lang {
    Lua,
}

impl FromStr for Lang {
    type Err = &'static str;

    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        match s {
            "lua" => Ok(Self::Lua),
            _ => Err("language not supported"),
        }
    }
}

pub fn format_files(config: &Config) -> Result<bool> {
    let mut no_errors: bool = true;
    let colored_output = config.colored_output;
    let files = &config.files;

    for file in files {
        let file_str = format!("{}", file.display());
        let content =
            fs::read_to_string(file).map_err(|e| format!("'{}': {}", file.display(), e))?;

        let lang = Lang::from_str(config.language)?;
        let new_content = match lang {
            Lang::Lua => format_lua(&content, config)?,
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
                    no_errors = false;
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

    Ok(no_errors)
}
