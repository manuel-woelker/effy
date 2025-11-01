use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub enum Value {
    Unit,
    Int(i64),
    Float(f64),
    String(String),
    Boolean(bool),
}

impl Value {
    pub fn unit() -> Self {
        Self::Unit
    }

    pub fn string(string: impl Into<String>) -> Self {
        Self::String(string.into())
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Unit => write!(f, "unit"),
            Value::String(string) => write!(f, "\"{string}\""),
            Value::Int(int) => write!(f, "{}i64", int),
            Value::Float(float) => write!(f, "{}f64", float),
            Value::Boolean(boolean) => write!(f, "#{}", boolean),
        }
    }
}
