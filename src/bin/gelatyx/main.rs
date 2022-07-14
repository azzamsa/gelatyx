#![deny(unsafe_code)]

mod app;
mod clap_app;

use std::process;

use gelatyx::{
    config::Config,
    controller::Controller,
    error::{default_error_handler, Result},
};

use crate::app::App;

fn run_controller(config: &Config) -> Result<bool> {
    let controller = Controller::new(config);
    controller.run()
}

fn run() -> Result<bool> {
    let app = App::new();
    let config = app.config()?;
    run_controller(&config)
}

fn main() {
    let result = run();

    match result {
        Err(error) => {
            default_error_handler(&error);
            process::exit(1);
        }
        Ok(false) => {
            process::exit(1);
        }
        Ok(true) => {
            process::exit(0);
        }
    }
}
