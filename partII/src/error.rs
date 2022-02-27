use thiserror::Error;

use std::io;

use crate::token::Token;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Setup(#[from] SetupError),
    #[error(transparent)]
    Scanner(#[from] ScannerErrors),
    #[error(transparent)]
    Parser(#[from] ParserErrors),
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

#[derive(Debug)]
pub struct ScannerErrors(pub Vec<ScannerError>);

impl std::fmt::Display for ScannerErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for error in &self.0 {
            writeln!(f, "{error}")?;
        }
        Ok(())
    }
}

impl std::error::Error for ScannerErrors {}

#[derive(Error, Debug)]
pub enum ScannerError {
    #[error("Unexpected character `{0}`.")]
    Character(char),
    #[error("Unterminated string.")]
    String,
    #[error("Could not convert {0} to a number: {1}")]
    Number(String, std::num::ParseFloatError),
}

#[derive(Debug)]
pub struct ParserErrors(pub Vec<ParserError>);

impl std::fmt::Display for ParserErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for error in &self.0 {
            writeln!(f, "{error}")?;
        }
        Ok(())
    }
}

impl std::error::Error for ParserErrors {}

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Expecting expression.")]
    ExpectingExpression,
    #[error("Can't have more than 255 arguments.")]
    TooManyArguments,
    #[error("Can't have more than 255 parameters.")]
    TooManyParameters,
    #[error("Invalid assignment target {0}.")]
    InvalidAssignmentTarget(Token),
    #[error("{0}")]
    Consume(String),
}
