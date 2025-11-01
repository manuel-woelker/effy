use crate::ast_node::AstNode;
use crate::statement::StatementNode;
use effy_base::error::EffyResult;
use effy_base::test_print::TestPrint;
use std::fmt::Write;

pub struct Script {
    pub statements: Vec<StatementNode>,
}

impl Script {
    pub fn new(statements: Vec<StatementNode>) -> Self {
        Self { statements }
    }
}

pub type ScriptNode = AstNode<Script>;

impl TestPrint for Script {
    fn test_print(&self, write: &mut dyn Write, indent: usize) -> EffyResult<()> {
        self.indent(write, indent)?;
        writeln!(write, "Script")?;
        for statement in &self.statements {
            statement.test_print(write, indent + 1)?;
        }
        Ok(())
    }
}
