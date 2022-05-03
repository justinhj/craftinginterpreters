// Scanner for lox
// Tools to turn a string of lox source into tokens
// TODO make into a library module

#[derive(Debug, PartialEq)]
pub enum Token {
    // literals
    IDENTIFIER(String),
    NUMBER(f64),
    STRING(String),
    // single character operators
    EQUAL,
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    STAR,
    // single or double
    BANG,
    BANG_EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,
    // Keywords
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,
    // End marker
    EOF,
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

// TODO handle whitespace and line counting, comments
pub fn scan_next(state: &mut ScanState) -> Result<(), ScanError> {
    if let Some(next_char) = state.source.chars().nth(0) {
        match next_char {
            // Single characters
            '(' => single_character_scanner(next_char, Token::LEFT_PAREN, state),
            ')' => single_character_scanner(next_char, Token::RIGHT_PAREN, state),
            '{' => single_character_scanner(next_char, Token::LEFT_BRACE, state),
            '}' => single_character_scanner(next_char, Token::RIGHT_BRACE, state),
            ',' => single_character_scanner(next_char, Token::COMMA, state),
            '.' => single_character_scanner(next_char, Token::DOT, state),
            '-' => single_character_scanner(next_char, Token::MINUS, state),
            '+' => single_character_scanner(next_char, Token::PLUS, state),
            ';' => single_character_scanner(next_char, Token::SEMICOLON, state),
            '*' => single_character_scanner(next_char, Token::STAR, state),

            // Single OR double characters
            '=' => single_or_double_character_scanner(
                next_char,
                '=',
                Token::EQUAL,
                Token::GREATER_EQUAL,
                state,
            ),
            '>' => single_or_double_character_scanner(
                next_char,
                '=',
                Token::GREATER,
                Token::GREATER_EQUAL,
                state,
            ),
            '<' => single_or_double_character_scanner(
                next_char,
                '=',
                Token::LESS,
                Token::LESS_EQUAL,
                state,
            ),

            // Numbers
            // TODO make a function
            m if m.is_ascii_digit() => {
                state.position = state.position + 1;
                state.source = &state.source[1..];
                state.tokens.push(TokenInstance {
                    token_type: Token::NUMBER(str::parse::<f64>(&m.to_string()).unwrap()),
                    lexeme: next_char.to_string(),
                    line: state.line,
                })
            }
            // Identifiers
            // TODO make a function
            m if m.is_ascii_alphabetic() => {
                state.position = state.position + 1;
                state.source = &state.source[1..];
                state.tokens.push(TokenInstance {
                    token_type: Token::IDENTIFIER(m.to_string()),
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

pub fn scan(input: &str) -> Result<Vec<TokenInstance>, ScanError> {
    let mut state: ScanState = begin_scan(input);
    while !is_scan_done(&state) {
        scan_next(&mut state)?;
    }
    state.tokens.push(TokenInstance {
        token_type: Token::EOF,
        lexeme: "".to_string(),
        line: state.line,
    });
    Ok(state.tokens)
}

mod tests {

    use super::*;

    #[test]
    fn scan_test_arithmetic_operators() {
        let input = "=+".to_string();

        let expected = vec![
            TokenInstance {
                token_type: Token::EQUAL,
                lexeme: "=".to_string(),
                line: 0,
            },
            TokenInstance {
                token_type: Token::PLUS,
                lexeme: "+".to_string(),
                line: 0,
            },
            TokenInstance {
                token_type: Token::EOF,
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
                token_type: Token::IDENTIFIER("a".to_string()),
                lexeme: "a".to_string(),
                line: 0,
            },
            TokenInstance {
                token_type: Token::EQUAL,
                lexeme: "=".to_string(),
                line: 0,
            },
            TokenInstance {
                token_type: Token::NUMBER(1.0),
                lexeme: "1".to_string(),
                line: 0,
            },
            TokenInstance {
                token_type: Token::PLUS,
                lexeme: "+".to_string(),
                line: 0,
            },
            TokenInstance {
                token_type: Token::NUMBER(2.0),
                lexeme: "2".to_string(),
                line: 0,
            },
            TokenInstance {
                token_type: Token::SEMICOLON,
                lexeme: ";".to_string(),
                line: 0,
            },
            TokenInstance {
                token_type: Token::EOF,
                lexeme: "".to_string(),
                line: 0,
            },
        ];

        assert_eq!(scan(&input).unwrap(), expected);
    }
}
