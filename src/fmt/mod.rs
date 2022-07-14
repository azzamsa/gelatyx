#[cfg(feature = "lua")]
pub mod lua;

use std::path::Path;
use std::str::FromStr;

use crate::util::{read_file, write_file};
use crate::Error;

#[cfg(feature = "lua")]
use crate::fmt::lua::format_lua;

/// Language choices
#[derive(Debug)]
pub enum Lang {
    Lua,
}

impl FromStr for Lang {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "lua" => Ok(Self::Lua),
            _ => Err("language not supported"),
        }
    }
}

pub fn format_file(filename: &Path, lang: &str) -> Result<(), Error> {
    let content = read_file(filename)?;

    let lang = Lang::from_str(lang)?;
    let new_content = match lang {
        Lang::Lua => format_lua(&content)?,
    };

    if content != new_content {
        println!("Formatting...");
        write_file(filename, new_content)?
    }

    Ok(())
}
