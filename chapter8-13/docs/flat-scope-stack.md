# Flat Scope Stack Environment

## Problem

The current `EvalState` uses `Rc<RefCell<EvalState>>` to allow mutation of parent scopes during variable assignment. This introduces reference counting, interior mutability, and requires cloning values out of the `RefCell` on every lookup. For the features implemented in chapters 8-9 (blocks, variable declaration, assignment, control flow), this complexity is unnecessary.

## Key Insight

During execution without closures, scopes form a strict stack: you enter a block, execute its statements, then leave. No scope ever outlives its enclosing block. This means a single `Vec` of hash maps, owned by one mutable reference, is sufficient.

## Implementation

### The Data Structure

Replace `EvalState` with:

```rust
use std::collections::HashMap;

pub struct Environment {
    scopes: Vec<HashMap<String, Option<Value>>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            scopes: vec![HashMap::new()], // global scope
        }
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn define(&mut self, name: String, value: Option<Value>) {
        self.scopes.last_mut().unwrap().insert(name, value);
    }

    pub fn lookup(&self, name: &str) -> EvalResult {
        for scope in self.scopes.iter().rev() {
            if let Some(entry) = scope.get(name) {
                return match entry {
                    Some(v) => Ok(v.clone()),
                    None => Err(RuntimeError(format!(
                        "Uninitialized variable: {}", name
                    ))),
                };
            }
        }
        Err(RuntimeError(format!("Unknown variable: {}", name)))
    }

    pub fn assign(&mut self, name: &str, value: Value) -> EvalResult {
        for scope in self.scopes.iter_mut().rev() {
            if scope.contains_key(name) {
                scope.insert(name.to_string(), Some(value.clone()));
                return Ok(value);
            }
        }
        Err(RuntimeError(format!("Unknown variable: {}", name)))
    }
}
```

### Changes to eval_statements

The function signature changes from passing `Rc<RefCell<EvalState>>` to passing `&mut Environment`:

```rust
pub fn eval_statements(
    stmts: &[Stmt],
    env: &mut Environment,
) -> Result<(), RuntimeError> {
    env.push_scope();

    for stmt in stmts {
        match stmt {
            Stmt::VarDecl(id, Some(expr)) => {
                let value = eval_expression(expr, env)?;
                env.define(id.to_string(), Some(value));
            }
            Stmt::VarDecl(id, None) => {
                env.define(id.to_string(), None);
            }
            Stmt::Block(stmts) => {
                eval_statements(stmts, env)?;
            }
            Stmt::Print(expr) => {
                let value = eval_expression(expr, env)?;
                println!("{}", value);
            }
            Stmt::Expression(expr) => {
                eval_expression(expr, env)?;
            }
            Stmt::If(expr, then_stmt, else_stmt) => {
                let cond = eval_expression(expr, env)?;
                if bool_value(&cond) {
                    eval_statements(then_stmt, env)?;
                } else {
                    eval_statements(else_stmt, env)?;
                }
            }
            Stmt::While(expr, body) => loop {
                let cond = eval_expression(expr, env)?;
                if bool_value(&cond) {
                    eval_statements(body, env)?;
                } else {
                    break;
                }
            },
        }
    }

    env.pop_scope();
    Ok(())
}
```

### Changes to eval_expression

The signature becomes `fn eval_expression(expr: &Expr, env: &mut Environment) -> EvalResult`. The `&mut` is needed because assignment expressions mutate the environment:

```rust
pub fn eval_expression(expr: &Expr, env: &mut Environment) -> EvalResult {
    match expr {
        Literal(value) => Ok(value.clone()),
        Variable(id) => env.lookup(id),
        Assign(id, expr) => {
            let value = eval_expression(expr, env)?;
            env.assign(id, value)
        }
        Binary(left, operator, right) => {
            let left = eval_expression(left, env)?;
            let right = eval_expression(right, env)?;
            // ... same arithmetic/comparison logic
        }
        // ... other variants unchanged in structure
    }
}
```

### Changes to main.rs

At the top level, create one `Environment` and pass it mutably:

```rust
let mut env = Environment::new();
eval_statements(&stmts, &mut env)?;
```

For the REPL, keep the `Environment` alive across iterations so global variables persist between lines.

## What You Gain

- No `Rc`, no `RefCell`, no `borrow()`/`borrow_mut()` calls
- No `Rc::clone(&eval_state)` noise at every recursive call
- Single owner (`&mut`) means no runtime borrow-check panics
- Slightly less cloning (no need to clone values out of a `RefCell` guard — though `lookup` still clones the `Value` itself since we can't return a reference into the vec without a borrow on the whole `Environment`)

## Limitation: No Closures

This design breaks when a function captures variables from an enclosing scope and is called after that scope has been popped. For example:

```lox
fun makeCounter() {
  var i = 0;
  fun count() {
    i = i + 1;
    print i;
  }
  return count;
}

var counter = makeCounter();
counter(); // should print 1
counter(); // should print 2
```

When `makeCounter` returns, its scope is popped. But `count` still needs access to `i`. The flat stack cannot represent this — the scope is gone. If you need closures, see the arena-based approach.
