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
pub struct ParseError(String);

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
    Block(Vec<Stmt>),
    Expression(Expr),
    If(Expr, Vec<Stmt>, Vec<Stmt>),
    Print(Expr),
    VarDecl(String, Option<Expr>),
    While(Expr, Vec<Stmt>),
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
            Stmt::If(cond, then_stmt, else_stmt) => {
                write!(f, "if {} then {:?} else {:?}", cond, then_stmt, else_stmt)
            }
            Stmt::While(expr, stmt) => write!(f, "while {} {:?}", expr, stmt),
        }
    }
}

#[derive(Debug)]
pub enum Expr {
    Assign(String, Box<Expr>),
    Binary(Box<Expr>, Operator, Box<Expr>),
    Call(Box<Expr>, Vec<Expr>),
    Grouping(Box<Expr>),
    Literal(Value),
    Logical(Box<Expr>, Operator, Box<Expr>),
    Unary(Operator, Box<Expr>),
    Variable(String),
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Assign(ident, expr) => write!(f, "(set {} {})", ident, expr),
            Expr::Binary(l, operator, r) => write!(f, "({} {} {})", operator, l, r),
            Expr::Call(callee, params) => write!(f, "({} {:?})", callee, params),
            Expr::Grouping(expr) => write!(f, "(grouping {})", expr),
            Expr::Literal(literal) => write!(f, "{}", literal),
            Expr::Logical(l, operator, r) => write!(f, "{} {} {}", l, operator, r),
            Expr::Unary(operator, expr) => write!(f, "({} {})", operator, expr),
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

// statement -> exprStatement | printStatement | ifStatement | whileStatement | forStatement | block ;
// forStatement -> "for" "(" ( varDecl | exprStmt | ";" )
//   expression? ";"
//   expression? ")" statement ;
// whileStatement -> "while" "(" expression ")" statement ;
// exprStatement -> expression ";" ;
// printStatement -> print expression ";" ;
// ifStatement -> "if" "(" expression ")" ( "else" expression )? ;

// expression -> assignment ;
// assignment -> IDENTIFIER "=" assignment | logic_or;
// logic_or -> logic_and ( "or" logic_and )* ;
// logic_and -> equality  ( "and" equality ) ;
// equality -> comparison ( ( "!=" | "==" ) ) comparison )* ;
// comparison -> term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
// term -> factor ( ( "-" | "+" ) ) factor )* ;
// factor -> unary ( ( "/" | "*" ) ) unary )* ;
// unary -> ( "!" | "-" ) unary | call ;
// call -> primary ( "(" arguments? ")" )* ;
// arguments -> expression ( "," expression )* ;

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
                        return Err(ParseError(
                            "Expected }} but reached end of input".to_string(),
                        ))
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
            let token = advance(ps).token_type.clone();
            match token {
                Token::Identifier(ident) => match advance(ps).token_type.clone() {
                    Token::Equal => {
                        let expr = parse_expression(ps)?;
                        match advance(ps).token_type.clone() {
                            Token::Semicolon | Token::Eof => Ok(Stmt::VarDecl(ident, Some(expr))),
                            token => Err(ParseError(format!(
                                "Unexpected token when parsing declaration: {}",
                                token
                            ))),
                        }
                    }
                    Token::Semicolon | Token::Eof => Ok(Stmt::VarDecl(ident, None)),
                    token => Err(ParseError(format!(
                        "Unexpected token when parsing declaration: {}",
                        token
                    ))),
                },
                thing => return Err(ParseError(format!("Expected identifier, got {}", thing))),
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
        Token::For => {
            advance(ps);
            return parse_for(ps);
        }
        Token::While => {
            advance(ps);
            return parse_while(ps);
        }
        Token::If => {
            advance(ps);
            return parse_if(ps);
        }
        _ => Stmt::Expression(parse_expression(ps)?),
    };

    match &advance(ps).token_type {
        Token::Semicolon | Token::Eof => Ok(response),
        token => Err(ParseError(format!(
            "Unexpected token when parsing statement: {}",
            token
        ))),
    }
}

fn parse_while(ps: &mut ParseState) -> Result<Stmt, ParseError> {
    expect(ps, Token::LeftParen)?;
    let cond = parse_expression(ps)?;
    expect(ps, Token::RightParen)?;
    let stmt = parse_block(ps)?;
    Ok(Stmt::While(cond, vec![stmt]))
}

fn parse_for(ps: &mut ParseState) -> Result<Stmt, ParseError> {
    expect(ps, Token::LeftParen)?;
    let initializer = match peek(ps).token_type.clone() {
        Token::Semicolon => {
            advance(ps);
            None
        }
        Token::Var => Some(parse_declaration(ps)?),
        _ => Some(parse_statement(ps)?),
    };
    let condition = match peek(ps).token_type.clone() {
        Token::Semicolon => {
            advance(ps);
            Expr::Literal(Value::Boolean(true))
        }
        _ => parse_expression(ps)?,
    };
    expect(ps, Token::Semicolon)?;
    let increment = match peek(ps).token_type.clone() {
        Token::RightParen => {
            advance(ps);
            None
        }
        _ => {
            let expr = parse_expression(ps)?;
            expect(ps, Token::RightParen)?;
            Some(expr)
        }
    };
    let mut while_body_stmts: Vec<Stmt> = vec![parse_block(ps)?];

    if let Some(inc) = increment {
        while_body_stmts.push(Stmt::Expression(inc))
    }

    let body = Stmt::While(condition, while_body_stmts);

    if let Some(init) = initializer {
        Ok(Stmt::Block(vec![init, body]))
    } else {
        Ok(body)
    }
}

