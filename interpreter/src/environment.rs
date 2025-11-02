use crate::value::InterpreterValue;
use std::collections::HashMap;

pub struct Environment {
    pub bindings: HashMap<String, InterpreterValue>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            bindings: HashMap::new(),
        }
    }

    pub fn add(&mut self, name: impl Into<String>, value: impl Into<InterpreterValue>) {
        self.bindings.insert(name.into(), value.into());
    }

    pub fn get(&self, name: impl AsRef<str>) -> Option<InterpreterValue> {
        self.bindings.get(name.as_ref()).cloned()
    }
}
