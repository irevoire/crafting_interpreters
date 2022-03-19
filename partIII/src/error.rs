use std::io;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Setup(#[from] SetupError),
    #[error(transparent)]
    Parser(#[from] ParserError),
    // #[error(transparent)]
    // Parser(#[from] ParserErrors),
    // #[error(transparent)]
    // Runtime(#[from] RuntimeError),
    #[error("Unexpected error: {0}")]
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
pub enum ParserError {
    #[error("[line {line}] Error at `{token}`: {message}.")]
    At {
        line: usize,
        token: String,
        message: String,
    },
}
