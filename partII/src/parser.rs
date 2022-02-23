use crate::{
    expr::Expr,
    token::{Token, TokenType},
};

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.follow([TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison();
            expr = Expr::binary(expr, operator, right);
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while self.follow([
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term();
            expr = Expr::binary(expr, operator, right);
        }

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.follow([TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor();
            expr = Expr::binary(expr, operator, right);
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.follow([TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary();
            expr = Expr::binary(expr, operator, right);
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        if self.follow([TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary();

            Expr::unary(operator, right)
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Expr {
        let token = self.advance();
        match token.ty {
            TokenType::False => Expr::literal("false".to_string()),
            TokenType::True => Expr::literal("true".to_string()),
            TokenType::Nil => Expr::literal("nil".to_string()),
            TokenType::Number(n) => Expr::literal(format!("{n}")),
            TokenType::String(ref s) => Expr::literal(format!("{s}")),
            TokenType::LeftParen => {
                let expr = self.expression();
                // TODO consume right paren
                let _ = self.advance();
                Expr::group(expr)
            }
            _ => unreachable!(),
        }
    }

    fn follow(&mut self, types: impl IntoIterator<Item = TokenType>) -> bool {
        types
            .into_iter()
            .find(|ty| self.check(ty))
            .map(|_| self.advance())
            .is_some()
    }

    fn check(&mut self, ty: &TokenType) -> bool {
        self.is_at_end() && (&self.peek().ty == ty)
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
        &self.tokens[self.current - 1]
    }
}
