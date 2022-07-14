use crate::{
    config::Config,
    error::{default_error_handler, Result},
    fmt::format_files,
};

pub struct Controller<'a> {
    config: &'a Config<'a>,
}

impl<'b> Controller<'b> {
    pub fn new<'a>(config: &'a Config) -> Controller<'a> {
        Controller { config }
    }

    pub fn run(&self) -> Result<bool> {
        let result = format_files(self.config);
        match result {
            Ok(true) => Ok(true),
            Ok(false) => Ok(false),
            Err(error) => {
                default_error_handler(&error);
                Ok(false)
            }
        }
    }
}
