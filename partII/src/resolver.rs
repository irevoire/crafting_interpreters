use std::collections::HashMap;

use crate::{callable::Function, expr::Expr, interpreter::Interpreter, stmt::Stmt, token::Token};

use anyhow::Result;

type Scope = HashMap<String, bool>;

#[derive(Debug)]
pub struct Resolver<'a> {
    pub interpreter: &'a mut Interpreter,
    pub scopes: Vec<Scope>,
}

impl<'a> Resolver<'a> {
    pub fn new(interpreter: &'a mut Interpreter) -> Self {
        Self {
            interpreter,
            scopes: Vec::new(),
        }
    }

    pub fn resolve(&mut self, stmts: &[Stmt]) -> Result<()> {
        self.resolve_stmts(stmts)
    }

    fn begin_scope(&mut self) {
        self.scopes.push(Scope::default());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: &Token) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.lexeme.clone(), false);
        }
    }

    fn define(&mut self, name: &Token) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.lexeme.clone(), true);
        }
    }

    fn resolve_stmts(&mut self, statements: &[Stmt]) -> Result<()> {
        for statement in statements {
            statement.resolve(self)?;
        }
        Ok(())
    }

    fn resolve_local(&mut self, expr: &Expr, name: &Token) -> Result<()> {
        for (idx, scope) in self.scopes.iter().enumerate() {
            if scope.contains_key(&name.lexeme) {
                self.interpreter.resolve(&expr, self.scopes.len() - idx - 1);
                return Ok(());
            }
        }
        Ok(())
    }

    fn resolve_function(&mut self, function: &Function) -> Result<()> {
        self.begin_scope();
        for param in &function.params {
            self.declare(param);
            self.define(param);
        }

        self.resolve_stmts(&function.body)?;
        dbg!(&self);
        self.end_scope();
        Ok(())
    }

    fn get(&self, name: &str) -> Option<bool> {
        if let Some(scope) = self.scopes.last() {
            scope.get(name).copied()
        } else {
            None
        }
    }

    fn is_empty(&self) -> bool {
        self.scopes.is_empty()
    }
}

impl Stmt {
    fn resolve(&self, resolver: &mut Resolver) -> Result<()> {
        match self {
            Stmt::Block(stmts) => {
                resolver.begin_scope();
                resolver.resolve_stmts(&stmts)?;
                resolver.end_scope();
                Ok(())
            }
            Stmt::Expression(expr) => expr.resolve(resolver),
            Stmt::Function(function @ Function { name, .. }) => {
                resolver.declare(name);
                resolver.define(name);
                resolver.resolve_function(function)
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                condition.resolve(resolver)?;
                then_branch.resolve(resolver)?;
                if let Some(else_branch) = else_branch {
                    else_branch.resolve(resolver)?
                }
                Ok(())
            }
            Stmt::Print(expr) => expr.resolve(resolver),
            Stmt::Return {
                value: Some(value), ..
            } => value.resolve(resolver),
            Stmt::Var { name, initializer } => {
                resolver.declare(name);
                if let Some(initializer) = initializer {
                    initializer.resolve(resolver)?;
                }
                resolver.define(name);
                Ok(())
            }
            Stmt::While { condition, body } => {
                condition.resolve(resolver)?;
                body.resolve(resolver)
            }
            _ => Ok(()),
        }
    }
}

impl Expr {
    fn resolve(&self, resolver: &mut Resolver) -> Result<()> {
        match self {
            Expr::Assign { name, value } => {
                value.resolve(resolver)?;
                resolver.resolve_local(value, name)
            }
            Expr::Binary { left, right, .. } => {
                left.resolve(resolver)?;
                right.resolve(resolver)
            }
            Expr::Call {
                callee, arguments, ..
            } => {
                callee.resolve(resolver)?;
                for argument in arguments {
                    argument.resolve(resolver)?;
                }
                Ok(())
            }
            Expr::Grouping { expression } => expression.resolve(resolver),
            Expr::Literal { .. } => Ok(()),
            Expr::Logical { left, right, .. } => {
                left.resolve(resolver)?;
                right.resolve(resolver)
            }
            Expr::Variable { name } => {
                if !resolver.is_empty() && resolver.get(&name.lexeme) == Some(false) {
                    return Err(anyhow::anyhow!(
                        "Can't read local variable in its own initializer"
                    ));
                }

                resolver.resolve_local(self, name)
            }
            Expr::Unary { right, .. } => right.resolve(resolver),
        }
    }
}
