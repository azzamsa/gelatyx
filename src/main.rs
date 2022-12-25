#![deny(unsafe_code)]
use std::process;

use atty::Stream;
use clap::Parser;
use miette::Result;

use gelatyx::{
    cli::{Color, Opts},
    config::{Config, Mode},
    exit_codes::ExitCode,
    fmt,
};

fn main() {
    let result = run();
    match result {
        Ok(exit_code) => {
            process::exit(exit_code.into());
        }
        Err(err) => {
            eprintln!("Error: {:?}", err);
            process::exit(ExitCode::GeneralError.into());
        }
    }
}

fn run() -> Result<ExitCode> {
    let opts = Opts::parse();

    let files = opts.file.clone();
    let config = construct_config(opts);
    let exit_code = fmt::format_files(&config, files.to_vec())?;

    Ok(exit_code)
}

fn construct_config(opts: Opts) -> Config {
    let colored_output = match opts.color {
        Color::Always => true,
        Color::Never => false,
        Color::Auto => atty::is(Stream::Stdout),
    };
    let mode = match opts.check {
        true => Mode::Check,
        false => Mode::Format,
    };

    Config {
        language: opts.language,
        colored_output,
        mode,
        language_config: opts.language_config,
    }
}
