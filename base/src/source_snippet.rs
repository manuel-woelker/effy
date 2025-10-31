use crate::{FilePath, SourceString};

#[derive(Debug)]
pub struct SourceSnippet {
    file_path: FilePath,
    source_snippet: SourceString,
    start_line: usize,
    start_offset: usize,
}

impl SourceSnippet {
    pub fn new(
        file_path: impl Into<FilePath>,
        source_snippet: impl Into<SourceString>,
        start_line: usize,
        start_offset: usize,
    ) -> Self {
        Self {
            file_path: file_path.into(),
            source_snippet: source_snippet.into(),
            start_line,
            start_offset,
        }
    }

    pub fn file_path(&self) -> &str {
        self.file_path.as_str()
    }

    pub fn source_snippet(&self) -> &str {
        self.source_snippet.as_str()
    }

    pub fn start_line(&self) -> usize {
        self.start_line
    }

    pub fn start_offset(&self) -> usize {
        self.start_offset
    }
}
