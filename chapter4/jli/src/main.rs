// Scanner for lox
// Tools to turn a string of lox source into tokens

fn main() {
    println!("Hello, world!");
}

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

struct TokenInstance {
    token_type: Token,
    lexeme: String,
    line: usize
}

fn scan(input: &str) -> Vec<TokenInstance> {
    let x = vec!(TokenInstance{token_type: Token::NUMBER(1.0), lexeme: "1".to_string(), line: 1},
                 TokenInstance{token_type: Token::IDENTIFIER("a".to_string()), lexeme: "a".to_string(), line: 2});
    x.into_iter().collect()
}

mod tests {

    #[test]
    fn scan_test() {
        let input = "a=1+2;".to_string();

// IDENTIFIER a null
// EQUAL = null
// NUMBER 1 1.0
// PLUS + null
// NUMBER 2 2.0
// SEMICOLON ; null
// EOF  null

        assert_eq!("something", "something");
    }


}
