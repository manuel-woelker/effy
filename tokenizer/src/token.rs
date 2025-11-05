use effy_base::SourceString;
use effy_base::source_span::SourceSpan;
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum TokenKind {
    // Keywords
    Fun,
    True,
    False,

    // Identifier
    Identifier,

    // Symbols
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
    At,
    Slash,
    Star,
    Plus,
    Minus,
    Exclamation,
    Equals,
    EqualsEquals,
    NotEquals,
    LessThan,
    LessThanEquals,
    GreaterThan,
    GreaterThanEquals,

    // Literals
    String,
    Integer,

    // EOF
    EndOfFile,
}

impl TokenKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            // keywords
            TokenKind::Fun => "keyword fun",
            TokenKind::True => "keyword true",
            TokenKind::False => "keyword false",

            // Identifier
            TokenKind::Identifier => "Identifier",

            // Symbols
            TokenKind::ParenOpen => "'('",
            TokenKind::ParenClose => "')'",
            TokenKind::BraceOpen => "'{'",
            TokenKind::BraceClose => "'}'",
            TokenKind::BracketOpen => "'['",
            TokenKind::BracketClose => "']'",
            TokenKind::Comma => "','",
            TokenKind::Semicolon => "';'",
            TokenKind::Colon => "':'",
            TokenKind::Dot => "'.'",
            TokenKind::At => "'@'",
            TokenKind::Slash => "'/'",
            TokenKind::Star => "'*'",
            TokenKind::Plus => "'+'",
            TokenKind::Minus => "'-'",
            TokenKind::Exclamation => "'!'",
            TokenKind::Equals => "'='",
            TokenKind::EqualsEquals => "'=='",
            TokenKind::NotEquals => "'!='",
            TokenKind::LessThan => "'<'",
            TokenKind::LessThanEquals => "'<='",
            TokenKind::GreaterThan => "'>'",
            TokenKind::GreaterThanEquals => "'>='",

            // Literals
            TokenKind::String => "String",
            TokenKind::Integer => "Integer",

            // EOF
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
