use core::fmt;

use crate::lexer::{
    token::{self, Token},
    token_type,
};

pub trait Visitor<T> {
    fn visit_binary(&self, left: &Expr, operator: &Operator, right: &Expr) -> T;
    fn visit_grouping(&self, expression: &Expr) -> T;
    fn visit_literal(&self, expr: &token::Literal) -> T;
    fn visit_unary(&self, operator: &Operator, right: &Expr) -> T;
}

pub trait Acceptor<T> {
    fn accept(&self, visitor: &dyn Visitor<T>) -> T;
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Operator,
        right: Box<Expr>,
    },
    Unary {
        operator: Operator,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: token::Literal,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Bang,
    Minus,
    Plus,
    Slash,
    Star,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    EqualEqual,
    BangEqual,
}

impl From<Token> for Operator {
    fn from(t: Token) -> Self {
        match t.token_type {
            token_type::TokenType::BANG => Operator::Bang,
            token_type::TokenType::MINUS => Operator::Minus,
            token_type::TokenType::PLUS => Operator::Plus,
            token_type::TokenType::SLASH => Operator::Slash,
            token_type::TokenType::STAR => Operator::Star,
            token_type::TokenType::GREATER => Operator::Greater,
            token_type::TokenType::GREATER_EQUAL => Operator::GreaterEqual,
            token_type::TokenType::LESS => Operator::Less,
            token_type::TokenType::LESS_EQUAL => Operator::LessEqual,
            token_type::TokenType::EQUAL_EQUAL => Operator::EqualEqual,
            token_type::TokenType::BANG_EQUAL => Operator::BangEqual,
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operator::Bang => write!(f, "!"),
            Operator::Minus => write!(f, "-"),
            Operator::Plus => write!(f, "+"),
            Operator::Slash => write!(f, "/"),
            Operator::Star => write!(f, "*"),
            Operator::Greater => write!(f, ">"),
            Operator::GreaterEqual => write!(f, ">="),
            Operator::Less => write!(f, "<"),
            Operator::LessEqual => write!(f, "<="),
            Operator::EqualEqual => write!(f, "=="),
            Operator::BangEqual => write!(f, "!="),
        }
    }
}

impl<T> Acceptor<T> for Expr {
    fn accept(&self, visitor: &dyn Visitor<T>) -> T {
        match self {
            Expr::Binary {
                left,
                operator,
                right,
            } => visitor.visit_binary(left, operator, right),
            Expr::Unary { operator, right } => visitor.visit_unary(operator, right),
            Expr::Grouping { expression } => visitor.visit_grouping(expression),
            Expr::Literal { value } => visitor.visit_literal(value),
        }
    }
}
