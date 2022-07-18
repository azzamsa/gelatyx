use std::path::Path;

use atty::{self, Stream};
use clap::ArgMatches;
use gelatyx::{
    config::{Config, Mode},
    error::Result,
};

use crate::clap_app;

pub struct App {
    pub matches: ArgMatches,
    interactive_output: bool,
}

impl App {
    pub fn new() -> Self {
        #[cfg(windows)]
        let _ = ansi_term::enable_ansi_support();

        let interactive_output = atty::is(Stream::Stdout);

        App {
            matches: Self::matches(interactive_output),
            interactive_output,
        }
    }

    fn matches(interactive_output: bool) -> ArgMatches {
        clap_app::build(interactive_output).get_matches_from(wild::args())
    }

    pub fn config(&self) -> Result<Config> {
        let language = self.language();
        let files = self.files();
        let colored_output = match self.matches.value_of("color") {
            Some("always") => true,
            Some("never") => false,
            _ => self.interactive_output,
        };
        let mode = match self.matches.is_present("check") {
            true => Mode::Check,
            false => Mode::Format,
        };
        let language_config = self.matches.value_of("language-config");
        Ok(Config {
            language,
            files,
            colored_output,
            mode,
            language_config,
        })
    }

    fn language(&self) -> &str {
        match self.matches.value_of("LANGUAGE") {
            Some(lang) => lang,
            None => unreachable!("No language supplied"),
        }
    }
    fn files(&self) -> Vec<&Path> {
        match self.matches.values_of("file") {
            Some(files) => {
                files
                    .into_iter()
                    .map(Path::new)
                    // .take_while(|f| f.exists())
                    // Filtering only existing fails can't fails the program if
                    // only one file passed
                    .collect()
            }
            None => unreachable!("No file supplied"),
        }
    }
}
