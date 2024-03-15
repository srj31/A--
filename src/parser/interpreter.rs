use std::fmt;

use crate::{
    lexer::token::{Literal, Token},
    log,
};

use super::{
    expr::{Acceptor, Expr, Operator, Visitor},
    stmt::{self, Acceptor as StmtAcceptor, Stmt},
};

enum Object {
    String(String),
    Int(i32),
    Boolean(bool),
    Float(f64),
    Nil,
    Identifier(String),
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::String(s) => write!(f, "{}", s),
            Object::Int(i) => write!(f, "{}", i),
            Object::Boolean(b) => write!(f, "{}", b),
            Object::Float(fl) => write!(f, "{}", fl),
            Object::Nil => write!(f, "nil"),
            Object::Identifier(i) => write!(f, "{}", i),
        }
    }
}

pub struct Interpreter;
struct InterpreterError {
    message: String,
    token: Token,
}

impl Interpreter {
    pub fn new() -> Self {
        Self
    }
    pub fn interpret(&self, stmts: &Vec<Stmt>) {
        for stmt in stmts {
            match self.execute(stmt) {
                Ok(_) => {}
                Err(err) => {
                    log::log_message::print_code_error(err.token.line, &err.message);
                }
            }
        }
    }

    fn evaluate(&self, expr: &Expr) -> Result<Object> {
        expr.accept(self)
    }

    fn execute(&self, stmt: &Stmt) -> Result<()> {
        stmt.accept(self)
    }
}

type Result<T> = std::result::Result<T, InterpreterError>;

impl stmt::Visitor<Result<()>> for Interpreter {
    fn visit_expr(&self, expr: &Expr) -> Result<()> {
        self.evaluate(expr)?;
        Ok(())
    }
    fn visit_print(&self, expr: &Expr) -> Result<()> {
        let value = self.evaluate(expr)?;
        println!("{}", value);
        Ok(())
    }

    fn visit_var(&self, name: &str, initializer: &Expr) -> Result<()> {
        let value = self.evaluate(initializer)?;
        println!("{} = {}", name, value);
        Ok(())
    }
}

impl Visitor<Result<Object>> for Interpreter {
    fn visit_literal(&self, expr: &Literal) -> Result<Object> {
        match expr {
            Literal::String(s) => Ok(Object::String(s.clone())),
            Literal::Int(i) => Ok(Object::Int(*i)),
            Literal::Boolean(b) => Ok(Object::Boolean(*b)),
            Literal::Float(fl) => Ok(Object::Float(*fl)),
            Literal::Nil => Ok(Object::Nil),
            Literal::Identifier(i) => Ok(Object::Identifier(i.clone())),
        }
    }

    fn visit_unary(&self, operator: &Operator, right: &Expr) -> Result<Object> {
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

    fn visit_grouping(&self, expression: &Expr) -> Result<Object> {
        expression.accept(self)
    }

    fn visit_binary(&self, left: &Expr, operator: &Operator, right: &Expr) -> Result<Object> {
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
}
