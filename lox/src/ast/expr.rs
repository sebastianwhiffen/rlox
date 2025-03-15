pub mod expr {
    use crate::ast::token::Token;

    
    pub enum Expr {
        Literal(Token),
        Unary(Token, Box<Expr>),
        Binary(Box<Expr>, Token, Box<Expr>),
        Grouping(Box<Expr>),
    }

    pub trait Visitor<T> {
        fn visit_literal(&mut self, expr: &Expr, token: &Token) -> T;
        fn visit_unary(&mut self, expr: &Expr, op: &Token, right: &Expr) -> T;
        fn visit_binary(&mut self, expr: &Expr, left: &Expr, op: &Token, right: &Expr) -> T;
        fn visit_grouping(&mut self, expr: &Expr, inside: &Expr) -> T;
    }

    impl Expr {
        pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
            match self {
                Expr::Literal(token) => visitor.visit_literal(self, token),
                Expr::Unary(op, right) => visitor.visit_unary(self, op, right),
                Expr::Binary(left, op, right) => visitor.visit_binary(self, left, op, right),
                Expr::Grouping(inside) => visitor.visit_grouping(self, inside),
            }
        }
    }
}
