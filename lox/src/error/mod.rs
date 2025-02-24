pub mod syntax_error;

pub enum Error {
    SyntaxError(syntax_error::SyntaxError),
}