use core::fmt;

use crate::{
    lexer::{
        token::{self, Token},
        token_type,
    },
    log,
};

use super::expr::Expr;

struct Parser {
    current: usize,
    tokens: Vec<Token>,
}

struct ParserError {
    message: String,
    token: Token,
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ParserError")
    }
}

type Result<T> = std::result::Result<T, ParserError>;

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        assert_ne!(tokens.len(), 0);
        Self { current: 0, tokens }
    }

    pub fn parse(&mut self) -> Option<Expr> {
        match self.expression() {
            Ok(expr) => Some(expr),
            Err(err) => {
                log::log_message::print_code_error(err.token.line, &err.message);
                None
            }
        }
    }

    fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if self.previous().token_type == token_type::TokenType::SEMICOLON {
                return;
            }
            match self.peek().token_type {
                token_type::TokenType::CLASS
                | token_type::TokenType::FUN
                | token_type::TokenType::VAR
                | token_type::TokenType::FOR
                | token_type::TokenType::IF
                | token_type::TokenType::WHILE
                | token_type::TokenType::PRINT
                | token_type::TokenType::RETURN => return,
                _ => {
                    self.advance();
                }
            }
        }
    }

    fn expression(&mut self) -> Result<Expr> {
        self.equality()
    }

    fn match_token(&mut self, types: Vec<token_type::TokenType>) -> bool {
        for t in types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&mut self, t: token_type::TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == t
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn previous(&mut self) -> &Token {
        self.tokens.get(self.current - 1).unwrap()
    }

    fn is_at_end(&mut self) -> bool {
        self.peek().token_type == token_type::TokenType::EOF
    }

    fn peek(&mut self) -> &Token {
        self.tokens.get(self.current).unwrap()
    }

    fn equality(&mut self) -> Result<Expr> {
        let mut expr = self.comparison()?;
        while self.match_token(vec![
            token_type::TokenType::BANG_EQUAL,
            token_type::TokenType::EQUAL_EQUAL,
        ]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator.into(),
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr> {
        let mut expr = self.term()?;
        while self.match_token(vec![
            token_type::TokenType::GREATER,
            token_type::TokenType::GREATER_EQUAL,
            token_type::TokenType::LESS,
            token_type::TokenType::LESS_EQUAL,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator.into(),
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr> {
        let mut expr = self.factor()?;
        while self.match_token(vec![
            token_type::TokenType::MINUS,
            token_type::TokenType::PLUS,
        ]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator.into(),
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr> {
        let mut expr = self.unary()?;
        while self.match_token(vec![
            token_type::TokenType::SLASH,
            token_type::TokenType::STAR,
        ]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator.into(),
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr> {
        if self.match_token(vec![
            token_type::TokenType::BANG,
            token_type::TokenType::MINUS,
        ]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            Ok(Expr::Unary {
                operator: operator.into(),
                right: Box::new(right),
            })
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expr> {
        if self.match_token(vec![token_type::TokenType::FALSE]) {
            return Ok(Expr::Literal {
                value: token::Literal::Boolean(false),
            });
        }
        if self.match_token(vec![token_type::TokenType::TRUE]) {
            return Ok(Expr::Literal {
                value: token::Literal::Boolean(true),
            });
        }
        if self.match_token(vec![token_type::TokenType::NIL]) {
            return Ok(Expr::Literal {
                value: token::Literal::Nil,
            });
        }

        if self.match_token(vec![
            token_type::TokenType::NUMBER,
            token_type::TokenType::STRING,
        ]) {
            return Ok(Expr::Literal {
                value: self.previous().literal.clone().unwrap(),
            });
        }

        if self.match_token(vec![token_type::TokenType::LEFT_PAREN]) {
            let expr = self.expression()?;
            self.consume(
                token_type::TokenType::RIGHT_PAREN,
                "Expect ')' after expression.",
            )?;
            return Ok(Expr::Grouping {
                expression: Box::new(expr),
            });
        }

        Err(ParserError {
            token: self.peek().clone(),
            message: "Expect expression.".to_string(),
        })
    }

    fn consume(&mut self, token_type: token_type::TokenType, message: &str) -> Result<Token> {
        if self.check(token_type) {
            return Ok(self.advance().clone());
        }
        Err(ParserError {
            token: self.peek().clone(),
            message: message.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use tests::token::Literal;

    use crate::parser::expr::Operator;

    use super::*;

    #[test]
    fn test_parser() {
        let tokens = vec![
            Token::new(
                token_type::TokenType::NUMBER,
                "1".to_string(),
                Some(Literal::Int(1)),
                1,
            ),
            Token::new(token_type::TokenType::PLUS, "+".to_string(), None, 1),
            Token::new(
                token_type::TokenType::NUMBER,
                "2".to_string(),
                Some(Literal::Int(2)),
                1,
            ),
            Token::new(token_type::TokenType::EOF, "".to_string(), None, 2),
        ];
        let expr = Expr::Binary {
            left: Box::new(Expr::Literal {
                value: token::Literal::Int(1),
            }),
            operator: Operator::Plus,
            right: Box::new(Expr::Literal {
                value: token::Literal::Int(2),
            }),
        };

        assert_eq!(Parser::new(tokens).parse(), Some(expr));
    }
}
