use token::TokenType;

pub mod token {
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
    pub token_type: token::TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: i32,
}

impl Default for Token {
    fn default() -> Self {
        Token {
            token_type: TokenType::Eof,
            lexeme: "".to_string(),
            literal: None,
            line: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Literal {
    Nil,
    Boolean(bool),
    Number(f64),
    String(String),
}

impl Token {
    pub fn to_string(&self) -> String {
        format!(
            "{:?}, {}, {:?}",
            self.token_type, self.lexeme, self.literal
        )
    }
}
