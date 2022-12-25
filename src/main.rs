#![deny(unsafe_code)]
use std::process;

use anyhow::Result;
use gelatyx::{app::App, exit_codes::ExitCode, fmt};

fn main() {
    let result = run();
    match result {
        Ok(exit_code) => {
            process::exit(exit_code.into());
        }
        Err(err) => {
            eprintln!("[gelatyx error]: {:#}", err);
            process::exit(ExitCode::GeneralError.into());
        }
    }
}

fn run() -> Result<ExitCode> {
    let app = App::new();
    let config = app.config()?;
    let exit_code = fmt::format_files(&config)?;

    Ok(exit_code)
}
