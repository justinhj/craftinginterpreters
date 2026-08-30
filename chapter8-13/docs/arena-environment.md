# Arena-Based Environment

> Implemented in branch `env-improve`.

## Problem

Once closures are introduced (chapter 10), environments can no longer be a simple stack. A closure captures a reference to the environment where it was defined, and that environment must survive after the defining block has exited. The current `Rc<RefCell<EvalState>>` solves this with reference counting and interior mutability, but at the cost of pervasive `Rc::clone`, runtime borrow checking, and mandatory `Value` cloning on every read.

The arena pattern is a Rust-idiomatic alternative that avoids both lifetime annotations and reference counting.

## Key Insight

Instead of each environment node owning or borrowing its parent, store all environments in a central `Vec` (the arena). Each environment refers to its parent by index (`usize`). Since indices are `Copy`, there's no ownership or borrowing relationship between environments — the arena owns everything.

## Implementation

### The Data Structures

```rust
use std::collections::HashMap;

type EnvId = usize;

#[derive(Debug)]
struct Env {
    parent: Option<EnvId>,
    symbols: HashMap<String, Option<Value>>,
}

#[derive(Debug)]
pub struct Environment {
    envs: Vec<Env>,
    current: EnvId,
}

impl Environment {
    pub fn new() -> Self {
        let global = Env {
            parent: None,
            symbols: HashMap::new(),
        };
        Environment {
            envs: vec![global],
            current: 0,
        }
    }

    pub fn capture(&self) -> EnvId {
        self.current
    }

    pub fn enter_scope(&mut self) -> EnvId {
        let new_id = self.envs.len();
        self.envs.push(Env {
            parent: Some(self.current),
            symbols: HashMap::new(),
        });
        let old = self.current;
        self.current = new_id;
        old // return previous scope for restoring later
    }

    pub fn enter_closure(&mut self, closure_parent: EnvId) -> EnvId {
        let new_id = self.envs.len();
        self.envs.push(Env {
            parent: Some(closure_parent),
            symbols: HashMap::new(),
        });
        let old = self.current;
        self.current = new_id;
        old
    }

    pub fn exit_scope(&mut self, previous: EnvId) {
        self.current = previous;
    }

    pub fn define(&mut self, name: String, value: Option<Value>) {
        self.envs[self.current].symbols.insert(name, value);
    }

    pub fn lookup(&self, name: &str) -> EvalResult {
        let mut env_id = Some(self.current);
        while let Some(id) = env_id {
            let env = &self.envs[id];
            if let Some(entry) = env.symbols.get(name) {
                return match entry {
                    Some(v) => Ok(v.clone()),
                    None => Err(RuntimeError(format!(
                        "Uninitialized variable: {}", name
                    ))),
                };
            }
            env_id = env.parent;
        }
        Err(RuntimeError(format!("Unknown variable: {}", name)))
    }

    /// Lookup starting from a specific environment (used by closures)
    pub fn lookup_in(&self, name: &str, start: EnvId) -> EvalResult {
        let mut env_id = Some(start);
        while let Some(id) = env_id {
            let env = &self.envs[id];
            if let Some(entry) = env.symbols.get(name) {
                return match entry {
                    Some(v) => Ok(v.clone()),
                    None => Err(RuntimeError(format!(
                        "Uninitialized variable: {}", name
                    ))),
                };
            }
            env_id = env.parent;
        }
        Err(RuntimeError(format!("Unknown variable: {}", name)))
    }

    pub fn assign(&mut self, name: &str, value: Value) -> EvalResult {
        let mut env_id = Some(self.current);
        while let Some(id) = env_id {
            let env = &mut self.envs[id];
            if env.symbols.contains_key(name) {
                env.symbols.insert(name.to_string(), Some(value.clone()));
                return Ok(value);
            }
            env_id = env.parent;
        }
        Err(RuntimeError(format!("Unknown variable: {}", name)))
    }

    /// Assign starting from a specific environment (used by closures)
    pub fn assign_in(&mut self, name: &str, value: Value, start: EnvId) -> EvalResult {
        let mut env_id = Some(start);
        while let Some(id) = env_id {
            let env = &mut self.envs[id];
            if env.symbols.contains_key(name) {
                env.symbols.insert(name.to_string(), Some(value.clone()));
                return Ok(value);
            }
            env_id = env.parent;
        }
        Err(RuntimeError(format!("Unknown variable: {}", name)))
    }
}
```

### How Closures Work

A closure captures the `EnvId` of the environment where it was defined:

