use crate::token::Token;
use crate::token::token_type::TokenType;

pub struct Scanner{
source: String,
tokens: Vec<Token>,
start: i32,
current: i32,
line: i32,
}

impl Scanner{
    fn scan_tokens(&mut self) -> &Vec<Token>{
        while ! &self.is_at_end(){
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token::new(TokenType::Eof, "".to_string(), "".to_string(), self.line));
        &self.tokens
    }

    fn scan_token(&mut self){
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Comma),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            // _ => self.add_token(TokenType::Eof),
        }
    }

    fn advance(&mut self)-> char{
        self.current += 1;
        self.source.chars().nth(self.current as usize).unwrap()
    }

    fn is_at_end(&self) -> bool{
        self.current >= self.source.len() as i32
    }

    fn add_token(&mut self, token_type: TokenType){
        self.tokens.push(Token::new(token_type, "".to_string(), "".to_string(), self.line));
    }
}