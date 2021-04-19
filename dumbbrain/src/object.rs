use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum DumbBrainObject {
    Number(f64),
}

impl DumbBrainObject {
    /// Returns `true` if the dumb_brain_object is [`Number`].
    pub fn is_number(&self) -> bool {
        matches!(self, Self::Number(..))
    }

    pub fn as_number(&self) -> Option<&f64> {
        match self {
            Self::Number(v) => Some(v),
            _ => None,
        }
    }

    pub fn try_into_number(self) -> Result<f64, Self> {
        match self {
            Self::Number(v) => Ok(v),
            _ => Err(self),
        }
    }
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
