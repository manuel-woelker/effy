use crate::environment::Environment;

pub struct EnvironmentStack {
    environments: Vec<Environment>,
}

impl EnvironmentStack {
    pub fn new() -> Self {
        Self {
            environments: vec![Environment::new()],
        }
    }

    pub fn top(&mut self) -> &mut Environment {
        self.environments.last_mut().expect("empty environment")
    }

    pub fn push(&mut self, environment: Environment) {
        self.environments.push(environment);
    }

    pub fn pop(&mut self) {
        self.environments.pop();
        assert!(!self.environments.is_empty());
    }
}
