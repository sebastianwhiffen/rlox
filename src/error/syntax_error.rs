use std::fmt;

pub struct SyntaxError {
    pub line: i32,
    pub message: String,
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Syntax error occurred. {} line: {}",
            self.message, self.line
        )
    }
}
