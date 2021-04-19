#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum SyntaxKind {
    NumberToken,
    WhitespaceToken,
    PlusToken,
    MinusToken,
    StarToken,
    SlashToken,
    LeftParenthesisToken,
    RightParenthesisToken,
    BadToken,

    LiteralExpression,
    BinaryExpression,
    UnaryExpression,
    ParenthesizedExpression,
}
