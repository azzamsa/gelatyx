use crate::{
    config::Config,
    error::{default_error_handler, Result},
    fmt::format_file,
};

pub struct Controller<'a> {
    config: &'a Config<'a>,
}

impl<'b> Controller<'b> {
    pub fn new<'a>(config: &'a Config) -> Controller<'a> {
        Controller { config }
    }

    pub fn run(&self) -> Result<bool> {
        let mut no_errors: bool = true;

        for file in &self.config.files {
            let result = format_file(file, self.config.language);
            if let Err(error) = result {
                default_error_handler(&error);
                no_errors = false;
            }
        }
        Ok(no_errors)
    }
}
