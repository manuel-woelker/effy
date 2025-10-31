use crate::ast_node::AstNode;
use crate::identifier::IdentifierNode;
use effy_base::error::EffyResult;
use effy_base::test_print::TestPrint;
use effy_base::value::Value;
use std::fmt::Write;
use std::ops::Deref;

pub enum Expression<'source> {
    Call(CallExpression<'source>),
    VarUse(VarUseExpression<'source>),
    Literal(LiteralExpression),
}

pub type ExpressionNode<'source> = AstNode<'source, Expression<'source>>;

impl<'source> Expression<'source> {
    pub fn call(callee: ExpressionNode<'source>, arguments: Vec<ExpressionNode<'source>>) -> Self {
        Self::Call(CallExpression {
            callee: Box::new(callee),
            arguments,
        })
    }

    pub fn var_use(name: IdentifierNode<'source>) -> Self {
        Self::VarUse(VarUseExpression { name })
    }

    pub fn literal(value: Value) -> Self {
        Self::Literal(LiteralExpression { value })
    }
}

pub struct CallExpression<'source> {
    callee: Box<ExpressionNode<'source>>,
    arguments: Vec<ExpressionNode<'source>>,
}

impl CallExpression<'_> {
    pub fn callee(&self) -> &ExpressionNode<'_> {
        &self.callee
    }
    pub fn arguments(&self) -> &[ExpressionNode<'_>] {
        &self.arguments
    }
}

pub struct VarUseExpression<'source> {
    name: IdentifierNode<'source>,
}

impl VarUseExpression<'_> {
    pub fn name(&self) -> &IdentifierNode<'_> {
        &self.name
    }
}

pub struct LiteralExpression {
    value: Value,
}

impl LiteralExpression {
    pub fn value(&self) -> &Value {
        &self.value
    }
}

impl TestPrint for Expression<'_> {
    fn test_print(&self, write: &mut dyn Write, indent: usize) -> EffyResult<()> {
        match self {
            Expression::Call(call) => {
                write!(write, " call ")?;
                call.callee.deref().deref().test_print(write, indent + 1)?;
                writeln!(write)?;
                for argument in &call.arguments {
                    argument.test_print(write, indent + 1)?;
                }
            }
            Expression::VarUse(var_use) => {
                write!(write, " var use ")?;
                var_use.name.deref().test_print(write, indent + 1)?;
            }
            Expression::Literal(literal) => {
                writeln!(write, " literal {}", &literal.value)?;
            }
        }
        Ok(())
    }
}
