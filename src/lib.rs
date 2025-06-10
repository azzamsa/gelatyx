#![deny(unsafe_code)]
#![allow(clippy::result_large_err)]

pub mod output;

pub mod cli;
pub mod config;
pub mod error;
pub mod exit_codes;
pub mod fmt;

// Aliases
pub use error::Error;
