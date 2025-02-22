pub mod token_type;

pub struct Token {
    pub token_type: token_type::TokenType,
    pub lexeme: String,
    pub literal: String,
    pub line: i32,
}

impl Token {
    pub fn to_string(&self) -> String {
        format!("token: {:?}, lexeme: {}, literal: {}", self.token_type, self.lexeme, self.literal)
    }

    pub fn new(token_type: token_type::TokenType, lexeme: String, literal: String, line: i32) -> Token {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}