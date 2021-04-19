use dumbbrain_macros::IsAs;

use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, IsAs)]
pub enum DumbBrainObject {
    Number(f64),
    Boolean(bool),
}

impl Display for DumbBrainObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Number(n) => n.to_string(),
                Self::Boolean(b) => b.to_string(),
            }
        )
    }
}
