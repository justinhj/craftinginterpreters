# Crafting Interpreters (Rust & C)

This repository contains implementations, exercises, and challenges from Robert Nystrom's book [Crafting Interpreters](https://craftinginterpreters.com/). The primary focus is building the tree-walking **jlox** interpreter in Rust (**rlox**), exploring Rust idioms such as algebraic data types (enums) and an arena-based environment management model.

---

## Repository Structure & Overview

```
craftinginterpreters/
├── chapter1/           # Chapter 1 C challenge: Doubly-linked list
│   └── c-double-linked-list/
├── chapter4/           # Chapter 4: Lexical Analysis / Scanning
│   ├── jli/            # Manual scanner (following jlox structure)
│   └── jlinom/         # Combinator-based scanner using nom
├── chapter5-7/         # Chapters 5–7: AST, Parsing & Expression Evaluation
│   └── src/            # scan.rs, parse.rs, eval.rs, main.rs
└── chapter8-13/        # Chapters 8–13: Full Tree-Walking Interpreter (rlox)
    ├── docs/           # Architecture writeups (arena environment, scope stacks)
    ├── samples/        # Sample Lox programs
    └── src/            # scan.rs, parse.rs, eval.rs, env.rs, main.rs
```

---

## Chapter Details

### [Chapter 1: Introduction & C Doubly-Linked List](chapter1/)
- Answers and notes for the Chapter 1 challenge questions.
- [c-double-linked-list/](chapter1/c-double-linked-list/): A C implementation of a doubly-linked list of heap-allocated strings with insertion, search, and deletion operations, unit-tested with the [CuTest](https://github.com/asimjalis/cutest) framework.

### [Chapter 4: Scanning / Lexical Analysis](chapter4/)
Explores two approaches to lexical analysis in Rust:
- [jli](chapter4/jli/): A hand-written scanner mirroring the Java implementation in *Crafting Interpreters*, using character iteration, peeking, and lexeme extraction.
- [jlinom](chapter4/jlinom/): A parser combinator implementation using [nom](https://github.com/Geal/nom), including support for C-style nested block comments (`/* ... /* ... */ ... */`).

### [Chapters 5–7: Representing Code, Parsing & Evaluating Expressions](chapter5-7/)
- Replaces Java's OOP Visitor pattern with Rust's algebraic data types (`enum Expr`, `enum Value`, `enum Operator`).
- Recursive-descent parser for expressions (`+`, `-`, `*`, `/`, comparison, equality, unary negation/bang, grouping).
- Tree-walking expression evaluator.

### [Chapters 8–13: Statements, State, Control Flow & Functions](chapter8-13/)
The core tree-walking interpreter crate (`rlox`), encompassing language features up through Chapter 10:

- **Statements & State (Chapter 8)**:
  - Statements (`Stmt::Expression`, `Stmt::Print`, `Stmt::VarDecl`, `Stmt::Block`).
  - Lexical block scoping (`{ ... }`) with shadowing and uninitialized variable detection.
  - Variable assignment expressions (`x = value`).
- **Control Flow (Chapter 9)**:
  - Conditional branching with `if` and `else` (`Stmt::If`).
  - Short-circuiting logical operators (`and`, `or`).
  - `while` loops (`Stmt::While`).
  - `for` loops desugared into `while` loops within synthetic block scopes.
- **Functions & Calls (Chapter 10)**:
  - Function call syntax parsing: `callee(arg1, arg2, ...)`.
  - First-class callable runtime values (`Value::Callable(Box<Value>, Vec<Value>)`).
- **Arena-Based Environment Architecture**:
  - Located in [`src/env.rs`](chapter8-13/src/env.rs).
  - Replaced reference-counted scope trees (`Rc<RefCell<EvalState>>`) with a centralized index-based arena (`Environment` with `Vec<Env>` and `EnvId = usize`).
  - Eliminates runtime borrow checks and `Rc::clone` overhead while cleanly providing persistent environments needed for closures.
  - Detailed design notes in [`docs/arena-environment.md`](chapter8-13/docs/arena-environment.md) and [`docs/flat-scope-stack.md`](chapter8-13/docs/flat-scope-stack.md).
- **CLI & Interactive REPL**:
  - Built with `structopt` for command-line flags and `rustyline` for an interactive REPL supporting persistent command history (`history.txt`).

---

## Building and Running

### Chapter 8–13 (`rlox`)

```sh
cd chapter8-13

# Start the interactive REPL
cargo run

# Execute a Lox script
cargo run -- samples/for3.lox

# Inspect token stream (scanning)
cargo run -- -s samples/conditionals1.lox

# Inspect parsed AST
cargo run -- -p samples/for.lox

# Parse only without evaluation
cargo run -- -e false samples/blocks1.lox

# Run all unit tests (57 tests)
cargo test

# Run clippy linter
cargo clippy
```

### Chapter 5–7 (`rlox` expression interpreter)

```sh
cd chapter5-7
cargo test
cargo run -- samples/expr1.lox
```

### Chapter 4 Scanners

```sh
# Manual scanner
cd chapter4/jli
cargo test
cargo run -- samples/sample1.lox

# Nom-based scanner
cd chapter4/jlinom
cargo test
cargo run -- samples/sample1.lox
```

### Chapter 1 C Doubly-Linked List

```sh
cd chapter1/c-double-linked-list
make
make test
```
