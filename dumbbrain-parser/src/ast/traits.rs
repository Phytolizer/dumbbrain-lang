use dumbbrain::object::DumbBrainObject;
use dumbbrain_lexer::syntax::SyntaxKind;
use dumbbrain_lexer::token::Token;

use super::ExpressionSyntax;

pub trait SyntaxNode {
    fn kind(&self) -> SyntaxKind;
    fn children(&self) -> Vec<&dyn SyntaxNode>;
    fn value(&self) -> Option<&DumbBrainObject>;
}

impl SyntaxNode for Token {
    fn kind(&self) -> SyntaxKind {
        self.kind
    }

    fn children(&self) -> Vec<&dyn SyntaxNode> {
        vec![]
    }

    fn value(&self) -> Option<&DumbBrainObject> {
        self.value.as_ref()
    }
}

impl SyntaxNode for ExpressionSyntax {
    fn kind(&self) -> SyntaxKind {
        match self {
            ExpressionSyntax::Literal { .. } => SyntaxKind::LiteralExpression,
            ExpressionSyntax::Binary { .. } => SyntaxKind::BinaryExpression,
            ExpressionSyntax::Unary { .. } => SyntaxKind::UnaryExpression,
        }
    }

    fn children(&self) -> Vec<&dyn SyntaxNode> {
        match self {
            ExpressionSyntax::Literal { literal_token } => vec![literal_token],
            ExpressionSyntax::Binary {
                left,
                operator_token,
                right,
            } => vec![left.as_ref(), operator_token, right.as_ref()],
            ExpressionSyntax::Unary {
                operator_token,
                right,
            } => vec![operator_token, right.as_ref()],
        }
    }

    fn value(&self) -> Option<&DumbBrainObject> {
        None
    }
}
