#[derive(Debug)]
pub struct Scanner<'a> {
    source: &'a str,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_token(&mut self) -> Token<'a> {
        log::trace!("scan_token");
        self.skip_whitespace();
        self.start = self.current;

        if self.is_at_end() {
            log::trace!("scan_token is at end. Return EoF");
            return self.make_token(TokenType::EoF);
        }

        let c = self.advance();

        match c {
            '(' => self.make_token(TokenType::LeftParen),
            ')' => self.make_token(TokenType::RightParen),
            '{' => self.make_token(TokenType::LeftBrace),
            '}' => self.make_token(TokenType::RightBrace),
            ';' => self.make_token(TokenType::Semicolon),
            ',' => self.make_token(TokenType::Comma),
            '.' => self.make_token(TokenType::Dot),
            '-' => self.make_token(TokenType::Minus),
            '+' => self.make_token(TokenType::Plus),
            '/' => self.make_token(TokenType::Slash),
            '*' => self.make_token(TokenType::Star),
            '!' if self.follow('=') => self.make_token(TokenType::BangEqual),
            '!' => self.make_token(TokenType::Bang),
            '=' if self.follow('=') => self.make_token(TokenType::EqualEqual),
            '=' => self.make_token(TokenType::Equal),
            '<' if self.follow('=') => self.make_token(TokenType::LessEqual),
            '<' => self.make_token(TokenType::Less),
            '>' if self.follow('=') => self.make_token(TokenType::GreaterEqual),
            '>' => self.make_token(TokenType::Greater),
            '"' => self.string(),
            c if c.is_digit(10) => self.number(),
            c if c.is_ascii_alphabetic() || c == '_' => self.identifier(),
            c => todo!("{:?}", c),
        };

        self.error_token("Unexpected character.")
    }

    fn identifier(&mut self) -> Token {
        let mut peek = self.peek();
        while peek.is_ascii_alphabetic() || peek == '_' || peek.is_digit(10) {
            self.advance();
            peek = self.peek();
        }
        self.make_token(self.identifier_type())
    }

    fn identifier_type(&self) -> TokenType {
        match &self.source[self.start..=self.current] {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fun" => TokenType::Fun,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ => TokenType::Identifier,
        }
    }

    fn number(&mut self) -> Token {
        while self.peek().is_digit(10) {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();
            while self.peek().is_digit(10) {
                self.advance();
            }
        }

        self.make_token(TokenType::Number)
    }

    fn string(&mut self) -> Token {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.error_token("Unterminated string")
        } else {
            self.advance();
            self.make_token(TokenType::String)
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.peek() {
                ' ' | '\r' | '\t' => drop(self.advance()),
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                '/' if self.peek_next() == '/' => {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                }
                _ => return,
            }
        }
    }

    fn peek(&self) -> char {
        self.source[self.current..=self.current]
            .chars()
            .next()
            .unwrap()
    }

    fn peek_next(&self) -> char {
        self.source[self.current + 1..=self.current + 1]
            .chars()
            .next()
            .unwrap()
    }

    fn follow(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            false
        } else if self.peek() != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn advance(&mut self) -> char {
        let current = self.peek();
        self.current += 1;
        current
    }

    fn make_token(&self, ty: TokenType) -> Token<'a> {
        Token {
            ty,
            lexeme: &self.source[self.start..self.current],
            line: self.line,
        }
    }

    fn error_token(&self, message: &'a str) -> Token<'a> {
        Token {
            ty: TokenType::Error,
            lexeme: message.as_ref(),
            line: self.line,
        }
    }

    fn is_at_end(&self) -> bool {
        self.source.len() <= self.current
    }
}

#[derive(Clone, Debug)]
pub struct Token<'a> {
    pub ty: TokenType,
    pub lexeme: &'a str,
    pub line: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    // Single character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    // One or two characters tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Literals
    Identifier,
    String,
    Number,
    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    // Meta
    EoF,
    Error,
}
