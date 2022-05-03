// Scanner for lox
// Tools to turn a string of lox source into tokens

#[derive(Debug, PartialEq)]
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

// Design decisions. Should lexeme exist for things that are constant like
// operators, keywords? It can be empty string but maybe it should be Option
#[derive(Debug, PartialEq)]
pub struct TokenInstance {
    token_type: Token,
    lexeme: String,
    line: usize,
}

#[derive(Debug)]
pub enum ScanError {
    UnexpectedChar(char),
    NumberFormatErr(String),
    EndOfInput,
}

pub struct ScanState<'a> {
    line: usize,
    position: usize,
    tokens: Vec<TokenInstance>,
    source: &'a str,
}

pub fn begin_scan(source: &str) -> ScanState {
    ScanState {
        line: 0,
        position: 0,
        tokens: vec![],
        source,
    }
}

pub fn is_scan_done(state: &ScanState) -> bool {
    state.source.is_empty()
}

pub fn scan_next(state: &mut ScanState) -> Result<(), ScanError> {
    if let Some(next_char) = state.source.chars().nth(0) {
        match next_char {
            // Skip whitespace, for it is not signicant, and handle line counting
            '\t' | ' ' | '\r' => skip_character(state),
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
                Token::GreaterEqual,
                state,
            ),
            '>' => single_or_double_character_scanner(
                next_char,
                '=',
                Token::Greater,
                Token::GreaterEqual,
                state,
            ),
            '<' => single_or_double_character_scanner(
                next_char,
                '=',
                Token::Less,
                Token::LessEqual,
                state,
            ),

            // Numbers
            m if m.is_ascii_digit() => number_scanner(state),
            // Identifiers
            // TODO make a function
            m if m.is_ascii_alphabetic() => {
                state.position = state.position + 1;
                state.source = &state.source[1..];
                state.tokens.push(TokenInstance {
                    token_type: Token::Identifier(m.to_string()),
                    lexeme: next_char.to_string(),
                    line: state.line,
                })
            }
            _ => return Err(ScanError::UnexpectedChar(next_char)),
        };
        Ok(())
    } else {
        Err(ScanError::EndOfInput)
    }
}

fn number_scanner(state: &mut ScanState) {
    if let Some(end_pos) = state
        .source
        .find(|c: char| !(c.is_ascii_digit() || c == '.'))
    {
        let numeric_characters = &state.source[..end_pos];
        state.position = state.position + end_pos;
        state.source = &state.source[end_pos..];

        let value = str::parse::<f64>(&numeric_characters).unwrap();

        state.tokens.push(TokenInstance {
            token_type: Token::Number(value), // TODO should catch and convert this on error
            lexeme: numeric_characters.to_string(),
            line: state.line,
        })
    } else {
        // Edge case that the file ends with one or more digits
        let numeric_characters = &state.source[..];
        state.position = state.position + state.source.len();
        state.source = "";
        state.tokens.push(TokenInstance {
            token_type: Token::Number(str::parse::<f64>(&numeric_characters).unwrap()), // TODO should catch and convert this on error
            lexeme: numeric_characters.to_string(),
            line: state.line,
        })
    }
}

// Handle single-character
pub fn single_character_scanner(c: char, token: Token, state: &mut ScanState) {
    state.position = state.position + 1;
    state.source = &state.source[1..];
    state.tokens.push(TokenInstance {
        token_type: token,
        lexeme: c.to_string(),
        line: state.line,
    })
}

pub fn single_or_double_character_scanner(
    c: char,
    double_char: char,
    single_token: Token,
    double_token: Token,
    state: &mut ScanState,
) {
    let next_char = state.source.chars().nth(1);
    if matches!(next_char, Some(next_char) if next_char == double_char) {
        state.position = state.position + 2;
        state.source = &state.source[2..];
        let c_arr = [c, double_char];
        state.tokens.push(TokenInstance {
            token_type: double_token,
            lexeme: String::from_iter(c_arr),
            line: state.line,
        })
    } else {
        state.position = state.position + 1;
        state.source = &state.source[1..];
        state.tokens.push(TokenInstance {
            token_type: single_token,
            lexeme: c.to_string(),
            line: state.line,
        })
    }
}