```rust
#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Boolean(bool),
    String(String),
    Nil,
    Function {
        params: Vec<String>,
        body: Vec<Stmt>,
        closure_env: EnvId, // captured environment
    },
}
```

When a function is called:

```rust
fn eval_call(
    callee: &Value,
    arguments: Vec<Value>,
    env: &mut Environment,
) -> EvalResult {
    match callee {
        Value::Function { params, body, closure_env } => {
            // Create a new scope whose parent is the closure's captured env,
            // NOT the caller's current env
            let call_env = env.envs.len();
            env.envs.push(Env {
                parent: Some(*closure_env),
                symbols: HashMap::new(),
            });

            // Bind parameters
            let old_current = env.current;
            env.current = call_env;
            for (param, arg) in params.iter().zip(arguments) {
                env.define(param.clone(), Some(arg));
            }

            // Execute body
            let result = eval_statements(body, env);

            // Restore caller's environment
            env.current = old_current;

            result
        }
        _ => Err(RuntimeError("Not callable".into())),
    }
}
```

The critical difference from the flat stack: when `makeCounter` returns, its environment (containing `i`) is still in the arena at its original index. The `count` closure holds that index and can still read/mutate `i` through it.

### Memory Considerations

Environments in the arena are never freed during execution — the `Vec` only grows. For a tree-walking interpreter this is fine; long-running programs with heavy closure allocation might accumulate garbage. Options:

1. **Do nothing** — for most Lox programs this is negligible.
2. **Mark-and-sweep** — periodically walk all live `EnvId` references (from the call stack and all `Value::Function` closures), then compact the arena. This is substantial work and rarely needed for an educational interpreter.
3. **Generational** — reuse indices from short-lived block scopes that have no closures pointing at them. Track this with a reference count per env slot (increment when a closure captures it, decrement when the closure is dropped).

For Crafting Interpreters chapters 8-13, option 1 is appropriate.

## Trade-offs vs Rc<RefCell>

| Aspect | Arena | Rc<RefCell> |
|--------|-------|-------------|
| Borrow checker | No issues — `&mut Environment` is the single owner | No issues — interior mutability bypasses it |
| Runtime panics | Impossible (no `borrow()`/`borrow_mut()`) | Possible if you accidentally nest borrows |
| Performance | One Vec lookup per scope level | One pointer chase + refcount per scope level |
| Closure support | Yes — captured `EnvId` stays valid | Yes — `Rc` keeps the env alive |
| Memory reclamation | Manual (or never) | Automatic via refcount |
| Code complexity | Moderate — explicit index management | Moderate — `Rc::clone` noise everywhere |
| Debugging | Easy — dump the whole `envs` Vec | Harder — chasing `Rc` pointers |

## Migration Path from Current Code

1. Replace `EvalState` with `Environment` (the arena struct above).
2. Change `eval_statements` and `eval_expression` to take `&mut Environment`.
3. Replace `Rc::clone(&eval_state)` calls with nothing — just pass `env` through.
4. Replace `eval_state.borrow().lookup(...)` with `env.lookup(...)`.
5. Replace `eval_state.borrow_mut().assign(...)` with `env.assign(...)`.
6. For blocks: `let prev = env.push_scope();` at entry, `env.pop_scope(prev);` at exit.
7. When implementing functions: store `closure_env: env.current` in the `Value::Function` variant at definition time.

  1. Replace EvalState with Environment — EvalState still exists in eval.rs:36-39. Your new Environment in env.rs is ready to replace it.
  2. Change signatures to &mut Environment — eval_statements currently takes Rc<RefCell<EvalState>> (line 98), eval_expression takes Rc<RefCell<EvalState>> (line 153). Both need to change to &mut Environment.
  3. Remove Rc::clone calls — there are ~16 of them in eval.rs. They all become just env passed through.
  4. Replace eval_state.borrow().lookup(...) — currently at line 215. Becomes env.lookup(...).
  5. Replace eval_state.borrow_mut().assign(...) — currently at line 222. Becomes env.assign(...).
  6. Blocks: push/pop — currently eval_statements creates a new EvalState::new_from_parent at line 99-101. Replace with let prev = env.push_scope(); at entry and env.pop_scope(prev); before returning.
  7. Closures — not yet implemented, so this is future work. Value::Callable (in parse.rs:10) would gain a closure_env: EnvId field when you get there.

  One additional thing to watch for: eval_statements currently always creates a child scope (line 99-101). You'll want to decide whether every call to eval_statements pushes a scope, or only Stmt::Block does — right
  now both the top-level call and blocks create scopes, which means blocks get double-nested. That's already the case in your current code, so it's not new, but worth being aware of as you migrate.
