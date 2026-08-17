# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```sh
cargo build              # debug build
cargo run                # start REPL
cargo run samples/foo.lox        # run a Lox file
cargo run -- -s samples/foo.lox  # show scan tokens
cargo run -- -p samples/foo.lox  # show parsed AST
cargo run -- -e false samples/foo.lox  # parse only, skip eval
cargo test               # run all tests
cargo test <test_name> -- --exact  # run a single test
cargo clippy             # lint
cargo fmt --check        # check formatting
```

## Architecture

The interpreter is a classic pipeline: **scan → parse → eval**.

```
src/scan.rs   — tokenises a source string into Vec<Token> using nom + nom_locate
src/parse.rs  — turns Vec<Token> into Vec<Stmt> (the AST) using recursive-descent
src/eval.rs   — walks Vec<Stmt> and evaluates them against an EvalState
src/main.rs   — wires the pipeline; provides CLI (structopt) and REPL (rustyline)
src/lib.rs    — re-exports the three modules for library use
```

**Key types** (all defined in `parse.rs`):
- `Token` — lexer output; carries a `TokenKind` and source-location span
- `Expr` — expression AST node (Literal, Binary, Unary, Logical, Grouping, Variable, Assign, Call)
- `Stmt` — statement AST node (Expression, Print, VarDecl, Block, If, While)
- `Value` — runtime value (Number, Boolean, String, Nil, Callable)
- `Operator` — shared enum used by both Expr variants and eval dispatch

**Scope / environment** (`eval.rs`):
- `EvalState` holds a `HashMap<String, Option<Value>>` (symbols) and an optional `Rc<RefCell<EvalState>>` parent pointer.
- Variable lookup and assignment walk the parent chain; `None` in the map represents a declared-but-uninitialised variable.
- Each block and the top-level call each get their own `EvalState` linked to the enclosing one via `new_from_parent`.

**Function calls** (work-in-progress as of chapter 10):
- `Expr::Call` is parsed and evaluated, but `eval_call` currently wraps the callee + evaluated arguments into `Value::Callable` without executing them — full function dispatch is the next step.

**Error types**:
- `ScanError`, `ParseError`, `RuntimeError` are newtype wrappers over `String`.
- `main.rs` defines `InterpreterError` which unifies all three via `From` impls, discarding the inner message (the inner `String` fields are currently unused — two active `dead_code` warnings).