fn parse_if(ps: &mut ParseState) -> Result<Stmt, ParseError> {
    expect(ps, Token::LeftParen)?;
    let cond = parse_expression(ps)?;
    expect(ps, Token::RightParen)?;
    let then_stmt = parse_block(ps)?;

    match peek(ps).token_type.clone() {
        Token::Else => {
            advance(ps);
            let else_stmt = parse_block(ps)?;
            Ok(Stmt::If(cond, vec![then_stmt], vec![else_stmt]))
        }
        _ => Ok(Stmt::If(cond, vec![then_stmt], vec![])),
    }
}

fn parse_expression(ps: &mut ParseState) -> ParseExprResult {
    parse_assignment(ps)
}

fn parse_assignment(ps: &mut ParseState) -> ParseExprResult {
    let expr = parse_or(ps)?;

    match peek(ps).token_type.clone() {
        Token::Equal => {
            advance(ps);
            let value = parse_assignment(ps)?;

            let name = match expr {
                Expr::Variable(name) => name,
                _ => {
                    return Err(ParseError(format!(
                        "Tried to assign to not a variable: {}",
                        expr
                    )))
                }
            };

            Ok(Expr::Assign(name, Box::new(value)))
        }
        _ => Ok(expr),
    }
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

fn parse_or(ps: &mut ParseState) -> ParseExprResult {
    let mut expr = parse_and(ps)?;
    loop {
        let peeked_token = peek(ps);
        match peeked_token.token_type {
            Token::Or => {
                advance(ps);
                let right = parse_and(ps)?;
                expr = Expr::Logical(Box::new(expr), Operator::Or, Box::new(right));
            }
            _ => return Ok(expr),
        }
    }
}

fn parse_and(ps: &mut ParseState) -> ParseExprResult {
    let mut expr = parse_equality(ps)?;
    loop {
        let peeked_token = peek(ps);
        match peeked_token.token_type {
            Token::And => {
                advance(ps);
                let right = parse_equality(ps)?;
                expr = Expr::Logical(Box::new(expr), Operator::And, Box::new(right));
            }
            _ => return Ok(expr),
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
        None => parse_call(ps),
    }
}

fn parse_call(ps: &mut ParseState) -> ParseExprResult {
    let mut expr = parse_primary(ps)?;

    loop {
        let peeked = peek(ps);
        match peeked.token_type.clone() {
            Token::LeftParen => {
                advance(ps);
                expr = finish_call(expr, ps)?
            }
            _ => break,
        };
    }

    Ok(expr)
}

fn finish_call(callee: Expr, ps: &mut ParseState) -> ParseExprResult {
    let mut argument_exprs = vec![];

    if !matches!(peek(ps).token_type.clone(), Token::RightParen) {
        loop {
            if argument_exprs.len() == 255 {
                return Err(ParseError(format!("Exceed maximum argument count calling function {:?}", callee)))
            } else {
                let expr = parse_expression(ps)?;
                argument_exprs.push(expr);
                match expect(ps, Token::Comma) {
                    Ok(_) => continue,
                    Err(_) => break,
                }
            }
        }
    }
    expect(ps, Token::RightParen)?;
    Ok(Expr::Call(Box::new(callee), argument_exprs))
}

// This is for when a primary finds a left paren. Parse an expression and expect
// a right paren.
fn parse_group(ps: &mut ParseState) -> ParseExprResult {
    let expr = parse_expression(ps)?;
    let token = advance(ps);

    match token.token_type {
        Token::RightParen => Ok(Expr::Grouping(Box::new(expr))),
        _ => Err(ParseError(format!(
            "Failed finding matching right paren {:?} {}",
            token, token.line
        ))),
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
        _ => Err(ParseError(format!(
            "Failed matching primary {:?} {}",
            token, token.line
        ))),
    }
}

// Helpers

// You are at the end if you encounter the Eof token or there are no more tokens
fn is_at_end(ps: &ParseState) -> bool {
    match ps.source.get(ps.current) {
        Some(token_instance) => matches!(token_instance.token_type, Token::Eof),
        _ => false,
    }
}

fn advance<'a>(ps: &'a mut ParseState) -> &'a TokenInstance {
    if !is_at_end(ps) {
        ps.current += 1;
    }
    previous(ps)
}

/// Expect will succeed and advance if the next token is the expected one, otherwise
/// it will return an error (and not advance in case you want to recover)
fn expect(ps: &mut ParseState, token: Token) -> Result<(), ParseError> {
    let next = advance(ps).token_type.clone();
    if next == token {
        Ok(())
    } else {
        if ps.current > 0 {
            ps.current -= 1;
        }
        Err(ParseError(format!(
            "Expected {} found {}",
            token, next
        )))
    }
}

// TODO could remove these unwraps and return results

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
