use crate::ast_node::AstNode;
use effy_base::error::EffyResult;
use effy_base::test_print::TestPrint;
use std::fmt::Write;

pub struct Identifier {
    pub name: String,
}

impl Identifier {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

pub type IdentifierNode<'source> = AstNode<'source, Identifier>;

impl TestPrint for Identifier {
    fn test_print(&self, write: &mut dyn Write, _indent: usize) -> EffyResult<()> {
        write!(write, "❮{}❯", self.name)?;
        Ok(())
    }
}
