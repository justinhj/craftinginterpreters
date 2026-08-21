use std::collections::HashMap;

use crate::parse::Value;
use crate::eval::EvalResult;
use crate::eval::RuntimeError;

type EnvId = usize;

// Represents part of an Environnment, which is a tree of Env
// Each env contains an optional parent and a mutable hashmap of symbol/value pairs
// Note it is an internal structure not part of the public API
#[derive(Debug)]
struct Env {
    parent: Option<EnvId>, 
    symbols: HashMap<String, Option<Value>>,
}


// Public Environment is an "arena" of environments. 
// We are using indexes here so that ownership is not a concern. Indexes are 'Copy' 
// and the arena owns everything
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

    pub fn push_scope(&mut self) -> EnvId {
        let new_id = self.envs.len();
        self.envs.push(Env {
            parent: Some(self.current),
            symbols: HashMap::new(),
        });
        let old = self.current;
        self.current = new_id;
        old
    }

    pub fn pop_scope(&mut self, previous: EnvId) {
        self.current = previous;
    }

    pub fn define(&mut self, name: String, value: Option<Value>) {
        self.envs[self.current].symbols.insert(name, value);
    }

    pub fn lookup(&self, name: &str) -> EvalResult {
        self.lookup_in(name, self.current)
    }
    
    /// looup starting from a specific environment (used by closures)
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
        self.assign_in(name, value, self.current)
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
