use core::fmt;

use crate::lexer::token::Token;

use super::expr::Expr;

pub trait Visitor<T> {
    fn visit_expr(&self, expr: &Expr) -> T;
    fn visit_print(&self, expr: &Expr) -> T;
    fn visit_var(&self, name: &str, initializer: &Expr) -> T;
}

pub trait Acceptor<T> {
    fn accept(&self, visitor: &dyn Visitor<T>) -> T;
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Expr(Expr),
    Print(Expr),
    Var { name: Token, initializer: Expr },
}

impl<T> Acceptor<T> for Stmt {
    fn accept(&self, visitor: &dyn Visitor<T>) -> T {
        match self {
            Stmt::Expr(expr) => visitor.visit_expr(expr),
            Stmt::Print(expr) => visitor.visit_print(expr),
            Stmt::Var { name, initializer } => visitor.visit_var(&name.lexeme, initializer),
        }
    }
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stmt::Expr(expr) => write!(f, "{}", expr),
            Stmt::Print(expr) => write!(f, "print {}", expr),
            Stmt::Var { name, .. } => write!(f, "var {};", name),
        }
    }
}
