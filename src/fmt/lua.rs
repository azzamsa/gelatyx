use regex::Regex;
use std::path::Path;
use std::str::FromStr;

use crate::util::{read_file, write_file};
use crate::Error;

#[cfg(feature = "lua")]
use stylua_lib::{format_code, Config, OutputVerification};

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

#[cfg(feature = "lua")]
pub fn format_lua(content: &str) -> Result<String, Error> {
    let re = Regex::new(
        r"(?xms)
           (?P<before>```lua\n)
           (?P<code>.*)
           (?P<after>```)
           ",
    )?;

    let caps = re.captures(content).unwrap();
    let code = &caps["code"];
    let new_code = format_code(code, Config::default(), None, OutputVerification::None).unwrap();
    let new_code_block = format!("{}{}{}", &caps["before"], new_code, &caps["after"]);
    let content = re.replace_all(content, new_code_block);
    Ok(content.to_string())
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
