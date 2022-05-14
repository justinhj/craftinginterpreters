# Chapter 5
In this chapter we implement the AST representation. Changing from Java's OOP model to Rust's enum.

## Visitor Pattern 
The implementation uses the visitor pattern to allow operations over the AST in an OOP style. Will need to translate that to Rust, but first what is the visitor pattern again?

### In Java and other OOP languages
Idea is to separate algorithms from the objects they operate on.

Pass the original data class as a parameter to a visitor.

This implements double-dispatch. The idea is we don't want to do a switch on instanceOf in a function that traverses the graph, instead each node has an `accept` method that you pass the visitors to. You may have an print json visitor and a print xml visitor. 

You pass the visitor as an argument to node accept. 

So in our example we have some types like Binary expression and Unary expression, we can make a visitor for pretty printing.

Expr types
Binary, Unary, Literal etc

Those nodes would get an accept method that takes a visitor, the visitor would have functions for each node types, so:

PrintBinary, PrintUnary and so on.

Clearly by changing the visitor you could change from pretty print to condensed print or other operations.

### In Rust
Proposal is to make a struct and matching impl for each type of `visitor`. That way your core data type remains a simple enum with zero functionality and all the operations on it are encoded by a stateful struct and implementation of some useful operations.

## Grammar
The idea of this chapter is to implement a grammar to parse the scanned program.



