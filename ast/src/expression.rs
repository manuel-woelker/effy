use crate::ast_node::AstNode;
use crate::identifier::IdentifierNode;
use effy_base::error::EffyResult;
use effy_base::test_print::TestPrint;
use effy_base::value::Value;
use std::fmt::Write;
use std::ops::Deref;

pub enum Expression {
    Call(CallExpression),
    VarUse(VarUseExpression),
    Literal(LiteralExpression),
}

pub type ExpressionNode = AstNode<Expression>;

impl Expression {
    pub fn call(callee: ExpressionNode, arguments: Vec<ExpressionNode>) -> Self {
        Self::Call(CallExpression {
            callee: Box::new(callee),
            arguments,
        })
    }

    pub fn var_use(name: IdentifierNode) -> Self {
        Self::VarUse(VarUseExpression { name })
    }

    pub fn literal(value: Value) -> Self {
        Self::Literal(LiteralExpression { value })
    }
}

pub struct CallExpression {
    callee: Box<ExpressionNode>,
    arguments: Vec<ExpressionNode>,
}

impl CallExpression {
    pub fn callee(&self) -> &ExpressionNode {
        &self.callee
    }
    pub fn arguments(&self) -> &[ExpressionNode] {
        &self.arguments
    }
}

pub struct VarUseExpression {
    name: IdentifierNode,
}

impl VarUseExpression {
    pub fn name(&self) -> &IdentifierNode {
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

impl TestPrint for Expression {
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
