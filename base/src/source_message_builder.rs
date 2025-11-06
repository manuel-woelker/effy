use crate::error::{EffyError, EffyResult};
use crate::source_error::SourceError;
use crate::source_file::SourceFile;
use crate::source_message::{SourceLabel, SourceMessage, SourceMessageLevel};
use crate::source_snippet::SourceSnippet;
use crate::source_span::SourceSpan;

pub struct SourceMessageBuilder<'source> {
    source_file: &'source SourceFile,
    level: SourceMessageLevel,
    message: String,
    labels: Vec<SourceLabel>,
}

impl<'source> SourceMessageBuilder<'source> {
    pub fn new(
        source_file: &'source SourceFile,
        level: SourceMessageLevel,
        message: impl Into<String>,
    ) -> Self {
        Self {
            source_file,
            level,
            message: message.into(),
            labels: Vec::new(),
        }
    }

    pub fn label(mut self, span: impl Into<SourceSpan>, label: impl Into<String>) -> Self {
        self.labels
            .push(SourceLabel::new(span.into(), label.into()));
        self
    }

    #[track_caller]
    pub fn build(self) -> SourceMessage {
        SourceMessage::new(
            self.level,
            self.message.clone(),
            create_source_snippet_from_labels(self.source_file, self.labels),
        )
    }

    #[track_caller]
    pub fn build_error_result<T>(self) -> EffyResult<T> {
        Err(SourceError::new(self.build()).into())
    }

    #[track_caller]
    pub fn build_error(self) -> EffyError {
        SourceError::new(self.build()).into()
    }
}

fn create_source_snippet_from_labels(
    source_file: &SourceFile,
    labels: Vec<SourceLabel>,
) -> SourceSnippet {
    if labels.is_empty() {
        panic!("No labels in source message for '{}'", source_file.path());
    }
    let mut range = labels[0].span().range.clone();
    for label in &labels {
        range.start = range.start.min(label.span().range.start);
        range.end = range.end.max(label.span().range.end);
    }
    let source = source_file.content();
    let prefix = &source[..range.start];
    let number_of_lines = prefix.matches('\n').count();
    let line_start = prefix.rfind('\n').map(|idx| idx + 1).unwrap_or(0);
    let line_end = source[range.end..]
        .find('\n')
        .map(|idx| idx + range.end)
        .unwrap_or(source.len());
    let line = &source[line_start..line_end];
    let mut snippet = SourceSnippet::new(
        source_file.path().to_string(),
        line.to_string(),
        number_of_lines + 1,
        line_start,
    );
    snippet.set_labels(labels);
    snippet
}
