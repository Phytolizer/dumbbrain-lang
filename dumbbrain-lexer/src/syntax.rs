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
    BangEqualsToken,
    LessToken,
    LessEqualsToken,
    GreaterToken,
    GreaterEqualsToken,

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
            Self::PlusToken | Self::MinusToken => 3,
            Self::StarToken | Self::SlashToken => 4,
            Self::EqualsEqualsToken | Self::BangEqualsToken => 1,
            Self::LessToken
            | Self::LessEqualsToken
            | Self::GreaterToken
            | Self::GreaterEqualsToken => 2,
            _ => 0,
        }
    }

    pub fn unary_precedence(self) -> usize {
        match self {
            Self::PlusToken | Self::MinusToken => 5,
            _ => 0,
        }
    }
}
