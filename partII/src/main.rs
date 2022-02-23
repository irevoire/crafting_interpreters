#![allow(dead_code)]
#![allow(non_snake_case)]

mod ast_printer;
mod expr;
mod parser;
mod scanner;
mod token;

use std::{
    io::{BufRead, Write},
    path::Path,
};

use anyhow::{anyhow, ensure, Result};
use scanner::Scanner;

use crate::{expr::Expr, parser::Parser};

fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();

    ensure!(args.len() < 3, "Usage {} [script]", args[0]);

    if let Some(filename) = args.get(1) {
        run_file(filename)
    } else if atty::is(atty::Stream::Stdin) {
        run_prompt()
    } else {
        run_file("/dev/stdin")
    }
}

fn run_file(filename: impl AsRef<Path>) -> Result<()> {
    let file = std::fs::read_to_string(filename)?;

    let expr = run(file)?;
    println!("{}", expr.graph());

    Ok(())
}

fn run_prompt() -> Result<()> {
    let stdin = std::io::stdin();
    let stdin = stdin.lock();
    let mut stdout = std::io::stdout();

    print!("> ");
    stdout.flush()?;

    for line in stdin.lines() {
        let expr = run(line?)?;
        println!("{}", expr.polish_notation());
        print!("> ");
        stdout.flush()?;
    }

    Ok(())
}

fn run(input: String) -> Result<Expr> {
    let scanner = Scanner::new(input);
    let tokens = match scanner.scan_tokens() {
        Ok(tokens) => tokens,
        Err(errors) => {
            errors.iter().for_each(|error| println!("{error:?}"));
            return Err(anyhow!("Scanning errors happened"));
        }
    };

    let mut parser = Parser::new(tokens);

    parser.parse()
}
