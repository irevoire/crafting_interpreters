use thiserror::Error;

use std::io;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Setup error")]
    Setup(#[from] SetupError),
    #[error("Scanner error")]
    Scanner(#[from] ScannerError),
    #[error("Unexpected")]
    Unexpected(#[from] anyhow::Error),
}

#[derive(Error, Debug)]
pub enum SetupError {
    #[error("Usage {} [script]", std::env::args().nth(0).unwrap())]
    Usage,
    #[error("IO Error: ")]
    Io(#[from] io::Error),
}

#[derive(Error, Debug)]
pub enum ScannerError {
    #[error("Unexpected character `{0}`.")]
    Character(char),
    #[error("Unterminated string.")]
    String,
    #[error("Could not convert {0} to a number: {1}")]
    Number(String, std::num::ParseFloatError),
}
