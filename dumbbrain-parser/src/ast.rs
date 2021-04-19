use dumbbrain_lexer::token::Token;

pub mod traits;

#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionSyntax {
    Literal {
        literal_token: Token,
    },
    Binary {
        left: Box<ExpressionSyntax>,
        operator_token: Token,
        right: Box<ExpressionSyntax>,
    },
    Unary {
        operator_token: Token,
        right: Box<ExpressionSyntax>,
    },
    Parenthesized {
        left_parenthesis_token: Token,
        expression: Box<ExpressionSyntax>,
        right_parenthesis_token: Token,
    },
}
