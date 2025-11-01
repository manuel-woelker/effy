use crate::ast_node::AstNode;
use crate::expression::ExpressionNode;
use crate::function_definition::FunctionDefinitionNode;
use effy_base::error::EffyResult;
use effy_base::test_print::TestPrint;
use std::fmt::Write;
use std::ops::Deref;

pub enum Statement {
    Expression(ExpressionStatement),
    FunctionDefinition(FunctionDefinitionStatement),
}

pub type StatementNode = AstNode<Statement>;

pub struct ExpressionStatement {
    pub expression: ExpressionNode,
}

pub struct FunctionDefinitionStatement {
    pub function_definition: FunctionDefinitionNode,
}

impl TestPrint for Statement {
    fn test_print(&self, write: &mut dyn Write, indent: usize) -> EffyResult<()> {
        write!(write, "stmt ")?;
        match self {
            Statement::Expression(expression) => expression
                .expression
                .deref()
                .test_print(write, indent + 1)?,
            Statement::FunctionDefinition(function_definition) => {
                writeln!(write, "function definition")?;
                function_definition
                    .function_definition
                    .test_print(write, indent + 1)?;
            }
        }
        Ok(())
    }
}
