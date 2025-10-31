use std::ops::Range;

#[derive(Debug, Clone)]
pub struct SourceSpan {
    pub range: Range<usize>,
}

impl SourceSpan {}

impl SourceSpan {
    pub fn new(range: impl Into<Range<usize>>) -> Self {
        Self {
            range: range.into(),
        }
    }

    pub fn start(&self) -> usize {
        self.range.start
    }

    pub fn end(&self) -> usize {
        self.range.end
    }

    pub fn len(&self) -> usize {
        self.range.len()
    }

    pub fn is_empty(&self) -> bool {
        self.range.is_empty()
    }
}

impl From<Range<usize>> for SourceSpan {
    fn from(range: Range<usize>) -> Self {
        Self { range }
    }
}
