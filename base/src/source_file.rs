use crate::{FilePath, SourceString};

pub struct SourceFile {
    path: FilePath,
    content: SourceString,
}

impl SourceFile {
    pub fn new(path: FilePath, content: SourceString) -> Self {
        Self { path, content }
    }

    pub fn path(&self) -> &FilePath {
        &self.path
    }

    pub fn content(&self) -> &SourceString {
        &self.content
    }
}
