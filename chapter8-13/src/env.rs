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
        let mut env_id = Some(self.current);
        while let Some(id) = env_id {
            let env = &self.envs[id];
            if let Some(entry) = env.symbols.get(name) {
                return match entry {
                    Some(v) => Ok(v.clone()),
                    None => Err(RuntimeError(format!("Uninitialized variable: {}", name))),
                };
            }
            env_id = env.parent
        }
        Err(RuntimeError(format!("Unknown variable: {}", name)))
    }

}
