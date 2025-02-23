pub mod parser {

    use crate::token::Token;

    trait Expr {
        fn get_left(&mut self) -> &Box<dyn Expr>;
        fn get_operator(&mut self) -> &Token;
        fn get_right(&mut self) -> &Box<dyn Expr>;
    }

    struct Binary {
        left: Box<dyn Expr>,
        operator: Token,
        right: Box<dyn Expr>,
    }

    impl Expr for Binary {
        fn get_left(&mut self) -> &Box<dyn Expr> {
            &self.left
        }

        fn get_operator(&mut self) -> &Token {
            &self.operator
        }

        fn get_right(&mut self) -> &Box<dyn Expr> {
            &self.right
        }
    }
}
