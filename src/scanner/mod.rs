use std::str::FromStr;
use std::thread::current;

use crate::error::syntax_error::SyntaxError;
use crate::token::token_type::{TokenType, KEYWORDS};
use crate::token::Token;

pub struct Scanner {
    pub source: String,
    pub tokens: Vec<Token>,
    pub start: i32,
    pub current: i32,
    pub line: i32,
}

impl Scanner {
    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>, Vec<SyntaxError>> {
        let mut errors: Vec<SyntaxError> = Default::default();
        while !&self.is_at_end() {
            self.start = self.current;
            if let Err(error) = self.scan_token() {
                errors.push(error);
            }
        }

        self.tokens.push(Token::new(
            TokenType::Eof,
            "".to_string(),
            "".to_string(),
            self.line,
        ));

        if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(&self.tokens)
        }
    }

    fn scan_token(&mut self) -> Result<(), SyntaxError> {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                if self.match_next('=') {
                    self.add_token(TokenType::BangEqual)
                } else {
                    self.add_token(TokenType::Bang)
                }
            }
            '=' => {
                if self.match_next('=') {
                    self.add_token(TokenType::EqualEqual)
                } else {
                    self.add_token(TokenType::Equal)
                }
            }
            '<' => {
                if self.match_next('=') {
                    self.add_token(TokenType::LessEqual)
                } else {
                    self.add_token(TokenType::Less)
                }
            }
            '>' => {
                if self.match_next('=') {
                    self.add_token(TokenType::GreaterEqual)
                } else {
                    self.add_token(TokenType::Greater)
                }
            }
            '/' => {
                if self.match_next('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            ' ' => {}
            '\r' => {}
            '\t' => {}
            '\n' => self.line += 1,
            '"' => {
                if let Err(error) = self.string() {
                    return Err(error);
                }
            }
            _ => {
                if self.is_digit(c) {
                    self.number()
                } else if self.is_alpha(c) {
                    self.identifier();
                } else {
                    return Err(SyntaxError {
                        line: self.line,
                        message: "".to_string(),
                    });
                };
            }
        };
        Ok(())
    }

    fn identifier(&mut self) {
        while self.is_alphanumeric(self.peek()) {
            self.advance();
        }
        let literal = self.source[(self.start as usize)..(self.current as usize)].to_string();
        if let Some((_, token)) = KEYWORDS.get_key_value(literal.as_str()) {
            self.add_token_with_literal(token.clone(), literal);
        }
        else{
            self.add_token(TokenType::Identifier);
        }
    }

    fn is_alpha(&self, c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
    }

    fn is_alphanumeric(&self, c: char) -> bool {
        self.is_alpha(c) || self.is_digit(c)
    }

    fn number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }
        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advance();
            while self.is_digit(self.peek()) {
                self.advance();
            }
        }
        self.add_token_with_literal(
            TokenType::Number,
            self.source[(self.start as usize)..(self.current as usize)].to_string(),
        );
    }

    fn peek_next(&self) -> char {
        if (self.current + 1) as usize >= self.source.len() {
            return '\0';
        };
        return self
            .source
            .chars()
            .nth((self.current + 1) as usize)
            .unwrap();
    }

    fn is_digit(&self, c: char) -> bool {
        return c >= '0' && c <= '9';
    }

    fn string(&mut self) -> Result<(), SyntaxError> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1
            }
            self.advance();
        }
        if self.is_at_end() {
            return Err(SyntaxError {
                line: self.line,
                message: "string was not terminated".to_string(),
            });
        }
        self.advance();

        let literal =
            self.source[((self.start + 1) as usize)..((self.current - 1) as usize)].to_string();
        self.add_token_with_literal(TokenType::String, literal);
        Ok(())
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        } else {
            return self.source.chars().nth(self.current as usize).unwrap();
        }
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth((self.current) as usize).unwrap() != expected {
            return false;
        }
        self.current += 1;
        return true;
    }

    fn advance(&mut self) -> char {
        let res = self.source.chars().nth((self.current) as usize).unwrap();
        self.current += 1;
        res
    }

    fn is_at_end(&self) -> bool {
        self.current >= (self.source.len() - 1) as i32
    }

    fn add_token(&mut self, token_type: TokenType) {
        let text = self.source[(self.start as usize)..(self.current as usize)].to_string();
        self.tokens
            .push(Token::new(token_type, text, "".to_string(), self.line));
    }

    fn add_token_with_literal(&mut self, token_type: TokenType, literal: String) {
        let text = self.source[(self.start as usize)..(self.current as usize)].to_string();
        self.tokens
            .push(Token::new(token_type, text, literal, self.line));
    }
}
