use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::character::complete::alphanumeric0;
use nom::character::complete::{
    alpha1, alphanumeric1, anychar, char, multispace0, multispace1, one_of,
};
use nom::combinator::{fail, map, peek, recognize, success, value};
use nom::multi::many0;
use nom::sequence::{delimited, pair, preceded};
use nom::IResult;
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
fn num_format(num: f64) -> String {
    let s = format!("{:.3}", num);
    if let Some(non_zero_pos) = s.rfind(|c: char| c != '0') {
        let zero_count = s.len() - (non_zero_pos + 1);
        let count = std::cmp::min(zero_count, 2);
        s[0..s.len() - count].to_string()
    } else {
        panic!("Unexpected number format {:?}", s);
    }
}
// Note this is the Debug implementation for TokenInstance, but it may be valuable to create a
// Display instance too.
impl fmt::Debug for TokenInstance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.token_type {
            Token::Identifier(string) => write!(f, "IDENTIFIER {} null", string),
            Token::Number(num) => write!(f, "NUMBER {} {}", self.lexeme, num_format(*num)),
            Token::String(string) => write!(f, "STRING \"{}\" {}", self.lexeme, string),
            Token::Equal => write!(f, "EQUAL = null"),
            Token::LeftParen => write!(f, "LEFT_PAREN ( null"),
            Token::RightParen => write!(f, "RIGHT_PAREN ) null"),
            Token::LeftBrace => write!(f, "LEFT_BRACE {{ null"),
            Token::RightBrace => write!(f, "RIGHT_BRACE }} null"),
            Token::Comma => write!(f, "COMMA , null"),
            Token::Dot => write!(f, "DOT . null"),
            Token::Minus => write!(f, "MINUS - null"),
            Token::Plus => write!(f, "PLUS + null"),
            Token::Semicolon => write!(f, "SEMICOLON ; null"),
            Token::Star => write!(f, "STAR * null"),
            Token::Bang => write!(f, "BANG ! null"),
            Token::BangEqual => write!(f, "BANG_EQUAL != null"),
            Token::EqualEqual => write!(f, "EQUAL_EQUAL == null"),
            Token::Greater => write!(f, "GREATER > null"),
            Token::GreaterEqual => write!(f, "GREATER_EQUAL >= null"),
            Token::Less => write!(f, "LESS < null"),
            Token::LessEqual => write!(f, "LESS_EQUAL <= null"),
            Token::And => write!(f, "AND and null"),
            Token::Class => write!(f, "CLASS class null"),
            Token::Else => write!(f, "ELSE else null"),
            Token::False => write!(f, "FALSE false null"),
            Token::Fun => write!(f, "FUN fun null"),
            Token::For => write!(f, "FOR for null"),
            Token::If => write!(f, "IF if null"),
            Token::Nil => write!(f, "NIL nil null"),
            Token::Or => write!(f, "OR or null"),
            Token::Print => write!(f, "PRINT print null"),
            Token::Return => write!(f, "RETURN return null"),
            Token::Super => write!(f, "SUPER super null"),
            Token::This => write!(f, "THIS this null"),
            Token::True => write!(f, "TRUE true null"),
            Token::Var => write!(f, "VAR var null"),
            Token::While => write!(f, "WHILE while null"),
            Token::Slash => write!(f, "SLASH / null"),
            Token::Eof => write!(f, "EOF  null"),
        }
    }
}

// This acts as a wrapper for tokens to enable addition information such as lexeme and position
// info
#[derive(PartialEq, Clone)]
pub struct TokenInstance {
    token_type: Token,
    lexeme: String,
    // line: usize, // TODO can we do state?
    // yes with https://github.com/fflorent/nom_locate
}

// TODO map errors into this format
#[derive(Debug)]
pub enum ScanError {
    UnexpectedChar(char),
    NumberFormatError(String),
    UnterminatedString(String),
    EndOfInput,
    Error,
}

// Scan the input string returning either a vector of tokens or the first error
pub fn scan(input: &str) -> Result<Vec<TokenInstance>, ScanError> {
    // let result = many0(scan_token)(input);
    let mut tokens: Vec<TokenInstance> = vec![];
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

    tokens.insert(
        tokens.len(),
        TokenInstance {
            token_type: Token::Eof,
            lexeme: "".to_string(),
        },
    );
    Ok(tokens)
}

