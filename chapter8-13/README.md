# Implementation of chapters 8 to 13
This culminates in the finished lox interpreter. It picks up from chapter 7 where I have implemented expression evaluation.

At chapter 7 the framework of the interpreter is done and all that remains is the other language features including functions and classes, scope and variables.

## What's next 

What was last worked on was `function calls parsed and first step of collecting params and calling in eval`

## Notes on the code
### src/main.rs
Obviously the main program. You can run the executable with no arguments, in which case I open a command line repl with history. If you provide a file path to a Lox file it will be executed.

It includes the code to load and interpret a lox file as well as act as a Lox repl.
### src/scan.rs
Scan a string into Lox tokens.
### src/parse.rs
Given the tokens created by the scan step it evaluates it according to Lox's grammar.
### src/eval.rs
Evaluates statements, after they have been scanned and parsed, using an execution environment.
### src/lib.rs
Just exposes the modules for when this crate us used as a library.
### samples/*
Example lox scripts you can run with the interpreter.
## Grammar changes

Add statements and print

### Original Grammar

expression -> equality ;
equality -> comparison ( ( "!=" | "==" ) ) comparison )* ;
comparison -> term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term -> factor ( ( "-" | "+" ) ) factor )* ;
factor -> unary ( ( "/" | "*" ) ) unary )* ;
unary -> ( "!" | "-" ) unary | primary ;
primary -> NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;

### Updated to add statements (8.2.1)
commit 13b96ef

program -> statement* EOF ;
statement -> exprStatement | printStatement ;
exprStatement -> expression ";" ;
printStatement -> print expression ";" ;

expression -> equality ;
equality -> comparison ( ( "!=" | "==" ) ) comparison )* ;
comparison -> term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term -> factor ( ( "-" | "+" ) ) factor )* ;
factor -> unary ( ( "/" | "*" ) ) unary )* ;
unary -> ( "!" | "-" ) unary | primary ;
primary -> NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;

### Updated to add declarations and variable usage (section 8.2.1)
commit 8628590

program -> declaration* EOF ;
declaration -> varDecl | statement ;
varDelc -> "var" IDENTIFIER ( "=" expression )? ";" ;

statement -> exprStatement | printStatement ;
exprStatement -> expression ";" ;
printStatement -> print expression ";" ;

expression -> equality ;
equality -> comparison ( ( "!=" | "==" ) ) comparison )* ;
comparison -> term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term -> factor ( ( "-" | "+" ) ) factor )* ;
factor -> unary ( ( "/" | "*" ) ) unary )* ;
unary -> ( "!" | "-" ) unary | primary ;
primary -> NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" | IDENTIFIER ;

### Updated to add block syntax and semantics (section 8.5.2)
commit 08438c0

Note that statement now can be a block and block is defined...

program -> block* EOF ;
block -> "{" declaration* "}" ;
declaration -> varDecl | statement ;
varDelc -> "var" IDENTIFIER ( "=" expression )? ";" ;

statement -> exprStatement | printStatement | block ;
exprStatement -> expression ";" ;
printStatement -> print expression ";" ;

expression -> equality ;
equality -> comparison ( ( "!=" | "==" ) ) comparison )* ;
comparison -> term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term -> factor ( ( "-" | "+" ) ) factor )* ;
factor -> unary ( ( "/" | "*" ) ) unary )* ;
unary -> ( "!" | "-" ) unary | primary ;
primary -> NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" | IDENTIFIER ;

### Challenges

2. Have a runtime error for accessing uninitialized variables.

To support this I need to allow varDecl without initializer expression, which I didn't, whoops!
Then it should store values in the symbol table as Options and use None to represent uninitialized.

### Assignment
commit 80a69a6

This last grammar change in chapter 8 allows assignment to existing variables rather than just declaration.

program -> block* EOF ;
block -> "{" declaration* "}" ;
declaration -> varDecl | statement ;
varDelc -> "var" IDENTIFIER ( "=" expression )? ";" ;

statement -> exprStatement | printStatement | block ;
exprStatement -> expression ";" ;
printStatement -> print expression ";" ;

expression -> assignment ;
assignment -> IDENTIFIER "=" assignment | equality;
equality -> comparison ( ( "!=" | "==" ) ) comparison )* ;
comparison -> term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term -> factor ( ( "-" | "+" ) ) factor )* ;
factor -> unary ( ( "/" | "*" ) ) unary )* ;
unary -> ( "!" | "-" ) unary | primary ;
primary -> NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" | IDENTIFIER ;

### Conditional statements
commit 45cb6d3

Here we add if else statements

program -> block* EOF ;
block -> "{" declaration* "}" ;
declaration -> varDecl | statement ;
varDelc -> "var" IDENTIFIER ( "=" expression )? ";" ;

statement -> exprStatement | printStatement | ifStatement | block ;
exprStatement -> expression ";" ;
printStatement -> print expression ";" ;
ifStatement -> "if" "(" expression ")" ( "else" expression )? ;

