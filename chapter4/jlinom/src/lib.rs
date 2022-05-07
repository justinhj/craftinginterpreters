use nom::character::complete::alphanumeric1;
use nom::character::complete::char;
use nom::sequence::delimited;
use nom::IResult;
use nom::combinator::map;
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
        return s[0..s.len() - count].to_string();
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
#[derive(PartialEq)]
pub struct TokenInstance {
    token_type: Token,
    lexeme: String,
    // line: usize, // TODO can we do state?
    // yes with https://github.com/fflorent/nom_locate
}
// String
fn scan_quoted_string(input: &str) -> IResult<&str, TokenInstance> {
    let quoted_string = delimited(char('"'), alphanumeric1, char('"'));
    let mut mr = map(quoted_string, |s: &str| {
        TokenInstance{
            token_type: Token::String(s.to_string()),
            lexeme: s.to_string(),
        }
    });
    mr(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_quoted_string() {
        let input = "\"Justin\"";
        let token = TokenInstance{token_type: Token::String("Justin".to_string()), lexeme: "Justin".to_string()}; 
        assert_eq!(Ok(("", token)), scan_quoted_string(input));
    }
    #[test]
    fn test_scan_quoted_string_fail() {
        let input = "\"Justin";
        let token = TokenInstance{token_type: Token::String("Justin".to_string()), lexeme: "Justin".to_string()}; 
        let r = scan_quoted_string(input);
        assert!(r.is_err()); 
    }
}
