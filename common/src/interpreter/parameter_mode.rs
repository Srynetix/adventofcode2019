//! Parameter mode

/// Parameter mode
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ParameterMode {
    Position = 0,
    Immediate = 1,
    Relative = 2,
}

impl ParameterMode {
    /// Parse parameter mode
    pub fn parse(value: i64) -> Self {
        match value {
            0 => Self::Position,
            1 => Self::Immediate,
            2 => Self::Relative,
            _ => panic!("Unsupported parameter mode: {}", value),
        }
    }
}
