# Lox Scanner

## Testing the book version (Java)

```
make java_chapters
```

Run chapter 4... 

```
java -cp build/gen/chap04_scanning com.craftinginterpreters.lox.Lox
```

Given

```
// A sample lox program
fun addPair(a, b) {
  return a + b;
}
 
fun identity(a) {
  return a;
}
print identity(addPair)(1, 2); // Prints "3".
```

we expect

```
FUN fun null
IDENTIFIER addPair null
LEFT_PAREN ( null
IDENTIFIER a null
COMMA , null
IDENTIFIER b null
RIGHT_PAREN ) null
LEFT_BRACE { null
RETURN return null
IDENTIFIER a null
PLUS + null
IDENTIFIER b null
SEMICOLON ; null
RIGHT_BRACE } null
FUN fun null
IDENTIFIER identity null
LEFT_PAREN ( null
IDENTIFIER a null
RIGHT_PAREN ) null
LEFT_BRACE { null
RETURN return null
IDENTIFIER a null
SEMICOLON ; null
RIGHT_BRACE } null
PRINT print null
IDENTIFIER identity null
LEFT_PAREN ( null
IDENTIFIER addPair null
RIGHT_PAREN ) null
LEFT_PAREN ( null
NUMBER 1 1.0
COMMA , null
NUMBER 2 2.0
RIGHT_PAREN ) null
SEMICOLON ; null
EOF  null
```

## Rust implementation notes

The basic structure is to use enums to represent the different type of Tokens and TokenInstances that represent scanned tokens at a particular point in the file.

The scan next loop is a lot of copy and pasting though, and not very pretty.

First of all we can assume that nothing is an identifier if it doesn't start with an alphanumeric.

We have a bunch of single character things like =,+ and /.

We can ignore to end of line if encounter //

Later on can implement /*

We can make a list of token parsers as helpers

SingleCharacter can go first, these are where a single character guarantees a token

```
  LEFT_PAREN, RIGHT_PAREN, LEFT_BRACE, RIGHT_BRACE,
  COMMA, DOT, MINUS, PLUS, SEMICOLON, STAR,
```

OneOrTwoCharacter

```
  BANG, BANG_EQUAL,
  EQUAL, EQUAL_EQUAL,
  GREATER, GREATER_EQUAL,
  LESS, LESS_EQUAL,
```

Here you have a choice of tokens depending on if the following character matches or not

SlashAndComment

This handles either a single slash which yields `SLASH` or it is comment and you must advance to after the next newline.

Skip whitespace 

newlines
  increment line count and continue
  
alpha 
  parse an identifier 
  after parsing it yield an identifier or see it matches a keyword
  
```
  AND, CLASS, ELSE, FALSE, FUN, FOR, IF, NIL, OR,
  PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE,
```

## Critique

Some of the code is horrible in that it is more complex than it needs to be. This mostly comes from me sticking to using a character iterator for the parsing and otherwise staying close to the book code. I should really have implemented the advance, peek functionality in the book. A better approach would be to swap out my code for a parser combinator libary like [nom](https://docs.rs/nom/latest/nom/)

## Testing

Some tests are added in `lib.rs` file but the book source code also has a test runner.

In the book source folder run:

```
dart tool/bin/test.dart chap04_scanning --interpreter ~/projects/craftinginterpreters/chapter4/jli/target/release/jli 
```
