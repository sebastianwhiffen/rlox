pub mod parser {

    use lox_macros::define_ast;

    use crate::token::Token;

    define_ast!(
    "Binary : left: Box<dyn Expr>, operator: Token, right: Box<dyn Expr>, 
    | Grouping : expression: Box<dyn Expr>
    | Literal : value: Box<dyn std::any::Any>
    | Unary : operator: Token, right: Box<dyn Expr> 
    "
    );
    // define_ast!("Grouping : expression: Expr ");
    // define_ast!("Literal : value: Object ");
    // define_ast!("Unary : operator: Token, right: Expr ");
}
