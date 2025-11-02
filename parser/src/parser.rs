use effy_ast::ast_node::AstNode;
use effy_ast::expression::{Expression, ExpressionNode};
use effy_ast::function_definition::FunctionDefinition;
use effy_ast::identifier::{Identifier, IdentifierNode};
use effy_ast::script::{Script, ScriptNode};
use effy_ast::statement::{
    ExpressionStatement, FunctionDefinitionStatement, Statement, StatementNode,
};
use effy_base::error::{EffyError, EffyResult, bail, err};
use effy_base::source_error::SourceError;
use effy_base::source_file::SourceFile;
use effy_base::source_message::{SourceLabel, SourceMessage};
use effy_base::source_snippet::SourceSnippet;
use effy_base::test_print::TestPrint;
use effy_base::value::Value;
use effy_tokenizer::token::{Token, TokenKind};
use effy_tokenizer::tokenize::tokenize;

pub fn parse_script(script_source: &SourceFile) -> EffyResult<ScriptNode> {
    let mut tokens = tokenize(script_source);
    let mut parser = Parser::new(script_source, &mut tokens)?;
    parser.parse_script()
}

#[allow(dead_code)]
struct Parser<'source, 'tokens> {
    source: &'source SourceFile,
    current_token: Token,
    last_position: usize,
    tokens: &'tokens mut dyn Iterator<Item = EffyResult<Token>>,
}

impl<'source, 'tokens> Parser<'source, 'tokens> {
    fn new(
        source: &'source SourceFile,
        tokens: &'tokens mut dyn Iterator<Item = EffyResult<Token>>,
    ) -> EffyResult<Self> {
        let current_token = tokens
            .next()
            .ok_or_else(|| err!("Expected at least EOF"))??;
        Ok(Self {
            source,
            current_token,
            tokens,
            last_position: 0,
        })
    }

    fn parse_script(&mut self) -> EffyResult<ScriptNode> {
        let mut statements = Vec::new();
        let start_position = self.last_position;
        while self.current_token.kind() != TokenKind::EndOfFile {
            statements.push(self.parse_statement()?);
        }
        self.create_node(start_position, Script::new(statements))
    }

    fn parse_statement(&mut self) -> EffyResult<StatementNode> {
        if let TokenKind::Fun | TokenKind::At = self.current_token.kind() {
            return self.parse_function_definition_statement();
        }
        let result = self.parse_expression_statement()?;
        self.consume(TokenKind::Semicolon)?;
        Ok(result)
    }

    fn parse_function_definition_statement(&mut self) -> EffyResult<StatementNode> {
        let start_position = self.current_position();
        let mut annotations: Vec<IdentifierNode> = vec![];
        while let TokenKind::At = self.current_token.kind() {
            self.consume(TokenKind::At)?;
            let annotation = self.parse_identifier("annotation")?;
            annotations.push(annotation);
        }
        self.consume(TokenKind::Fun)?;
        let name = self.parse_identifier("function name")?;
        self.consume(TokenKind::ParenOpen)?;
        self.consume(TokenKind::ParenClose)?;
        self.consume(TokenKind::BraceOpen)?;
        let mut statements = Vec::new();
        while self.current_token.kind() != TokenKind::BraceClose {
            statements.push(self.parse_statement()?);
        }
        self.consume(TokenKind::BraceClose)?;
        let function_definition_node =
            self.create_node(start_position, FunctionDefinition::new(name, annotations, statements))?;
        self.create_node(
            start_position,
            Statement::FunctionDefinition(FunctionDefinitionStatement {
                function_definition: function_definition_node,
            }),
        )
    }

    fn parse_expression_statement(&mut self) -> EffyResult<StatementNode> {
        let start_position = self.current_position();
        let expression = self.parse_expression()?;
        self.create_node(
            start_position,
            Statement::Expression(ExpressionStatement { expression }),
        )
    }

    fn parse_expression(&mut self) -> EffyResult<ExpressionNode> {
        let expression = self.parse_call()?;
        Ok(expression)
    }

    fn parse_call(&mut self) -> EffyResult<ExpressionNode> {
        let start_position = self.current_position();
        let expr = self.parse_primary_expression()?;
        if !self.is_at(TokenKind::ParenOpen) {
            return Ok(expr);
        }
        self.consume(TokenKind::ParenOpen)?;
        let argument = self.parse_expression()?;
        self.consume(TokenKind::ParenClose)?;
        self.create_node(start_position, Expression::call(expr, vec![argument]))
    }

    fn parse_primary_expression(&mut self) -> EffyResult<ExpressionNode> {
        let start_position = self.current_position();
        let result = match self.current_token.kind() {
            TokenKind::Identifier => {
                let name = self.parse_identifier("variable name")?;
                self.create_node(start_position, Expression::var_use(name))
            }
            TokenKind::String => {
                let token = self.consume(TokenKind::String)?;
                self.create_node(
                    start_position,
                    Expression::literal(extract_string_from_lexeme(self.lexeme(&token))?),
                )
            }
            TokenKind::Integer => {
                let token = self.consume(TokenKind::Integer)?;
                self.create_node(
                    start_position,
                    Expression::literal(Value::Int(self.lexeme(&token).parse::<i64>()?)),
                )
            }
            _other => self.create_token_error(
                format!(
                    "Unexpected token: “{}” ({})",
                    self.lexeme(&self.current_token),
                    self.current_token
                ),
                "expected primary expression here".to_string(),
            ),
        }?;
        Ok(result)
    }

