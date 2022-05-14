use crate::scan::{num_format, Token};
use std::fmt::Display;
use nom::bytes::complete::take;
use nom:: {
    IResult
};

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

// Implement the expression parser

#[derive(Debug)]
pub enum ParseError {
    Error,
}

// Grammar
// expression -> equality ;
// equality -> comparison ( ( "!=" | "==" ) ) comparison )* ;
// comparison -> term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
// term -> factor ( ( "-" | "+" ) ) factor )* ;
// factor -> unary ( ( "/" | "*" ) ) unary )* ;
// unary -> ( "!" | "-" ) unary | primary ;
// primary -> NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;

fn parse(input: &Token) -> Result<Vec<Token>, ParseError> {
    let tokens = parse_statements(input);


    todo!()
}

fn parse_statements(input: &Token) -> IResult<&Token, Vec<Token>> {
    todo!()
}

 
fn parse_primary(input: &Token) -> IResult<&Token, Vec<Token>> {
    // alt((
    // tag(Token::Number(_))
    todo!()
}

fn parse_number<'a>(input: Vec<Token>) -> IResult<&'a Token, Expr> {
    let (remainder, token) = take(1usize)(input)?;
    match token {
       Token::Number(n) => Ok((remainder, Expr::Literal(Literal::Number(n)))),
       Err(err) => Err(err)
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
