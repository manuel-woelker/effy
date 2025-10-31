use crate::source_file::SourceFile;
use crate::source_span::SourceSpan;
use std::fmt::{Debug, Formatter};

pub struct SourceLocation<'source> {
    pub span: SourceSpan,
    pub source_file: &'source SourceFile,
}

impl<'source> Debug for SourceLocation<'source> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}>{}+{}",
            self.source_file.path(),
            self.span.range.start,
            self.span.len(),
        )
    }
}

impl<'source> SourceLocation<'source> {
    pub fn new(source_file: &'source SourceFile, source_span: impl Into<SourceSpan>) -> Self {
        Self {
            source_file,
            span: source_span.into(),
        }
    }

    pub fn start(&self) -> usize {
        self.span.start()
    }

    pub fn end(&self) -> usize {
        self.span.end()
    }

    pub fn source_file(&self) -> &'source SourceFile {
        self.source_file
    }
}
