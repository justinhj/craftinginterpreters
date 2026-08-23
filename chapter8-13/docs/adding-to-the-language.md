# Adding to the language
## Steps
Here are the steps you typically need to go through.
Grammar Rule ──> 1. Scanner ──> 2. AST Definition ──> 3. Parser ──> 4. Evaluator / Environment ──> 5. Tests
──────
### Step 1: Lexer / Scanner (scan.rs)

(Only needed if the feature introduces new keywords or punctuation, e.g. return, fun, ,, etc.)

1. Add to scan.rs:8 enum:
  pub enum Token {
      // ...
      Return, // e.g. new keyword
  }

2. **Register in scan.rs:290** (for keywords) or scan.rs:238 (for symbols).
3. Add to Display and Debug implementations in scan.rs.
──────
### Step 2: AST Nodes & Values (parse.rs)

1. Decide if it is a Statement, Expression, or Value:
    • Statement (executes side-effects, produces no value): Add variant to parse.rs:63 (e.g. Stmt::Return(Option<Expr>),
    Stmt::Function(...)).
    • Expression (evaluates to a value): Add variant to parse.rs:94 (e.g. Expr::Binary, Expr::Call).
    • Runtime Value: Add variant to parse.rs:6 if it introduces a new runtime type (e.g. Value::Function).
2. Implement Display: Add formatting for the new variant in impl Display for Stmt/Expr/Value.
──────
### Step 3: Parser Implementation (parse.rs)

1. Place in the Grammar Hierarchy:
    • Declarations: parse_declaration (var, fun, class).
    • Statements: parse_statement (if, while, for, print, return, {...}).
    • Expressions: Place at the correct precedence tier (parse_assignment → parse_or → parse_and → parse_equality →
    parse_comparison → parse_term → parse_factor → parse_unary → parse_call → parse_primary).
2. Write the parsing function:
    • Consume expected tokens using expect(ps, Token::...).
    • Peek tokens with peek(ps).
    • Advance with advance(ps).
    • Return clear ParseError on syntax mistakes.

──────
### Step 4: Runtime & Evaluation (eval.rs & env.rs)

1. Match the variant in the evaluator:
    • In eval.rs:33 (for Stmt) or eval.rs:84 (for Expr).
2. Environment Interactions:
    • Does it create a new scope? let prev = environment.push_scope(); ... environment.pop_scope(prev);
    • Does it declare/mutate a variable? environment.define(...) or environment.assign(...).
    • Does it look up a variable? environment.lookup(...).
3. Runtime Error Checking:
    • Return Err(RuntimeError(...)) for type mismatches, arity errors, undefined variables, or invalid operations.
4. Control Flow Unwinding:
    • For statements that interrupt normal linear flow (like return, break), return a dedicated signal/enum variant in
    the eval result.

──────
### Step 5: Unit Tests & Samples

1. Unit Tests in parse.rs: Test AST structure from source strings.
2. Unit Tests in eval.rs: Test runtime value evaluation and expected RuntimeErrors.
3. Add a .lox sample: Create samples/my_feature.lox and verify via:
  cargo run -- -s samples/my_feature.lox  # Inspect tokens
  cargo run -- -p samples/my_feature.lox  # Inspect AST
  cargo run -- samples/my_feature.lox     # Run program
