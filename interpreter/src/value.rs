use crate::native_function::NativeFunction;
use effy_base::value::Value;
use std::fmt::Display;
use std::sync::Arc;

#[derive(Clone)]
pub enum ValueKind {
    PrimitiveValue(Value),
    NativeFunction(Arc<NativeFunction>),
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

    pub fn native_function(native_function: NativeFunction) -> Self {
        Self {
            value_kind: ValueKind::NativeFunction(Arc::new(native_function)),
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
        }
    }
}
