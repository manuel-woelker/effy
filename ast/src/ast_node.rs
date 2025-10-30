use effy_base::error::EffyResult;
use effy_base::indent;
use effy_base::source_location::SourceLocation;
use effy_base::test_print::TestPrint;
use std::fmt::Write;
use std::ops::Deref;

pub struct AstNode<'source, T: TestPrint> {
    pub data: T,
    pub source_location: SourceLocation<'source>,
}

impl<'source, T: TestPrint> AstNode<'source, T> {
    pub fn new(data: T, source_location: SourceLocation<'source>) -> Self {
        Self {
            data,
            source_location,
        }
    }
}

impl<T: TestPrint> TestPrint for AstNode<'_, T> {
    fn test_print(&self, write: &mut dyn Write, indent: usize) -> EffyResult<()> {
        write!(
            write,
            "🌲 {:3}+{:<3}",
            self.source_location.start,
            self.source_location.end - self.source_location.start
        )?;
        indent::indent(write, indent)?;
        self.data.test_print(write, indent)
    }
}

impl<T: TestPrint> Deref for AstNode<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
