use core::fmt;

use crate::lexer::token::Token;

use super::expr::Expr;

pub trait Visitor<T> {
    fn visit_expr(&mut self, expr: &Expr) -> T;
    fn visit_print(&mut self, expr: &Expr) -> T;
    fn visit_var(&mut self, name: &Token, initializer: &Option<Expr>) -> T;
    fn visit_block(&mut self, statements: &Vec<Stmt>) -> T;
    fn visit_if(
        &mut self,
        condition: &Expr,
        then_branch: &Stmt,
        else_branch: &Option<Box<Stmt>>,
    ) -> T;
    fn visit_while(&mut self, condition: &Expr, body: &Stmt) -> T;
}

pub trait Acceptor<T> {
    fn accept(&self, visitor: &mut dyn Visitor<T>) -> T;
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Expr(Expr),
    Print(Expr),
    Var {
        name: Token,
        initializer: Option<Expr>,
    },
    Block {
        statements: Vec<Stmt>,
    },
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    While {
        condition: Expr,
        body: Box<Stmt>,
    },
}

impl<T> Acceptor<T> for Stmt {
    fn accept(&self, visitor: &mut dyn Visitor<T>) -> T {
        match self {
            Stmt::Expr(expr) => visitor.visit_expr(expr),
            Stmt::Print(expr) => visitor.visit_print(expr),
            Stmt::Var { name, initializer } => visitor.visit_var(&name, initializer),
            Stmt::Block { statements } => visitor.visit_block(statements),
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => visitor.visit_if(condition, then_branch, else_branch),
            Stmt::While { condition, body } => visitor.visit_while(condition, body),
        }
    }
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stmt::Expr(expr) => write!(f, "{}", expr),
            Stmt::Print(expr) => write!(f, "print {}", expr),
            Stmt::Var { name, .. } => write!(f, "var {};", name),
            Stmt::Block { statements } => {
                write!(
                    f,
                    "{}",
                    statements
                        .iter()
                        .map(|s| s.to_string())
                        .collect::<Vec<String>>()
                        .join("\n")
                )
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => write!(
                f,
                "if {} {} {}",
                condition,
                then_branch,
                else_branch
                    .as_ref()
                    .map_or("".to_string(), |b| b.to_string())
            ),

            Stmt::While { condition, body } => write!(f, "while ({}) {} ", condition, body),
        }
    }
}
