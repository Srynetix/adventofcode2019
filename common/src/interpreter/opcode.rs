//! OpCode module

use super::parameter_mode::ParameterMode;

/// Register
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Register {
    /// Value
    pub value: i64,
    /// Mode
    pub mode: ParameterMode,
}

impl Register {
    /// New register
    pub fn new(value: i64, mode: ParameterMode) -> Self {
        Self { value, mode }
    }

    /// From stream
    pub fn from_stream(idx: usize, stream: &[i64], parameters: &[ParameterMode]) -> Self {
        Self::new(
            stream[idx],
            parameters
                .get(idx)
                .copied()
                .unwrap_or(ParameterMode::Position),
        )
    }

    /// From first arg
    pub fn from_first_arg(stream: &[i64], parameters: &[ParameterMode]) -> Self {
        Self::from_stream(0, stream, parameters)
    }

    /// From second arg
    pub fn from_second_arg(stream: &[i64], parameters: &[ParameterMode]) -> Self {
        Self::from_stream(1, stream, parameters)
    }

    /// From third output arg
    pub fn from_third_arg(stream: &[i64], parameters: &[ParameterMode]) -> Self {
        Self::from_stream(2, stream, parameters)
    }
}

/// OpCode
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum OpCode {
    Add(Register, Register, Register),
    Multiply(Register, Register, Register),
    Store(Register),
    Show(Register),
    JumpIfTrue(Register, Register),
    JumpIfFalse(Register, Register),
    LessThan(Register, Register, Register),
    Equals(Register, Register, Register),
    AdjustRelativeBase(Register),
    Exit,
}

impl OpCode {
    /// Parse code stream
    pub fn parse(code_stream: &[i64]) -> (Self, usize) {
        let parametered_code = code_stream[0];
        let code = parametered_code % 100;

        let mut base = parametered_code / 100;
        let mut parameters = vec![];
        while base > 0 {
            let value = base % 10;
            parameters.push(ParameterMode::parse(value));
            base /= 10;
        }

        let arg_stream = &code_stream[1..];

        match code {
            1 => (
                Self::Add(
                    Register::from_first_arg(arg_stream, &parameters),
                    Register::from_second_arg(arg_stream, &parameters),
                    Register::from_third_arg(arg_stream, &parameters),
                ),
                4,
            ),
            2 => (
                Self::Multiply(
                    Register::from_first_arg(arg_stream, &parameters),
                    Register::from_second_arg(arg_stream, &parameters),
                    Register::from_third_arg(arg_stream, &parameters),
                ),
                4,
            ),
            3 => (
                Self::Store(Register::from_first_arg(arg_stream, &parameters)),
                2,
            ),
            4 => (
                Self::Show(Register::from_first_arg(arg_stream, &parameters)),
                2,
            ),
            5 => (
                Self::JumpIfTrue(
                    Register::from_first_arg(arg_stream, &parameters),
                    Register::from_second_arg(arg_stream, &parameters),
                ),
                3,
            ),
            6 => (
                Self::JumpIfFalse(
                    Register::from_first_arg(arg_stream, &parameters),
                    Register::from_second_arg(arg_stream, &parameters),
                ),
                3,
            ),
            7 => (
                Self::LessThan(
                    Register::from_first_arg(arg_stream, &parameters),
                    Register::from_second_arg(arg_stream, &parameters),
                    Register::from_third_arg(arg_stream, &parameters),
                ),
                4,
            ),
            8 => (
                Self::Equals(
                    Register::from_first_arg(arg_stream, &parameters),
                    Register::from_second_arg(arg_stream, &parameters),
                    Register::from_third_arg(arg_stream, &parameters),
                ),
                4,
            ),
            9 => (
                Self::AdjustRelativeBase(Register::from_first_arg(arg_stream, &parameters)),
                2,
            ),
            99 => (Self::Exit, 1),
            _ => panic!("Unsupported opcode: {}", code),
        }
    }

    /// Dump opcode
    pub fn dump(&self) -> String {
        fn show_reg(reg: Register) -> String {
            match reg.mode {
                ParameterMode::Position => format!("{}", reg.value),
                ParameterMode::Immediate => format!("[{}]", reg.value),
                ParameterMode::Relative if reg.value < 0 => format!("[B{}]", reg.value),
                ParameterMode::Relative => format!("[B+{}]", reg.value),
            }
        }

        match *self {
            Self::Add(r1, r2, r3) => {
                format!("ADD {}, {}, {}", show_reg(r1), show_reg(r2), show_reg(r3))
            }
            Self::Multiply(r1, r2, r3) => {
                format!("MUL {}, {}, {}", show_reg(r1), show_reg(r2), show_reg(r3))
            }
            Self::Store(r) => format!("STORE {}", show_reg(r)),
            Self::Show(r) => format!("SHOW {}", show_reg(r)),
            Self::JumpIfTrue(ri, ro) => format!("JMPT {}, {}", show_reg(ri), show_reg(ro)),
            Self::JumpIfFalse(ri, ro) => format!("JMPF {}, {}", show_reg(ri), show_reg(ro)),
            Self::LessThan(r1, r2, r3) => {
                format!("LT {}, {}, {}", show_reg(r1), show_reg(r2), show_reg(r3))
            }
            Self::Equals(r1, r2, r3) => {
                format!("EQ {}, {}, {}", show_reg(r1), show_reg(r2), show_reg(r3))
            }
            Self::AdjustRelativeBase(r) => format!("ARB {}", show_reg(r)),
            Self::Exit => "EXIT".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn str_to_code_stream(input: &str) -> Vec<i64> {
        input.split(',').map(|x| x.parse().unwrap()).collect()
    }

    #[test]
    fn test_params() {
        let stream = str_to_code_stream("1001,8,10,8");
        let (code, count) = OpCode::parse(&stream);
        assert_eq!(
            code,
            OpCode::Add(
                Register::new(8, ParameterMode::Position),
                Register::new(10, ParameterMode::Immediate),
                Register::new(8, ParameterMode::Position)
            )
        );
        assert_eq!(count, 4);
        assert_eq!(code.dump(), "ADD 8, [10], 8");
    }

    #[test]
    fn test_parse() {
        let stream = str_to_code_stream("1,0,2,2,4,1");
        let (code, count) = OpCode::parse(&stream);
        assert_eq!(
            code,
            OpCode::Add(
                Register::new(0, ParameterMode::Position),
                Register::new(2, ParameterMode::Position),
                Register::new(2, ParameterMode::Position)
            )
        );
        assert_eq!(count, 4);
        assert_eq!(code.dump(), "ADD 0, 2, 2".to_owned());

        // Code 2
        let n_stream = &stream[4..];
        let (code, count) = OpCode::parse(n_stream);
        assert_eq!(
            code,
            OpCode::Show(Register::new(1, ParameterMode::Position))
        );
        assert_eq!(count, 2);
        assert_eq!(code.dump(), "SHOW 1".to_owned())
    }

    #[test]
    fn test_relative() {
        let stream =
            str_to_code_stream("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99");
        let (code, count) = OpCode::parse(&stream);
        assert_eq!(
            code,
            OpCode::AdjustRelativeBase(Register::new(1, ParameterMode::Immediate))
        );
        assert_eq!(count, 2);
        assert_eq!(code.dump(), "ARB [1]".to_owned());

        let n_stream = &stream[2..];
        let (code, count) = OpCode::parse(n_stream);
        assert_eq!(
            code,
            OpCode::Show(Register::new(-1, ParameterMode::Relative))
        );
        assert_eq!(count, 2);
        assert_eq!(code.dump(), "SHOW [B-1]")
    }
}
