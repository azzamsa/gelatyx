use std::str::FromStr;

use regex::{Captures, Regex};
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
           (?P<before>^```lua\n)
           (?P<code>.*?)
           (?P<after>^```$)
           ",
    )?;

    let new_content = re.replace_all(content, |capture: &Captures<'_>| {
        let code = &capture["code"];
        let new_code = format_code(code, Config::default(), None, OutputVerification::None)
            .unwrap_or_else(|_| "".into());
        let new_code_block = format!("{}{}{}", &capture["before"], new_code, &capture["after"]);
        new_code_block
    });

    Ok(new_content.to_string())
}
