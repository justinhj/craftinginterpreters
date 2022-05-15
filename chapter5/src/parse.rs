use crate::scan::{num_format, Token, TokenInstance};
use std::fmt::Display;

pub enum Literal {
    String(String),
    Boolean(bool),
    Number(f64),
    Nil,
}

#[derive(Debug)]
pub enum Operator {
    Equal,
    Minus,
    Plus,
    Star,
    Bang,
    BangEqual,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Slash,
    And,
    Or,
}

pub enum Expr {
    Binary(Box<Expr>, Operator, Box<Expr>),
    Unary(Operator, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Literal),
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Binary(l, operator, r) => write!(f, "({:?} {} {})",operator,l,r),
            Expr::Unary(operator, expr) => write!(f, "({:?} {})", operator, expr),
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

// Implement the expression parser

// Not used yet
#[derive(Debug)]
pub struct CustomError {
    message: String,
}

struct ParseState<'a> {
    source: &'a [TokenInstance],
    current: usize,
}

// Grammar
// expression -> equality ;
// equality -> comparison ( ( "!=" | "==" ) ) comparison )* ;
// comparison -> term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
// term -> factor ( ( "-" | "+" ) ) factor )* ;
// factor -> unary ( ( "/" | "*" ) ) unary )* ;
// unary -> ( "!" | "-" ) unary | primary ;
// primary -> NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;


fn parse(input: &TokenInstance) -> Result<Vec<TokenInstance>, CustomError> {
    // let tokens = parse_statements(input);

    todo!()
}

fn parse_expression(ps: &mut ParseState) -> Expr {
    parse_equality(ps)
}

fn parse_equality(ps: &mut ParseState) -> Expr {
    let left = parse_comparison(ps);
    let token = advance(ps);
    let comparison_operator = match &token.token_type {
        Token::BangEqual => Operator::BangEqual,
        Token::EqualEqual => Operator::EqualEqual,
        _ => panic!("Failed matching comparison operator {:?} {}", token, token.line)
    };
    let right = parse_comparison(ps);
    Expr::Binary(Box::new(left), comparison_operator, Box::new(right))
}

fn parse_comparison(ps: &mut ParseState) -> Expr {
    todo!()
}

fn parse_primary(ps: &mut ParseState) -> Expr {
    let token = advance(ps);

    match &token.token_type {
        Token::True => Expr::Literal(Literal::Boolean(true)),
        Token::False => Expr::Literal(Literal::Boolean(false)),
        Token::Nil => Expr::Literal(Literal::Nil),
        Token::Number(n) => Expr::Literal(Literal::Number(*n)),
        Token::String(s) => Expr::Literal(Literal::String(s.clone())),
        _ => panic!("Failed matching primary {:?} {}", token, token.line)
    }
    // TODO parse ( expression )
}

// Helpers
fn is_at_end(ps: &ParseState) -> bool {
    match ps.source.get(ps.current) {
        Some(token_instance) => {
            match token_instance.token_type {
                Token::Eof => true,
                _ => false
            }
        },
        _ => false
    }
}

fn advance<'a>(ps: &'a mut ParseState) -> &'a TokenInstance {
    if !is_at_end(ps) {
        ps.current = ps.current + 1;
    }
    previous(ps)
}

fn previous<'a>(ps: &'a ParseState) -> &'a TokenInstance {
    ps.source.get(ps.current - 1).unwrap()
}

fn peek<'a>(ps: &'a ParseState) -> &'a TokenInstance {
    ps.source.get(ps.current).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_expression_kitchen_sink() {
        let expr: Expr = Expr::Binary(
            Box::new(Expr::Literal(Literal::Number(100.0))),
            Operator::Plus,
            Box::new(Expr::Literal(Literal::Number(200.0))),
        );

        assert_eq!("(+ 100.0 200.0)", format!("{}", expr));
    }
}
