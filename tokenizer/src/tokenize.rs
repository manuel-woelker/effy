use crate::token::{Token, TokenKind};
use effy_base::error::EffyResult;
use effy_base::source_error::make_source_error_result;
use effy_base::source_file::SourceFile;
use effy_base::source_span::SourceSpan;
use std::str::Chars;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum TokenizerState {
    Reading,
    LastChar,
    EndOfFile,
    Done,
}

const EOF: char = '␄';

pub fn tokenize(source_file: &'_ SourceFile) -> impl Iterator<Item = EffyResult<Token>> {
    let mut tokenizer = Tokenizer {
        source_file,
        start_position: 0,
        current_position: 0,
        chars: source_file.content().chars(),
        current_char: '\0',
        next_char: '\0',
        state: TokenizerState::Reading,
    };
    // Initialize next_char and current_char
    tokenizer.advance();
    tokenizer.advance();
    tokenizer.current_position = 0;
    tokenizer
}

pub struct Tokenizer<'source> {
    source_file: &'source SourceFile,
    start_position: usize,
    current_position: usize,
    chars: Chars<'source>,
    current_char: char,
    next_char: char,
    state: TokenizerState,
}

impl<'source> Tokenizer<'source> {
    fn advance(&mut self) {
        match self.state {
            TokenizerState::EndOfFile | TokenizerState::Done => {
                // Nothing left to do
                return;
            }
            TokenizerState::LastChar => {
                self.current_position += self.current_char.len_utf8();
                self.state = TokenizerState::EndOfFile;
                self.current_char = EOF;
                return;
            }
            TokenizerState::Reading => {
                // continue
            }
        }
        self.current_position += self.current_char.len_utf8();
        self.current_char = self.next_char;
        match self.chars.next() {
            None => {
                self.next_char = EOF;
                self.state = TokenizerState::LastChar;
            }
            Some(next_char) => {
                self.next_char = next_char;
            }
        }
    }

    pub fn create_token(&mut self, token_kind: TokenKind) -> EffyResult<Token> {
        self.advance();
        let location = self.create_span();
        self.start_position = self.current_position;
        Ok(Token::new(token_kind, location))
    }

    fn create_span(&mut self) -> SourceSpan {
        SourceSpan::new(self.start_position..self.current_position)
    }

    fn next_token(&mut self) -> Option<EffyResult<Token>> {
        loop {
            if !self.current_char.is_whitespace() {
                break;
            }
            self.advance();
        }
        self.start_position = self.current_position;
        match self.state {
            TokenizerState::Reading | TokenizerState::LastChar => {
                // continue
            }
            TokenizerState::EndOfFile => {
                self.state = TokenizerState::Done;
                return Some(self.create_token(TokenKind::EndOfFile));
            }
            TokenizerState::Done => {
                return None;
            }
        }
        Some(match self.current_char {
            // Symbols
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
            // Strings
            '"' => loop {
                self.advance();
                match self.current_char {
                    EOF => {
                        return Some(make_source_error_result(
                            self.source_file,
                            "Unterminated string",
                            "This string requires a terminating \" character here",
                            self.current_position..self.current_position + 1,
                        ));
                    }
                    '"' => {
                        return Some(self.create_token(TokenKind::String));
                    }
                    _ => {}
                }
            },
            // Integers
            '0'..='9' => {
                loop {
                    self.advance();
                    match self.next_char {
                        '0'..='9' => {}
                        _ => break,
                    }
                }
                return Some(self.create_token(TokenKind::Integer));
            }

            // Identifiers and keywords
            'a'..='z' | 'A'..='Z' | '_' => {
                loop {
                    self.advance();
                    if !(self.next_char.is_alphanumeric() || self.next_char == '_') {
                        break;
                    }
                }
                let identifier =
                    &self.source_file.content()[self.start_position..self.current_position];
                let token_kind = match identifier {
                    "fun" => TokenKind::Fun,
                    _ => TokenKind::Identifier,
                };
                self.create_token(token_kind)
            }
            unexpected => {
                self.state = TokenizerState::EndOfFile;
                return Some(make_source_error_result(
                    self.source_file,
                    format!("Unexpected character '{unexpected}'"),
                    "This character is not expected here",
                    self.current_position..self.current_position + unexpected.len_utf8(),
                ));
            }
        })
    }
}

impl<'source> Iterator for Tokenizer<'source> {
    type Item = EffyResult<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

#[cfg(test)]
mod tests {
    use crate::tokenize::tokenize;
    use effy_base::source_file::SourceFile;
    use effy_base::{FilePath, unansi};
    use expect_test::{Expect, expect};
    use std::fmt::Write;

