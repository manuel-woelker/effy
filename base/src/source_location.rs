use crate::source_file::SourceFile;
use std::fmt::{Debug, Formatter};

pub struct SourceLocation<'source> {
    pub start: usize,
    pub end: usize,
    pub source_file: &'source SourceFile,
}

impl<'source> Debug for SourceLocation<'source> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}>{}+{}",
            self.source_file.path(),
            self.start,
            self.end - self.start
        )
    }
}

impl<'source> SourceLocation<'source> {
    pub fn new(source_file: &'source SourceFile, start: usize, end: usize) -> Self {
        Self {
            source_file,
            start,
            end,
        }
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }

    pub fn source_file(&self) -> &'source SourceFile {
        self.source_file
    }
}
