use dumbbrain::object::DumbBrainObject;
use dumbbrain::types::DumbBrainType;

#[derive(Debug)]
pub struct BoundExpression {
    pub node: BoundExpressionNode,
    pub kind: DumbBrainType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BinaryOperation {
    Add,
    Subtract,
    Multiply,
    Divide,

    Equality,
    Inequality,
    Less,
    LessEquals,
    Greater,
    GreaterEquals,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnaryOperation {
    Identity,
    Negation,
}

#[derive(Debug)]
pub enum BoundExpressionNode {
    Literal {
        value: Option<DumbBrainObject>,
    },
    Binary {
        left: Box<BoundExpression>,
        right: Box<BoundExpression>,
        operation: BinaryOperation,
    },
    Unary {
        operand: Box<BoundExpression>,
        operation: UnaryOperation,
    },
}
