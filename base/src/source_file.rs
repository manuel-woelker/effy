use crate::{FilePath, SourceString};
use std::sync::Arc;

#[derive(Clone)]
pub struct SourceFile {
    inner: Arc<SourceFileInner>,
}

struct SourceFileInner {
    path: FilePath,
    content: SourceString,
}

impl SourceFile {
    pub fn new(path: impl Into<FilePath>, content: impl Into<SourceString>) -> Self {
        Self {
            inner: Arc::new(SourceFileInner {
                path: path.into(),
                content: content.into(),
            }),
        }
    }

    pub fn path(&self) -> &FilePath {
        &self.inner.path
    }

    pub fn content(&self) -> &SourceString {
        &self.inner.content
    }
}
