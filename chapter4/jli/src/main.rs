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

enum ScanError {
    UnexpectedChar(char),
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
        source: source,
    }
}

fn is_scan_done(state: &ScanState) -> bool {
    state.position == state.source.len()
}

fn scan_next(state: &mut ScanState) -> Result<(), ScanError> {

    Ok(())
}

fn scan(input: &str) -> Vec<TokenInstance> {
    let x = vec!(TokenInstance{token_type: Token::NUMBER(1.0), lexeme: "1".to_string(), line: 1},
                 TokenInstance{token_type: Token::IDENTIFIER("a".to_string()), lexeme: "a".to_string(), line: 2});
    x.into_iter().collect()
}

mod tests {

    use super::*;

    #[test]
    fn scan_test() {
        let input = "a=1+2;".to_string();

        let expected = vec!(
                TokenInstance{token_type: Token::IDENTIFIER("a".to_string()), lexeme: "a".to_string(), line: 0},
                TokenInstance{token_type: Token::EQUAL, lexeme: "=".to_string(), line: 0},
                TokenInstance{token_type: Token::NUMBER(1.0), lexeme: "1".to_string(), line: 0},
                TokenInstance{token_type: Token::PLUS, lexeme: "+".to_string(), line: 0},
                TokenInstance{token_type: Token::NUMBER(2.0), lexeme: "2".to_string(), line: 0},
                TokenInstance{token_type: Token::EOF, lexeme: "".to_string(), line: 0},
                );

        assert_eq!(scan(&input), expected);
    }
}
