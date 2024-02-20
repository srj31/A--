use crate::error;

use super::token;
use super::token::Literal;
use super::token_type;
use super::token_type::TokenType;

pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<token::Token>,
    start: usize,
    current: usize,
    line: u32,
}

impl Scanner {
    pub fn new(source: &str) -> Scanner {
        Scanner {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<token::Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(token::Token::new(
            token_type::TokenType::EOF,
            "".to_string(),
            None,
            self.line,
        ));
        &self.tokens
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LEFT_PAREN, None),
            ')' => self.add_token(TokenType::RIGHT_PAREN, None),
            '{' => self.add_token(TokenType::LEFT_BRACE, None),
            '}' => self.add_token(TokenType::RIGHT_BRACE, None),
            ',' => self.add_token(TokenType::COMMA, None),
            '.' => self.add_token(TokenType::DOT, None),
            '-' => self.add_token(TokenType::MINUS, None),
            '+' => self.add_token(TokenType::PLUS, None),
            ';' => self.add_token(TokenType::SEMICOLON, None),
            '*' => self.add_token(TokenType::STAR, None),
            '!' => {
                let token_type = if self.is_next_char('=') {
                    TokenType::BANG_EQUAL
                } else {
                    TokenType::BANG
                };
                self.add_token(token_type, None);
            }
            '=' => {
                let token_type = if self.is_next_char('=') {
                    TokenType::EQUAL_EQUAL
                } else {
                    TokenType::EQUAL
                };
                self.add_token(token_type, None);
            }
            '<' => {
                let token_type = if self.is_next_char('=') {
                    TokenType::LESS_EQUAL
                } else {
                    TokenType::LESS
                };
                self.add_token(token_type, None);
            }
            '>' => {
                let token_type = if self.is_next_char('=') {
                    TokenType::GREATER_EQUAL
                } else {
                    TokenType::GREATER
                };
                self.add_token(token_type, None);
            }
            '/' => {
                if self.is_next_char('/') {
                    while !self.is_at_end() && self.peek() != '\n' {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::SLASH, None);
                }
            }
            ' ' | '\t' | '\r' => {}
            '\n' => self.line += 1,
            '"' => self.handle_string(),
            '0'..='9' => self.handle_number(),
            'a'..='z' | 'A'..='Z' => self.handle_identifier(),
            _ => {
                error(self.line, "Unexpected character.");
            }
        }
    }

    fn handle_identifier(&mut self) {
        while self.peek().is_alphanumeric() {
            self.advance();
        }
        let text = self.source[self.start..self.current]
            .iter()
            .collect::<String>();
        let token_type = self.lookup_keyword(&text);
        self.add_token(token_type, None);
    }

    fn lookup_keyword(&self, text: &str) -> TokenType {
        token_type::KEYWORDS
            .get(text)
            .unwrap_or(&token_type::TokenType::IDENTIFIER)
            .clone()
    }

    fn handle_number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();
            while self.peek().is_ascii_digit() {
                self.advance();
            }
            self.add_token(
                token_type::TokenType::NUMBER,
                Some(Literal::Float(
                    self.source[self.start..self.current]
                        .iter()
                        .collect::<String>()
                        .parse::<f64>()
                        .unwrap(),
                )),
            );
        } else {
            self.add_token(
                token_type::TokenType::NUMBER,
                Some(Literal::Int(
                    self.source[self.start..self.current]
                        .iter()
                        .collect::<String>()
                        .parse::<i32>()
                        .unwrap(),
                )),
            );
        }
    }

    fn peek_next(&self) -> char {
        *self
            .source
            .get(self.current + 1)
            .or_else(|| Some(&'\0'))
            .unwrap()
    }

    fn handle_string(&mut self) {
        while !self.is_at_end() && self.peek() != '"' {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            error(self.line, "Unterminated string.");
            return;
        }
        self.advance();
        self.add_token(
            token_type::TokenType::STRING,
            Some(Literal::String(
                self.source[self.start + 1..self.current - 1]
                    .iter()
                    .collect(),
            )),
        );
    }

    fn is_next_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if *self.source.get(self.current).unwrap() != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn advance(&mut self) -> char {
        let c = self
            .source
            .get(self.current)
            .expect("Current is out of bounds");
        self.current += 1;
        *c
    }

    fn peek(&self) -> char {
        *self
            .source
            .get(self.current)
            .or_else(|| Some(&'\0'))
            .unwrap()
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<Literal>) {
        self.tokens.push(token::Token::new(
            token_type,
            self.source[self.start..self.current].iter().collect(),
            literal,
            self.line,
        ));
    }
}
