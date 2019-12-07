//! OpCode module

use super::parameter_mode::ParameterMode;

/// Register
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Register {
    /// Value
    pub value: i32,
    /// Mode
    pub mode: ParameterMode,
}

impl Register {
    /// New register
    pub fn new(value: i32, mode: ParameterMode) -> Self {
        Self { value, mode }
    }

    /// From stream
    pub fn from_stream(idx: usize, stream: &[i32], parameters: &[ParameterMode]) -> Self {
        Self::new(
            stream[idx],
            parameters
                .get(idx)
                .copied()
                .unwrap_or(ParameterMode::Position),
        )
    }

    /// From first arg
    pub fn from_first_arg(stream: &[i32], parameters: &[ParameterMode]) -> Self {
        Self::from_stream(0, stream, parameters)
    }

    /// From second arg
    pub fn from_second_arg(stream: &[i32], parameters: &[ParameterMode]) -> Self {
        Self::from_stream(1, stream, parameters)
    }
}

/// Address
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Address(usize);

impl Address {
    pub fn new(value: usize) -> Self {
        Self(value)
    }

    /// Read
    pub fn read(self) -> usize {
        self.0
    }

    /// From stream
    pub fn from_stream(idx: usize, stream: &[i32]) -> Self {
        Self::new(stream[idx] as usize)
    }

    /// From first arg
    pub fn from_first_arg(stream: &[i32]) -> Self {
        Self::from_stream(0, stream)
    }

    /// From second arg
    pub fn from_second_arg(stream: &[i32]) -> Self {
        Self::from_stream(1, stream)
    }

    /// From third arg
    pub fn from_third_arg(stream: &[i32]) -> Self {
        Self::from_stream(2, stream)
    }
}

/// OpCode
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum OpCode {
    Add(Register, Register, Address),
    Multiply(Register, Register, Address),
    Store(Address),
    Show(Register),
    JumpIfTrue(Register, Register),
    JumpIfFalse(Register, Register),
    LessThan(Register, Register, Address),
    Equals(Register, Register, Address),
    Exit,
}

impl OpCode {
    /// Parse code stream
    pub fn parse(code_stream: &[i32]) -> (Self, usize) {
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
                    Address::from_third_arg(arg_stream),
                ),
                4,
            ),
            2 => (
                Self::Multiply(
                    Register::from_first_arg(arg_stream, &parameters),
                    Register::from_second_arg(arg_stream, &parameters),
                    Address::from_third_arg(arg_stream),
                ),
                4,
            ),
            3 => (Self::Store(Address::from_first_arg(arg_stream)), 2),
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
                    Address::from_third_arg(arg_stream),
                ),
                4,
            ),
            8 => (
                Self::Equals(
                    Register::from_first_arg(arg_stream, &parameters),
                    Register::from_second_arg(arg_stream, &parameters),
                    Address::from_third_arg(arg_stream),
                ),
                4,
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
            }
        }

        fn show_addr(addr: Address) -> String {
            addr.0.to_string()
        }

        match *self {
            Self::Add(r1, r2, a) => {
                format!("ADD {}, {}, {}", show_reg(r1), show_reg(r2), show_addr(a))
            }
            Self::Multiply(r1, r2, a) => {
                format!("MUL {}, {}, {}", show_reg(r1), show_reg(r2), show_addr(a))
            }
            Self::Store(a) => format!("STORE {}", show_addr(a)),
            Self::Show(r) => format!("SHOW {}", show_reg(r)),
            Self::JumpIfTrue(ri, ro) => format!("JMPT {}, {}", show_reg(ri), show_reg(ro)),
            Self::JumpIfFalse(ri, ro) => format!("JMPF {}, {}", show_reg(ri), show_reg(ro)),
            Self::LessThan(r1, r2, a) => {
                format!("LT {}, {}, {}", show_reg(r1), show_reg(r2), show_addr(a))
            }
            Self::Equals(r1, r2, a) => {
                format!("EQ {}, {}, {}", show_reg(r1), show_reg(r2), show_addr(a))
            }
            Self::Exit => "EXIT".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn str_to_code_stream(input: &str) -> Vec<i32> {
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
                Address::new(8)
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
                Address::new(2)
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
}
