use crate::ast_node::AstNode;
use crate::identifier::IdentifierNode;
use effy_base::error::EffyResult;
use effy_base::test_print::TestPrint;
use effy_base::value::Value;
use std::fmt::{Display, Write};
use std::ops::Deref;

pub enum Expression {
    Call(CallExpression),
    VarUse(VarUseExpression),
    Literal(LiteralExpression),
    Binary(BinaryExpression),
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

    pub fn binary(left: ExpressionNode, operator: BinaryOperator, right: ExpressionNode) -> Self {
        Self::Binary(BinaryExpression {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
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

#[derive(Debug)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
    Modulo,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Equals,
    NotEquals,
    And,
    Or,
}

impl Display for BinaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryOperator::Add => write!(f, "+"),
            BinaryOperator::Subtract => write!(f, "-"),
            BinaryOperator::Multiply => write!(f, "*"),
            BinaryOperator::Divide => write!(f, "/"),
            BinaryOperator::Power => write!(f, "^"),
            BinaryOperator::Modulo => write!(f, "%"),
            BinaryOperator::LessThan => write!(f, "<"),
            BinaryOperator::LessThanOrEqual => write!(f, "<="),
            BinaryOperator::GreaterThan => write!(f, ">"),
            BinaryOperator::GreaterThanOrEqual => write!(f, ">="),
            BinaryOperator::Equals => write!(f, "=="),
            BinaryOperator::NotEquals => write!(f, "!="),
            BinaryOperator::And => write!(f, "and"),
            BinaryOperator::Or => write!(f, "or"),
        }
    }
}

pub struct BinaryExpression {
    left: Box<ExpressionNode>,
    operator: BinaryOperator,
    right: Box<ExpressionNode>,
}

impl BinaryExpression {
    pub fn left(&self) -> &ExpressionNode {
        &self.left
    }
    pub fn operator(&self) -> &BinaryOperator {
        &self.operator
    }
    pub fn right(&self) -> &ExpressionNode {
        &self.right
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
            Expression::Binary(binary) => {
                writeln!(write, " binary {}", binary.operator)?;
                binary.left.deref().test_print(write, indent + 1)?;
                binary.right.deref().test_print(write, indent + 1)?;
            }
        }
        Ok(())
    }
}
