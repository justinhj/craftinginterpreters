// Scanner for Lox
// Tools to turn a string of lox source into tokens
use lazy_static::lazy_static;
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

#[derive(PartialEq)]
pub struct TokenInstance {
    pub token_type: Token,
    pub lexeme: String,
    pub line: usize,
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

#[derive(Debug)]
pub enum ScanError {
    UnexpectedChar(char),
    NumberFormatError(String),
    UnterminatedString(String),
    EndOfInput,
}

#[derive(Debug)]
struct ScanState<'a> {
    line: usize,
    tokens: Vec<TokenInstance>,
    source: &'a str,
    start: usize,
    current: usize,
}

fn begin_scan(source: &str) -> ScanState {
    ScanState {
        line: 1,
        tokens: vec![],
        source,
        start: 0,
        current: 0,
    }
}

fn is_scan_done(state: &ScanState) -> bool {
    state.current == state.source.len()
}

fn peek(state: &ScanState) -> char {
    if is_scan_done(state) {
        '\0'
    } else {
        state.source.chars().nth(state.current).unwrap()
    }
}

fn peek_next(state: &ScanState) -> char {
    if state.current + 1 >= state.source.len() {
        '\0'
    } else {
        state.source.chars().nth(state.current + 1).unwrap()
    }
}

fn advance(state: &mut ScanState) -> char {
    let c = state.source.chars().nth(state.current);
    state.current += 1;
    c.unwrap()
}

fn match_next(n: char, state: &mut ScanState) -> bool {
    match state.source.chars().nth(state.current) {
        Some(d) if n == d => {
            state.current += 1;
            true
        }
        _ => false,
    }
}

