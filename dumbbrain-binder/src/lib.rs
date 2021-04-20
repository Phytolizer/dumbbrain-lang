use dumbbrain::types::DumbBrainType;
use dumbbrain_lexer::syntax::SyntaxKind;
use dumbbrain_lexer::token::Token;
use dumbbrain_parser::ast::traits::SyntaxNode;
use dumbbrain_parser::ast::ExpressionSyntax;

mod bound_tree;
pub use bound_tree::*;

pub struct Binder {}

impl Binder {
    pub fn bind_expression(expression: &ExpressionSyntax) -> BoundExpression {
        match expression {
            ExpressionSyntax::Literal { literal_token } => {
                Self::bind_literal_expression(literal_token)
            }
            ExpressionSyntax::Binary {
                left,
                operator_token,
                right,
            } => Self::bind_binary_expression(left, operator_token, right),
            ExpressionSyntax::Unary {
                operator_token,
                right,
            } => Self::bind_unary_expression(operator_token, right),
            ExpressionSyntax::Parenthesized { expression, .. } => Self::bind_expression(expression),
        }
    }

    fn resolve_binary_type(
        left: &BoundExpression,
        operator_token: &Token,
        right: &BoundExpression,
    ) -> DumbBrainType {
        match operator_token.kind() {
            SyntaxKind::PlusToken
            | SyntaxKind::MinusToken
            | SyntaxKind::StarToken
            | SyntaxKind::SlashToken => {
                if left.kind == DumbBrainType::Number && right.kind == DumbBrainType::Number {
                    DumbBrainType::Number
                } else {
                    panic!(
                        "unexpected types for {:?}: {:?}, {:?}",
                        operator_token.kind(),
                        left.kind,
                        right.kind
                    )
                }
            }
            SyntaxKind::EqualsEqualsToken
            | SyntaxKind::BangEqualsToken
            | SyntaxKind::LessToken
            | SyntaxKind::LessEqualsToken
            | SyntaxKind::GreaterToken
            | SyntaxKind::GreaterEqualsToken
            | SyntaxKind::AmpersandAmpersandToken
            | SyntaxKind::PipePipeToken => DumbBrainType::Boolean,
            _ => unreachable!(),
        }
    }

    fn resolve_unary_type(operator_token: &Token, operand: &BoundExpression) -> DumbBrainType {
        match operator_token.kind() {
            SyntaxKind::PlusToken | SyntaxKind::MinusToken => {
                if operand.kind == DumbBrainType::Number {
                    DumbBrainType::Number
                } else {
                    todo!()
                }
            }
            _ => unreachable!(),
        }
    }

    fn bind_literal_expression(literal_token: &Token) -> BoundExpression {
        match literal_token.kind() {
            SyntaxKind::NumberToken => BoundExpression {
                node: BoundExpressionNode::Literal {
                    value: literal_token.value.clone(),
                },
                kind: DumbBrainType::Number,
            },
            SyntaxKind::TrueKeyword | SyntaxKind::FalseKeyword => BoundExpression {
                node: BoundExpressionNode::Literal {
                    value: literal_token.value.clone(),
                },
                kind: DumbBrainType::Boolean,
            },
            _ => unreachable!(),
        }
    }

    fn bind_binary_expression(
        left: &ExpressionSyntax,
        operator_token: &Token,
        right: &ExpressionSyntax,
    ) -> BoundExpression {
        let left = Self::bind_expression(left);
        let right = Self::bind_expression(right);

        let resolved_type = Self::resolve_binary_type(&left, operator_token, &right);
        let left = Box::new(left);
        let right = Box::new(right);
        match operator_token.kind() {
            SyntaxKind::PlusToken => BoundExpression {
                node: BoundExpressionNode::Binary {
                    left,
                    right,
                    operation: BinaryOperation::Add,
                },
                kind: resolved_type,
            },
            SyntaxKind::MinusToken => BoundExpression {
                node: BoundExpressionNode::Binary {
                    left,
                    right,
                    operation: BinaryOperation::Subtract,
                },
                kind: resolved_type,
            },
            SyntaxKind::StarToken => BoundExpression {
                node: BoundExpressionNode::Binary {
                    left,
                    right,
                    operation: BinaryOperation::Multiply,
                },
                kind: resolved_type,
            },
            SyntaxKind::SlashToken => BoundExpression {
                node: BoundExpressionNode::Binary {
                    left,
                    right,
                    operation: BinaryOperation::Divide,
                },
                kind: resolved_type,
            },
            SyntaxKind::EqualsEqualsToken => BoundExpression {
                node: BoundExpressionNode::Binary {
                    left,
                    right,
                    operation: BinaryOperation::Equality,
                },
                kind: resolved_type,
            },
            SyntaxKind::BangEqualsToken => BoundExpression {
                node: BoundExpressionNode::Binary {
                    left,
                    right,
                    operation: BinaryOperation::Inequality,
                },
                kind: resolved_type,
            },
            SyntaxKind::LessToken => BoundExpression {
                node: BoundExpressionNode::Binary {
                    left,
                    right,
                    operation: BinaryOperation::Less,
                },
                kind: resolved_type,
            },
            SyntaxKind::LessEqualsToken => BoundExpression {
                node: BoundExpressionNode::Binary {
                    left,
                    right,
                    operation: BinaryOperation::LessEquals,
                },
                kind: resolved_type,
            },
            SyntaxKind::GreaterToken => BoundExpression {
                node: BoundExpressionNode::Binary {
                    left,
                    right,
                    operation: BinaryOperation::Greater,
                },
                kind: resolved_type,
            },
            SyntaxKind::GreaterEqualsToken => BoundExpression {
                node: BoundExpressionNode::Binary {
                    left,
                    right,
                    operation: BinaryOperation::GreaterEquals,
                },
                kind: resolved_type,
            },
            SyntaxKind::AmpersandAmpersandToken => BoundExpression {
                node: BoundExpressionNode::Binary {
                    left,
                    right,
                    operation: BinaryOperation::LogicalAnd,
                },
                kind: resolved_type,
            },
            SyntaxKind::PipePipeToken => BoundExpression {
                node: BoundExpressionNode::Binary {
                    left,
                    right,
                    operation: BinaryOperation::LogicalOr,
                },
                kind: resolved_type,
            },
            _ => unreachable!(),
        }
    }

    fn bind_unary_expression(
        operator_token: &Token,
        operand: &ExpressionSyntax,
    ) -> BoundExpression {
        let operand = Self::bind_expression(operand);
        let resolved_type = Self::resolve_unary_type(operator_token, &operand);
        let operand = Box::new(operand);
        match operator_token.kind() {
            SyntaxKind::PlusToken => BoundExpression {
                node: BoundExpressionNode::Unary {
                    operand,
                    operation: UnaryOperation::Identity,
                },
                kind: resolved_type,
            },
            SyntaxKind::MinusToken => BoundExpression {
                node: BoundExpressionNode::Unary {
                    operand,
                    operation: UnaryOperation::Negation,
                },
                kind: resolved_type,
            },
            _ => unreachable!(),
        }
    }
}
