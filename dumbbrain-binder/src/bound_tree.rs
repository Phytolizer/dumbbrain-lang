use dumbbrain::object::DumbBrainObject;
use dumbbrain::types::DumbBrainType;

pub struct BoundExpression {
    pub node: BoundExpressionNode,
    pub kind: DumbBrainType,
}

pub enum BinaryOperation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

pub enum UnaryOperation {
    Identity,
    Negation,
}

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
