# Implementation of chapters 8 to 13
This culminates in the finished lox interpreter. It picks up from chapter 7 where I have implemented expression evaluation.

At chapter 7 the framework of the interpreter is done and all that remains is the other language features including functions and classes, scope and variables.

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

### Updated to add statements
complete at commit 13b96ef

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

### Updated to add declarations and variable usage

program -> declaration* EOF ;
declaration -> varDecl | statement ;
varDelc -> "var" IDENTIFIER ( "=" expression )? ";" ;

expression -> equality ;
equality -> comparison ( ( "!=" | "==" ) ) comparison )* ;
comparison -> term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term -> factor ( ( "-" | "+" ) ) factor )* ;
factor -> unary ( ( "/" | "*" ) ) unary )* ;
unary -> ( "!" | "-" ) unary | primary ;
primary -> NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" | IDENTIFIER ;


