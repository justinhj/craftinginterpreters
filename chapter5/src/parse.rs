use crate::scan::{num_format, Token, TokenInstance};
use std::fmt::Display;

#[derive(Debug,Clone)]
pub enum Value {
    String(String),
    Boolean(bool),
    Number(f64),
    Nil,
}

#[derive(Debug)]
pub struct ParseError {
    message: String,
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

#[derive(Debug)]
pub enum Expr {
    Binary(Box<Expr>, Operator, Box<Expr>),
    Unary(Operator, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Value),
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Binary(l, operator, r) => write!(f, "({:?} {} {})", operator, l, r),
            Expr::Unary(operator, expr) => write!(f, "({:?} {})", operator, expr),
            Expr::Grouping(expr) => write!(f, "(grouping {})", expr),
            Expr::Literal(literal) => write!(f, "{}", literal),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", num_format(*n)),
            Value::String(string) => write!(f, "{}", string),
            Value::Boolean(b) => {
                if *b {
                    write!(f, "true")
                } else {
                    write!(f, "false")
                }
            }
            Value::Nil => write!(f, "nil"),
        }
    }
}

// Implement the expression parser

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

type ParseResult = Result<Expr, ParseError>;

pub fn parse(input: &[TokenInstance]) -> ParseResult {
    let mut ps = ParseState {
        source: input,
        current: 0,
    };
    parse_expression(&mut ps)
}

fn parse_expression(ps: &mut ParseState) -> ParseResult {
    parse_equality(ps)
}

fn parse_equality(ps: &mut ParseState) -> ParseResult {
    let mut expr = parse_comparison(ps)?;

    loop {
        let peeked_token = peek(ps);
        let equality_operator = match &peeked_token.token_type {
            Token::BangEqual => Some(Operator::BangEqual),
            Token::EqualEqual => Some(Operator::EqualEqual),
            _ => None,
        };

        match equality_operator {
            Some(operator) => {
                advance(ps);
                let right = parse_comparison(ps)?;
                expr = Expr::Binary(Box::new(expr), operator, Box::new(right))
            }
            None => return Ok(expr),
        }
    }
}

fn parse_comparison(ps: &mut ParseState) -> ParseResult {
    let mut expr = parse_term(ps)?;
    loop {
        let peeked_token = peek(ps);
        let operator = match peeked_token.token_type {
            Token::LessEqual => Some(Operator::LessEqual),
            Token::GreaterEqual => Some(Operator::GreaterEqual),
            Token::Greater => Some(Operator::Greater),
            Token::Less => Some(Operator::Less),
            _ => None,
        };
        match operator {
            Some(operator) => {
                advance(ps);
                let right = parse_term(ps)?;
                expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
            }
            None => return Ok(expr),
        }
    }
}

fn parse_term(ps: &mut ParseState) -> ParseResult {
    let mut expr = parse_factor(ps)?;
    loop {
        let peeked_token = peek(ps);
        let operator = match peeked_token.token_type {
            Token::Minus => Some(Operator::Minus),
            Token::Plus => Some(Operator::Plus),
            _ => None,
        };
        match operator {
            Some(operator) => {
                advance(ps);
                let right = parse_factor(ps)?;
                expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
            }
            None => return Ok(expr),
        }
    }
}

fn parse_factor(ps: &mut ParseState) -> ParseResult {
    let mut expr = parse_unary(ps)?;
    loop {
        let peeked_token = peek(ps);
        let operator = match peeked_token.token_type {
            Token::Slash => Some(Operator::Slash),
            Token::Star => Some(Operator::Star),
            _ => None,
        };
        match operator {
            Some(operator) => {
                advance(ps);
                let right = parse_unary(ps)?;
                expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
            }
            None => return Ok(expr),
        }
    }
}

fn parse_unary(ps: &mut ParseState) -> ParseResult {
    let token = peek(ps);

    let unary_op = match &token.token_type {
        Token::Bang => Some(Operator::Bang),
        Token::Minus => Some(Operator::Minus),
        _ => None,
    };

    match unary_op {
        Some(uo) => {
            advance(ps); // Need to advance since we peeked before only
            let unary = parse_unary(ps)?;
            Ok(Expr::Unary(uo, Box::new(unary)))
        }
        None => parse_primary(ps),
    }
}

// This is for when a primary finds a left paren. Parse an expression and expect
// a right paren.
fn parse_group(ps: &mut ParseState) -> ParseResult {
    let expr = parse_expression(ps);
    let token = advance(ps);

    match token.token_type {
        Token::RightParen => expr,
        _ => Err(ParseError {
            message: format!(
                "Failed finding matching right paren {:?} {}",
                token, token.line
            ),
        }),
    }
}

fn parse_primary(ps: &mut ParseState) -> ParseResult {
    let token = advance(ps);

    match &token.token_type {
        Token::True => Ok(Expr::Literal(Value::Boolean(true))),
        Token::False => Ok(Expr::Literal(Value::Boolean(false))),
        Token::Nil => Ok(Expr::Literal(Value::Nil)),
        Token::Number(n) => Ok(Expr::Literal(Value::Number(*n))),
        Token::String(s) => Ok(Expr::Literal(Value::String(s.clone()))),
        Token::LeftParen => parse_group(ps),
        _ => Err(ParseError {
            message: format!("Failed matching primary {:?} {}", token, token.line),
        }),
    }
}

// Helpers
fn is_at_end(ps: &ParseState) -> bool {
    match ps.source.get(ps.current) {
        Some(token_instance) => match token_instance.token_type {
            Token::Eof => true,
            _ => false,
        },
        _ => false,
    }
}

fn advance<'a>(ps: &'a mut ParseState) -> &'a TokenInstance {
    if !is_at_end(ps) {
        ps.current = ps.current + 1;
    }
    previous(ps)
}

// TODO can remove these unwraps and return results

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
            Box::new(Expr::Literal(Value::Number(100.0))),
            Operator::Plus,
            Box::new(Expr::Literal(Value::Number(200.0))),
        );

        assert_eq!("(+ 100.0 200.0)", format!("{}", expr));
    }
}
