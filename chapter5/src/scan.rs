use lazy_static::lazy_static;
use nom::branch::alt;
use nom::bytes::complete::{is_not, tag, take_till};
use nom::character::complete::{
    alpha1, alphanumeric1, anychar, char, digit1, multispace1, not_line_ending,
};
use nom::combinator::{eof, fail, map, peek, recognize, value};
use nom::multi::many0;
use nom::sequence::{delimited, pair, tuple};
use nom::IResult;
use std::collections::HashMap;
use std::fmt;

#[derive(PartialEq, Clone)]
pub enum Token {
    // literals
    Identifier(String),
    Number(f64),
    String(String),
    // single character operators
    Equal,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Star,
    // single or double
    Bang,
    BangEqual,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Special
    Slash,
    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    // End marker
    Eof,
}

// We want to emulate the String format of Double in Java to make the tests pass
// This is described in the Java source/documentation as:
//   How many digits must be printed for the fractional part of
//   <i>m</i> or <i>a</i>? There must be at least one digit to represent
//   the fractional part, and beyond that as many, but only as many, more
//   digits as are needed to uniquely distinguish the argument value from
//   adjacent values of type {@code double}.
pub fn num_format(num: f64) -> String {
    let s = format!("{:.3}", num);
    if let Some(non_zero_pos) = s.rfind(|c: char| c != '0') {
        let zero_count = s.len() - (non_zero_pos + 1);
        let count = std::cmp::min(zero_count, 2);
        s[0..s.len() - count].to_string()
    } else {
        panic!("Unexpected number format {:?}", s);
    }
}

// Debug is written to pass the tests in the book rather for any specific purpose
impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Token::Identifier(string) => write!(f, "IDENTIFIER {}", string),
            Token::Number(num) => write!(f, "NUMBER {}", num_format(*num)),
            Token::String(string) => write!(f, "STRING \"{}\"", string),
            Token::Equal => write!(f, "EQUAL ="),
            Token::LeftParen => write!(f, "LEFT_PAREN ("),
            Token::RightParen => write!(f, "RIGHT_PAREN )"),
            Token::LeftBrace => write!(f, "LEFT_BRACE {{"),
            Token::RightBrace => write!(f, "RIGHT_BRACE }}"),
            Token::Comma => write!(f, "COMMA ,"),
            Token::Dot => write!(f, "DOT ."),
            Token::Minus => write!(f, "MINUS -"),
            Token::Plus => write!(f, "PLUS +"),
            Token::Semicolon => write!(f, "SEMICOLON ;"),
            Token::Star => write!(f, "STAR *"),
            Token::Bang => write!(f, "BANG !"),
            Token::BangEqual => write!(f, "BANG_EQUAL !="),
            Token::EqualEqual => write!(f, "EQUAL_EQUAL =="),
            Token::Greater => write!(f, "GREATER >"),
            Token::GreaterEqual => write!(f, "GREATER_EQUAL >="),
            Token::Less => write!(f, "LESS <"),
            Token::LessEqual => write!(f, "LESS_EQUAL <="),
            Token::And => write!(f, "AND and"),
            Token::Class => write!(f, "CLASS class"),
            Token::Else => write!(f, "ELSE else"),
            Token::False => write!(f, "FALSE false"),
            Token::Fun => write!(f, "FUN fun"),
            Token::For => write!(f, "FOR for"),
            Token::If => write!(f, "IF if"),
            Token::Nil => write!(f, "NIL nil"),
            Token::Or => write!(f, "OR or"),
            Token::Print => write!(f, "PRINT print"),
            Token::Return => write!(f, "RETURN return"),
            Token::Super => write!(f, "SUPER super"),
            Token::This => write!(f, "THIS this"),
            Token::True => write!(f, "TRUE true"),
            Token::Var => write!(f, "VAR var"),
            Token::While => write!(f, "WHILE while"),
            Token::Slash => write!(f, "SLASH /"),
            Token::Eof => write!(f, "EOF "),
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Token::Identifier(string) => write!(f, "{}", string),
            Token::Number(num) => write!(f, "{}", num_format(*num)),
            Token::String(string) => write!(f, "\"{}\"", string),
            Token::Equal => write!(f, "="),
            Token::LeftParen => write!(f, "("),
            Token::RightParen => write!(f, ")"),
            Token::LeftBrace => write!(f, "{{"),
            Token::RightBrace => write!(f, "}}"),
            Token::Comma => write!(f, ","),
            Token::Dot => write!(f, "."),
            Token::Minus => write!(f, "-"),
            Token::Plus => write!(f, "+"),
            Token::Semicolon => write!(f, ";"),
            Token::Star => write!(f, "*"),
            Token::Bang => write!(f, "!"),
            Token::BangEqual => write!(f, "!="),
            Token::EqualEqual => write!(f, "=="),
            Token::Greater => write!(f, ">"),
            Token::GreaterEqual => write!(f, ">="),
            Token::Less => write!(f, "<"),
            Token::LessEqual => write!(f, "<="),
            Token::And => write!(f, "and"),
            Token::Class => write!(f, "class"),
            Token::Else => write!(f, "else"),
            Token::False => write!(f, "false"),
            Token::Fun => write!(f, "fun"),
            Token::For => write!(f, "f"),
            Token::If => write!(f, "if"),
            Token::Nil => write!(f, "nil"),
            Token::Or => write!(f, "or"),
            Token::Print => write!(f, "print"),
            Token::Return => write!(f, "return"),
            Token::Super => write!(f, "super"),
            Token::This => write!(f, "this"),
            Token::True => write!(f, "true"),
            Token::Var => write!(f, "var"),
            Token::While => write!(f, "while"),
            Token::Slash => write!(f, "/"),
            Token::Eof => write!(f, ""),
        }
    }
}

