// Scanner for lox
// Tools to turn a string of lox source into tokens

// TODO make into a library module

fn main() {
    println!("Hello, world!");
}

#[derive(Debug,PartialEq)]
enum Token {
    // literals
    IDENTIFIER(String),
    NUMBER(f64),
    // operators
    EQUAL,
    PLUS,
    // mechanics
    SEMICOLON,
    EOF,
}

// Design decisions. Should lexeme exist for things that are constant like 
// operators, keywords? It can be empty string but maybe it should be Option
#[derive(Debug,PartialEq)]
struct TokenInstance {
    token_type: Token,
    lexeme: String,
    line: usize
}

#[derive(Debug)]
enum ScanError {
    UnexpectedChar(char),
    EndOfInput,
}

struct ScanState<'a> {
    line: usize,
    position: usize,
    tokens: Vec<TokenInstance>,
    source: &'a str,
}

fn begin_scan(source: &str) -> ScanState {
    ScanState {
        line: 0,
        position: 0,
        tokens: vec!(),
        source,
    }
}

fn is_scan_done(state: &ScanState) -> bool {
    state.source.is_empty()
}

fn scan_next(state: &mut ScanState) -> Result<(), ScanError> {
    // TODO how to nicely represent and loop through all candidates instead of doing it inline
    if let Some(next_char) = state.source.chars().nth(0) {
        println!("next_char {:?}", next_char);
        match next_char {
            '=' => {
                state.position = state.position + 1;
                state.source = &state.source[1..];
                state.tokens.push(
                TokenInstance{token_type: Token::EQUAL, lexeme: next_char.to_string(), line: state.line})
            },
            '+' => {
                state.position = state.position + 1;
                state.source = &state.source[1..];
                state.tokens.push(
                TokenInstance{token_type: Token::PLUS, lexeme: next_char.to_string(), line: state.line})
            },
            // Mechanics
            ';' => {
                state.position = state.position + 1;
                state.source = &state.source[1..];
                state.tokens.push(
                TokenInstance{token_type: Token::SEMICOLON, lexeme: next_char.to_string(), line: state.line})
            },
            // Numbers
            m if m.is_ascii_digit() => {
                state.position = state.position + 1;
                state.source = &state.source[1..];
                state.tokens.push(
                TokenInstance{token_type: Token::NUMBER(str::parse::<f64>(&m.to_string()).unwrap()), lexeme: next_char.to_string(), line: state.line})
            },
            // Identifiers
            m if m.is_ascii_alphabetic() => {
                state.position = state.position + 1;
                state.source = &state.source[1..];
                state.tokens.push(
                TokenInstance{token_type: Token::IDENTIFIER(m.to_string()), lexeme: next_char.to_string(), line: state.line})
            },
            _ => return Err(ScanError::UnexpectedChar(next_char)),
        };
        Ok(())
    } else {
        Err(ScanError::EndOfInput)
    }
}

fn scan(input: &str) -> Result<Vec<TokenInstance>, ScanError> {
    let mut state: ScanState = begin_scan(input);
    while !is_scan_done(&state) {
        scan_next(&mut state)?;
    }
    state.tokens.push(TokenInstance{token_type: Token::EOF, lexeme: "".to_string(), line: state.line});
    Ok(state.tokens)
}

mod tests {

    use super::*;

    #[test]
    fn scan_test_arithmetic_operators() {
        let input = "=+".to_string();

        let expected = vec!(
                TokenInstance{token_type: Token::EQUAL, lexeme: "=".to_string(), line: 0},
                TokenInstance{token_type: Token::PLUS, lexeme: "+".to_string(), line: 0},
                TokenInstance{token_type: Token::EOF, lexeme: "".to_string(), line: 0},
                );

        assert_eq!(scan(&input).unwrap(), expected);
    }

    #[test]
    fn scan_test_arithmetic_expression() {
        let input = "a=1+2;".to_string();

        let expected = vec!(
                TokenInstance{token_type: Token::IDENTIFIER("a".to_string()), lexeme: "a".to_string(), line: 0},
                TokenInstance{token_type: Token::EQUAL, lexeme: "=".to_string(), line: 0},
                TokenInstance{token_type: Token::NUMBER(1.0), lexeme: "1".to_string(), line: 0},
                TokenInstance{token_type: Token::PLUS, lexeme: "+".to_string(), line: 0},
                TokenInstance{token_type: Token::NUMBER(2.0), lexeme: "2".to_string(), line: 0},
                TokenInstance{token_type: Token::SEMICOLON, lexeme: ";".to_string(), line: 0},
                TokenInstance{token_type: Token::EOF, lexeme: "".to_string(), line: 0},
                );

        assert_eq!(scan(&input).unwrap(), expected);
    }
}
