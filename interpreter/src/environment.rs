use crate::value::InterpreterValue;
use effy_base::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone)]
pub struct Environment {
    inner: Arc<EnvironmentInner>,
}

pub struct EnvironmentInner {
    parent: Option<Environment>,
    bindings: RwLock<HashMap<String, InterpreterValue>>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            inner: Arc::new(EnvironmentInner {
                parent: None,
                bindings: RwLock::new(HashMap::new()),
            }),
        }
    }

    pub fn new_child(&self) -> Environment {
        Environment {
            inner: Arc::new(EnvironmentInner {
                parent: Some(self.clone()),
                bindings: RwLock::new(HashMap::new()),
            }),
        }
    }

    pub fn add(&mut self, name: impl Into<String>, value: impl Into<InterpreterValue>) {
        self.inner
            .bindings
            .write()
            .insert(name.into(), value.into());
    }

    pub fn get(&self, name: impl AsRef<str>) -> Option<InterpreterValue> {
        if let Some(value) = self.inner.bindings.read().get(name.as_ref()) {
            return Some(value.clone());
        }
        if let Some(parent) = &self.inner.parent {
            return parent.get(name);
        }
        None
    }
}
