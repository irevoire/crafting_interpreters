use crate::{
    chunk::Chunk,
    error::ParserError,
    scanner::{Scanner, Token, TokenType},
};

type Result<T> = std::result::Result<T, ParserError>;

pub struct Parser<'a> {
    scanner: Scanner<'a>,
    current: Token<'a>,
    previous: Token<'a>,
}

impl<'a> Parser<'a> {
    pub fn compile(source: &str) -> Chunk {
        let mut scanner = Scanner::new(source);
        let tok = scanner.scan_token();

        let mut parser = Self {
            scanner,
            current: tok,
            previous: tok,
        };

        parser.advance();
        parser.expression();
        parser.consume(TokenType::EoF, "Expect end of expression.");
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
    }

    fn advance(&mut self) -> Result<()> {
        self.previous = self.current;

        loop {
            self.current = self.scanner.scan_token();
            if self.current.ty != TokenType::Error {
                break;
            }

            self.error_at_current(self.current.lexeme)?;
        }
        Ok(())
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
