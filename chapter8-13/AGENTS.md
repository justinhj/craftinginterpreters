# AGENTS.md

## Build/Lint/Test Commands
- Build: cargo build
- Run: cargo run [options] [file]
- Test all: cargo test
- Test single: cargo test &lt;test_name&gt; -- --exact
- Lint: cargo clippy
- Format: cargo fmt --check
- Run examples: cargo run samples/&lt;file&gt;.lox

## Code Style Guidelines
- Naming: snake_case for vars/functions, CamelCase for types/enums.
- Imports: Group std, external crates, then local modules.
- Formatting: Use cargo fmt; 100 char line limit, indent 4 spaces.
- Types: Strong typing with enums for Token, Expr, Stmt, Value; prefer Options over nulls.
- Error Handling: Custom error enums with From impls; propagate with ?.
- Structure: Modular (scan.rs, parse.rs, eval.rs); use Rc<RefCell> for shared state.
- Comments: Sparse, focus on non-obvious logic; no unnecessary comments.
- Tests: Unit tests in modules; use pretty_assertions for comparisons.
- Conventions: Follow Rust idioms; mimic book’s Lox grammar in parse.rs.

