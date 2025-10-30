use crate::ast_node::AstNode;
use crate::expression::ExpressionNode;
use effy_base::error::EffyResult;
use effy_base::test_print::TestPrint;
use std::fmt::Write;

pub enum Statement<'source> {
    Expression(ExpressionStatement<'source>),
}

pub type StatementNode<'source> = AstNode<'source, Statement<'source>>;

pub struct ExpressionStatement<'source> {
    pub expression: ExpressionNode<'source>,
}

impl TestPrint for Statement<'_> {
    fn test_print(&self, write: &mut dyn Write, indent: usize) -> EffyResult<()> {
        write!(write, "stmt ")?;
        match self {
            Statement::Expression(expression) => {
                expression.expression.test_print(write, indent + 1)?
            }
        }
        Ok(())
    }
}
