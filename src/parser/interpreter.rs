use std::fmt;

use crate::{
    lexer::token::{Literal, Token},
    log,
};

use super::{
    environment,
    expr::{Acceptor, Expr, Operator, Visitor},
    stmt::{self, Acceptor as StmtAcceptor, Stmt},
};

#[derive(Clone)]
pub enum Object {
    String(String),
    Int(i32),
    Boolean(bool),
    Float(f64),
    Nil,
    Identifier(String),
}

impl Object {
    pub fn is_truthy(&self) -> bool {
        match self {
            Object::Nil => false,
            Object::Boolean(b) => *b,
            Object::Int(i) => *i != 0,
            Object::Float(fl) => *fl != 0.0,
            Object::String(s) => !s.is_empty(),
            _ => true,
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::String(s) => write!(f, "{}", s),
            Object::Int(i) => write!(f, "{}", i),
            Object::Boolean(b) => write!(f, "{}", b),
            Object::Float(fl) => write!(f, "{}", fl),
            Object::Nil => write!(f, "why am i nil?"),
            Object::Identifier(i) => write!(f, "{}", i),
        }
    }
}

pub struct Interpreter {
    environment: environment::Environment,
}
struct InterpreterError {
    message: String,
    token: Token,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: environment::Environment::new(),
        }
    }
    pub fn interpret(&mut self, stmts: &Vec<Stmt>) {
        for stmt in stmts {
            match self.execute(stmt) {
                Ok(_) => {}
                Err(err) => {
                    log::log_message::print_code_error(err.token.line, &err.message);
                }
            }
        }
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Object> {
        expr.accept(self)
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<()> {
        stmt.accept(self)
    }

    fn execute_block(&mut self, stmts: &Vec<Stmt>, env: environment::Environment) {
        let previous = self.environment.clone();
        self.environment = env;
        for stmt in stmts {
            match self.execute(stmt) {
                Ok(_) => {}
                Err(err) => {
                    log::log_message::print_code_error(err.token.line, &err.message);
                }
            }
        }
        /*
         * TODO: the environment reveresed is not correct since the update inside the block is lost
         */
        self.environment = previous;
    }
}

type Result<T> = std::result::Result<T, InterpreterError>;

impl stmt::Visitor<Result<()>> for Interpreter {
    fn visit_expr(&mut self, expr: &Expr) -> Result<()> {
        self.evaluate(expr)?;
        Ok(())
    }
    fn visit_print(&mut self, expr: &Expr) -> Result<()> {
        let value = self.evaluate(expr)?;
        println!("{}", value);
        Ok(())
    }

    fn visit_var(&mut self, name: &Token, initializer: &Option<Expr>) -> Result<()> {
        match initializer {
            Some(expr) => {
                let value = self.evaluate(expr)?;
                self.environment.define(name.lexeme.clone(), value);
            }
            None => self.environment.define(name.lexeme.clone(), Object::Nil),
        }
        Ok(())
    }

    fn visit_block(&mut self, stmts: &Vec<Stmt>) -> Result<()> {
        self.execute_block(
            stmts,
            environment::Environment::new_enclosed(self.environment.clone()),
        );
        Ok(())
    }

    fn visit_if(
        &mut self,
        condition: &Expr,
        then_branch: &Stmt,
        else_branch: &Option<Box<Stmt>>,
    ) -> Result<()> {
        if self.evaluate(condition)?.is_truthy() {
            self.execute(then_branch)?;
        } else if let Some(else_branch) = else_branch {
            self.execute(else_branch)?;
        }
        Ok(())
    }

    fn visit_while(&mut self, condition: &Expr, body: &Stmt) -> Result<()> {
        while self.evaluate(condition)?.is_truthy() {
            self.execute(body)?;
        }
        Ok(())
    }
}

impl Visitor<Result<Object>> for Interpreter {
    fn visit_literal(&mut self, expr: &Literal) -> Result<Object> {
        match expr {
            Literal::String(s) => Ok(Object::String(s.clone())),
            Literal::Int(i) => Ok(Object::Int(*i)),
            Literal::Boolean(b) => Ok(Object::Boolean(*b)),
            Literal::Float(fl) => Ok(Object::Float(*fl)),
            Literal::Nil => Ok(Object::Nil),
            Literal::Identifier(i) => Ok(Object::Identifier(i.clone())),
        }
    }

    fn visit_unary(&mut self, operator: &Operator, right: &Expr) -> Result<Object> {
        let right = right.accept(self);
        match operator {
            Operator::Bang => match right? {
                Object::Boolean(b) => Ok(Object::Boolean(!b)),
                _ => Ok(Object::Nil),
            },
            Operator::Minus => match right? {
                Object::Int(i) => Ok(Object::Int(-i)),
                Object::Float(fl) => Ok(Object::Float(-fl)),
                _ => Ok(Object::Nil),
            },
            _ => Ok(Object::Nil),
        }
    }

    fn visit_grouping(&mut self, expression: &Expr) -> Result<Object> {
        expression.accept(self)
    }

    fn visit_binary(&mut self, left: &Expr, operator: &Operator, right: &Expr) -> Result<Object> {
        let left = left.accept(self)?;
        let right = right.accept(self)?;
        match operator {
            Operator::Plus => match (left, right) {
                (Object::Int(l), Object::Int(r)) => Ok(Object::Int(l + r)),
                (Object::Float(l), Object::Float(r)) => Ok(Object::Float(l + r)),
                (Object::String(l), Object::String(r)) => Ok(Object::String(format!("{}{}", l, r))),
                _ => Ok(Object::Nil),
            },
            Operator::Minus => match (left, right) {
                (Object::Int(l), Object::Int(r)) => Ok(Object::Int(l - r)),
                (Object::Float(l), Object::Float(r)) => Ok(Object::Float(l - r)),
                _ => Ok(Object::Nil),
            },
            Operator::Slash => match (left, right) {
                (Object::Int(l), Object::Int(r)) => Ok(Object::Int(l / r)),
                (Object::Float(l), Object::Float(r)) => Ok(Object::Float(l / r)),
                _ => Ok(Object::Nil),
            },
            Operator::Star => match (left, right) {
                (Object::Int(l), Object::Int(r)) => Ok(Object::Int(l * r)),
                (Object::Float(l), Object::Float(r)) => Ok(Object::Float(l * r)),
                _ => Ok(Object::Nil),
            },
            Operator::Greater => match (left, right) {
                (Object::Int(l), Object::Int(r)) => Ok(Object::Boolean(l > r)),
                (Object::Float(l), Object::Float(r)) => Ok(Object::Boolean(l > r)),
                _ => Ok(Object::Nil),
            },
            Operator::GreaterEqual => match (left, right) {
                (Object::Int(l), Object::Int(r)) => Ok(Object::Boolean(l >= r)),
                (Object::Float(l), Object::Float(r)) => Ok(Object::Boolean(l >= r)),
                _ => Ok(Object::Nil),
            },
            Operator::Less => match (left, right) {
                (Object::Int(l), Object::Int(r)) => Ok(Object::Boolean(l < r)),
                (Object::Float(l), Object::Float(r)) => Ok(Object::Boolean(l < r)),
                _ => Ok(Object::Nil),
            },
            Operator::LessEqual => match (left, right) {
                (Object::Int(l), Object::Int(r)) => Ok(Object::Boolean(l <= r)),
                (Object::Float(l), Object::Float(r)) => Ok(Object::Boolean(l <= r)),
                _ => Ok(Object::Nil),
            },
            _ => Ok(Object::Nil),
        }
    }

    fn visit_variable(&mut self, name: &Token) -> Result<Object> {
        Ok(self.environment.get(name))
    }

    fn visit_assignment(&mut self, name: &Token, value: &Expr) -> Result<Object> {
        let value = value.accept(self)?;
        self.environment.assign(name, value.clone());
        Ok(value)
    }

    fn visit_logical(&mut self, left: &Expr, operator: &Operator, right: &Expr) -> Result<Object> {
        let left = left.accept(self)?;
        match operator {
            Operator::Or => {
                if left.is_truthy() {
                    Ok(left)
                } else {
                    right.accept(self)
                }
            }
            Operator::And => {
                if !left.is_truthy() {
                    Ok(left)
                } else {
                    right.accept(self)
                }
            }
            _ => Ok(Object::Nil),
        }
    }
}
