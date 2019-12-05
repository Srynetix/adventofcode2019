//! OpCode module

use super::parameter_mode::ParameterMode;

/// OpCode
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum OpCode {
    Add = 1,
    Multiply = 2,
    Store = 3,
    Show = 4,
    JumpIfTrue = 5,
    JumpIfFalse = 6,
    LessThan = 7,
    Equals = 8,
    Exit = 99,
}

impl OpCode {
    /// Parse opcode
    pub fn parse(code: i32) -> Self {
        if code > 99 {
            panic!("Opcode value is too high: {}", code);
        }

        match code {
            1 => Self::Add,
            2 => Self::Multiply,
            3 => Self::Store,
            4 => Self::Show,
            5 => Self::JumpIfTrue,
            6 => Self::JumpIfFalse,
            7 => Self::LessThan,
            8 => Self::Equals,
            99 => Self::Exit,
            _ => panic!("Unsupported opcode: {}", code),
        }
    }
}

/// Parametered OpCode
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ParameteredOpCode {
    /// OpCode
    pub code: OpCode,
    /// Modes
    pub modes: Vec<ParameterMode>,
}

impl ParameteredOpCode {
    /// Parse parametered opcode
    pub fn parse(code: i32) -> Self {
        let mut base = code;
        let opcode = OpCode::parse(base % 100);
        let mut parameters = vec![];
        base /= 100;

        while base > 0 {
            let value = base % 10;
            parameters.push(ParameterMode::parse(value));
            base /= 10;
        }

        Self {
            code: opcode,
            modes: parameters,
        }
    }

    /// Get parameter mode for argument index
    pub fn get_parameter_mode(&self, index: usize) -> ParameterMode {
        self.modes
            .get(index)
            .copied()
            .unwrap_or_else(|| ParameterMode::Position)
    }
}
