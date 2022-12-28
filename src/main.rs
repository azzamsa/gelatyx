#![deny(unsafe_code)]
use std::io::{self, Write};
use std::process;

use atty::Stream;
use clap::Parser;
use miette::{Context, Result};
use owo_colors::OwoColorize;

use gelatyx::{
    cli::{Color, Opts},
    config::{Config, Mode},
    exit_codes::ExitCode,
    fmt::{self, FormatStatus},
};

fn main() {
    let result = run();
    match result {
        Ok(exit_code) => {
            process::exit(exit_code.into());
        }
        Err(err) => {
            writeln!(io::stderr(), "Error: {:?}", err).ok();
            process::exit(ExitCode::GeneralError.into());
        }
    }
}

fn run() -> Result<ExitCode> {
    let mut statuses: Vec<FormatStatus> = Vec::new();
    let opts = Opts::parse();

    let files = opts.file.clone();
    let config = construct_config(opts);
    for file in files {
        match fmt::format_files(&config, &file).context("") {
            Ok(status) => {
                statuses.push(status);
            }
            Err(e) => {
                statuses.push(FormatStatus::Failed);
                writeln!(io::stderr(), "{}: {:?}", &file.display(), e).ok();
            }
        };
    }

    let (formatted, unchanged, failed) = count_status(statuses);
    print_summary(formatted, unchanged, failed, config.mode)
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

fn count_status(statuses: Vec<FormatStatus>) -> (usize, usize, usize) {
    let count =
        |status: FormatStatus| -> usize { statuses.iter().filter(|e| **e == status).count() };

    let formatted = count(FormatStatus::Formatted);
    let unchanged = count(FormatStatus::Unchanged);
    let failed = count(FormatStatus::Failed);

    (formatted, unchanged, failed)
}

fn print_summary(
    formatted: usize,
    unchanged: usize,
    failed: usize,
    mode: Mode,
) -> Result<ExitCode> {
    let file_or_files = |usize| -> &str {
        if usize <= 1 {
            "file"
        } else {
            "files"
        }
    };
    let message = match mode {
        Mode::Format => {
            let formatted = format!("{} {} formatted", formatted, file_or_files(formatted))
                .green()
                .to_string();
            let unchanged = format!("{} {} unchanged", unchanged, file_or_files(unchanged));
            let failed = format!("{} {} failed to format", failed, file_or_files(failed))
                .red()
                .to_string();
            format!("{}. {}. {}", formatted, unchanged, failed)
        }
        Mode::Check => {
            let formatted = format!(
                "{} {} would be formatted",
                formatted,
                file_or_files(formatted)
            )
            .green()
            .to_string();
            let unchanged = format!(
                "{} {} would be left unchanged",
                unchanged,
                file_or_files(unchanged)
            );
            let failed = format!("{} {} would fail to format", failed, file_or_files(failed))
                .red()
                .to_string();
            format!("{}. {}. {}", formatted, unchanged, failed)
        }
    };

    if failed != 0 {
        let msg = format!("\nOh no! 💥 💔 💥\n{}", message).bold().to_string();
        writeln!(io::stderr(), "{}", msg).ok();
        Ok(ExitCode::GeneralError)
    } else {
        let msg = format!("\nAll done! ✨ 🍰 ✨\n{}", message)
            .bold()
            .to_string();
        writeln!(io::stdout(), "{}", msg).ok();
        Ok(ExitCode::Success)
    }
}
