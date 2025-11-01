use crate::ast_node::AstNode;
use crate::identifier::IdentifierNode;
use crate::statement::StatementNode;
use effy_base::error::EffyResult;
use effy_base::test_print::TestPrint;
use std::fmt::Write;

pub struct FunctionDefinition {
    pub name: IdentifierNode,
    pub statements: Vec<StatementNode>,
}

impl FunctionDefinition {
    pub fn new(name: IdentifierNode, statements: Vec<StatementNode>) -> Self {
        Self { name, statements }
    }
}

pub type FunctionDefinitionNode = AstNode<FunctionDefinition>;

impl TestPrint for FunctionDefinition {
    fn test_print(&self, write: &mut dyn Write, indent: usize) -> EffyResult<()> {
        writeln!(write, "fun {}", self.name.data.name)?;
        for statement in &self.statements {
            statement.test_print(write, indent + 1)?;
        }
        Ok(())
    }
}
