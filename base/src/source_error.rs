use crate::error::EffyResult;
use crate::source_file::SourceFile;
use crate::source_location::SourceLocation;
use annotate_snippets::renderer::DecorStyle;
use annotate_snippets::{AnnotationKind, Group, Level, Renderer, Snippet};
use std::error::Error;
use std::fmt::{Debug, Display};

pub struct SourceError {
    groups: Vec<Group<'static>>,
}

impl SourceError {
    pub fn new(group: Group<'static>) -> Self {
        Self {
            groups: vec![group],
        }
    }

    pub fn add_group(&mut self, group: Group<'static>) {
        self.groups.push(group);
    }
}

impl Debug for SourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SourceError")
            .field("groups", &self.groups)
            .finish()
    }
}

impl Error for SourceError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl Display for SourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let renderer = Renderer::styled().decor_style(DecorStyle::Unicode);
        writeln!(f, "{}", renderer.render(&self.groups))?;
        Ok(())
    }
}

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
    let group = Level::ERROR.primary_title(primary_message).element(
        // TODO: extract only relevant line
        Snippet::source(source_file.content().to_string())
            .fold(false)
            .line_start(1)
            .path(source_file.path().to_string())
            .annotation(
                AnnotationKind::Primary
                    .span(source_location.start()..source_location.end())
                    .label(annotation_message),
            ),
    );
    SourceError::new(group)
}

#[cfg(test)]
mod tests {
    use crate::source_error::SourceError;
    use annotate_snippets::{AnnotationKind, Level, Snippet};
    use expect_test::expect;

    #[test]
    pub fn render_error() {
        let source = r#"
        one
        twy
        three
        four
        five"#;
        let group = Level::ERROR
            .primary_title("expected `two`, found `twy`")
            .element(
                Snippet::source(source.to_string())
                    .fold(false)
                    .line_start(26)
                    .path("examples/footer.rs")
                    .annotation(
                        AnnotationKind::Primary
                            .span(21..24)
                            .label("expected `two` here"),
                    )
                    .annotation(
                        AnnotationKind::Context
                            .span(33..38)
                            .label("because it is followed by `three`"),
                    ),
            );
        let error = SourceError::new(group);
        expect![[r#"
            error: expected `two`, found `twy`
               ╭▸ examples/footer.rs:28:9
               │
            26 │
            27 │         one
            28 │         twy
               │         ━━━ expected `two` here
            29 │         three
               │         ───── because it is followed by `three`
            30 │         four
            31 │         five
               ╰╴
        "#]]
        .assert_eq(&strip_ansi_escapes::strip_str(&error.to_string()));
    }
}
