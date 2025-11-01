use effy_base::SourceString;
use effy_base::source_span::SourceSpan;
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum TokenKind {
    Fun,
    Identifier,
    ParenOpen,
    ParenClose,
    BraceOpen,
    BraceClose,
    BracketOpen,
    BracketClose,
    Comma,
    Semicolon,
    Colon,
    Dot,
    String,
    Integer,
    EndOfFile,
}

impl TokenKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            TokenKind::Fun => "keyword fun",
            TokenKind::Identifier => "Identifier",
            TokenKind::ParenOpen => "Open Parenthesis",
            TokenKind::ParenClose => "Close Parenthesis",
            TokenKind::BraceOpen => "Open Brace",
            TokenKind::BraceClose => "Close Brace",
            TokenKind::BracketOpen => "Open Bracket",
            TokenKind::BracketClose => "Close Bracket",
            TokenKind::Comma => "Comma",
            TokenKind::Semicolon => "Semicolon",
            TokenKind::Colon => "Colon",
            TokenKind::Dot => "Dot",
            TokenKind::String => "String",
            TokenKind::Integer => "Integer",
            TokenKind::EndOfFile => "End of File",
        }
    }
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad(self.as_str())
    }
}

pub struct Token {
    kind: TokenKind,
    span: SourceSpan,
}

impl Token {
    pub fn new(kind: TokenKind, span: impl Into<SourceSpan>) -> Self {
        Self {
            kind,
            span: span.into(),
        }
    }

    pub fn kind(&self) -> TokenKind {
        self.kind
    }

    pub fn span(&self) -> &SourceSpan {
        &self.span
    }

    pub fn lexeme<'source>(&self, source: &'source SourceString) -> &'source str {
        &source[self.span.range.clone()]
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}
