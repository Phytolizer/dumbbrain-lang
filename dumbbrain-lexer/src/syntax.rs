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
    AmpersandAmpersandToken,
    PipePipeToken,

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
            Self::PlusToken | Self::MinusToken => 4,
            Self::StarToken | Self::SlashToken => 5,
            Self::EqualsEqualsToken | Self::BangEqualsToken => 1,
            Self::AmpersandAmpersandToken | Self::PipePipeToken => 2,
            Self::LessToken
            | Self::LessEqualsToken
            | Self::GreaterToken
            | Self::GreaterEqualsToken => 3,
            _ => 0,
        }
    }

    pub fn unary_precedence(self) -> usize {
        match self {
            Self::PlusToken | Self::MinusToken => 6,
            _ => 0,
        }
    }
}
