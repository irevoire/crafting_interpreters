use std::rc::Rc;

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
        let result = if self.follow([TokenType::Fun]) {
            self.function("function")
        } else if self.follow([TokenType::Var]) {
            self.var_declaration()
        } else {
            self.statement()
        };

        if result.is_err() {
            self.synchronize();
        }

        result
    }

    fn function(&mut self, kind: &str) -> Result<Stmt> {
        let name = self.consume_ident(format!("Expect {kind} name."))?;

        self.consume(
            &TokenType::LeftParen,
            &format!("Expect `(` after {kind} name."),
        )?;

        let mut params = Vec::new();

        if !self.check(&TokenType::RightParen) {
            params.push(self.consume_ident("Expect parameter name.")?);
            while self.follow([TokenType::Comma]) {
                if params.len() >= 255 {
                    return Err(ParserError::TooManyParameters);
                }
                params.push(self.consume_ident("Expect parameter name.")?);
            }
        }

        self.consume(&TokenType::RightParen, "Expect `)` after parameters.")?;

        self.consume(
            &TokenType::LeftBrace,
            format!("Expect `{{` before {kind} body."),
        )?;

        let body = self.block()?;

        Ok(Stmt::Function(crate::callable::Function {
            name,
            params,
            body: Rc::new(body),
        }))
    }

    fn var_declaration(&mut self) -> Result<Stmt> {
        let name = self.consume_ident("Expect variable name.")?;
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
        if self.follow([TokenType::If]) {
            self.if_statement()
        } else if self.follow([TokenType::Print]) {
            self.print_statement()
        } else if self.follow([TokenType::Return]) {
            self.return_statement()
        } else if self.follow([TokenType::While]) {
            self.while_statement()
        } else if self.follow([TokenType::For]) {
            self.for_statement()
        } else if self.follow([TokenType::LeftBrace]) {
            Ok(Stmt::Block(Rc::new(self.block()?)))
        } else {
            self.expression_statement()
        }
    }

    fn if_statement(&mut self) -> Result<Stmt> {
        self.consume(&TokenType::LeftParen, "Expect `(` after `if`.")?;
        let condition = self.expression()?;
        self.consume(&TokenType::RightParen, "Expect `)` after `if` condition.")?;

        let then_branch = self.statement()?;
        let mut else_branch = None;

        if self.follow([TokenType::Else]) {
            else_branch = Some(self.statement()?);
        }

        Ok(Stmt::If {
            condition,
            then_branch: Box::new(then_branch),
            else_branch: else_branch.map(Box::new),
        })
    }

    fn print_statement(&mut self) -> Result<Stmt> {
        let value = self.expression()?;
        self.consume(&TokenType::Semicolon, "Expect `;` after value.")?;

        Ok(Stmt::Print(value))
    }

    fn return_statement(&mut self) -> Result<Stmt> {
        let keyword = self.previous().clone();

        let value = if self.check(&TokenType::Semicolon) {
            None
        } else {
            Some(self.expression()?)
        };

        self.consume(&TokenType::Semicolon, "Expect `;` after return.")?;

        Ok(Stmt::Return { keyword, value })
    }

    fn while_statement(&mut self) -> Result<Stmt> {
        self.consume(&TokenType::LeftParen, "Expect `(` after `while`.")?;
        let condition = self.expression()?;
        self.consume(
            &TokenType::RightParen,
            "Expect `)` after `while` condition.",
        )?;

        let body = self.statement()?;

        Ok(Stmt::While {
            condition,
            body: Box::new(body),
        })
    }

    fn for_statement(&mut self) -> Result<Stmt> {
        self.consume(&TokenType::LeftParen, "Expect `(` after `for`.")?;

        let initializer = if self.follow([TokenType::Semicolon]) {
            None
        } else if self.follow([TokenType::Var]) {
            Some(self.var_declaration()?)
        } else {
            Some(self.expression_statement()?)
        };

        let condition = if self.check(&TokenType::Semicolon) {
            None
        } else {
            Some(self.expression()?)
        };
        self.consume(&TokenType::Semicolon, "Expect `;` after loop condition.")?;

        let increment = if self.check(&TokenType::RightParen) {
            None
        } else {
            Some(self.expression()?)
        };

        self.consume(&TokenType::RightParen, "Expect `)` after for clauses.")?;

        let mut body = self.statement()?;

        // desugar the for loop
        if let Some(increment) = increment {
            body = Stmt::Block(Rc::new(vec![body, Stmt::Expression(increment)]));
        }

        let condition = condition.unwrap_or(Expr::literal(true));

        body = Stmt::While {
            condition,
            body: Box::new(body),
        };

        if let Some(initializer) = initializer {
            body = Stmt::Block(Rc::new(vec![initializer, body]));
        }

        Ok(body)
    }

    fn block(&mut self) -> Result<Vec<Stmt>> {
        let mut statements = Vec::new();

        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        self.consume(&TokenType::RightBrace, "Expect `}` after block.")?;

        Ok(statements)
    }

    fn expression_statement(&mut self) -> Result<Stmt> {
        let expr = self.expression()?;
        self.consume(&TokenType::Semicolon, "Expect `;` after expression.")?;

        Ok(Stmt::Expression(expr))
    }

    fn expression(&mut self) -> Result<Expr> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr> {
        let expr = self.or()?;

        if self.follow([TokenType::Equal]) {
            let equals = self.previous().clone();
            let value = self.assignment()?;

            if let Expr::Variable { name } = expr {
                return Ok(Expr::Assign {
                    name,
                    value: Box::new(value),
                });
            }

            return Err(ParserError::InvalidAssignmentTarget(equals));
        }

        Ok(expr)
    }

    fn or(&mut self) -> Result<Expr> {
        let mut expr = self.and()?;

        while self.follow([TokenType::Or]) {
            let operator = self.previous().clone();
            let right = self.and()?;
            expr = Expr::logical(expr, operator, right);
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr> {
        let mut expr = self.comparison()?;

        while self.follow([TokenType::And]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::logical(expr, operator, right);
        }

        Ok(expr)
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
            self.call()
        }
    }

    fn call(&mut self) -> Result<Expr> {
        let mut expr = self.primary()?;

        loop {
            if self.follow([TokenType::LeftParen]) {
                expr = self.finish_call(expr)?;
            } else {
                break Ok(expr);
            }
        }
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr> {
        let mut arguments = Vec::new();

        if !self.check(&TokenType::RightParen) {
            arguments.push(self.expression()?);
            while self.follow([TokenType::Comma]) {
                if arguments.len() >= 255 {
                    return Err(ParserError::TooManyArguments);
                }
                arguments.push(self.expression()?);
            }
        }

        let paren = self.consume(&TokenType::RightParen, "Expect `)` after arguments.")?;

        Ok(Expr::Call {
            callee: Box::new(callee),
            paren,
            arguments,
        })
    }

    fn primary(&mut self) -> Result<Expr> {
        let token = self.advance();
        let expr = match token.ty {
            TokenType::Nil => Expr::literal(Value::Nil),
            TokenType::False => Expr::literal(false),
            TokenType::True => Expr::literal(true),
            TokenType::Number(n) => Expr::literal(n),
            TokenType::String(ref s) => Expr::literal(s.to_string()),
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

    fn consume(&mut self, ty: &TokenType, msg: impl AsRef<str>) -> Result<Token> {
        if self.check(ty) {
            Ok(self.advance().clone())
        } else {
            Err(ParserError::Consume(format!(
                "Got `{}`. {}",
                self.peek().lexeme,
                msg.as_ref()
            )))
        }
    }

    // TODO: I shouldn’t need this method but since I badly designed my types I can’t call consume directly
    fn consume_ident(&mut self, msg: impl AsRef<str>) -> Result<Token> {
        if matches!(self.peek().ty, TokenType::Identifier(_)) {
            Ok(self.advance().clone())
        } else {
            Err(ParserError::Consume(format!(
                "Got `{}`. {}",
                self.peek().lexeme,
                msg.as_ref()
            )))
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

            self.advance();
        }
    }
}
