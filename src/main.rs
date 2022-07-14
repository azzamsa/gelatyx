use anyhow::{bail, Result};
use std::env;
use std::path::Path;
use std::process;

use gelatyx::app;
use gelatyx::fmt::format_file;
use gelatyx::util::is_exist;

fn run() -> Result<()> {
    let matches = app::build().get_matches_from(env::args_os());

    let path: &Path = match matches.value_of("path") {
        Some(path) => {
            if is_exist(path) {
                Path::new(path)
            } else {
                bail!("No such file")
            }
        }
        None => bail!("No file supplied"),
    };

    format_file(path)?;

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {:?}", err);
        process::exit(1);
    }
}
