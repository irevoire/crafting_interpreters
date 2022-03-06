use std::collections::HashMap;

use crate::{callable::Function, expr::Expr, interpreter::Interpreter, stmt::Stmt, token::Token};

use anyhow::{anyhow, Result};

type Scope<'a> = HashMap<&'a str, bool>;

#[derive(Debug)]
pub struct Resolver<'a> {
    interpreter: &'a mut Interpreter,
    scopes: Vec<Scope<'a>>,
    current_function: FunctionType,
    current_class: ClassType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FunctionType {
    None,
    Function,
    Initializer,
    Method,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ClassType {
    None,
    Class,
}

impl<'a> Resolver<'a> {
    pub fn new(interpreter: &'a mut Interpreter) -> Self {
        Self {
            interpreter,
            scopes: Vec::new(),
            current_function: FunctionType::None,
            current_class: ClassType::None,
        }
    }

    pub fn resolve(&mut self, stmts: &'a [Stmt]) -> Result<()> {
        self.resolve_stmts(stmts)
    }

    fn begin_scope(&mut self) {
        self.scopes.push(Scope::default());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: &'a Token) -> Result<()> {
        if let Some(scope) = self.scopes.last_mut() {
            if scope.contains_key(name.lexeme.as_str()) {
                return Err(anyhow!("Already a variable with this name in this scope."))?;
            }
            scope.insert(&name.lexeme, false);
        }
        Ok(())
    }

    fn define(&mut self, name: &'a Token) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(&name.lexeme, true);
        }
    }

    fn resolve_stmts(&mut self, statements: &'a [Stmt]) -> Result<()> {
        for statement in statements {
            statement.resolve(self)?;
        }
        Ok(())
    }

    fn resolve_local(&mut self, expr: &'a Expr, name: &'a Token) -> Result<()> {
        for (idx, scope) in self.scopes.iter().enumerate() {
            if scope.contains_key(&name.lexeme as &str) {
                self.interpreter.resolve(expr, self.scopes.len() - idx - 1);
                return Ok(());
            }
        }
        Ok(())
    }

    fn resolve_function(&mut self, function: &'a Function, ty: FunctionType) -> Result<()> {
        self.current_function = ty;

        self.begin_scope();
        for param in &function.params {
            self.declare(param)?;
            self.define(param);
        }

        self.resolve_stmts(&function.body)?;
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

impl<'a> Stmt {
    fn resolve(&'a self, resolver: &mut Resolver<'a>) -> Result<()> {
        match self {
            Stmt::Class {
                name,
                superclass,
                methods,
            } => {
                let enclosing_class = resolver.current_class;
                resolver.current_class = ClassType::Class;

                resolver.declare(name)?;
                resolver.define(name);
                if let Some(superclass) = superclass {
                    if name.lexeme == superclass.unwrap_variable().lexeme {
                        return Err(anyhow!("A class can't inherit from itself."));
                    }
                    superclass.resolve(resolver)?;
                }

                resolver.begin_scope();
                resolver.scopes.last_mut().unwrap().insert("this", true);

                for method in methods {
                    let declaration = if method.is_initializer {
                        FunctionType::Initializer
                    } else {
                        FunctionType::Method
                    };
                    resolver.resolve_function(method, declaration)?;
                }

                resolver.end_scope();

                resolver.current_class = enclosing_class;
                Ok(())
            }
            Stmt::Block(stmts) => {
                resolver.begin_scope();
                resolver.resolve_stmts(&stmts)?;
                resolver.end_scope();
                Ok(())
            }
            Stmt::Expression(expr) => expr.resolve(resolver),
            Stmt::Function(function @ Function { name, .. }) => {
                resolver.declare(name)?;
                resolver.define(name);
                resolver.resolve_function(function, FunctionType::Function)
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
            } => {
                if resolver.current_function == FunctionType::None {
                    return Err(anyhow!("Can't return from top-level code."))?;
                } else if resolver.current_function == FunctionType::Initializer {
                    return Err(anyhow!("Can't return a value from an initializer."))?;
                }
                value.resolve(resolver)
            }
            Stmt::Var { name, initializer } => {
                resolver.declare(name)?;
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
    fn resolve<'a>(&'a self, resolver: &mut Resolver<'a>) -> Result<()> {
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
            Expr::Get { object, .. } => object.resolve(resolver),
            Expr::Grouping { expression } => expression.resolve(resolver),
            Expr::Literal { .. } => Ok(()),
            Expr::Logical { left, right, .. } => {
                left.resolve(resolver)?;
                right.resolve(resolver)
            }
            Expr::Set { object, value, .. } => {
                value.resolve(resolver)?;
                object.resolve(resolver)
            }
            Expr::Unary { right, .. } => right.resolve(resolver),
            Expr::Variable { name } => {
                if !resolver.is_empty() && resolver.get(&name.lexeme) == Some(false) {
                    return Err(anyhow::anyhow!(
                        "Can't read local variable in its own initializer"
                    ));
                }

                resolver.resolve_local(self, name)
            }
            Expr::This { keyword } => {
                if resolver.current_class == ClassType::None {
                    return Err(anyhow::anyhow!("Can't use `this` outside of a class."));
                }
                resolver.resolve_local(self, keyword)
            }
        }
    }
}