expression -> assignment ;
assignment -> IDENTIFIER "=" assignment | equality;
equality -> comparison ( ( "!=" | "==" ) ) comparison )* ;
comparison -> term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term -> factor ( ( "-" | "+" ) ) factor )* ;
factor -> unary ( ( "/" | "*" ) ) unary )* ;
unary -> ( "!" | "-" ) unary | primary ;

### And Or (logical operators)
commit 5641fbb

logic_or and logic_and added below

program -> block* EOF ;
block -> "{" declaration* "}" ;
declaration -> varDecl | statement ;
varDelc -> "var" IDENTIFIER ( "=" expression )? ";" ;

statement -> exprStatement | printStatement | ifStatement | block ;
exprStatement -> expression ";" ;
printStatement -> print expression ";" ;
ifStatement -> "if" "(" expression ")" ( "else" expression )? ;

expression -> assignment ;
assignment -> IDENTIFIER "=" assignment | logic_or;
logic_or -> logic_and ( "or" logic_and )* ;
logic_and -> equality  ( "and" equality ) ;
equality -> comparison ( ( "!=" | "==" ) ) comparison )* ;
comparison -> term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term -> factor ( ( "-" | "+" ) ) factor )* ;
factor -> unary ( ( "/" | "*" ) ) unary )* ;
unary -> ( "!" | "-" ) unary | primary ;

### While loop
commit 83a9b0e

Here we add the while statement

program -> block* EOF ;
block -> "{" declaration* "}" ;
declaration -> varDecl | statement ;
varDelc -> "var" IDENTIFIER ( "=" expression )? ";" ;

statement -> exprStatement | printStatement | ifStatement | whileStatement | block ;
whileStatement -> "while" "(" expression ")" statement ;
exprStatement -> expression ";" ;
printStatement -> print expression ";" ;
ifStatement -> "if" "(" expression ")" ( "else" expression )? ;

expression -> assignment ;
assignment -> IDENTIFIER "=" assignment | logic_or;
logic_or -> logic_and ( "or" logic_and )* ;
logic_and -> equality  ( "and" equality ) ;
equality -> comparison ( ( "!=" | "==" ) ) comparison )* ;
comparison -> term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term -> factor ( ( "-" | "+" ) ) factor )* ;
factor -> unary ( ( "/" | "*" ) ) unary )* ;
unary -> ( "!" | "-" ) unary | primary ;

### End of chapter 9 - For loop (desugaring to while)
commit 9578d37

program -> block* EOF ;
block -> "{" declaration* "}" ;
declaration -> varDecl | statement ;
varDelc -> "var" IDENTIFIER ( "=" expression )? ";" ;

statement -> exprStatement | printStatement | ifStatement | whileStatement | forStatement | block ;
forStatement -> "for" "(" ( varDecl | exprStmt | ";" )
  expression? ";"
  expression? ")" statement ;
whileStatement -> "while" "(" expression ")" statement ;
exprStatement -> expression ";" ;
printStatement -> print expression ";" ;
ifStatement -> "if" "(" expression ")" ( "else" expression )? ;

expression -> assignment ;
assignment -> IDENTIFIER "=" assignment | logic_or;
logic_or -> logic_and ( "or" logic_and )* ;
logic_and -> equality  ( "and" equality ) ;
equality -> comparison ( ( "!=" | "==" ) ) comparison )* ;
comparison -> term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term -> factor ( ( "-" | "+" ) ) factor )* ;
factor -> unary ( ( "/" | "*" ) ) unary )* ;
unary -> ( "!" | "-" ) unary | primary ;

### Chapter 10 - Functions
commit 

program -> block* EOF ;
block -> "{" declaration* "}" ;
declaration -> varDecl | statement ;
varDelc -> "var" IDENTIFIER ( "=" expression )? ";" ;

statement -> exprStatement | printStatement | ifStatement | whileStatement | forStatement | block ;
forStatement -> "for" "(" ( varDecl | exprStmt | ";" )
  expression? ";"
  expression? ")" statement ;
whileStatement -> "while" "(" expression ")" statement ;
exprStatement -> expression ";" ;
printStatement -> print expression ";" ;
ifStatement -> "if" "(" expression ")" ( "else" expression )? ;

expression -> assignment ;
assignment -> IDENTIFIER "=" assignment | logic_or;
logic_or -> logic_and ( "or" logic_and )* ;
logic_and -> equality  ( "and" equality ) ;
equality -> comparison ( ( "!=" | "==" ) ) comparison )* ;
comparison -> term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term -> factor ( ( "-" | "+" ) ) factor )* ;
factor -> unary ( ( "/" | "*" ) ) unary )* ;
unary -> ( "!" | "-" ) unary | call ;
call -> primary ( "(" arguments? ")" )* ;
arguments -> expression ( "," expression )* ;

Work in progress commit cbb0b98. At this stage I was figuring out how to represent lox callable
in Rust and settled as creating a new Value of type Callable. When evaluated this contains a 
value and the values of the arguments.

The next step will be to write some way to convert a value to a function, probably with a function
lookup table to mirror the variable one, similar to how some lisps have a function and variable
symbol table.

Next section of the book
https://craftinginterpreters.com/functions.html#call-type-errors




