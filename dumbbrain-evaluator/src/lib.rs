use dumbbrain::object::DumbBrainObject;
use dumbbrain::types::DumbBrainType;
use dumbbrain_binder::BinaryOperation;
use dumbbrain_binder::BoundExpression;
use dumbbrain_binder::BoundExpressionNode;
use dumbbrain_binder::UnaryOperation;

const FLOATING_POINT_DELTA: f64 = 1e-6;

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
                _ => panic!("unexpected type for {:?}: {:?}", operation, expression.kind),
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
                _ => panic!("unexpected type for {:?}: {:?}", operation, expression.kind),
            },
            BinaryOperation::Subtract => match expression.kind {
                DumbBrainType::Number => DumbBrainObject::Number(
                    left.unwrap().try_into_number().unwrap()
                        - right.unwrap().try_into_number().unwrap(),
                ),
                _ => panic!("unexpected type for {:?}: {:?}", operation, expression.kind),
            },
            BinaryOperation::Multiply => match expression.kind {
                DumbBrainType::Number => DumbBrainObject::Number(
                    left.unwrap().try_into_number().unwrap()
                        * right.unwrap().try_into_number().unwrap(),
                ),
                _ => panic!("unexpected type for {:?}: {:?}", operation, expression.kind),
            },
            BinaryOperation::Divide => match expression.kind {
                DumbBrainType::Number => DumbBrainObject::Number(
                    left.unwrap().try_into_number().unwrap()
                        / right.unwrap().try_into_number().unwrap(),
                ),
                _ => panic!("unexpected type for {:?}: {:?}", operation, expression.kind),
            },
            BinaryOperation::Equality
            | BinaryOperation::Inequality
            | BinaryOperation::Less
            | BinaryOperation::LessEquals
            | BinaryOperation::Greater
            | BinaryOperation::GreaterEquals => evaluate_comparison(left, right, *operation),
            BinaryOperation::LogicalAnd => evaluate_boolean_operation(left, right, *operation),
            BinaryOperation::LogicalOr => evaluate_boolean_operation(left, right, *operation),
        }
    }
}

fn evaluate_boolean_operation(
    left: Option<DumbBrainObject>,
    right: Option<DumbBrainObject>,
    operation: BinaryOperation,
) -> DumbBrainObject {
    let value = match *left.as_ref().unwrap() {
        DumbBrainObject::Boolean(b) if right.as_ref().unwrap().is_boolean() => {
            let c = right.unwrap().try_into_boolean().unwrap();
            match operation {
                BinaryOperation::LogicalAnd => b && c,
                BinaryOperation::LogicalOr => b || c,
                _ => unreachable!(),
            }
        }
        _ => {
            panic!(
                "mismatched types for {:?}: {:?} and {:?}",
                operation,
                *left.as_ref().unwrap(),
                *right.as_ref().unwrap()
            )
        }
    };
    DumbBrainObject::Boolean(value)
}

fn evaluate_comparison(
    left: Option<DumbBrainObject>,
    right: Option<DumbBrainObject>,
    operation: BinaryOperation,
) -> DumbBrainObject {
    let value = match *left.as_ref().unwrap() {
        DumbBrainObject::Number(n) if right.as_ref().unwrap().is_number() => {
            let m = right.unwrap().try_into_number().unwrap();
            match operation {
                BinaryOperation::Equality => (n - m).abs() < FLOATING_POINT_DELTA,
                BinaryOperation::Inequality => (n - m).abs() > FLOATING_POINT_DELTA,
                BinaryOperation::Less => n < m,
                BinaryOperation::LessEquals => n <= m,
                BinaryOperation::Greater => n > m,
                BinaryOperation::GreaterEquals => n >= m,
                _ => unreachable!(),
            }
        }
        DumbBrainObject::Boolean(b) if right.as_ref().unwrap().is_boolean() => {
            let c = right.unwrap().try_into_boolean().unwrap();
            match operation {
                BinaryOperation::Equality => b == c,
                BinaryOperation::Inequality => b != c,
                _ => panic!(
                    "type mismatch: cannot perform comparison {:?} on Boolean and Boolean",
                    operation
                ),
            }
        }
        _ => panic!(
            "type mismatch on ==: {} vs {}",
            left.as_ref().unwrap(),
            right.as_ref().unwrap()
        ),
    };
    DumbBrainObject::Boolean(value)
}

#[cfg(test)]
mod tests {
    use dumbbrain_binder::Binder;
    use dumbbrain_parser::Parser;
    use expect_test::expect;
    use expect_test::Expect;

    use super::*;

    fn check(input: &str, expected: Expect) {
        let tree = Parser::new(input).parse();
        let bound_tree = Binder::bind_expression(&tree);
        let value = Evaluator::new(bound_tree).evaluate();
        let formatted = format!("{:#?}", value);
        expected.assert_eq(&formatted);
    }

    #[test]
    fn evaluate_number_literal() {
        check(
            "3",
            expect![[r#"
            Some(
                Number(
                    3.0,
                ),
            )"#]],
        )
    }

    #[test]
    fn evaluate_boolean_literal() {
        check(
            "true",
            expect![[r#"
            Some(
                Boolean(
                    true,
                ),
            )"#]],
        )
    }

    #[test]
    fn evaluate_addition() {
        check(
            "3 + 4",
            expect![[r#"
            Some(
                Number(
                    7.0,
                ),
            )"#]],
        )
    }

    #[test]
    fn evaluate_subtraction() {
        check(
            "1 - 2",
            expect![[r#"
            Some(
                Number(
                    -1.0,
                ),
            )"#]],
        )
    }

    #[test]
    fn evaluate_multiplication() {
        check(
            "2 * 4",
            expect![[r#"
            Some(
                Number(
                    8.0,
                ),
            )"#]],
        )
    }

    #[test]
    fn evaluate_division() {
        check(
            "5 / 6",
            expect![[r#"
            Some(
                Number(
                    0.8333333333333334,
                ),
            )"#]],
        )
    }

    #[test]
    fn evaluate_equality() {
        check(
            "5 == 6",
            expect![[r#"
            Some(
                Boolean(
                    false,
                ),
            )"#]],
        )
    }

    #[test]
    fn evaluate_inequality() {
        check(
            "5 != 6",
            expect![[r#"
            Some(
                Boolean(
                    true,
                ),
            )"#]],
        )
    }

    #[test]
    fn evaluate_comparison() {
        check(
            "5 > 6",
            expect![[r#"
            Some(
                Boolean(
                    false,
                ),
            )"#]],
        )
    }

    #[test]
    fn evaluate_complex_expression() {
        check(
            "(5 + 6) * 3 > 2 + 4 == true",
            expect![[r#"
            Some(
                Boolean(
                    true,
                ),
            )"#]],
        )
    }
}
