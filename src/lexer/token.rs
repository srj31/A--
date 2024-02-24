use std::fmt;

use super::token_type;

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    String(String),
    Int(i32),
    Boolean(bool),
    Float(f64),
    Nil,
    Identifier(String),
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::String(s) => write!(f, "{}", s),
            Literal::Int(i) => write!(f, "{}", i),
            Literal::Float(fl) => write!(f, "{}", fl),
            Literal::Boolean(b) => write!(f, "{}", b),
            Literal::Nil => write!(f, "nil"),
            Literal::Identifier(i) => write!(f, "{}", i),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: token_type::TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: u32,
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

    pub fn get_lexeme(&self) -> String {
        self.lexeme.clone()
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
