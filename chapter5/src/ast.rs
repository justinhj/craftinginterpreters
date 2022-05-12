use crate::scan::{num_format, Token};
use std::fmt::Display;

pub enum Literal {
    String(String),
    Boolean(bool),
    Number(f64),
    Nil,
}

pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Unary(Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Literal),
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Binary(l, operator, r) => write!(f, "({} {} {})",operator,l,r),
            Expr::Unary(operator, expr) => write!(f, "({} {})", operator, expr),
            Expr::Grouping(expr) => write!(f, "(grouping {})", expr),
            Expr::Literal(literal) => write!(f, "{}", literal),
        }
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Number(n) => write!(f, "{}", num_format(*n)),
            Literal::String(string) => write!(f, "{}", string),
            Literal::Boolean(b) => {
                if *b {
                    write!(f, "true")
                } else {
                    write!(f, "false")
                }
            }
            Literal::Nil => write!(f, "nil"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_expression_kitchen_sink() {
        let expr = Expr::Binary(
            Box::new(Expr::Literal(Literal::Number(100.0))),
            Token::Plus,
            Box::new(Expr::Literal(Literal::Number(200.0))),
        );

        assert_eq!("(+ 100.0 200.0)", format!("{}", expr));
    }
}