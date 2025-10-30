use crate::{FilePath, SourceString};

pub struct SourceFile {
    path: FilePath,
    content: SourceString,
}

impl SourceFile {
    pub fn new(path: impl Into<FilePath>, content: impl Into<SourceString>) -> Self {
        Self {
            path: path.into(),
            content: content.into(),
        }
    }

    pub fn path(&self) -> &FilePath {
        &self.path
    }

    pub fn content(&self) -> &SourceString {
        &self.content
    }
}
