use anyhow::{bail, Result};
use std::env;
use std::path::Path;
use std::process;

use gelatyx::app;
use gelatyx::fmt::format_file;

fn run() -> Result<()> {
    let matches = app::build().get_matches_from(env::args_os());

    let files: Vec<&Path> = match matches.values_of("file") {
        Some(files) => files
            .into_iter()
            .map(Path::new)
            // .take_while(|f| f.exists())
            // Filtering only existing fails can't fails the program if
            // only one file passed
            .collect(),
        None => bail!("No file supplied"),
    };

    let lang = match matches.value_of("lang") {
        Some(lang) => lang,
        None => bail!("No language supplied"),
    };

    for file in files {
        if file.exists() {
            format_file(file, lang)?;
        } else {
            bail!("No such file")
        }
    }

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {:?}", err);
        process::exit(1);
    }
}
