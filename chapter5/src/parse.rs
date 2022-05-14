use crate::scan::{num_format, Token, TokenInstance};
use std::fmt::Display;
use nom:: {
    IResult,
    bytes::complete::take,
    InputTake,
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

// Not used yet
#[derive(Debug)]
pub struct CustomError {
    message: String,
}

struct ParseState<'a> {
    source: &'a [TokenInstance],
    position: usize,
}

// Grammar
// expression -> equality ;
// equality -> comparison ( ( "!=" | "==" ) ) comparison )* ;
// comparison -> term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
// term -> factor ( ( "-" | "+" ) ) factor )* ;
// factor -> unary ( ( "/" | "*" ) ) unary )* ;
// unary -> ( "!" | "-" ) unary | primary ;
// primary -> NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;

fn parse_primary(input: &TokenInstance) -> IResult<&TokenInstance, Vec<TokenInstance>> {
    // alt((
    todo!()
}

fn parse(input: &TokenInstance) -> Result<Vec<TokenInstance>, CustomError> {
    let tokens = parse_statements(input);


    todo!()
}

fn parse_statements(input: &TokenInstance) -> IResult<&TokenInstance, Vec<TokenInstance>> {
    todo!()
}

// impl InputTake for &[TokenInstance] {
// }

// fn parse_number<'a>(input: &mut ParseState) -> IResult<ParseState<'a>, Expr> {
//     let (remainder, token_instance) = take(1usize)(input)?;
//     // match token_instance.token_type {
//     //    Token::Number(n) => Ok((remainder, Expr::Literal(Literal::Number(n)))),
//     //    Err(err) => Err(err)
//     // }
//     Err(nom::Err::Error((remainder, nom::error::ErrorKind::Char)))
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_expression_kitchen_sink() {
        let expr: Expr = Expr::Binary(
            Box::new(Expr::Literal(Literal::Number(100.0))),
            Token::Plus,
            Box::new(Expr::Literal(Literal::Number(200.0))),
        );

        assert_eq!("(+ 100.0 200.0)", format!("{}", expr));
    }
}
