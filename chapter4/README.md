# Lox Scanner
## Chapter 4 challenges
1. Python and Haskell are not regular; what does that mean?
Regular languages can be described by regular expressions.

This seems interesting:
[Monadic second-order logic](https://en.wikipedia.org/wiki/Monadic_second-order_logic)

Python and Haskell both have whitespace sensitive syntax which means
keeping track of state.

[Reddit/the lexical grammars of python and haskell are](https://www.reddit.com/r/compsci/comments/kkzn3r/the_lexical_grammars_of_python_and_haskell_are/)

"each lexical element can contain multiple derivations"

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

## Critique of jli - my first effort

Some of the code is horrible in that it is more complex than it needs to be. This mostly comes from me sticking to using a character iterator for the parsing and otherwise staying close to the book code. I should really have implemented the advance, peek functionality in the book. 

Notes on making it more like the book...

lexemes are the raw text of compilation, for example they still include the quotes of quoted strings.

scanner has some variables to track state 
start, current and line

isAtEnd returns current >= source length

advance - returns the next character with consuming. current += 1

addToken just adds to the array list of tokens (helper function)

match - this is a conditional peek, see if the character is next and if it is, eat it
returns boolean
used for the single or double types 

at any point you can use start (the start of the lexeme being scanned) and current
start is reset to current when you finish the current call to scantoken

digits ...

while peek isdigit advance
if peek is . and peek next isdigit
  advance
end
this safely gobbles all the digits 

ok reimplemeted jli version with these changes and it's a lot nicer

## Critique of jlinom - second effort using nom

Using the nom parser combinator library really cleaned things up

[nom](https://github.com/Geal/nom) 

Need to implement error location

## Testing

Some tests are added in `lib.rs` file but the book source code also has a test runner.

In the book source folder run:

```
dart tool/bin/test.dart chap04_scanning --interpreter ~/projects/craftinginterpreters/chapter4/jli/target/release/jli 
```
