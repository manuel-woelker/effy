use effy_ast::ast_node::AstNode;
use effy_ast::script::{Script, ScriptNode};
use effy_base::error::{EffyResult, err};
use effy_base::source_file::SourceFile;
use effy_base::source_location::SourceLocation;
use effy_base::test_print::TestPrint;
use effy_tokenizer::token::Token;
use effy_tokenizer::tokenize::tokenize;

pub fn parse_script(script_source: &SourceFile) -> EffyResult<ScriptNode<'_>> {
    let mut tokens = tokenize(script_source);
    let mut parser = Parser::new(script_source, &mut tokens)?;
    parser.parse_script()
}

#[allow(dead_code)]
struct Parser<'source, 'tokens> {
    source: &'source SourceFile,
    current_token: Token<'source>,
    last_position: usize,
    tokens: &'tokens mut dyn Iterator<Item = EffyResult<Token<'source>>>,
}

impl<'source, 'tokens> Parser<'source, 'tokens> {
    fn new(
        source: &'source SourceFile,
        tokens: &'tokens mut dyn Iterator<Item = EffyResult<Token<'source>>>,
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

    fn parse_script(&mut self) -> EffyResult<ScriptNode<'source>> {
        let statements = Vec::new();
        let start_position = self.last_position;
        /*        while self.current_token.kind() != TokenKind::EOF {
            statements.push(self.parse_statement()?);
        }*/
        self.create_node(start_position, Script::new(statements))
    }

    fn create_node<T: TestPrint>(
        &mut self,
        start_position: usize,
        node: T,
    ) -> EffyResult<AstNode<'source, T>> {
        Ok(AstNode::new(
            node,
            SourceLocation::new(self.source, start_position, self.last_position),
        ))
    }
}

#[cfg(test)]
mod test {
    use crate::parser::parse_script;
    use effy_base::error::EffyResult;
    use effy_base::source_file::SourceFile;
    use effy_base::test_print::TestPrint;
    use expect_test::{Expect, expect};

    fn test_parse(source: &str, expected: Expect) -> EffyResult<()> {
        let source_file = SourceFile::new("test.effy", source);
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
}
