use crate::ast_node::AstNode;
use crate::identifier::IdentifierNode;
use crate::statement::StatementNode;
use effy_base::error::EffyResult;
use effy_base::test_print::TestPrint;
use std::fmt::Write;
use std::sync::Arc;

#[derive(Clone)]
pub struct FunctionDefinition {
    inner: Arc<FunctionDefinitionInner>,
}

pub struct FunctionDefinitionInner {
    pub name: IdentifierNode,
    pub annotations: Vec<IdentifierNode>,
    pub parameters: Vec<IdentifierNode>,
    pub statements: Vec<StatementNode>,
}

impl FunctionDefinition {
    pub fn new(
        name: IdentifierNode,
        annotations: Vec<IdentifierNode>,
        parameters: Vec<IdentifierNode>,
        statements: Vec<StatementNode>,
    ) -> Self {
        Self {
            inner: Arc::new(FunctionDefinitionInner {
                name,
                annotations,
                parameters,
                statements,
            }),
        }
    }

    pub fn name(&self) -> &IdentifierNode {
        &self.inner.name
    }

    pub fn annotations(&self) -> &Vec<IdentifierNode> {
        &self.inner.annotations
    }

    pub fn parameters(&self) -> &Vec<IdentifierNode> {
        &self.inner.parameters
    }

    pub fn statements(&self) -> &Vec<StatementNode> {
        &self.inner.statements
    }
}

pub type FunctionDefinitionNode = AstNode<FunctionDefinition>;

impl TestPrint for FunctionDefinition {
    fn test_print(&self, write: &mut dyn Write, indent: usize) -> EffyResult<()> {
        let inner = &self.inner;
        writeln!(write, "fun {}", inner.name.data.name)?;
        for annotation in &inner.annotations {
            //            self.indent(write, indent+1)?;
            //            writeln!(write, "@{}", annotation.name)?;
            annotation.test_print(write, indent + 1)?;
            writeln!(write)?;
        }
        for parameter in &inner.parameters {
            parameter.test_print(write, indent + 1)?;
            writeln!(write)?;
        }
        for statement in &inner.statements {
            statement.test_print(write, indent + 1)?;
        }
        Ok(())
    }
}
