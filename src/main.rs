use anyhow::Result;
use std::env;
use std::path::PathBuf;
use std::process;

use gelatyx::app;
use gelatyx::fmt::format_file;
use gelatyx::util::is_exist;

fn run() -> Result<()> {
    let matches = app::build().get_matches_from(env::args_os());

    let path: PathBuf = match matches.value_of("path") {
        Some(path) => is_exist(path)?,
        None => anyhow::bail!("No file supplied"),
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
