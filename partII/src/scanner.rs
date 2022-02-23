use crate::token::{Token, TokenType};

use anyhow::{bail, ensure, Error, Result};

#[derive(Debug)]
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,

    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(mut self) -> Result<Vec<Token>, Vec<Error>> {
        let mut errors = Vec::new();

        while !self.is_at_end() {
            self.start = self.current;
            if let Err(e) = self.scan_token() {
                errors.push(e);
            }
        }

        self.tokens.push(Token {
            ty: TokenType::EoF,
            lexeme: String::new(),
            line: self.line,
        });

        if errors.is_empty() {
            Ok(self.tokens)
        } else {
            Err(errors)
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.chars().count()
    }

    fn scan_token(&mut self) -> Result<()> {
        let c = self.advance().unwrap();

        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),

            '!' if self.follow('=') => self.add_token(TokenType::BangEqual),
            '!' => self.add_token(TokenType::Bang),
            '=' if self.follow('=') => self.add_token(TokenType::EqualEqual),
            '=' => self.add_token(TokenType::Equal),
            '<' if self.follow('=') => self.add_token(TokenType::LessEqual),
            '<' => self.add_token(TokenType::Less),
            '>' if self.follow('=') => self.add_token(TokenType::GreaterEqual),
            '>' => self.add_token(TokenType::Greater),
            '/' if self.follow('/') => {
                while self.peek() != '\n' && !self.is_at_end() {
                    self.advance();
                }
            }
            '/' => self.add_token(TokenType::Slash),
            '\n' => self.line += 1,
            c if c.is_whitespace() => (),

            '"' => self.string()?,
            c if c.is_ascii_digit() => self.number()?,
            c if c.is_ascii_alphabetic() || c == '_' => self.identifier(),

            c => bail!("Unexpected character `{c}`."),
        }

        Ok(())
    }

    fn peek(&self) -> char {
        self.current().unwrap_or('\0')
    }

    fn peek_next(&self) -> char {
        self.source.chars().nth(self.current + 1).unwrap_or('\0')
    }

    fn follow(&mut self, c: char) -> bool {
        if let Some(cur) = self.current() {
            if c == cur {
                self.advance();
                return true;
            }
        }
        false
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.source.chars().nth(self.current);
        if c.is_some() {
            self.current += 1;
        }
        c
    }

    fn current(&self) -> Option<char> {
        self.source.chars().nth(self.current)
    }

    fn add_token(&mut self, token_type: TokenType) {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token {
            ty: token_type,
            lexeme: text.to_string(),
            line: self.line,
        });
    }

    fn string(&mut self) -> Result<()> {
        // TODO: remove the double peek
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        ensure!(!self.is_at_end(), "Unterminated String");

        self.advance(); // skip the closing `"`

        // skip the starting `"`
        let usr_str = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token(TokenType::String(usr_str));
        Ok(())
    }

    fn number(&mut self) -> Result<()> {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let number = self.source[self.start..self.current].parse()?;
        self.add_token(TokenType::Number(number));

        Ok(())
    }

    fn identifier(&mut self) {
        while self.peek().is_ascii_alphanumeric() {
            self.advance();
        }

        let ident = self.source[self.start..self.current].to_string();

        if let Some(keyword) = TokenType::from_keyword(&ident) {
            self.add_token(keyword);
        } else {
            self.add_token(TokenType::Identifier(ident));
        }
    }
}
