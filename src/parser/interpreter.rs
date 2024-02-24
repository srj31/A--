use crate::lexer::{self, token::Literal};

use super::expr::{Acceptor, Expr, Operator, Visitor};

enum Object {
    String(String),
    Int(i32),
    Boolean(bool),
    Float(f64),
    Nil,
    Identifier(String),
}

struct Interpreter;

impl Interpreter {
    pub fn interpret(&self, expr: &Expr) -> Object {
        expr.accept(self)
    }
}

impl Visitor<Object> for Interpreter {
    fn visit_literal(&self, expr: &Literal) -> Object {
        match expr {
            Literal::String(s) => Object::String(s.clone()),
            Literal::Int(i) => Object::Int(*i),
            Literal::Boolean(b) => Object::Boolean(*b),
            Literal::Float(fl) => Object::Float(*fl),
            Literal::Nil => Object::Nil,
            Literal::Identifier(i) => Object::Identifier(i.clone()),
        }
    }

    fn visit_unary(&self, operator: &Operator, right: &Expr) -> Object {
        let right = right.accept(self);
        match operator {
            Operator::Bang => match right {
                Object::Boolean(b) => Object::Boolean(!b),
                _ => Object::Nil,
            },
            Operator::Minus => match right {
                Object::Int(i) => Object::Int(-i),
                Object::Float(fl) => Object::Float(-fl),
                _ => Object::Nil,
            },
            _ => Object::Nil,
        }
    }

    fn visit_grouping(&self, expression: &Expr) -> Object {
        expression.accept(self)
    }

    fn visit_binary(&self, left: &Expr, operator: &Operator, right: &Expr) -> Object {
        let left = left.accept(self);
        let right = right.accept(self);
        match operator {
            Operator::Plus => match (left, right) {
                (Object::Int(l), Object::Int(r)) => Object::Int(l + r),
                (Object::Float(l), Object::Float(r)) => Object::Float(l + r),
                (Object::String(l), Object::String(r)) => Object::String(format!("{}{}", l, r)),
                _ => Object::Nil,
            },
            Operator::Minus => match (left, right) {
                (Object::Int(l), Object::Int(r)) => Object::Int(l - r),
                (Object::Float(l), Object::Float(r)) => Object::Float(l - r),
                _ => Object::Nil,
            },
            Operator::Slash => match (left, right) {
                (Object::Int(l), Object::Int(r)) => Object::Int(l / r),
                (Object::Float(l), Object::Float(r)) => Object::Float(l / r),
                _ => Object::Nil,
            },
            Operator::Star => match (left, right) {
                (Object::Int(l), Object::Int(r)) => Object::Int(l * r),
                (Object::Float(l), Object::Float(r)) => Object::Float(l * r),
                _ => Object::Nil,
            },
            Operator::Greater => match (left, right) {
                (Object::Int(l), Object::Int(r)) => Object::Boolean(l > r),
                (Object::Float(l), Object::Float(r)) => Object::Boolean(l > r),
                _ => Object::Nil,
            },
            Operator::GreaterEqual => match (left, right) {
                (Object::Int(l), Object::Int(r)) => Object::Boolean(l >= r),
                (Object::Float(l), Object::Float(r)) => Object::Boolean(l >= r),
                _ => Object::Nil,
            },
            Operator::Less => match (left, right) {
                (Object::Int(l), Object::Int(r)) => Object::Boolean(l < r),
                (Object::Float(l), Object::Float(r)) => Object::Boolean(l < r),
                _ => Object::Nil,
            },
            Operator::LessEqual => match (left, right) {
                (Object::Int(l), Object::Int(r)) => Object::Boolean(l <= r),
                (Object::Float(l), Object::Float(r)) => Object::Boolean(l <= r),
                _ => Object::Nil,
            },
            _ => Object::Nil,
        }
    }
}