    fn input_to_test_string(input: &str) -> String {
        let source_file = SourceFile::new(FilePath::from("test.effy"), input.to_string());
        let tokenizer = tokenize(&source_file);
        let mut test_string = String::new();

        for token in tokenizer {
            let token = match token {
                Ok(token) => token,
                Err(err) => {
                    writeln!(test_string, "⚠ ERROR:\n{}", unansi(&err.to_string())).unwrap();
                    return test_string;
                }
            };

            writeln!(
                test_string,
                "🧩 {:3}+{:<2} {:14} {}",
                token.span().start(),
                token.span().end() - token.span().start(),
                token.kind(),
                token.lexeme(source_file.content()),
            )
            .unwrap();
        }

        test_string
    }

    fn test_lexer(input: &str, expected: Expect) {
        let test_string = input_to_test_string(input);
        expected.assert_eq(&test_string);
    }

    fn test_lex_symbol(input: &str, expected: &str) {
        let test_string = input_to_test_string(input);
        assert_eq!(
            test_string,
            format!("🧩   0+1  {expected:14} {input}\n🧩   1+0  End of File    \n")
        );
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

    macro_rules! test_lex {
        ($name:ident, $input:literal, $expected:expr) => {
            #[test]
            fn $name() {
                test_lexer($input, $expected);
            }
        };
    }

    test_lex!(
        empty,
        "",
        expect!([r#"
            🧩   0+0  End of File    
        "#])
    );

    test_lex!(
        parens,
        "()",
        expect!([r#"
            🧩   0+1  Open Parenthesis (
            🧩   1+1  Close Parenthesis )
            🧩   2+0  End of File    
        "#])
    );

    test_lex!(
        integer_0,
        "0",
        expect!([r#"
            🧩   0+1  Integer        0
            🧩   1+0  End of File    
        "#])
    );

    test_lex!(
        integer_19,
        "19;",
        expect!([r#"
            🧩   0+2  Integer        19
            🧩   2+1  Semicolon      ;
            🧩   3+0  End of File    
        "#])
    );

    test_lex!(
        integer_12345667890,
        "1234567890",
        expect!([r#"
            🧩   0+10 Integer        1234567890
            🧩  10+0  End of File    
        "#])
    );

    test_lex!(
        string_empty,
        "\"\"",
        expect!([r#"
            🧩   0+2  String         ""
            🧩   2+0  End of File    
        "#])
    );

    test_lex!(
        string_one_char,
        "\"x\"",
        expect!([r#"
            🧩   0+3  String         "x"
            🧩   3+0  End of File    
        "#])
    );

    test_lex!(
        string_multiple_chars,
        "\"hello\"",
        expect!([r#"
            🧩   0+7  String         "hello"
            🧩   7+0  End of File    
        "#])
    );

    test_lex!(
        string_astronaut,
        "\"👨‍🚀\"",
        expect!([r#"
            🧩   0+13 String         "👨‍🚀"
            🧩  13+0  End of File    
        "#])
    );

    test_lex!(
        fun,
        "fun ",
        expect!([r#"
            🧩   0+3  Identifier     fun
            🧩   4+0  End of File    
        "#])
    );

    test_lex!(
        identifier,
        "foobar",
        expect!([r#"
            🧩   0+6  Identifier     foobar
            🧩   6+0  End of File    
        "#])
    );

    test_lex!(
        function,
        "fun foo() {}",
        expect!([r#"
            🧩   0+3  Identifier     fun
            🧩   4+3  Identifier     foo
            🧩   7+1  Open Parenthesis (
            🧩   8+1  Close Parenthesis )
            🧩  10+1  Open Brace     {
            🧩  11+1  Close Brace    }
            🧩  12+0  End of File    
        "#])
    );
    test_lex!(
        function_call,
        "print(\"hello\");",
        expect!([r#"
            🧩   0+5  Identifier     print
            🧩   5+1  Open Parenthesis (
            🧩   6+7  String         "hello"
            🧩  13+1  Close Parenthesis )
            🧩  14+1  Semicolon      ;
            🧩  15+0  End of File    
        "#])
    );

    test_lex!(
        unterminated_string,
        "\"foo",
        expect!([r#"
            ⚠ ERROR:
            error: Unterminated string
              ╭▸ test.effy:1:5
              │
            1 │ "foo
              ╰╴    ━ This string requires a terminating " character here
        "#])
    );

    test_lex!(
        unexpected,
        "👨‍🚀",
        expect!([r#"
            ⚠ ERROR:
            error: Unexpected character '👨'
              ╭▸ test.effy:1:1
              │
            1 │ 👨🚀
              ╰╴━━ This character is not expected here
        "#])
    );
}
