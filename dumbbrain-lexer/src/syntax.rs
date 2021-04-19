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
    EqualsEqualsToken,

    IdentifierToken,

    TrueKeyword,
    FalseKeyword,

    BadToken,

    LiteralExpression,
    BinaryExpression,
    UnaryExpression,
    ParenthesizedExpression,
}

pub(crate) fn check_keyword(lexeme: &str) -> SyntaxKind {
    match lexeme {
        "true" => SyntaxKind::TrueKeyword,
        "false" => SyntaxKind::FalseKeyword,
        _ => SyntaxKind::IdentifierToken,
    }
}
impl SyntaxKind {
    pub fn binary_precedence(self) -> usize {
        match self {
            Self::PlusToken | Self::MinusToken => 2,
            Self::StarToken | Self::SlashToken => 3,
            Self::EqualsEqualsToken => 1,
            _ => 0,
        }
    }

    pub fn unary_precedence(self) -> usize {
        match self {
            Self::PlusToken | Self::MinusToken => 4,
            _ => 0,
        }
    }
}
