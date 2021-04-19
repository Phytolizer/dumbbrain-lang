use dumbbrain::object::DumbBrainObject;

use crate::syntax::SyntaxKind;

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: SyntaxKind,
    pub position: usize,
    pub text: String,
    pub value: Option<DumbBrainObject>,
}

impl Token {
    pub(crate) fn new<O>(kind: SyntaxKind, position: usize, text: String, value: O) -> Self
    where
        O: Into<Option<DumbBrainObject>>,
    {
        Self {
            kind,
            position,
            text,
            value: value.into(),
        }
    }
}
