use crate::source_file::SourceFile;
use crate::source_message::SourceLabel;
use crate::source_span::SourceSpan;
use crate::{FilePath, SourceString};

#[derive(Debug)]
pub struct SourceSnippet {
    file_path: FilePath,
    source_excerpt: SourceString,
    start_line: usize,
    start_offset_in_bytes: usize,
    labels: Vec<SourceLabel>,
}

impl SourceSnippet {
    pub fn new(
        file_path: impl Into<FilePath>,
        source_excerpt: impl Into<SourceString>,
        start_line: usize,
        start_offset_in_bytes: usize,
    ) -> Self {
        Self {
            file_path: file_path.into(),
            source_excerpt: source_excerpt.into(),
            start_line,
            start_offset_in_bytes,
            labels: Vec::new(),
        }
    }

    pub fn file_path(&self) -> &str {
        self.file_path.as_str()
    }

    pub fn source_excerpt(&self) -> &str {
        self.source_excerpt.as_str()
    }

    pub fn start_line(&self) -> usize {
        self.start_line
    }

    pub fn labels(&self) -> &Vec<SourceLabel> {
        &self.labels
    }

    pub fn set_labels(&mut self, labels: Vec<SourceLabel>) {
        self.labels = labels;
    }

    pub fn start_offset_in_bytes(&self) -> usize {
        self.start_offset_in_bytes
    }

    pub fn add_label(&mut self, label: SourceLabel) {
        self.labels.push(label);
    }
}

pub fn create_source_snippet_from_span(
    span: &mut SourceSpan,
    source_file: &SourceFile,
) -> SourceSnippet {
    let source = source_file.content();
    let prefix = &source[..span.range.start];
    let number_of_lines = prefix.matches('\n').count();
    let line_start = prefix.rfind('\n').map(|idx| idx + 1).unwrap_or(0);
    let line_end = source[span.range.end..]
        .find('\n')
        .map(|idx| idx + span.range.end)
        .unwrap_or(source.len());
    span.range.start -= line_start;
    span.range.end -= line_start;
    let line = &source[line_start..line_end];
    SourceSnippet::new(
        source_file.path().to_string(),
        line.to_string(),
        number_of_lines + 1,
        line_start,
    )
}
