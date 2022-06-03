use crate::scan::{num_format, Token, TokenInstance};
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Operator::Equal => write!(f, "="),
            Operator::Minus => write!(f, "-"),
            Operator::Plus => write!(f, "+"),
            Operator::Star => write!(f, "*"),
            Operator::Bang => write!(f, "!"),
            Operator::BangEqual => write!(f, "!="),
            Operator::EqualEqual => write!(f, "=="),
            Operator::Greater => write!(f, ">"),
            Operator::GreaterEqual => write!(f, ">="),
            Operator::Less => write!(f, "<"),
            Operator::LessEqual => write!(f, "<="),
            Operator::Slash => write!(f, "/"),
            Operator::And => write!(f, "and"),
            Operator::Or => write!(f, "or"),
        }
    }
}

#[derive(Debug)]
pub enum Stmt {
    VarDecl(String, Option<Expr>),
    Expression(Expr),
    Print(Expr),
    Block(Vec<Stmt>),
}

impl Display for Stmt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Stmt::Block(stmts) => {
                write!(f, "{{")?;
                for stmt in stmts {
                    write!(f, "{}", stmt)?;
                }
                write!(f, "}}")
            }
            Stmt::VarDecl(ident, expr) => write!(f, "var {} = {:?};", ident, expr),
            Stmt::Expression(expr) => write!(f, "{};", expr),
            Stmt::Print(expr) => write!(f, "print {};", expr),
        }
    }
}

