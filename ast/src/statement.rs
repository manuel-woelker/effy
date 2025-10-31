use crate::ast_node::AstNode;
use crate::expression::ExpressionNode;
use effy_base::error::EffyResult;
use effy_base::test_print::TestPrint;
use std::fmt::Write;
use std::ops::Deref;

pub enum Statement {
    Expression(ExpressionStatement),
}

pub type StatementNode = AstNode<Statement>;

pub struct ExpressionStatement {
    pub expression: ExpressionNode,
}

impl TestPrint for Statement {
    fn test_print(&self, write: &mut dyn Write, indent: usize) -> EffyResult<()> {
        write!(write, "stmt ")?;
        match self {
            Statement::Expression(expression) => expression
                .expression
                .deref()
                .test_print(write, indent + 1)?,
        }
        Ok(())
    }
}
