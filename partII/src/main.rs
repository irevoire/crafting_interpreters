mod scanner;
mod token;

use std::{
    io::{BufRead, Read, Write},
    path::Path,
};

use anyhow::{ensure, Result};
use scanner::Scanner;

fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();

    ensure!(args.len() < 3, "Usage {} [script]", args[0]);

    if let Some(filename) = args.get(1) {
        run_file(filename)
    } else {
        run_prompt()
    }
}

fn run_file(filename: impl AsRef<Path>) -> Result<()> {
    let file = std::fs::read_to_string(filename)?;

    run(file)
}

fn run_prompt() -> Result<()> {
    let stdin = std::io::stdin();
    let stdin = stdin.lock();
    let mut stdout = std::io::stdout();

    print!("> ");
    stdout.flush()?;

    for line in stdin.lines() {
        print!("> ");
        stdout.flush()?;
        run(line?)?;
    }

    Ok(())
}

fn run(input: String) -> Result<()> {
    let scanner = Scanner::new(input);
    let tokens = scanner.scan_tokens()?;

    tokens.iter().for_each(|token| println!("{token}"));

    Ok(())
}
