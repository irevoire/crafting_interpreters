use std::ops::Add;

use crate::{
    chunk::{Chunk, OpCode},
    error::ParserError,
    scanner::{Scanner, Token, TokenType},
    value::Value,
};

type Result<T> = std::result::Result<T, ParserError>;

#[derive(Debug)]
pub struct Parser<'a> {
    scanner: Scanner<'a>,
    current: Token<'a>,
    previous: Token<'a>,
    chunk: Chunk,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u8)]
pub enum Precedence {
    None,
    Assignment, // =
    Or,         // or
    And,        // and
    Equality,   // == !=
    Comparison, // < > <= >=
    Term,       // + -
    Factor,     // * /
    Unary,      // ! -
    Call,       // . ()
    Primary,
}

impl Add<u8> for Precedence {
    type Output = Precedence;

    fn add(self, rhs: u8) -> Self::Output {
        let lhs = self as u8;
        if rhs + lhs > Self::Primary as u8 {
            panic!("Bad precedence");
        }
        // this is safe since we checked we were not out of bound right before
        unsafe { std::mem::transmute(rhs + lhs) }
    }
}

type ParseFn<'a> = fn(&mut Parser<'a>) -> Result<()>;

pub struct ParseRule<'a> {
    pub prefix: Option<ParseFn<'a>>,
    pub infix: Option<ParseFn<'a>>,
    pub precedence: Precedence,
}

impl<'a> ParseRule<'a> {
    fn prec(precedence: Precedence) -> Self {
        Self {
            prefix: None,
            infix: None,
            precedence,
        }
    }

    fn prefix(prefix: ParseFn<'a>, precedence: Precedence) -> Self {
        Self {
            prefix: Some(prefix),
            infix: None,
            precedence,
        }
    }

    fn infix(infix: ParseFn<'a>, precedence: Precedence) -> Self {
        Self {
            prefix: None,
            infix: Some(infix),
            precedence,
        }
    }

    fn full(prefix: ParseFn<'a>, infix: ParseFn<'a>, precedence: Precedence) -> Self {
        Self {
            prefix: Some(prefix),
            infix: Some(infix),
            precedence,
        }
    }

    fn get_rule(ty: TokenType) -> Self {
        use TokenType::*;

        match ty {
            RightParen | LeftBrace | RightBrace | Comma | Semicolon | Dot | Bang | BangEqual
            | Equal | EqualEqual | Greater | GreaterEqual | Less | LessEqual | Identifier | And
            | Class | Else | False | Fun | For | If | Nil | Or | Print | Return | Super | This
            | True | Var | While | EoF | Error | String => Self::prec(Precedence::None),
            LeftParen => Self::prefix(Parser::grouping, Precedence::None),
            Minus => Self::full(Parser::unary, Parser::binary, Precedence::Term),
            Plus => Self::infix(Parser::binary, Precedence::Term),
            Slash | Star => Self::infix(Parser::binary, Precedence::Factor),
            Number => Self::prefix(Parser::number, Precedence::None),
        }
    }
}

impl<'a> Parser<'a> {
    pub fn compile(source: &'a str) -> Result<Chunk> {
        let mut scanner = Scanner::new(source);
        let tok = scanner.scan_token();

        let mut parser = Self {
            scanner,
            current: tok.clone(),
            previous: tok,
            chunk: Chunk::new(),
        };

        parser.advance()?;
        parser.expression()?;
        parser.consume(TokenType::EoF, "Expect end of expression.")?;
        parser.end_compiler();
        /*
        let mut line = usize::MAX;
        loop {
            let token = scanner.scan_token();
            if token.line != line {
                print!("{:4} ", token.line);
                line = token.line;
            } else {
                print!("   | ");
            }
            println!("{:8?} `{}`", token.ty, token.lexeme);

            if token.ty == TokenType::EoF {
                break;
            }
        }
        */

        Ok(parser.chunk)
    }

