#![deny(unsafe_code)]
use std::process;

use anyhow::Result;
use gelatyx::{app::App, fmt};

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {:?}", err);
        process::exit(1);
    }
}

fn run() -> Result<()> {
    let app = App::new();
    let config = app.config()?;
    fmt::format_files(&config)?;

    Ok(())
}
