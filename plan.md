# Implementation Progress & Plan (Chapters 8–13)

This document tracks current implementation progress through Robert Nystrom's [Crafting Interpreters](https://craftinginterpreters.com/contents.html) (Part II: A Tree-Walk Interpreter) and outlines the remaining roadmap for `chapter8-13` (`rlox`).

---

## Current Status Overview

- **Reached**: [Chapter 10: Functions](https://craftinginterpreters.com/functions.html)
- **Exact Location**: Section [10.1.2: Interpreting Function Calls](https://craftinginterpreters.com/functions.html#interpreting-function-calls) / [10.1.3: Call Type Errors](https://craftinginterpreters.com/functions.html#call-type-errors)

---

## 1. Completed Chapters in `chapter8-13`

### [Chapter 8: Statements and State](https://craftinginterpreters.com/statements-and-state.html)
- **Statements**: Implemented [`Stmt::Print`](chapter8-13/src/parse.rs) and [`Stmt::Expression`](chapter8-13/src/parse.rs).
- **Variables**: Variable declarations with/without initializers ([`Stmt::VarDecl`](chapter8-13/src/parse.rs)).
- **Assignment**: Assignment expressions ([`Expr::Assign`](chapter8-13/src/parse.rs)).
- **Lexical Scoping**: Block statements ([`Stmt::Block`](chapter8-13/src/parse.rs)) with variable shadowing and runtime errors for uninitialized/undefined variables.
- **Arena-Based Environment**: Refactored from `Rc<RefCell<EvalState>>` to an arena-based [`Environment`](chapter8-13/src/env.rs) using `EnvId = usize` index pointers in a `Vec<Env>` (documented in [`docs/arena-environment.md`](chapter8-13/docs/arena-environment.md)).

### [Chapter 9: Control Flow](https://craftinginterpreters.com/control-flow.html)
- **Conditionals**: `if` and `else` branching ([`Stmt::If`](chapter8-13/src/parse.rs)).
- **Logical Operators**: Short-circuiting `and` / `or` evaluation ([`Expr::Logical`](chapter8-13/src/parse.rs)).
- **While Loops**: `while` loops ([`Stmt::While`](chapter8-13/src/parse.rs)).
- **For Loops**: Desugared during parsing into `while` loops enclosed in block scopes.

---

## 2. Chapter 10: Functions (In Progress)

### Completed So Far
- **Parsing Call Expressions**: `callee(arg1, arg2, ...)` parsed in [`parse_call`](chapter8-13/src/parse.rs) into [`Expr::Call`](chapter8-13/src/parse.rs).
- **Preliminary Call Evaluation**: [`eval_call`](chapter8-13/src/eval.rs) evaluates the callee and argument expressions and packages them into [`Value::Callable`](chapter8-13/src/parse.rs).
- **Closure Scope Foundation**: Arena environment architecture in [`src/env.rs`](chapter8-13/src/env.rs) is ready to hold captured `EnvId` references without lifetime issues or ref-counting cycles.

### Immediate Next Steps to Complete Chapter 10
1. **[10.1.3 & 10.1.4 Call Type Errors & Arity Checks](https://craftinginterpreters.com/functions.html#call-type-errors)**:
   - Ensure runtime error when attempting to call non-callable values.
   - Enforce arity matching between arguments provided and expected parameters.
   - Optional: limit maximum arguments (255) during parsing.
2. **[10.2 Native Functions](https://craftinginterpreters.com/functions.html#native-functions)**:
   - Define a native callable representation (e.g. `clock()`) in the global environment.
3. **[10.3 Function Declarations](https://craftinginterpreters.com/functions.html#function-declarations)**:
   - Add `Stmt::Function { name: String, params: Vec<String>, body: Vec<Stmt> }`.
   - Update `parse_declaration` to parse `fun name(param1, param2) { body }`.
4. **[10.4 Function Objects](https://craftinginterpreters.com/functions.html#function-objects)**:
   - Implement user-defined function representation (storing parameter names, body statements, and closure `EnvId`).
   - Execute body statements within a new child environment bound to parameters.
5. **[10.5 Return Statements](https://craftinginterpreters.com/functions.html#return-statements)**:
   - Add `Stmt::Return(Option<Expr>)`.
   - Implement control flow unwinding for `return` values during statement evaluation (e.g., via a dedicated `Result` or return error signal).
6. **[10.6 Local Functions & Closures](https://craftinginterpreters.com/functions.html#local-functions-and-closures)**:
   - Capture `closure_env: EnvId` when function values are created.
   - Run closure body within a child scope attached to `closure_env` (already detailed in [`docs/arena-environment.md`](chapter8-13/docs/arena-environment.md)).

---

## 3. Remaining Roadmap (Part II: jlox / rlox)

### [Chapter 11: Resolving and Binding](https://craftinginterpreters.com/resolving-and-binding.html)
- Implement a semantic analysis pass between parsing and execution.
- Resolve local variable references to static scope depths/slot offsets to fix closure and variable resolution corner cases.

### [Chapter 12: Classes](https://craftinginterpreters.com/classes.html)
- Class declarations (`class Name { ... }`).
- Class instances (first-class objects) and property `get` / `set` expressions (`object.field`).
- Method calls on instances and method binding.
- `this` keyword resolution.
- Constructors and initialization (`init` methods).

### [Chapter 13: Inheritance](https://craftinginterpreters.com/inheritance.html)
- Class inheritance syntax (`class Sub < Super { ... }`).
- Inherited method lookup.
- `super` method calls and `super` binding in methods.
