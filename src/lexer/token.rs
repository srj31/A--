use std::fmt;

use super::token_type;

#[derive(Debug)]
pub enum Literal {
    String(String),
    Int(i32),
    Float(f64),
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::String(s) => write!(f, "{}", s),
            Literal::Int(i) => write!(f, "{}", i),
            Literal::Float(fl) => write!(f, "{}", fl),
        }
    }
}

#[derive(Debug)]
pub struct Token {
    token_type: token_type::TokenType,
    lexeme: String,
    literal: Option<Literal>,
    line: u32,
}

impl Token {
    pub fn new(
        token_type: token_type::TokenType,
        lexeme: String,
        literal: Option<Literal>,
        line: u32,
    ) -> Token {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Token {{ token_type: {:?}, lexeme: {}, literal: {}, line: {} }}",
            self.token_type,
            self.lexeme,
            match &self.literal {
                Some(l) => l.to_string(),
                None => "null".to_string(),
            },
            self.line
        )
    }
}