    fn advance(&mut self) -> Result<()> {
        log::trace!("advance");

        self.previous = self.current.clone();
        log::debug!("previous token becomes {:?}", self.previous.ty);

        loop {
            self.current = self.scanner.scan_token();

            log::debug!("current token becomes {:?}", self.current.ty);

            if self.current.ty != TokenType::Error {
                break;
            }

            self.error_at_current(self.current.lexeme)?;
        }
        Ok(())
    }

    fn emit_byte(&mut self, byte: impl Into<u8>) {
        self.chunk.write(byte, self.previous.line)
    }

    fn emit_bytes(&mut self, byte1: impl Into<u8>, byte2: impl Into<u8>) {
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }

    fn emit_return(&mut self) {
        self.emit_byte(OpCode::Return);
    }

    fn make_constant(&mut self, value: impl Into<Value>) -> Result<u8> {
        let value = value.into();
        let constant = self.chunk.add_constant(value);
        if constant > u8::MAX as usize {
            self.error_at_current("Too many constant in one chunk")?;
        }

        Ok(constant as u8)
    }

    fn emit_constant(&mut self, value: impl Into<Value>) -> Result<()> {
        let constant = self.make_constant(value)?;
        self.emit_bytes(OpCode::Constant, constant);
        Ok(())
    }

    fn end_compiler(&mut self) {
        log::trace!("end compiler");
        self.emit_return();
    }

    fn grouping(&mut self) -> Result<()> {
        log::trace!("parsing grouping");
        self.expression()?;
        self.consume(TokenType::RightParen, "Expect `)` after expression.")
    }

    fn expression(&mut self) -> Result<()> {
        log::trace!("parsing expression");
        self.parse_precedence(Precedence::Assignment)
    }

    fn number(&mut self) -> Result<()> {
        log::trace!("parsing number");
        let value: f64 = self.previous.lexeme.parse().unwrap();
        self.emit_constant(value)
    }

    fn unary(&mut self) -> Result<()> {
        log::trace!("parsing unary");
        let operator_type = self.previous.ty;

        // compile the operand
        self.parse_precedence(Precedence::Unary)?;

        // emit the operator instruction
        match operator_type {
            TokenType::Minus => self.emit_byte(OpCode::Negate),
            _ => unreachable!(),
        }

        Ok(())
    }

    fn binary(&mut self) -> Result<()> {
        log::trace!("parsing binary");
        let operator_type = self.previous.ty;

        let rule = ParseRule::get_rule(operator_type);

        self.parse_precedence(rule.precedence + 1)?;
        match operator_type {
            TokenType::Plus => self.emit_byte(OpCode::Add),
            TokenType::Minus => self.emit_byte(OpCode::Subtract),
            TokenType::Star => self.emit_byte(OpCode::Multiply),
            TokenType::Slash => self.emit_byte(OpCode::Divide),
            _ => unreachable!(),
        }

        Ok(())
    }

    fn parse_precedence(&mut self, precedence: Precedence) -> Result<()> {
        log::trace!("parsing precedence");

        self.advance()?;

        println!("{:?}", self);

        let parse_rule = ParseRule::get_rule(self.previous.ty);
        if let Some(prefix_rule) = parse_rule.prefix {
            prefix_rule(self)?;
        } else {
            self.error_at_current("Expect expression.")?;
        }

        while precedence <= ParseRule::get_rule(self.current.ty).precedence {
            self.advance()?;
            if let Some(infix_rule) = ParseRule::get_rule(self.current.ty).infix {
                infix_rule(self)?;
            } else {
                self.error_at_current("Unreachable.")?;
            }
        }
        Ok(())
    }

    fn consume(&mut self, ty: TokenType, message: impl AsRef<str>) -> Result<()> {
        if self.current.ty == ty {
            return self.advance();
        }

        self.error_at_current(message)
    }

    fn error_at_current(&self, message: impl AsRef<str>) -> Result<()> {
        self.error_at(&self.current, message)
    }

    fn error_at(&self, token: &Token, message: impl AsRef<str>) -> Result<()> {
        Err(ParserError::At {
            line: token.line,
            token: token.lexeme.to_string(),
            message: message.as_ref().to_string(),
        })
    }
}
