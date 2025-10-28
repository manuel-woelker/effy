use crate::token::{Token, TokenKind};
use effy_base::source_file::SourceFile;
use effy_base::source_location::SourceLocation;
use std::str::Chars;

pub fn tokenize(source_file: &'_ SourceFile) -> impl Iterator<Item = Token<'_>> {
    let mut tokenizer = Tokenizer {
        source_file,
        start_position: 0,
        current_position: 0,
        chars: source_file.content().chars(),
        current_char: '\0',
        next_char: '\0',
        is_done: false,
    };
    // Initialize next_char
    tokenizer.advance();
    tokenizer.current_position = 0;
    tokenizer
}

pub struct Tokenizer<'src> {
    source_file: &'src SourceFile,
    start_position: usize,
    current_position: usize,
    chars: Chars<'src>,
    current_char: char,
    next_char: char,
    is_done: bool,
}

impl<'src> Tokenizer<'src> {
    fn advance(&mut self) {
        self.current_char = self.next_char;
        self.current_position += self.current_char.len_utf8();
        self.next_char = self.chars.next().unwrap_or('\0');
    }

    pub fn create_token(&mut self, token_kind: TokenKind) -> Token<'src> {
        let location =
            SourceLocation::new(self.source_file, self.start_position, self.current_position);
        self.start_position = self.current_position;
        Token::new(token_kind, location)
    }
    fn next_token(&mut self) -> Option<Token<'src>> {
        if self.is_done {
            return None;
        }
        if self.next_char == '\0' {
            self.is_done = true;
            return None;
        }
        loop {
            self.start_position = self.current_position;
            self.advance();
            if !self.current_char.is_whitespace() {
                break;
            }
        }

        Some(match self.current_char {
            '(' => self.create_token(TokenKind::ParenOpen),
            ')' => self.create_token(TokenKind::ParenClose),
            '{' => self.create_token(TokenKind::BraceOpen),
            '}' => self.create_token(TokenKind::BraceClose),
            '[' => self.create_token(TokenKind::BracketOpen),
            ']' => self.create_token(TokenKind::BracketClose),
            ',' => self.create_token(TokenKind::Comma),
            ';' => self.create_token(TokenKind::Semicolon),
            ':' => self.create_token(TokenKind::Colon),
            '.' => self.create_token(TokenKind::Dot),
            _other => {
                self.is_done = true;
                self.create_token(TokenKind::Unexpected)
            }
        })
    }
}

impl<'src> Iterator for Tokenizer<'src> {
    type Item = Token<'src>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

#[cfg(test)]
mod tests {
    use crate::tokenize::tokenize;
    use effy_base::FilePath;
    use effy_base::source_file::SourceFile;
    use expect_test::Expect;
    use std::fmt::Write;

    fn input_to_test_string(input: &str) -> String {
        let source_file = SourceFile::new(FilePath::from("test"), input.to_string());
        let mut tokenizer = tokenize(&source_file);
        let mut test_string = String::new();
        loop {
            let Some(token) = tokenizer.next() else {
                break;
            };
            writeln!(
                test_string,
                "🧩 {:3}+{:<2} {:14} {}",
                token.location().start(),
                token.location().end() - token.location().start(),
                token.kind(),
                token.lexeme(),
            )
            .unwrap();
        }
        test_string
    }

    #[allow(dead_code)]
    fn test_lexer(input: &str, expected: Expect) {
        let test_string = input_to_test_string(input);
        expected.assert_eq(&test_string);
    }

    fn test_lex_symbol(input: &str, expected: &str) {
        let test_string = input_to_test_string(input);
        assert_eq!(test_string, format!("🧩   0+1  {expected:14} {input}\n"));
    }

    macro_rules! test_lex_symbol {
        ($(($name:ident $input:literal $expected:literal))*) => {
            $(
            #[test]
            fn $name() {
                test_lex_symbol($input, $expected);
            }
            )*
        };
    }

    test_lex_symbol!(
        (paren_open "(" "Open Parenthesis")
        (paren_close ")" "Close Parenthesis")
        (brace_open "{" "Open Brace")
        (brace_close "}" "Close Brace")
        (bracket_open "[" "Open Bracket")
        (bracket_close "]" "Close Bracket")
        (semicolon ";" "Semicolon")
        (colon ":" "Colon")
        (dot "." "Dot")
        (comma "," "Comma")
    );
}
