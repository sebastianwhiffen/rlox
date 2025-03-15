pub mod printer {
    use crate::ast::{
        expr::expr::{Expr, Visitor},
        token::Token,
    };

    pub struct Printer {}

    impl Printer {
        pub fn print(&mut self, expr: Expr) -> String {
            expr.accept(self)
        }
    }

    impl Visitor<String> for Printer {
        fn visit_literal(&mut self, _expr: &Expr, token: &Token) -> String {
            token.lexeme.clone()
        }

        fn visit_unary(&mut self, _expr: &Expr, op: &Token, right: &Expr) -> String {
            parenthesize(&op.lexeme, vec![right], self)
        }

        fn visit_binary(&mut self, _expr: &Expr, left: &Expr, op: &Token, right: &Expr) -> String {
            parenthesize(&op.lexeme, vec![left, right], self)
        }

        fn visit_grouping(&mut self, _expr: &Expr, inside: &Expr) -> String {
            parenthesize(&"group".to_string(), vec![inside], self)
        }
    }

    fn parenthesize(name: &String, exprs: Vec<&Expr>, v: &mut dyn Visitor<String>) -> String {
        let mut s = String::new();
        s.push_str(&format!("({}", name));
        for expr in exprs {
            s.push_str(&format!(" {}", expr.accept(v)));
        }
        s.push_str(")");
        s
    }
}
