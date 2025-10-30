use effy_base::source_location::SourceLocation;
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
    Unexpected,
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
            TokenKind::Unexpected => "Unexpected",
            TokenKind::EndOfFile => "End of File",
        }
    }
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad(self.as_str())
    }
}

pub struct Token<'source> {
    kind: TokenKind,
    location: SourceLocation<'source>,
}

impl<'source> Token<'source> {
    pub fn new(kind: TokenKind, location: SourceLocation<'source>) -> Self {
        Self { kind, location }
    }

    pub fn kind(&self) -> TokenKind {
        self.kind
    }

    pub fn location(&self) -> &SourceLocation<'source> {
        &self.location
    }

    pub fn lexeme(&self) -> &'source str {
        &self.location.source_file().content()[self.location.start()..self.location.end()]
    }
}
