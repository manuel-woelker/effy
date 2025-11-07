use crate::coded_function::CodedFunction;
use crate::environment::Environment;
use crate::native_function::NativeFunction;
use effy_ast::function_definition::FunctionDefinition;
use effy_base::value::Value;
use std::fmt::Display;
use std::sync::Arc;

#[derive(Clone)]
pub enum ValueKind {
    PrimitiveValue(Value),
    NativeFunction(Arc<NativeFunction>),
    CodedFunction(Arc<CodedFunction>),
}

#[derive(Clone)]
pub struct InterpreterValue {
    value_kind: ValueKind,
}

impl InterpreterValue {
    pub fn unit() -> Self {
        Self::primitive(Value::Unit)
    }

    pub fn primitive(value: Value) -> Self {
        Self {
            value_kind: ValueKind::PrimitiveValue(value),
        }
    }

    pub fn int(value: i64) -> Self {
        Self::primitive(Value::Int(value))
    }

    pub fn bool(value: bool) -> Self {
        Self::primitive(Value::Boolean(value))
    }

    pub fn native_function(native_function: NativeFunction) -> Self {
        Self {
            value_kind: ValueKind::NativeFunction(Arc::new(native_function)),
        }
    }

    pub fn coded_function(
        function_definition: FunctionDefinition,
        environment: Environment,
    ) -> Self {
        Self {
            value_kind: ValueKind::CodedFunction(Arc::new(CodedFunction::new(
                function_definition,
                environment,
            ))),
        }
    }

    pub fn value_kind(&self) -> &ValueKind {
        &self.value_kind
    }
}

impl From<Value> for InterpreterValue {
    fn from(value: Value) -> Self {
        Self::primitive(value)
    }
}

impl Display for InterpreterValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.value_kind {
            ValueKind::PrimitiveValue(value) => {
                write!(f, "{}", value)
            }
            ValueKind::NativeFunction(native_function) => {
                write!(f, "<native function '{}'>", native_function.name())
            }
            ValueKind::CodedFunction(coded_function) => {
                write!(
                    f,
                    "<coded function '{}'>",
                    coded_function.function_definition().name().data.name
                )
            }
        }
    }
}
