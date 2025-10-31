use crate::error::EffyResult;
use crate::source_file::SourceFile;
use crate::source_location::SourceLocation;
use crate::source_message::{SourceLabel, SourceMessage};
use crate::source_snippet::SourceSnippet;
use crate::source_span::SourceSpan;

#[derive(Debug)]
pub struct SourceError {
    pub source_message: SourceMessage,
}

impl SourceError {
    pub fn new(source_message: SourceMessage) -> Self {
        Self { source_message }
    }
}

impl std::fmt::Display for SourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.source_message.render())
    }
}

impl std::error::Error for SourceError {}

pub fn make_source_error_result<T>(
    source_file: &SourceFile,
    primary_message: impl Into<String>,
    annotation_message: impl Into<String>,
    source_location: SourceLocation,
) -> EffyResult<T> {
    Err(make_source_error(
        source_file,
        primary_message.into(),
        annotation_message.into(),
        source_location,
    )
    .into())
}

pub fn make_source_error(
    source_file: &SourceFile,
    primary_message: String,
    annotation_message: String,
    source_location: SourceLocation,
) -> SourceError {
    // TODO: extract only relevant line
    let source_snippet = SourceSnippet::new(
        source_file.path().clone(),
        source_file.content().clone(),
        1,
        0,
    );
    let mut source_message = SourceMessage::error(primary_message, source_snippet);
    source_message.add_label(SourceLabel::new(
        SourceSpan::new(source_location.start, source_location.end),
        annotation_message,
    ));
    SourceError::new(source_message)
}
