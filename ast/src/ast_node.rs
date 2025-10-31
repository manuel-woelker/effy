use effy_base::error::EffyResult;
use effy_base::indent;
use effy_base::source_span::SourceSpan;
use effy_base::test_print::TestPrint;
use std::fmt::Write;
use std::ops::Deref;

pub struct AstNode<T: TestPrint> {
    pub data: T,
    pub span: SourceSpan,
}

impl<T: TestPrint> AstNode<T> {
    pub fn new(data: T, span: impl Into<SourceSpan>) -> Self {
        Self {
            data,
            span: span.into(),
        }
    }
}

impl<T: TestPrint> TestPrint for AstNode<T> {
    fn test_print(&self, write: &mut dyn Write, indent: usize) -> EffyResult<()> {
        write!(write, "🌲 {:3}+{:<3}", self.span.start(), self.span.len())?;
        indent::indent(write, indent)?;
        self.data.test_print(write, indent)
    }
}

impl<T: TestPrint> Deref for AstNode<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