#[derive(Debug)]
pub enum Expr {
    Assign(String,Box<Expr>),
    Binary(Box<Expr>, Operator, Box<Expr>),
    Unary(Operator, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Value),
    Variable(String),
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Assign(ident,expr) => write!(f, "(set {} {})", ident, expr),
            Expr::Binary(l, operator, r) => write!(f, "({} {} {})", operator, l, r),
            Expr::Unary(operator, expr) => write!(f, "({} {})", operator, expr),
            Expr::Grouping(expr) => write!(f, "(grouping {})", expr),
            Expr::Literal(literal) => write!(f, "{}", literal),
            Expr::Variable(name) => write!(f, "{}", name),
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
//
// program -> block* EOF ;
// block -> "{" declaration* "}" ;
// declaration -> varDecl | statement ;
// varDelc -> "var" IDENTIFIER ( "=" expression )? ";" ;
// statement -> exprStatement | printStatement | block ;
// exprStatement -> expression ";" ;
// printStatement -> print expression ";" ;
// expression -> equality ;
// equality -> comparison ( ( "!=" | "==" ) ) comparison )* ;
// comparison -> term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
// term -> factor ( ( "-" | "+" ) ) factor )* ;
// factor -> unary ( ( "/" | "*" ) ) unary )* ;
// unary -> ( "!" | "-" ) unary | primary ;
// primary -> NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" | IDENTIFIER ;

type ParseExprResult = Result<Expr, ParseError>;

pub fn parse(input: &[TokenInstance]) -> Result<Vec<Stmt>, ParseError> {
    let mut ps = ParseState {
        source: input,
        current: 0,
    };

    let mut statements = vec![];
    while !is_at_end(&ps) {
        let statement = parse_block(&mut ps)?;
        statements.push(statement);
    }
    Ok(statements)
}

fn parse_block(ps: &mut ParseState) -> Result<Stmt, ParseError> {
    let mut statements = vec![];
    match peek(ps).token_type.clone() {
        Token::LeftBrace => {
            advance(ps);
            loop {
                match peek(ps).token_type.clone() {
                    Token::RightBrace => {
                        advance(ps);
                        return Ok(Stmt::Block(statements));
                    }
                    Token::Eof => {
                        return Err(ParseError {
                            message: format!("Expected }} but reached end of input"),
                        })
                    }
                    _ => {
                        let stmt = parse_block(ps)?;
                        statements.push(stmt)
                    }
                }
            }
        }
        _ => parse_declaration(ps),
    }
}

fn parse_declaration(ps: &mut ParseState) -> Result<Stmt, ParseError> {
    match peek(ps).token_type.clone() {
        Token::Var => {
            advance(ps);
            let ident = advance(ps).token_type.clone();
            match ident {
                Token::Identifier(ident) => match advance(ps).token_type.clone() {
                    Token::Equal => {
                        let expr = parse_expression(ps)?;
                        match advance(ps).token_type.clone() {
                            Token::Semicolon | Token::Eof => {
                                Ok(Stmt::VarDecl(ident.to_string(), Some(expr)))
                            }
                            token @ _ => Err(ParseError {
                                message: format!(
                                    "Unexpected token when parsing declaration: {}",
                                    token
                                ),
                            }),
                        }
                    }
                    Token::Semicolon | Token::Eof => Ok(Stmt::VarDecl(ident.to_string(), None)),
                    token @ _ => Err(ParseError {
                        message: format!("Unexpected token when parsing declaration: {}", token),
                    }),
                },
                thing @ _ => {
                    return Err(ParseError {
                        message: format!("Expected identifier, got {}", thing),
                    })
                }
            }
        }
        _ => parse_statement(ps),
    }
}

fn parse_statement(ps: &mut ParseState) -> Result<Stmt, ParseError> {
    let peeked = peek(ps);

    let response = match peeked.token_type {
        Token::Print => {
            advance(ps);
            let expr = parse_expression(ps)?;
            Stmt::Print(expr)
        }
        _ => Stmt::Expression(parse_expression(ps)?),
    };

    match &advance(ps).token_type {
        Token::Semicolon | Token::Eof => Ok(response),
        token @ _ => Err(ParseError {
            message: format!("Unexpected token when parsing statement: {}", token),
        }),
    }
}

fn parse_expression(ps: &mut ParseState) -> ParseExprResult {
    parse_equality(ps)
}

fn parse_equality(ps: &mut ParseState) -> ParseExprResult {
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

fn parse_comparison(ps: &mut ParseState) -> ParseExprResult {
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

fn parse_term(ps: &mut ParseState) -> ParseExprResult {
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

fn parse_factor(ps: &mut ParseState) -> ParseExprResult {
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

fn parse_unary(ps: &mut ParseState) -> ParseExprResult {
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
fn parse_group(ps: &mut ParseState) -> ParseExprResult {
    let expr = parse_expression(ps)?;
    let token = advance(ps);

    match token.token_type {
        Token::RightParen => Ok(Expr::Grouping(Box::new(expr))),
        _ => Err(ParseError {
            message: format!(
                "Failed finding matching right paren {:?} {}",
                token, token.line
            ),
        }),
    }
}

fn parse_primary(ps: &mut ParseState) -> ParseExprResult {
    let token = advance(ps);

    match &token.token_type {
        Token::True => Ok(Expr::Literal(Value::Boolean(true))),
        Token::False => Ok(Expr::Literal(Value::Boolean(false))),
        Token::Nil => Ok(Expr::Literal(Value::Nil)),
        Token::Number(n) => Ok(Expr::Literal(Value::Number(*n))),
        Token::String(s) => Ok(Expr::Literal(Value::String(s.clone()))),
        Token::Identifier(i) => Ok(Expr::Variable(i.to_string())),
        Token::LeftParen => parse_group(ps),
        _ => Err(ParseError {
            message: format!("Failed matching primary {:?} {}", token, token.line),
        }),
    }
}

// Helpers

// You are at the end if you encounter the Eof token or there are no more tokens
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

fn expect(ps: &mut ParseState, token: Token) -> Result<(), ParseError> {
    let next = advance(ps);
    if next.token_type == token {
        Ok(())
    } else {
        Err(ParseError {
            message: format!("Expected {} found {}", token, next.token_type),
        })
    }
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