fn scan_token(input: &str) -> IResult<&str, Option<TokenInstance>> {
    let peeker: IResult<&str, char> = peek(anychar)(input);
    match peeker {
        Ok((rest, c)) if c.is_ascii_whitespace() => value(None, multispace1)(rest),
        Ok((rest, c)) if c.is_ascii_alphabetic() => scan_identifier(rest),
        Ok((rest, '"')) => scan_quoted_string(rest),
        Ok((rest, '(')) => single_char_to_token(rest, Token::LeftParen),
        Ok((rest, ')')) => single_char_to_token(rest, Token::RightParen),
        Ok((rest, '{')) => single_char_to_token(rest, Token::LeftBrace),
        Ok((rest, '}')) => single_char_to_token(rest, Token::RightBrace),
        Ok((rest, ',')) => single_char_to_token(rest, Token::Comma),
        Ok((rest, '.')) => single_char_to_token(rest, Token::Dot),
        Ok((rest, '-')) => single_char_to_token(rest, Token::Minus),
        Ok((rest, '+')) => single_char_to_token(rest, Token::Plus),
        Ok((rest, ';')) => single_char_to_token(rest, Token::Semicolon),
        Ok((rest, '*')) => single_char_to_token(rest, Token::Star),
        Ok((rest, '=')) => scan_single_or_double(rest, '=', '=', Token::Equal, Token::EqualEqual),
        Ok((rest, '!')) => scan_single_or_double(rest, '!', '=', Token::Bang, Token::BangEqual),
        Ok((rest, '>')) => scan_single_or_double(rest, '>', '=', Token::Greater, Token::GreaterEqual),
        Ok((rest, '<')) => scan_single_or_double(rest, '<', '=', Token::Less, Token::LessEqual),
        // TODO slash or comment
        // TODO numbers
        Ok((rest, w)) => {
            println!("unknown {:?} {:?}", rest, w);
            fail(rest)
        }
        Err(err) => {
            println!("err {:?}", err);
            Err(err)
        }
    }
}

// Single character symbols like *
fn single_char_to_token(input: &str, token: Token) -> IResult<&str, Option<TokenInstance>> {
    map(anychar, |c| {
        Some(TokenInstance {
            token_type: token.clone(),
            lexeme: c.to_string(),
        })
    })(input)
}

// Single OR double character symbols like = and ==
fn scan_single_or_double(
    input: &str,
    single: char,
    double: char,
    single_token: Token,
    double_token: Token,
) -> IResult<&str, Option<TokenInstance>> {
    let double_target: String = [single, double].iter().collect();
    let mut parser = map(
        alt((tag(&double_target[..]), tag(&double_target[0..1]))),
        |m: &str| {
            if m.len() == 2 {
                Some(TokenInstance {
                    token_type: double_token.clone(),
                    lexeme: m.to_string(),
                })
            } else {
                println!("m {:?}", m);
                Some(TokenInstance {
                    token_type: single_token.clone(),
                    lexeme: m.to_string(),
                })
            }
        },
    );
    parser(input)
}

// Skip "//" to end of line
pub fn scan_skip_eol_comment(input: &str) -> IResult<&str, Option<TokenInstance>> {
    value(None, pair(tag("//"), is_not("\n\r")))(input)
}

// Identifier. Begins with ascii alphabetic, followed by alphanumeric, dash and underscores
fn scan_identifier(input: &str) -> IResult<&str, Option<TokenInstance>> {
    let ident = recognize(pair(
        alpha1,
        many0(alt((alphanumeric1, tag("-"), tag("_")))),
    ));
    map(ident, |s: &str| {
        Some(TokenInstance {
            token_type: Token::Identifier(s.to_string()),
            lexeme: s.to_string(),
        })
    })(input)
}

// String
fn scan_quoted_string(input: &str) -> IResult<&str, Option<TokenInstance>> {
    // TODO is there a better way to handle empty quoted string?
    let quoted_string = alt((tag("\"\""), delimited(char('"'), is_not("\""), char('"'))));
    let mut mr = map(quoted_string, |s: &str| {
        Some(TokenInstance {
            token_type: Token::String(s.to_string()),
            lexeme: s.to_string(),
        })
    });
    mr(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_quoted_string() {
        let input = "\"Justin\"";
        let token = TokenInstance {
            token_type: Token::String("Justin".to_string()),
            lexeme: "Justin".to_string(),
        };
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
