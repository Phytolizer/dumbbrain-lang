use dumbbrain::object::DumbBrainObject;

use crate::span::Span;
use crate::syntax::SyntaxKind;

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: SyntaxKind,
    pub position: usize,
    pub text: String,
    pub value: Option<DumbBrainObject>,
    pub span: Span,
}

impl Token {
    pub(crate) fn new<O>(
        kind: SyntaxKind,
        position: usize,
        text: String,
        value: O,
        span: Span,
    ) -> Self
    where
        O: Into<Option<DumbBrainObject>>,
    {
        Self {
            kind,
            position,
            text,
            value: value.into(),
            span,
        }
    }
}
