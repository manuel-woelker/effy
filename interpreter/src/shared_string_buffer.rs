use effy_base::RwLock;
use std::fmt::{Arguments, Display, Write};
use std::sync::Arc;

#[derive(Clone)]
pub struct SharedStringBuffer {
    buffer: Arc<RwLock<String>>,
}

impl SharedStringBuffer {
    pub fn new() -> Self {
        Self {
            buffer: Arc::new(RwLock::new(String::new())),
        }
    }

    pub fn push_str(&self, s: impl AsRef<str>) {
        self.buffer.write().push_str(s.as_ref());
    }

    pub fn write_fmt(&self, args: Arguments) {
        write!(self.buffer.write(), "{}", args).unwrap();
    }

    pub fn into_string(self) -> String {
        Arc::into_inner(self.buffer)
            .expect("Arc should be unique, but there are still more references to it")
            .into_inner()
    }
}

impl Write for SharedStringBuffer {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.push_str(s);
        Ok(())
    }
}

impl Display for SharedStringBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.buffer.read())
    }
}

impl<T> From<T> for SharedStringBuffer
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        Self {
            buffer: Arc::new(RwLock::new(value.into())),
        }
    }
}
