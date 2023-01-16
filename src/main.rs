#![deny(unsafe_code)]
use std::io::{self, Write};
use std::process;

use clap::Parser;
use miette::{Context, Result};
use owo_colors::{
    OwoColorize,
    Stream::{Stderr, Stdout},
};

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

    let files = opts.files()?;
    let config = construct_config(opts);

    for file in files {
        match fmt::format_files(&config, &file).context("Invalid syntax") {
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
    match opts.color {
        Color::Always => {
            owo_colors::set_override(true);
        }
        Color::Never => {
            owo_colors::set_override(false);
        }
        Color::Auto => {
            owo_colors::unset_override();
        }
    };
    let mode = match opts.check {
        true => Mode::Check,
        false => Mode::Format,
    };

    Config {
        language: opts.language,
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
    let file_or_files = |file_count: usize| -> &str {
        if file_count <= 1 {
            "file"
        } else {
            "files"
        }
    };

    let failed_summary = |formatted: &str, unchanged: &str, failed: &str| -> Result<ExitCode> {
        writeln!(
            io::stderr(),
            "\n{}\n{}. {}. {}.",
            "Oh no! 💥 💔 💥".if_supports_color(Stderr, |text| text.bold()),
            formatted.if_supports_color(Stderr, |text| text.green()),
            unchanged,
            failed.if_supports_color(Stderr, |text| text.red()),
        )
        .ok();

        Ok(ExitCode::GeneralError)
    };
    let success_summary = |formatted: &str, unchanged: &str, failed: &str| -> Result<ExitCode> {
        writeln!(
            io::stdout(),
            "\n{}\n{}. {}. {}.",
            "All done! ✨ 🍰 ✨".if_supports_color(Stdout, |text| text.bold()),
            formatted.if_supports_color(Stdout, |text| text.green()),
            unchanged,
            failed.if_supports_color(Stdout, |text| text.red()),
        )
        .ok();
        Ok(ExitCode::Success)
    };

    match mode {
        Mode::Format => {
            let is_formatted = format!("{} {} formatted", formatted, file_or_files(formatted));
            let is_unchanged = format!("{} {} unchanged", unchanged, file_or_files(unchanged));
            let is_failed = format!("{} {} failed to format", failed, file_or_files(failed));

            if failed != 0 {
                failed_summary(&is_formatted, &is_unchanged, &is_failed)
            } else {
                success_summary(&is_formatted, &is_unchanged, &is_failed)
            }
        }
        Mode::Check => {
            let would_be_formatted = format!(
                "{} {} would be formatted",
                formatted,
                file_or_files(formatted)
            );
            let would_be_unchanged = format!(
                "{} {} would be left unchanged",
                unchanged,
                file_or_files(unchanged)
            );
            let would_fail = format!("{} {} would fail to format", failed, file_or_files(failed));

            if failed != 0 || formatted != 0 {
                failed_summary(&would_be_formatted, &would_be_unchanged, &would_fail)
            } else {
                success_summary(&would_be_formatted, &would_be_unchanged, &would_fail)
            }
        }
    }
}