    fn lexeme(&self, token: &Token) -> &str {
        token.lexeme(self.source.content())
    }

    fn parse_identifier(&mut self, expected_identifier_role: &str) -> EffyResult<IdentifierNode> {
        let start_position = self.current_position();
        let name = self.consume_with_role(TokenKind::Identifier, expected_identifier_role)?;
        self.create_node(start_position, Identifier::new(self.lexeme(&name)))
    }

    fn advance(&mut self) -> EffyResult<Token> {
        self.last_position = self.current_token.span().end();
        let mut token = self
            .tokens
            .next()
            .ok_or_else(|| err!("No more token in source file, expected at least EOF"))??;
        std::mem::swap(&mut self.current_token, &mut token);
        Ok(token)
    }

    #[track_caller]
    fn consume(&mut self, token_kind: TokenKind) -> EffyResult<Token> {
        if self.current_token.kind() != token_kind {
            return self.create_token_error(
                format!(
                    "Unexpected token: {}, expected {}",
                    self.current_token, token_kind
                ),
                format!("expected {token_kind} here"),
            );
        }
        let token = self.advance()?;
        Ok(token)
    }

    #[track_caller]
    fn consume_with_role(
        &mut self,
        token_kind: TokenKind,
        expected_role: &str,
    ) -> EffyResult<Token> {
        if self.current_token.kind() != token_kind {
            return self.create_token_error(
                format!(
                    "Unexpected token: {}, expected {expected_role} ({token_kind})",
                    self.current_token
                ),
                format!("expected {expected_role} ({token_kind}) here"),
            );
        }
        let token = self.advance()?;
        Ok(token)
    }

    #[track_caller]
    fn create_token_error<T>(
        &mut self,
        error_message: String,
        token_label: String,
    ) -> EffyResult<T> {
        Err(self.create_token_error_internal(error_message, token_label))
    }

    #[track_caller]
    fn create_token_error_internal(
        &mut self,
        error_message: String,
        token_label: String,
    ) -> EffyError {
        let source_snippet = SourceSnippet::new(
            self.source.path().to_string(),
            self.source.content().to_string(),
            1,
            0,
        );
        let mut source_message = SourceMessage::error(error_message, source_snippet);
        source_message.add_label(SourceLabel::new(
            self.current_token.span().clone(),
            token_label,
        ));
        SourceError::new(source_message).into()
    }

    fn is_at(&self, token_kind: TokenKind) -> bool {
        self.current_token.kind() == token_kind
    }

    fn current_position(&mut self) -> usize {
        self.current_token.span().start()
    }

    fn create_node<T: TestPrint>(
        &mut self,
        start_position: usize,
        node: T,
    ) -> EffyResult<AstNode<T>> {
        Ok(AstNode::new(node, start_position..self.last_position))
    }
}

fn extract_string_from_lexeme(lexeme: &str) -> EffyResult<Value> {
    assert!(lexeme.starts_with('"') && lexeme.ends_with('"'));
    let string_content = &lexeme[1..lexeme.len() - 1];
    let string = if !string_content.contains("\\") {
        string_content.to_string()
    } else {
        // unescape backslash escape codes
        let mut unescaped = String::new();
        let mut chars = string_content.chars().peekable();
        while let Some(c) = chars.next() {
            if c == '\\' {
                match chars.peek() {
                    Some('n') => {
                        unescaped.push('\n');
                        chars.next();
                    }
                    Some('t') => {
                        unescaped.push('\t');
                        chars.next();
                    }
                    Some('"') => {
                        unescaped.push('\"');
                        chars.next();
                    }
                    Some('\\') => {
                        unescaped.push('\\');
                        chars.next();
                    }
                    Some(other) => {
                        bail!("Invalid escape sequence: \\{other}")
                    }
                    None => bail!("Incomplete escape sequence"),
                }
            } else {
                unescaped.push(c);
            }
        }
        unescaped
    };
    Ok(Value::String(string))
}

#[cfg(test)]
mod test {
    use crate::parser::parse_script;
    use effy_base::error::{EffyResult, bail, to_test_string};
    use effy_base::source_file::SourceFile;
    use effy_base::test_print::TestPrint;
    use expect_test::{Expect, expect};

    fn test_parse(source: &str, expected: Expect) -> EffyResult<()> {
        let source_file = SourceFile::new("script.effy", source);
        let result = parse_script(&source_file)?;
        let mut test_string = String::new();
        result.test_print(&mut test_string, 0)?;
        expected.assert_eq(&test_string);
        Ok(())
    }

