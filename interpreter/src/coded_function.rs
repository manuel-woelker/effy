use crate::environment::Environment;
use effy_ast::function_definition::FunctionDefinition;

pub struct CodedFunction {
    function_definition: FunctionDefinition,
    environment: Environment,
}

impl CodedFunction {
    pub fn new(function_definition: FunctionDefinition, environment: Environment) -> Self {
        Self {
            function_definition,
            environment,
        }
    }

    pub fn function_definition(&self) -> &FunctionDefinition {
        &self.function_definition
    }

    pub fn environment(&self) -> &Environment {
        &self.environment
    }
}
