use std::str::FromStr;

use regex::Regex;
#[cfg(feature = "lua")]
use stylua_lib::{format_code, Config, OutputVerification};

use crate::Error;

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

    match re.captures(content) {
        Some(caps) => {
            let code = &caps["code"];
            let new_code = format_code(code, Config::default(), None, OutputVerification::None)?;
            let new_code_block = format!("{}{}{}", &caps["before"], new_code, &caps["after"]);
            let new_content = re.replace_all(content, new_code_block);
            Ok(new_content.to_string())
        }
        None => Ok(content.into()),
    }
}
