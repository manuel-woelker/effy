use crate::source_file::SourceFile;
use std::fmt::{Debug, Formatter};

pub struct SourceLocation<'src> {
    start: usize,
    end: usize,
    source_file: &'src SourceFile,
}

impl<'src> Debug for SourceLocation<'src> {
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

impl<'src> SourceLocation<'src> {
    pub fn new(source_file: &'src SourceFile, start: usize, end: usize) -> Self {
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

    pub fn source_file(&self) -> &'src SourceFile {
        self.source_file
    }
}
