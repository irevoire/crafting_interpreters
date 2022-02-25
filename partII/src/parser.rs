use crate::{
    error::{ParserError, ParserErrors},
    expr::Expr,
    stmt::Stmt,
    token::{Token, TokenType},
    value::Value,
};

type Result<T> = std::result::Result<T, ParserError>;

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(mut self) -> std::result::Result<Vec<Stmt>, ParserErrors> {
        let mut stmts = Vec::new();
        let mut errors = Vec::new();

        while !self.is_at_end() {
            match self.declaration() {
                Ok(stmt) => stmts.push(stmt),
                Err(error) => errors.push(error),
            }
        }

        if errors.is_empty() {
            Ok(stmts)
        } else {
            Err(ParserErrors(errors))
        }
    }

    fn declaration(&mut self) -> Result<Stmt> {
        let result = if self.follow([TokenType::Var]) {
            self.var_declaration()
        } else {
            self.statement()
        };

        if result.is_err() {
            self.synchronize();
        }

        result
    }

    fn var_declaration(&mut self) -> Result<Stmt> {
        // TODO: I should use consume here but I badly designed my types and can't
        let name = if matches!(self.peek().ty, TokenType::Identifier(_)) {
            self.advance().clone()
        } else {
            return Err(ParserError::Consume(String::from("Expect variable name.")));
        };

        let mut initializer = None;

        if self.follow([TokenType::Equal]) {
            initializer = Some(self.expression()?);
        }

        self.consume(
            &TokenType::Semicolon,
            "Expect `;` after variable declaration.",
        )?;

        Ok(Stmt::Var { name, initializer })
    }

    fn statement(&mut self) -> Result<Stmt> {
        if self.follow([TokenType::Print]) {
            self.print_statement()
        } else {
            self.expression_statement()
        }
    }

    fn print_statement(&mut self) -> Result<Stmt> {
        let value = self.expression()?;
        self.consume(&TokenType::Semicolon, "Expect `;` after value.")?;

        Ok(Stmt::Print(value))
    }

    fn expression_statement(&mut self) -> Result<Stmt> {
        let expr = self.expression()?;
        self.consume(&TokenType::Semicolon, "Expect `;` after expression.")?;

        Ok(Stmt::Expression(expr))
    }

    fn expression(&mut self) -> Result<Expr> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr> {
        let mut expr = self.comparison()?;

        while self.follow([TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::binary(expr, operator, right);
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr> {
        let mut expr = self.term()?;

        while self.follow([
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::binary(expr, operator, right);
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr> {
        let mut expr = self.factor()?;

        while self.follow([TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::binary(expr, operator, right);
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr> {
        let mut expr = self.unary()?;

        while self.follow([TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::binary(expr, operator, right);
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr> {
        if self.follow([TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;

            Ok(Expr::unary(operator, right))
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expr> {
        let token = self.advance();
        let expr = match token.ty {
            TokenType::Nil => Expr::literal(Value::Nil),
            TokenType::False => Expr::literal(Value::Bool(false)),
            TokenType::True => Expr::literal(Value::Bool(true)),
            TokenType::Number(n) => Expr::literal(Value::Number(n)),
            TokenType::String(ref s) => Expr::literal(Value::String(s.to_string())),
            TokenType::LeftParen => {
                let expr = self.expression()?;
                self.consume(&TokenType::RightParen, "Expect `)` after expression.")?;
                Expr::group(expr)
            }
            TokenType::Identifier(_) => Expr::Variable {
                name: token.clone(),
            },
            _ => return Err(ParserError::ExpectingExpression),
        };

        Ok(expr)
    }

    fn consume(&mut self, ty: &TokenType, msg: &str) -> Result<Token> {
        if self.check(ty) {
            Ok(self.advance().clone())
        } else {
            Err(ParserError::Consume(format!("{} {}", self.peek(), msg)))
        }
    }

    fn follow(&mut self, types: impl IntoIterator<Item = TokenType>) -> bool {
        for ty in types {
            if self.check(&ty) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&mut self, ty: &TokenType) -> bool {
        (!self.is_at_end()) && (&self.peek().ty == ty)
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().ty == TokenType::EoF
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current.saturating_sub(1)]
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().ty == TokenType::Semicolon {
                return;
            }

            match self.peek().ty {
                TokenType::Class
                | TokenType::For
                | TokenType::Fun
                | TokenType::If
                | TokenType::Print
                | TokenType::Return
                | TokenType::Var
                | TokenType::While => return,
                _ => (),
            }
        }

        self.advance();
    }
}