// TODO map errors into this format
// see
// https://github.com/Geal/nom/blob/main/examples/custom_error.rs
#[derive(Debug)]
pub enum ScanError {
    UnexpectedChar(char),
    NumberFormatError(String),
    UnterminatedString(String),
    EndOfInput,
    Error,
}

// Scan the input string returning either a vector of tokens or the first error
pub fn scan(input: &str) -> Result<Vec<Token>, ScanError> {
    // let result = many0(scan_token)(input);
    let mut tokens: Vec<Token> = vec![];
    let mut rest = input;
    while !rest.is_empty() {
        match scan_token(rest) {
            Ok((new_rest, Some(token))) => {
                tokens.insert(tokens.len(), token);
                rest = new_rest;
            }
            Ok((new_rest, None)) => rest = new_rest,
            Err(err) => {
                println!("error {:?}", err);
                return Err(ScanError::Error); // TODO do better error handling here
            }
        }
    }

    tokens.push(Token::Eof);
    Ok(tokens)
}

fn scan_token(input: &str) -> IResult<&str, Option<Token>> {
    let peeker: IResult<&str, char> = peek(anychar)(input);
    match peeker {
        Ok((rest, c)) if c.is_ascii_whitespace() => value(None, multispace1)(rest),
        Ok((rest, '(')) => scan_single(rest, Token::LeftParen),
        Ok((rest, ')')) => scan_single(rest, Token::RightParen),
        Ok((rest, '{')) => scan_single(rest, Token::LeftBrace),
        Ok((rest, '}')) => scan_single(rest, Token::RightBrace),
        Ok((rest, ',')) => scan_single(rest, Token::Comma),
        Ok((rest, '.')) => scan_single(rest, Token::Dot),
        Ok((rest, '-')) => scan_single(rest, Token::Minus),
        Ok((rest, '+')) => scan_single(rest, Token::Plus),
        Ok((rest, ';')) => scan_single(rest, Token::Semicolon),
        Ok((rest, '*')) => scan_single(rest, Token::Star),
        Ok((rest, '=')) => scan_single_or_double(rest, '=', '=', Token::Equal, Token::EqualEqual),
        Ok((rest, '!')) => scan_single_or_double(rest, '!', '=', Token::Bang, Token::BangEqual),
        Ok((rest, '>')) => {
            scan_single_or_double(rest, '>', '=', Token::Greater, Token::GreaterEqual)
        }
        Ok((rest, '<')) => scan_single_or_double(rest, '<', '=', Token::Less, Token::LessEqual),
        Ok((rest, '/')) => scan_slash_or_comment(rest),
        Ok((rest, c)) if c.is_ascii_alphabetic() || c == '_' => scan_identifier_or_keyword(rest),
        Ok((rest, '"')) => scan_quoted_string(rest),
        Ok((rest, c)) if c.is_ascii_digit() => scan_number(rest),
        Ok((rest, unknown)) => {
            println!("unknown {:?} {:?}", rest, unknown);
            fail(rest)
        }
        Err(err) => {
            println!("err {:?}", err);
            Err(err)
        }
    }
}

// Single character symbols like *
fn scan_single(input: &str, token: Token) -> IResult<&str, Option<Token>> {
    map(anychar, |c| Some(token.clone()))(input)
}

fn scan_number(input: &str) -> IResult<&str, Option<Token>> {
    let fractional = recognize(tuple((digit1, tag("."), digit1)));
    map(alt((fractional, digit1)), |s: &str| {
        let number = s.parse::<f64>().unwrap();
        Some(Token::Number(number))
    })(input)
}

