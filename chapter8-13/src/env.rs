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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn define_and_lookup() {
        let mut env = Environment::new();
        env.define("x".to_string(), Some(Value::Number(42.0)));
        assert_eq!(env.lookup("x").unwrap(), Value::Number(42.0));
    }

    #[test]
    fn lookup_unknown_variable_errors() {
        let env = Environment::new();
        assert!(env.lookup("x").is_err());
    }

    #[test]
    fn lookup_uninitialized_variable_errors() {
        let mut env = Environment::new();
        env.define("x".to_string(), None);
        assert!(env.lookup("x").is_err());
    }

    #[test]
    fn child_scope_shadows_parent() {
        let mut env = Environment::new();
        env.define("x".to_string(), Some(Value::Number(1.0)));
        let prev = env.push_scope();
        env.define("x".to_string(), Some(Value::Number(2.0)));
        assert_eq!(env.lookup("x").unwrap(), Value::Number(2.0));
        env.pop_scope(prev);
        assert_eq!(env.lookup("x").unwrap(), Value::Number(1.0));
    }

    #[test]
    fn child_scope_sees_parent_variables() {
        let mut env = Environment::new();
        env.define("x".to_string(), Some(Value::Number(10.0)));
        let _prev = env.push_scope();
        assert_eq!(env.lookup("x").unwrap(), Value::Number(10.0));
    }

    #[test]
    fn assign_in_current_scope() {
        let mut env = Environment::new();
        env.define("x".to_string(), Some(Value::Number(1.0)));
        env.assign("x", Value::Number(99.0)).unwrap();
        assert_eq!(env.lookup("x").unwrap(), Value::Number(99.0));
    }

    #[test]
    fn assign_in_parent_scope() {
        let mut env = Environment::new();
        env.define("x".to_string(), Some(Value::Number(1.0)));
        let _prev = env.push_scope();
        env.assign("x", Value::Number(5.0)).unwrap();
        assert_eq!(env.lookup("x").unwrap(), Value::Number(5.0));
    }

    #[test]
    fn assign_unknown_variable_errors() {
        let mut env = Environment::new();
        assert!(env.assign("x", Value::Number(1.0)).is_err());
    }

    #[test]
    fn nested_scopes() {
        let mut env = Environment::new();
        env.define("a".to_string(), Some(Value::Number(1.0)));
        let prev1 = env.push_scope();
        env.define("b".to_string(), Some(Value::Number(2.0)));
        let prev2 = env.push_scope();
        env.define("c".to_string(), Some(Value::Number(3.0)));

        assert_eq!(env.lookup("a").unwrap(), Value::Number(1.0));
        assert_eq!(env.lookup("b").unwrap(), Value::Number(2.0));
        assert_eq!(env.lookup("c").unwrap(), Value::Number(3.0));

        env.pop_scope(prev2);
        assert_eq!(env.lookup("a").unwrap(), Value::Number(1.0));
        assert_eq!(env.lookup("b").unwrap(), Value::Number(2.0));
        assert!(env.lookup("c").is_err());

        env.pop_scope(prev1);
        assert_eq!(env.lookup("a").unwrap(), Value::Number(1.0));
        assert!(env.lookup("b").is_err());
    }
}
