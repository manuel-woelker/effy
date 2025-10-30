use std::fmt::{Display, Formatter};

pub enum Value {
    Int(i64),
    Float(f64),
    String(String),
    Boolean(bool),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::String(string) => write!(f, "\"{string}\""),
            Value::Int(int) => write!(f, "{}i64", int),
            Value::Float(float) => write!(f, "{}f64", float),
            Value::Boolean(boolean) => write!(f, "#{}", boolean),
        }
    }
}