    macro_rules! test_parse {
        ($name:ident, $source:literal, $expected:expr) => {
            #[test]
            fn $name() -> EffyResult<()> {
                test_parse($source, $expected)
            }
        };
    }

    test_parse!(
        empty,
        "",
        expect![[r#"
            🌲   0+0  Script
        "#]]
    );

    fn test_parse_script(source: &str, expected: Expect) -> EffyResult<()> {
        let source_file = SourceFile::new("script.effy", source);
        let result = parse_script(&source_file)?;
        let mut test_string = String::new();
        result.test_print(&mut test_string, 0)?;
        expected.assert_eq(&test_string);
        Ok(())
    }

    macro_rules! test_parse_script {
        ($name:ident, $source:literal, $expected:expr) => {
            #[test]
            fn $name() -> EffyResult<()> {
                test_parse_script($source, $expected)
            }
        };
    }

    test_parse_script!(
        script_empty,
        "",
        expect![[r#"
            🌲   0+0  Script
        "#]]
    );

    test_parse_script!(
        literal_string,
        " \"hello\"; ",
        expect![[r#"
            🌲   0+9  Script
            🌲   1+7   stmt  literal "hello"
        "#]]
    );

    test_parse_script!(
        literal_integer,
        " 19; ",
        expect![[r#"
            🌲   0+4  Script
            🌲   1+2   stmt  literal 19i64
        "#]]
    );

    test_parse_script!(
        script_print,
        "print(\"hello\");",
        expect![[r#"
            🌲   0+15 Script
            🌲   0+14  stmt  call  var use ❮print❯
            🌲   6+7      literal "hello"
        "#]]
    );

    test_parse_script!(
        script_print_twice,
        r#"
            print("hello");
            print("world");
        "#,
        expect![[r#"
            🌲   0+56 Script
            🌲  13+14  stmt  call  var use ❮print❯
            🌲  19+7      literal "hello"
            🌲  41+14  stmt  call  var use ❮print❯
            🌲  47+7      literal "world"
        "#]]
    );

    test_parse_script!(
        fun_empty,
        "fun empty() {}",
        expect![[r#"
            🌲   0+14 Script
            🌲   0+14  stmt function definition
            🌲   0+14   fun empty
        "#]]
    );

    test_parse_script!(
        fun_simple,
        r#"fun simple() {
            print("hello");
        }"#,
        expect![[r#"
            🌲   0+52 Script
            🌲   0+52  stmt function definition
            🌲   0+52   fun simple
            🌲  27+14    stmt  call  var use ❮print❯
            🌲  33+7        literal "hello"
        "#]]
    );

    test_parse_script!(
        fun_with_annotation,
        r#"
        @Test
        fun simple() {
            print("hello");
        }"#,
        expect![[r#"
            🌲   0+75 Script
            🌲   9+66  stmt function definition
            🌲   9+66   fun simple
            🌲  10+4     ❮Test❯
            🌲  50+14    stmt  call  var use ❮print❯
            🌲  56+7        literal "hello"
        "#]]
    );


    fn test_parse_error(source: &str, expected: Expect) -> EffyResult<()> {
        let source_file = SourceFile::new("script.effy", source);
        let Err(error) = parse_script(&source_file) else {
            bail!("expected error")
        };
        expected.assert_eq(&to_test_string(&error));
        Ok(())
    }

    macro_rules! test_parse_error {
        ($name:ident, $source:literal, $expected:expr) => {
            #[test]
            fn $name() -> EffyResult<()> {
                test_parse_error($source, $expected)
            }
        };
    }

    test_parse_error!(
        error_close_paren,
        ")",
        expect![[r#"
            error: Unexpected token: “)” (Close Parenthesis)
              ╭▸ script.effy:1:1
              │
            1 │ )
              ╰╴━ expected primary expression here
        "#]]
    );

    test_parse_error!(
        error_fun_no_name,
        "fun () {};",
        expect![[r#"
            error: Unexpected token: Open Parenthesis, expected function name (Identifier)
              ╭▸ script.effy:1:5
              │
            1 │ fun () {};
              ╰╴    ━ expected function name (Identifier) here
        "#]]
    );

    fn test_parse_script_error(source: &str, expected: Expect) -> EffyResult<()> {
        let source_file = SourceFile::new("script.effy", source);
        let Err(error) = parse_script(&source_file) else {
            bail!("expected error")
        };
        expected.assert_eq(&to_test_string(&error));
        Ok(())
    }

    macro_rules! test_parse_script_error {
        ($name:ident, $source:literal, $expected:expr) => {
            #[test]
            fn $name() -> EffyResult<()> {
                test_parse_script_error($source, $expected)
            }
        };
    }

    test_parse_script_error!(
        error_no_expression,
        "}",
        expect![[r#"
            error: Unexpected token: “}” (Close Brace)
              ╭▸ script.effy:1:1
              │
            1 │ }
              ╰╴━ expected primary expression here
        "#]]
    );
}
