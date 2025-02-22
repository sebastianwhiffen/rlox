use crate::error::syntax_error::SyntaxError;
use crate::token::token_type::TokenType;
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
            },
            '<' => {
                if self.match_next('=') {
                    self.add_token(TokenType::LessEqual)
                } else {
                    self.add_token(TokenType::Less)
                }
            },
            '>' => {
                if self.match_next('=') {
                    self.add_token(TokenType::GreaterEqual)
                } else {
                    self.add_token(TokenType::Greater)
                }
            }
            _ => return Err(SyntaxError),
        };
        Ok(())
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false
        }
        if self.source.chars().nth((self.current) as usize).unwrap() != expected {
            return false
        }
        self.current += 1;
        return true
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
        self.tokens.push(Token::new(
            token_type,
            "".to_string(),
            "".to_string(),
            self.line,
        ));
    }
}
