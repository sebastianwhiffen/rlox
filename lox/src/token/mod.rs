pub mod token_type {
    use lazy_static::lazy_static;
    use std::collections::HashMap;
    use strum_macros::EnumString;

    #[derive(Debug, Clone, PartialEq, EnumString)]
    pub enum TokenType {
        LeftParen,
        RightParen,
        LeftBrace,
        RightBrace,
        Comma,
        Dot,
        Minus,
        Plus,
        Semicolon,
        Slash,
        Star,
        Bang,
        BangEqual,
        Equal,
        EqualEqual,
        Greater,
        GreaterEqual,
        Less,
        LessEqual,
        Identifier,
        String,
        Number,
        And,
        Class,
        Else,
        False,
        Fun,
        For,
        If,
        Nil,
        Or,
        Print,
        Return,
        Super,
        This,
        True,
        Var,
        While,
        Eof,
    }

    lazy_static! {
        pub static ref KEYWORDS: HashMap<&'static str, TokenType> = {
            let mut m = HashMap::new();
            m.insert("and", TokenType::And);
            m.insert("class", TokenType::Class);
            m.insert("else", TokenType::Else);
            m.insert("false", TokenType::False);
            m.insert("for", TokenType::For);
            m.insert("fun", TokenType::Fun);
            m.insert("if", TokenType::If);
            m.insert("nil", TokenType::Nil);
            m.insert("or", TokenType::Or);
            m.insert("print", TokenType::Print);
            m.insert("return", TokenType::Return);
            m.insert("super", TokenType::Super);
            m.insert("this", TokenType::This);
            m.insert("true", TokenType::True);
            m.insert("var", TokenType::Var);
            m.insert("while", TokenType::While);
            m
        };
    }
}

pub struct Token {
    pub token_type: token_type::TokenType,
    pub lexeme: String,
    pub literal: String,
    pub line: i32,
}

impl Token {
    pub fn to_string(&self) -> String {
        format!(
            "token: {:?}, lexeme: {}, literal: {}",
            self.token_type, self.lexeme, self.literal
        )
    }

    pub fn new(
        token_type: token_type::TokenType,
        lexeme: String,
        literal: String,
        line: i32,
    ) -> Token {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}