pub fn skip_character_new_line(state: &mut ScanState) -> () {
    state.position = state.position + 1;
    state.source = &state.source[1..];
    state.line = state.line + 1;
}

pub fn skip_character(state: &mut ScanState) -> () {
    state.position = state.position + 1;
    state.source = &state.source[1..];
}

pub fn scan(input: &str) -> Result<Vec<TokenInstance>, ScanError> {
    let mut state: ScanState = begin_scan(input);
    while !is_scan_done(&state) {
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

    #[test]
    fn scan_test_arithmetic_operators() {
        let input = "=+".to_string();

        let expected = vec![
            TokenInstance {
                token_type: Token::Equal,
                lexeme: "=".to_string(),
                line: 0,
            },
            TokenInstance {
                token_type: Token::Plus,
                lexeme: "+".to_string(),
                line: 0,
            },
            TokenInstance {
                token_type: Token::Eof,
                lexeme: "".to_string(),
                line: 0,
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
                line: 0,
            },
            TokenInstance {
                token_type: Token::Equal,
                lexeme: "=".to_string(),
                line: 0,
            },
            TokenInstance {
                token_type: Token::Number(1.0),
                lexeme: "1".to_string(),
                line: 0,
            },
            TokenInstance {
                token_type: Token::Plus,
                lexeme: "+".to_string(),
                line: 0,
            },
            TokenInstance {
                token_type: Token::Number(2.0),
                lexeme: "2".to_string(),
                line: 0,
            },
            TokenInstance {
                token_type: Token::Semicolon,
                lexeme: ";".to_string(),
                line: 0,
            },
            TokenInstance {
                token_type: Token::Eof,
                lexeme: "".to_string(),
                line: 0,
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
                line: 0,
            },
            TokenInstance {
                token_type: Token::Equal,
                lexeme: "=".to_string(),
                line: 0,
            },
            TokenInstance {
                token_type: Token::Number(1.0),
                lexeme: "1".to_string(),
                line: 0,
            },
            TokenInstance {
                token_type: Token::Plus,
                lexeme: "+".to_string(),
                line: 0,
            },
            TokenInstance {
                token_type: Token::Number(2.0),
                lexeme: "2".to_string(),
                line: 0,
            },
            TokenInstance {
                token_type: Token::Semicolon,
                lexeme: ";".to_string(),
                line: 0,
            },
            TokenInstance {
                token_type: Token::Eof,
                lexeme: "".to_string(),
                line: 0,
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
                line: 0,
            },
            TokenInstance {
                token_type: Token::Equal,
                lexeme: "=".to_string(),
                line: 0,
            },
            TokenInstance {
                token_type: Token::Identifier("b".to_string()),
                lexeme: "b".to_string(),
                line: 1,
            },
            TokenInstance {
                token_type: Token::Plus,
                lexeme: "+".to_string(),
                line: 1,
            },
            TokenInstance {
                token_type: Token::Identifier("c".to_string()),
                lexeme: "c".to_string(),
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
    fn scan_test_numerics() {
        let input = "120,120.5,121".to_string();

        let expected = vec![
            TokenInstance {
                token_type: Token::Number(120.0),
                lexeme: "120".to_string(),
                line: 0,
            },
            TokenInstance {
                token_type: Token::Comma,
                lexeme: ",".to_string(),
                line: 0,
            },
            TokenInstance {
                token_type: Token::Number(120.5),
                lexeme: "120.5".to_string(),
                line: 0,
            },
            TokenInstance {
                token_type: Token::Comma,
                lexeme: ",".to_string(),
                line: 0,
            },
            TokenInstance {
                token_type: Token::Number(121.0),
                lexeme: "121".to_string(),
                line: 0,
            },
            TokenInstance {
                token_type: Token::Eof,
                lexeme: "".to_string(),
                line: 0,
            },
        ];

        assert_eq!(scan(&input).unwrap(), expected);
    }
}
