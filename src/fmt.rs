use regex::Regex;
use std::path::Path;

use crate::util::{read_file, write_file};
use crate::Error;
use stylua_lib::{format_code, Config, OutputVerification};

pub fn format_str(content: &str) -> Result<String, Error> {
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

pub fn format_file(filename: &Path) -> Result<(), Error> {
    let content = read_file(filename)?;
    let new_content = format_str(&content)?;

    if content != new_content {
        println!("Formatting...");
        write_file(filename, new_content)?
    }

    Ok(())
}
