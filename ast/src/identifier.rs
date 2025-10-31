use crate::ast_node::AstNode;
use effy_base::error::EffyResult;
use effy_base::test_print::TestPrint;
use std::fmt::Write;

pub struct Identifier {
    pub name: String,
}

impl Identifier {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

pub type IdentifierNode = AstNode<Identifier>;

impl TestPrint for Identifier {
    fn test_print(&self, write: &mut dyn Write, _indent: usize) -> EffyResult<()> {
        write!(write, "❮{}❯", self.name)?;
        Ok(())
    }
}
