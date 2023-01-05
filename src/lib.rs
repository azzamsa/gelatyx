#![deny(unsafe_code)]

pub mod cli;
pub mod config;
pub mod error;
pub mod exit_codes;
pub mod fmt;

// Aliases
pub use error::Error;
