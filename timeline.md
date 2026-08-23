# Project Timeline & Development History

This document outlines the chronological development history of the `craftinginterpreters` repository based on the Git commit history.

---

## Timeline Summary by Phase

```
April 2022          May 2022              May–June 2022        June–Nov 2022        Aug 2026
┌────────────────┐  ┌──────────────────┐  ┌─────────────────┐  ┌─────────────────┐  ┌──────────────────┐
│  Chapter 1     │  │  Chapters 4–7    │  │  Chapters 8–9   │  │  Chapter 10     │  │  Arena Redesign  │
│  C Linked List │─>│  Scanners & Expr │─>│  State & Loops  │─>│  Call Syntax    │─>│  Environment     │
│  (CuTest)      │  │  (jli & jlinom)  │  │  (rlox)         │  │  (Value::Call)  │  │  (57 unit tests) │
└────────────────┘  └──────────────────┘  └─────────────────┘  └─────────────────┘  └──────────────────┘
```

---

## Detailed Chronological Milestones

### Phase 1: Project Initiation & Chapter 1 (April 2022)
- **2022-04-27 to 2022-04-28**: Project repository initialized.
- **2022-04-28 to 2022-04-30**: Implemented Chapter 1 challenge: heap-allocated doubly-linked list in C ([`chapter1/c-double-linked-list/`](chapter1/c-double-linked-list/)) with unit tests using [CuTest](https://github.com/asimjalis/cutest) and Makefile build system.

### Phase 2: Lexical Analysis & Scanning (Early May 2022)
- **2022-04-30 to 2022-05-04**: Built the initial Lox scanner in Rust ([`chapter4/jli/`](chapter4/jli/)), implementing character iteration, keyword/identifier scanning, string/number literals, and passing the official book tests.
- **2022-05-06 to 2022-05-10**: Reimplemented the scanner using the [nom](https://github.com/Geal/nom) parser-combinator library ([`chapter4/jlinom/`](chapter4/jlinom/)), adding support for C-style nested block comments (`/* ... /* ... */ ... */`).

### Phase 3: AST, Expression Parsing & Evaluation (Mid May 2022)
- **2022-05-10 to 2022-05-18**: Transitioned from Java's OOP Visitor pattern to Rust enums (`Expr`, `Operator`, `Value`) in [`chapter5-7/`](chapter5-7/). Built recursive-descent expression parser.
- **2022-05-18 to 2022-05-20**: Implemented expression evaluation for arithmetic (`+`, `-`, `*`, `/`), comparisons (`<`, `<=`, `>`, `>=`), equality (`==`, `!=`), unary operators (`!`, `-`), and string concatenation. Added CLI flags with `structopt`.

### Phase 4: Statements, State & Control Flow (Late May – June 2022)
- **2022-05-20**: Created [`chapter8-13/`](chapter8-13/) (`rlox`) to build the full tree-walking interpreter.
- **2022-05-21 to 2022-05-28**: Added statement parsing (`Stmt::Print`, `Stmt::Expression`), variable declarations (`var x = 1;`), and lexical block scopes (`{ ... }`). Implemented runtime error challenge for uninitialized variable reads.
- **2022-06-02 to 2022-06-03**: Implemented assignment expressions (`x = 2;`) with parent-scope mutation.
- **2022-06-04 to 2022-06-06**: Implemented conditional branching (`if`/`else`), short-circuiting logical operators (`and`/`or`), `while` loops, and `for` loop desugaring.

### Phase 5: Function Call Parsing (June – November 2022)
- **2022-06-17 to 2022-11-02**: Implemented function call expression parsing (`callee(args...)`) in `parse.rs` and initial argument evaluation in `eval.rs`, storing results in `Value::Callable`.

### Phase 6: Maintenance & Modernization (2022–2025)
- **2022-12-13 to 2022-12-21**: Upgraded `rustyline` for REPL command history (`history.txt`) and improved error propagation.
- **2024-11-17 to 2025-10-16**: Updated dependencies, upgraded to Rust 2024 edition, and resolved compiler warnings.

### Phase 7: Arena-Based Environment Architecture (August 2026)
- **2026-08-16**: Implemented `Display` formatting for errors, fixed compiler warnings, and added Claude/Agent guidance docs.
- **2026-08-21**: Major architectural refactor:
  - Replaced `Rc<RefCell<EvalState>>` with an index-based arena `Environment` ([`chapter8-13/src/env.rs`](chapter8-13/src/env.rs)).
  - Added comprehensive unit tests across `env.rs`, `eval.rs`, `parse.rs`, and `scan.rs` (57 passing tests).
  - Documented arena design in [`docs/arena-environment.md`](chapter8-13/docs/arena-environment.md) and [`docs/flat-scope-stack.md`](chapter8-13/docs/flat-scope-stack.md).

---

## Detailed Commit Log Reference

| Hash | Date | Description |
| :--- | :--- | :--- |
| `5421bea` | 2022-04-27 | get started |
| `b0423f0` | 2022-04-28 | f cmake for now |
| `629c6da` | 2022-04-28 | add tests and move to files |
| `ad5a704` | 2022-04-30 | update tests |
| `c5e19c1` | 2022-04-30 | start justins lox interpreter |
| `f6113b0` | 2022-05-01 | Add sample lox code and begin design of Lox scanner |
| `ce30cde` | 2022-05-01 | basic scanner working |
| `198c328` | 2022-05-02 | add keywords scanner |
| `f17be09` | 2022-05-04 | Finish first version of lox scanner |
| `a7c2e76` | 2022-05-06 | starting new version (nom scanner) |
| `24fd384` | 2022-05-08 | all book tests pass |
| `41a5058` | 2022-05-10 | Allow nested comments |
| `0c6cb7a` | 2022-05-10 | start chapter 5 from jlinom |
| `aaa7c5c` | 2022-05-15 | Add an Operator type, implement some more parsing |
| `953f86d` | 2022-05-16 | expression parser mostly working needs error handler |
| `9593a47` | 2022-05-19 | interpreter can run comparison operators |
| `78b0946` | 2022-05-19 | arithmetic |
| `346b2d6` | 2022-05-19 | support string append with plus |
| `043d678` | 2022-05-20 | start chapter 8 onwards, fix display bug |
| `13b96ef` | 2022-05-21 | parsing and evaluation of expression and print statements |
| `175284d` | 2022-05-23 | global variable declaration and use |
| `c8ec51e` | 2022-05-28 | variable block scope |
| `ffe2c72` | 2022-05-28 | do uninit var challenge |
| `80a69a6` | 2022-06-03 | finally! assignment |
| `45cb6d3` | 2022-06-04 | Add conditionals |
| `5641fbb` | 2022-06-05 | add logical conditions |
| `83a9b0e` | 2022-06-05 | while loops |
| `9578d37` | 2022-06-06 | for loops desugared to while |
| `42a39d3` | 2022-06-17 | can parse function calls |
| `cbb0b98` | 2022-11-02 | function calls parsed and first step of collecting params and calling in eval |
| `3b1b12c` | 2022-12-13 | Upgrade rustyline and fiddle with options |
| `4257f92` | 2024-11-17 | clean up and update libraries |
| `e40acfc` | 2025-10-14 | use more recent rust edition and update crates |
| `837194c` | 2026-08-21 | reworking env - adding env |
| `3a01004` | 2026-08-21 | Implementation of environment completed |
| `cfd08bc` | 2026-08-21 | Merge env-improve: replace Rc<RefCell<EvalState>> with arena-based Environment |
| `c0422cd` | 2026-08-21 | Add unit tests (57 tests passing) |
| `ee0e024` | 2026-08-21 | clippy is happy |
