#![allow(dead_code)]
#![allow(non_snake_case)]

mod ast_printer;
mod error;
mod expr;
mod interpreter;
mod parser;
mod scanner;
mod token;
mod value;

use std::{
    io::{BufRead, Write},
    path::Path,
};

use scanner::Scanner;
use value::Value;

use crate::error::{Result, SetupError};
use crate::{expr::Expr, parser::Parser};

fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();

    if args.len() > 3 {
        return Err(SetupError::Usage)?;
    }

    if let Some(filename) = args.get(1) {
        run_file(filename)
    } else if atty::is(atty::Stream::Stdin) {
        run_prompt()
    } else {
        run_file("/dev/stdin")
    }
}

fn run_file(filename: impl AsRef<Path>) -> Result<()> {
    let file = std::fs::read_to_string(filename).map_err(SetupError::from)?;

    let value = run(file)?;
    println!("{}", value);

    Ok(())
}

fn run_prompt() -> Result<()> {
    let stdin = std::io::stdin();
    let stdin = stdin.lock();
    let mut stdout = std::io::stdout();

    print!("> ");
    stdout.flush().map_err(SetupError::from)?;

    for line in stdin.lines() {
        let line = line.map_err(SetupError::from);
        match run(line?) {
            Ok(value) => println!("{}", value),
            Err(error) => println!("{}", error),
        }
        print!("> ");
        stdout.flush().map_err(SetupError::from)?;
    }

    Ok(())
}

fn run(input: String) -> Result<Value> {
    let scanner = Scanner::new(input);
    let tokens = scanner.scan_tokens()?;
    let parser = Parser::new(tokens);
    let expr = parser.parse()?;

    Ok(expr.evaluate()?)
}
