use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum DumbBrainObject {
    Number(f64),
}

impl Display for DumbBrainObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Number(n) => n.to_string(),
            }
        )
    }
}
