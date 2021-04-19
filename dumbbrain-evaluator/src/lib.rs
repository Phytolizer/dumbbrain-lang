use dumbbrain::object::DumbBrainObject;
use dumbbrain::types::DumbBrainType;
use dumbbrain_binder::BinaryOperation;
use dumbbrain_binder::BoundExpression;
use dumbbrain_binder::BoundExpressionNode;
use dumbbrain_binder::UnaryOperation;

pub struct Evaluator {
    bound_tree: BoundExpression,
}

impl Evaluator {
    pub fn new(bound_tree: BoundExpression) -> Self {
        Self { bound_tree }
    }

    pub fn evaluate(&self) -> Option<DumbBrainObject> {
        self.evaluate_expression(&self.bound_tree)
    }

    fn evaluate_expression(&self, expression: &BoundExpression) -> Option<DumbBrainObject> {
        match &expression.node {
            BoundExpressionNode::Literal { value } => value.clone(),
            BoundExpressionNode::Binary {
                left,
                right,
                operation,
            } => Some(self.evaluate_binary_expression(left, right, operation, expression)),
            BoundExpressionNode::Unary { operand, operation } => {
                self.evaluate_unary_expression(operand, operation, expression)
            }
        }
    }

    fn evaluate_unary_expression(
        &self,
        operand: &BoundExpression,
        operation: &UnaryOperation,
        expression: &BoundExpression,
    ) -> Option<DumbBrainObject> {
        let operand = self.evaluate_expression(&operand);
        match operation {
            UnaryOperation::Identity => operand,
            UnaryOperation::Negation => match expression.kind {
                DumbBrainType::Number => Some(DumbBrainObject::Number(
                    -operand.unwrap().try_into_number().unwrap(),
                )),
            },
        }
    }

    fn evaluate_binary_expression(
        &self,
        left: &BoundExpression,
        right: &BoundExpression,
        operation: &BinaryOperation,
        expression: &BoundExpression,
    ) -> DumbBrainObject {
        let left = self.evaluate_expression(&left);
        let right = self.evaluate_expression(&right);
        match operation {
            BinaryOperation::Add => match expression.kind {
                DumbBrainType::Number => DumbBrainObject::Number(
                    left.unwrap().try_into_number().unwrap()
                        + right.unwrap().try_into_number().unwrap(),
                ),
            },
            BinaryOperation::Subtract => match expression.kind {
                DumbBrainType::Number => DumbBrainObject::Number(
                    left.unwrap().try_into_number().unwrap()
                        - right.unwrap().try_into_number().unwrap(),
                ),
            },
            BinaryOperation::Multiply => match expression.kind {
                DumbBrainType::Number => DumbBrainObject::Number(
                    left.unwrap().try_into_number().unwrap()
                        * right.unwrap().try_into_number().unwrap(),
                ),
            },
            BinaryOperation::Divide => match expression.kind {
                DumbBrainType::Number => DumbBrainObject::Number(
                    left.unwrap().try_into_number().unwrap()
                        / right.unwrap().try_into_number().unwrap(),
                ),
            },
        }
    }
}
