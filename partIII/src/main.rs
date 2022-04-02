mod chunk;
mod compiler;
mod error;
mod scanner;
mod value;
mod vm;

use std::{
    io::{BufRead, Write},
    path::Path,
};

use chunk::*;
use error::*;
use vm::*;

fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();

    if args.len() > 3 {
        return Err(SetupError::Usage)?;
    }

    pretty_env_logger::init();

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
    let mut vm = Vm::new();

    run(file, &mut vm)?;

    Ok(())
}

fn run_prompt() -> Result<()> {
    let stdin = std::io::stdin();
    let stdin = stdin.lock();
    let mut stdout = std::io::stdout();

    let mut vm = Vm::new();

    print!("> ");
    stdout.flush().map_err(SetupError::from)?;

    for line in stdin.lines() {
        let line = line.map_err(SetupError::from);
        match run(line?, &mut vm) {
            Ok(_) => (),
            Err(error) => println!("{}", error),
        }
        print!("> ");
        stdout.flush().map_err(SetupError::from)?;
    }

    Ok(())
}

fn run(input: String, vm: &mut Vm) -> Result<()> {
    println!("interpreting {input}");
    vm.interpret(&input);
    Ok(())
    // compiler::Parser::compile(&input)?;
    // todo!()

    /*
    let scanner = Scanner::new(input);
    let tokens = scanner.scan_tokens()?;
    let parser = Parser::new(tokens);
    let stmts = parser.parse()?;

    let mut resolver = Resolver::new(interpreter);
    resolver.resolve(&stmts)?;

    Ok(interpreter.interpret(&stmts)?)
    */
}
