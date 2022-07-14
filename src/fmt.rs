use regex::Regex;
use std::path::PathBuf;

use crate::util::{read_file, write_file};
use crate::Error;
use stylua_lib::{format_code, Config, OutputVerification};

pub fn format_str(content: String) -> Result<String, Error> {
    let re = Regex::new(
        r"(?xms)
           (?P<before>```lua\n)
           (?P<code>.*)
           (?P<after>```)
           ",
    )?;

    let caps = re.captures(&content).unwrap();
    let code = caps["code"].to_owned();
    let new_code = format_code(&code, Config::default(), None, OutputVerification::None).unwrap();
    let new_code_block = format!(
        "{}{}{}",
        caps["before"].to_owned(),
        new_code,
        caps["after"].to_owned()
    );
    let content = re.replace_all(&content, new_code_block);
    Ok(content.to_string())
}

pub fn format_file(filename: PathBuf) -> Result<(), Error> {
    let content = read_file(filename.clone())?;
    let new_content = format_str(content.clone())?;

    if content != new_content {
        println!("Formatting...");
        write_file(filename, new_content)?
    }

    Ok(())
}
