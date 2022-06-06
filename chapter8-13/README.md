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

### While loop
commit 

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
