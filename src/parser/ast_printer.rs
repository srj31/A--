struct AstPrinter;
use super::expr::{Acceptor, Expr, Operator, Visitor};
use crate::lexer::{token, token::Token};

impl AstPrinter {
    pub fn print(&mut self, expr: &Expr) -> String {
        expr.accept(self)
    }

    fn parenthesize(&mut self, name: String, exprs: Vec<Expr>) -> String {
        let mut string = String::new();
        string.push_str("(");
        string.push_str(&name);
        for expr in exprs {
            string.push_str(" ");
            string.push_str(&expr.accept(self))
        }
        string.push_str(")");
        string
    }
}

impl Visitor<String> for AstPrinter {
    fn visit_binary(&mut self, left: &Expr, operator: &Operator, right: &Expr) -> String {
        self.parenthesize(operator.to_string(), vec![left.clone(), right.clone()])
    }

    fn visit_grouping(&mut self, expression: &Expr) -> String {
        self.parenthesize("group".to_string(), vec![expression.clone()])
    }

    fn visit_literal(&mut self, expr: &token::Literal) -> String {
        expr.to_string()
    }

    fn visit_unary(&mut self, operator: &Operator, right: &Expr) -> String {
        self.parenthesize(operator.to_string(), vec![right.clone()])
    }

    fn visit_variable(&mut self, name: &Token) -> String {
        name.lexeme.clone()
    }

    fn visit_assignment(&mut self, name: &Token, value: &Expr) -> String {
        self.parenthesize(format!("= {}", name.lexeme), vec![value.clone()])
    }

    fn visit_logical(&mut self, left: &Expr, operator: &Operator, right: &Expr) -> String {
        self.parenthesize(operator.to_string(), vec![left.clone(), right.clone()])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::expr::Operator;

    #[test]
    fn test_print() {
        let expr = Expr::Binary {
            left: Box::new(Expr::Binary {
                left: Box::new(Expr::Literal {
                    value: token::Literal::Int(1),
                }),
                operator: Operator::Plus,
                right: Box::new(Expr::Literal {
                    value: token::Literal::Int(2),
                }),
            }),
            operator: Operator::Star,
            right: Box::new(Expr::Literal {
                value: token::Literal::Int(3),
            }),
        };
        assert_eq!(AstPrinter {}.print(&expr), "(* (+ 1 2) 3)");
    }
}