// Single OR double character symbols like = and ==
fn scan_single_or_double(
    input: &str,
    single: char,
    double: char,
    single_token: Token,
    double_token: Token,
) -> IResult<&str, Option<Token>> {
    let double_target: String = [single, double].iter().collect();
    let mut parser = map(
        alt((tag(&double_target[..]), tag(&double_target[0..1]))),
        |m: &str| {
            if m.len() == 2 {
                Some(double_token.clone())
            } else {
                Some(single_token.clone())
            }
        },
    );
    parser(input)
}

// Should be called after /* consumed
//   now find */ or /*
//   if you find /* call recursively
//    if you find */ you are done
fn scan_slash_star_comment(input: &str) -> IResult<&str, Option<Token>> {
    let mut open = 1;
    let mut remainder = input;
    loop {
        let (new_remainder, _) = take_till(|c| c == '*' || c == '/')(remainder)?;
        if new_remainder.len() == 0 {
            // TODO this should return unterminated comment error
            return fail(new_remainder);
        }
        remainder = new_remainder;
        let next_close: IResult<&str, &str> = tag("*/")(remainder);
        match next_close {
            Ok((new_remainder, _)) => {
                open = open - 1;
                if open == 0 {
                    return Ok((new_remainder, None));
                } else {
                    remainder = new_remainder;
                }
            }
            Err(_) => (),
        }
        let next_open: IResult<&str, &str> = tag("/*")(remainder);
        match next_open {
            Ok((new_remainder, _)) => {
                open = open + 1;
                remainder = new_remainder;
            }
            Err(_) => (),
        }
    }
}

// Skip "//" to end of line
fn scan_slash_or_comment(input: &str) -> IResult<&str, Option<Token>> {
    let r: IResult<&str, &str> = peek(tag("//"))(input);
    match r {
        Ok((rest, _)) => value(None, pair(tag("//"), alt((eof, not_line_ending))))(rest),
        Err(_) => {
            let r2: IResult<&str, &str> = tag("/*")(input);
            match r2 {
                Ok((rest, _)) => scan_slash_star_comment(rest),
                Err(_) => map(tag("/"), |c: &str| {
                    Some(Token::Slash)
                })(input),
            }
        }
    }
}

// Identifier. Begins with ascii alphabetic, followed by alphanumeric, dash and underscores
fn scan_identifier_or_keyword(input: &str) -> IResult<&str, Option<Token>> {
    let ident = recognize(pair(
        alt((alpha1, tag("_"))),
        many0(alt((alphanumeric1, tag("-"), tag("_")))),
    ));
    map(ident, |s: &str| {
        if let Some(keyword_token) = KEY_WORDS.get(s) {
            Some(keyword_token.clone())
        } else {
            Some(Token::Identifier(s.to_string()))
        }
    })(input)
}

// String
fn scan_quoted_string(input: &str) -> IResult<&str, Option<Token>> {
    // TODO is there a better way to handle empty quoted string?
    let quoted_string = alt((
        value("", tag("\"\"")),
        delimited(char('"'), is_not("\""), char('"')),
    ));
    let mut mr = map(quoted_string, |s: &str| { Some(Token::String(s.to_string())) });
    mr(input)
}

lazy_static! {
    static ref KEY_WORDS: HashMap<String, Token> = {
        let mut m = HashMap::new();
        m.insert("and".to_string(), Token::And);
        m.insert("class".to_string(), Token::Class);
        m.insert("else".to_string(), Token::Else);
        m.insert("false".to_string(), Token::False);
        m.insert("fun".to_string(), Token::Fun);
        m.insert("for".to_string(), Token::For);
        m.insert("if".to_string(), Token::If);
        m.insert("nil".to_string(), Token::Nil);
        m.insert("or".to_string(), Token::Or);
        m.insert("print".to_string(), Token::Print);
        m.insert("return".to_string(), Token::Return);
        m.insert("super".to_string(), Token::Super);
        m.insert("this".to_string(), Token::This);
        m.insert("true".to_string(), Token::True);
        m.insert("var".to_string(), Token::Var);
        m.insert("while".to_string(), Token::While);
        m
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_quoted_string() {
        let input = "\"Justin\"";
        let token = Token::String("Justin".to_string());
        assert_eq!(Ok(("", Some(token))), scan_quoted_string(input));
    }
    #[test]
    fn test_scan_quoted_string_fail() {
        let input = "\"Justin";
        let r = scan_quoted_string(input);
        assert!(r.is_err());
    }

    #[test]
    fn test_scan_assignment_statement() {
        let input = "string \"Justin\"";
        let items = scan(input).unwrap();
        println!("{:?}", items);
        assert!(items.len() == 3);
    }

    #[test]
    fn test_scan_multiple_quoted_string() {
        let input = "\"Justin\"\"Was\"\"Here\"";
        let items = scan(input).unwrap();
        println!("{:?}", items);
        assert!(items.len() == 4);
    }
}