fn scan_next(state: &mut ScanState) -> Result<(), ScanError> {
    let next_char = advance(state);
    match next_char {
        // Skip whitespace, for it is not signicant, and handle line counting
        '\t' | ' ' | '\r' => Ok(()),
        '\n' => skip_character_new_line(state),
        // Single characters
        '(' => single_character_scanner(next_char, Token::LeftParen, state),
        ')' => single_character_scanner(next_char, Token::RightParen, state),
        '{' => single_character_scanner(next_char, Token::LeftBrace, state),
        '}' => single_character_scanner(next_char, Token::RightBrace, state),
        ',' => single_character_scanner(next_char, Token::Comma, state),
        '.' => single_character_scanner(next_char, Token::Dot, state),
        '-' => single_character_scanner(next_char, Token::Minus, state),
        '+' => single_character_scanner(next_char, Token::Plus, state),
        ';' => single_character_scanner(next_char, Token::Semicolon, state),
        '*' => single_character_scanner(next_char, Token::Star, state),

        // Single OR double characters
        '=' => single_or_double_character_scanner(
            next_char,
            '=',
            Token::Equal,
            Token::EqualEqual,
            state,
        ),
        '!' => {
            single_or_double_character_scanner(next_char, '=', Token::Bang, Token::BangEqual, state)
        }
        '>' => single_or_double_character_scanner(
            next_char,
            '=',
            Token::Greater,
            Token::GreaterEqual,
            state,
        ),
        '<' => {
            single_or_double_character_scanner(next_char, '=', Token::Less, Token::LessEqual, state)
        }
        // Slash or comment
        '/' => slash_or_comment_scanner(state),
        // Numbers
        m if m.is_ascii_digit() => number_scanner(state),
        // Identifiers
        m if m.is_ascii_alphabetic() || m == '_' => identifier_or_keyword_scanner(state),
        // String literals
        '"' => string_scanner(state),
        _ => Err(ScanError::UnexpectedChar(next_char)),
    }
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

fn string_scanner(state: &mut ScanState) -> Result<(), ScanError> {
    while peek(state) != '"' && !is_scan_done(state) {
        if peek(state) == '\n' {
            state.line += 1;
        }
        advance(state);
    }

    if is_scan_done(state) {
        Err(ScanError::UnterminatedString(format!(
            "Unterminated string {:?}",
            state.source
        )))
    } else {
        advance(state);
        let word = &state.source[state.start + 1..state.current - 1];
        state.tokens.push(TokenInstance {
            token_type: Token::String(word.to_string()),
            lexeme: word.to_string(),
            line: state.line,
        });
        Ok(())
    }
}

fn identifier_or_keyword_scanner(state: &mut ScanState) -> Result<(), ScanError> {
    loop {
        let peeked = peek(state);
        if peeked.is_ascii_alphanumeric() || peeked == '_' {
            advance(state);
        } else {
            break;
        }
    }
    let word = &state.source[state.start..state.current];
    if let Some(keyword_token) = KEY_WORDS.get(word) {
        state.tokens.push(TokenInstance {
            token_type: keyword_token.clone(),
            lexeme: word.to_string(),
            line: state.line,
        })
    } else {
        state.tokens.push(TokenInstance {
            token_type: Token::Identifier(word.to_string()),
            lexeme: word.to_string(),
            line: state.line,
        })
    }
    Ok(())
}

fn number_scanner(state: &mut ScanState) -> Result<(), ScanError> {
    while peek(state).is_ascii_digit() {
        advance(state);
    }

    if peek(state) == '.' && peek_next(state).is_ascii_digit() {
        advance(state);
        while peek(state).is_ascii_digit() {
            advance(state);
        }
    }

    let number_str = &state.source[state.start..state.current];
    match str::parse::<f64>(number_str) {
        Ok(value) => {
            state.tokens.push(TokenInstance {
                token_type: Token::Number(value),
                lexeme: number_str.to_string(),
                line: state.line,
            });
            Ok(())
        }
        Err(_) => Err(ScanError::NumberFormatError(number_str.to_string())),
    }
}

fn slash_or_comment_scanner(state: &mut ScanState) -> Result<(), ScanError> {
    if match_next('/', state) {
        while peek(state) != '\n' && !is_scan_done(state) {
            advance(state);
        }
    } else {
        state.tokens.push(TokenInstance {
            token_type: Token::Slash,
            lexeme: '/'.to_string(),
            line: state.line,
        })
    }
    Ok(())
}

// Handle single-character
fn single_character_scanner(c: char, token: Token, state: &mut ScanState) -> Result<(), ScanError> {
    state.tokens.push(TokenInstance {
        token_type: token,
        lexeme: c.to_string(),
        line: state.line,
    });
    Ok(())
}

fn single_or_double_character_scanner(
    c: char,
    double_char: char,
    single_token: Token,
    double_token: Token,
    state: &mut ScanState,
) -> Result<(), ScanError> {
    if match_next(double_char, state) {
        let c_arr = [c, double_char];
        state.tokens.push(TokenInstance {
            token_type: double_token,
            lexeme: String::from_iter(c_arr),
            line: state.line,
        })
    } else {
        state.tokens.push(TokenInstance {
            token_type: single_token,
            lexeme: c.to_string(),
            line: state.line,
        })
    }
    Ok(())
}

fn skip_character_new_line(state: &mut ScanState) -> Result<(), ScanError> {
    state.line += 1;
    Ok(())
}

pub fn scan(input: &str) -> Result<Vec<TokenInstance>, ScanError> {
    let mut state: ScanState = begin_scan(input);
    while !is_scan_done(&state) {
        state.start = state.current;
        scan_next(&mut state)?;
    }
    state.tokens.push(TokenInstance {
        token_type: Token::Eof,
        lexeme: "".to_string(),
        line: state.line,
    });
    Ok(state.tokens)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn scan_test_arithmetic_operators() {
        let input = "=+".to_string();

        let expected = vec![
            TokenInstance {
                token_type: Token::Equal,
                lexeme: "=".to_string(),
                line: 1,
            },
            TokenInstance {
                token_type: Token::Plus,
                lexeme: "+".to_string(),
                line: 1,
            },
            TokenInstance {
                token_type: Token::Eof,
                lexeme: "".to_string(),
                line: 1,
            },
        ];

        assert_eq!(scan(&input).unwrap(), expected);
    }

    #[test]
    fn scan_test_arithmetic_expression_with_spaces() {
        let input = " a = 1 + 2 ; ".to_string();

        let expected = vec![
            TokenInstance {
                token_type: Token::Identifier("a".to_string()),
                lexeme: "a".to_string(),
                line: 1,
            },
            TokenInstance {
                token_type: Token::Equal,
                lexeme: "=".to_string(),
                line: 1,
            },
            TokenInstance {
                token_type: Token::Number(1.0),
                lexeme: "1".to_string(),
                line: 1,
            },
            TokenInstance {
                token_type: Token::Plus,
                lexeme: "+".to_string(),
                line: 1,
            },
            TokenInstance {
                token_type: Token::Number(2.0),
                lexeme: "2".to_string(),
                line: 1,
            },
            TokenInstance {
                token_type: Token::Semicolon,
                lexeme: ";".to_string(),
                line: 1,
            },
            TokenInstance {
                token_type: Token::Eof,
                lexeme: "".to_string(),
                line: 1,
            },
        ];

        assert_eq!(scan(&input).unwrap(), expected);
    }

    #[test]
    fn scan_test_arithmetic_expression() {
        let input = "a=1+2;".to_string();

        let expected = vec![
            TokenInstance {
                token_type: Token::Identifier("a".to_string()),
                lexeme: "a".to_string(),
                line: 1,
            },
            TokenInstance {
                token_type: Token::Equal,
                lexeme: "=".to_string(),
                line: 1,
            },
            TokenInstance {
                token_type: Token::Number(1.0),
                lexeme: "1".to_string(),
                line: 1,
            },
            TokenInstance {
                token_type: Token::Plus,
                lexeme: "+".to_string(),
                line: 1,
            },
            TokenInstance {
                token_type: Token::Number(2.0),
                lexeme: "2".to_string(),
                line: 1,
            },
            TokenInstance {
                token_type: Token::Semicolon,
                lexeme: ";".to_string(),
                line: 1,
            },
            TokenInstance {
                token_type: Token::Eof,
                lexeme: "".to_string(),
                line: 1,
            },
        ];

        assert_eq!(scan(&input).unwrap(), expected);
    }

    #[test]
    fn scan_test_new_lines() {
        let input = "a=\r\nb+c".to_string();

        let expected = vec![
            TokenInstance {
                token_type: Token::Identifier("a".to_string()),
                lexeme: "a".to_string(),
                line: 1,
            },
            TokenInstance {
                token_type: Token::Equal,
                lexeme: "=".to_string(),
                line: 1,
            },
            TokenInstance {
                token_type: Token::Identifier("b".to_string()),
                lexeme: "b".to_string(),
                line: 2,
            },
            TokenInstance {
                token_type: Token::Plus,
                lexeme: "+".to_string(),
                line: 2,
            },
            TokenInstance {
                token_type: Token::Identifier("c".to_string()),
                lexeme: "c".to_string(),
                line: 2,
            },
            TokenInstance {
                token_type: Token::Eof,
                lexeme: "".to_string(),
                line: 2,
            },
        ];

        assert_eq!(scan(&input).unwrap(), expected);
    }

    #[test]
    fn scan_test_function() {
        let input = "\
            fun addPair(a, b) {\n\
              return a + b;\n\
            }";

        let expected = vec![
            TokenInstance {
                token_type: Token::Fun,
                lexeme: "fun".to_string(),
                line: 1,
            },
            TokenInstance {
                token_type: Token::Identifier("addPair".to_string()),
                lexeme: "addPair".to_string(),
                line: 1,
            },
            TokenInstance {
                token_type: Token::LeftParen,
                lexeme: "(".to_string(),
                line: 1,
            },
            TokenInstance {
                token_type: Token::Identifier("a".to_string()),
                lexeme: "a".to_string(),
                line: 1,
            },
            TokenInstance {
                token_type: Token::Comma,
                lexeme: ",".to_string(),
                line: 1,
            },
            TokenInstance {
                token_type: Token::Identifier("b".to_string()),
                lexeme: "b".to_string(),
                line: 1,
            },
            TokenInstance {
                token_type: Token::RightParen,
                lexeme: ")".to_string(),
                line: 1,
            },
            TokenInstance {
                token_type: Token::LeftBrace,
                lexeme: "{".to_string(),
                line: 1,
            },
            TokenInstance {
                token_type: Token::Return,
                lexeme: "return".to_string(),
                line: 2,
            },
            TokenInstance {
                token_type: Token::Identifier("a".to_string()),
                lexeme: "a".to_string(),
                line: 2,
            },
            TokenInstance {
                token_type: Token::Plus,
                lexeme: "+".to_string(),
                line: 2,
            },
            TokenInstance {
                token_type: Token::Identifier("b".to_string()),
                lexeme: "b".to_string(),
                line: 2,
            },
            TokenInstance {
                token_type: Token::Semicolon,
                lexeme: ";".to_string(),
                line: 2,
            },
            TokenInstance {
                token_type: Token::RightBrace,
                lexeme: "}".to_string(),
                line: 3,
            },
            TokenInstance {
                token_type: Token::Eof,
                lexeme: "".to_string(),
                line: 3,
            },
        ];

        assert_eq!(scan(&input).unwrap(), expected);
    }

    #[test]
    fn scan_test_numerics() {
        let input = "120,120.5,121".to_string();

        let expected = vec![
            TokenInstance {
                token_type: Token::Number(120.0),
                lexeme: "120".to_string(),
                line: 1,
            },
            TokenInstance {
                token_type: Token::Comma,
                lexeme: ",".to_string(),
                line: 1,
            },
            TokenInstance {
                token_type: Token::Number(120.5),
                lexeme: "120.5".to_string(),
                line: 1,
            },
            TokenInstance {
                token_type: Token::Comma,
                lexeme: ",".to_string(),
                line: 1,
            },
            TokenInstance {
                token_type: Token::Number(121.0),
                lexeme: "121".to_string(),
                line: 1,
            },
            TokenInstance {
                token_type: Token::Eof,
                lexeme: "".to_string(),
                line: 1,
            },
        ];

        assert_eq!(scan(&input).unwrap(), expected);
    }

    #[test]
    fn num_formatter_test() {
        let s1: f64 = 100.;
        let s2: f64 = 100.1;
        let s3: f64 = 100.12;
        let s4: f64 = 100.123;
        let s5: f64 = 100.1234;

        assert_eq! {num_format(s1), "100.0".to_string()}
        assert_eq! {num_format(s2), "100.1".to_string()}
        assert_eq! {num_format(s3), "100.12".to_string()}
        assert_eq! {num_format(s4), "100.123".to_string()}
        assert_eq! {num_format(s5), "100.123".to_string()}
    }
}
