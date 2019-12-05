//! Interpreter code

/// Interpreter
#[derive(Debug)]
pub struct Interpreter {
    data: Vec<i32>,
    initial: Vec<i32>,
    cursor: usize,
    input_stream: Vec<i32>,
    output_stream: Vec<i32>,
}

/// OpCode
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum OpCode {
    Add = 1,
    Multiply = 2,
    Store = 3,
    Show = 4,
    Exit = 99,
}

/// Parameter mode
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ParameterMode {
    Position = 0,
    Immediate = 1,
}

impl ParameterMode {
    /// Parse parameter mode
    pub fn parse(value: i32) -> Self {
        match value {
            0 => Self::Position,
            1 => Self::Immediate,
            _ => panic!("Unsupported parameter mode: {}", value),
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
            99 => Self::Exit,
            _ => panic!("Unsupported opcode: {}", code),
        }
    }
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

impl Interpreter {
    /// Create intepreter from input text
    pub fn new(input_txt: &str) -> Self {
        let data: Vec<i32> = input_txt.split(',').map(|x| x.parse().unwrap()).collect();

        Self {
            initial: data.clone(),
            data,
            cursor: 0,
            output_stream: vec![],
            input_stream: vec![],
        }
    }

    /// Push input value
    pub fn push_input(&mut self, input: i32) {
        self.input_stream.push(input);
    }

    /// Pop input
    pub fn pop_input(&mut self) -> Option<i32> {
        self.input_stream.pop()
    }

    /// Push output value
    pub fn push_output(&mut self, value: i32) {
        self.output_stream.push(value);
    }

    /// Pop output
    pub fn pop_output(&mut self) -> Option<i32> {
        self.output_stream.pop()
    }

    /// Run and dump
    pub fn run_and_dump(input_txt: &str) -> String {
        let mut interpreter = Self::new(input_txt);
        interpreter.run();
        interpreter.dump()
    }

    /// Run and dump with output
    pub fn run_and_dump_with_output(input_txt: &str) -> (String, String) {
        let mut interpreter = Self::new(input_txt);
        interpreter.run();

        (interpreter.dump(), interpreter.dump_output())
    }

    /// Get value at position
    pub fn get_value(&self, position: usize) -> Option<i32> {
        self.data.get(position).cloned()
    }

    /// Set value at position
    pub fn set_value(&mut self, position: usize, value: i32) {
        self.data[position] = value;
    }

    /// Get cursor value
    pub fn get_value_at_cursor(&self) -> Option<i32> {
        self.get_value(self.cursor)
    }

    /// Get parametered value
    pub fn get_parametered_value(&self, value: i32, mode: ParameterMode) -> Option<i32> {
        match mode {
            ParameterMode::Position => self.get_value(value as usize),
            ParameterMode::Immediate => Some(value),
        }
    }

    /// Increment cursor
    pub fn increment_cursor(&mut self) {
        self.cursor += 1;
    }

    /// Restore 1202 program alarm state
    pub fn restore_alarm_state(&mut self) {
        self.set_input_values(12, 2);
    }

    /// Set input values
    pub fn set_input_values(&mut self, noun: i32, verb: i32) {
        self.data[1] = noun;
        self.data[2] = verb;
    }

    /// Restore data
    pub fn reset_intepreter(&mut self) {
        self.data = self.initial.clone();
        self.cursor = 0;
    }

    /// Dump intepreter data
    pub fn dump(&self) -> String {
        let str_dump: Vec<String> = self.data.iter().map(|x| x.to_string()).collect();
        str_dump.join(",")
    }

    /// Dump output
    pub fn dump_output(&self) -> String {
        let str_dump: Vec<String> = self.output_stream.iter().map(|x| x.to_string()).collect();
        str_dump.join(",")
    }

    /// Get output stream
    pub fn get_output_stream(&self) -> &[i32] {
        &self.output_stream
    }

    /// Run intepreter on initial data
    pub fn run(&mut self) {
        loop {
            let opcode = self.get_value_at_cursor().map(ParameteredOpCode::parse);
            self.increment_cursor();

            if let Some(opcode) = opcode {
                match opcode.code {
                    OpCode::Add => {
                        let v1 = self.get_value_at_cursor().unwrap();
                        let v1 = self
                            .get_parametered_value(v1, opcode.get_parameter_mode(0))
                            .unwrap();
                        self.increment_cursor();
                        let v2 = self.get_value_at_cursor().unwrap();
                        let v2 = self
                            .get_parametered_value(v2, opcode.get_parameter_mode(1))
                            .unwrap();
                        self.increment_cursor();
                        let v3 = self.get_value_at_cursor().unwrap();
                        self.increment_cursor();

                        self.set_value(v3 as usize, v1 + v2);
                    }
                    OpCode::Multiply => {
                        let v1 = self.get_value_at_cursor().unwrap();
                        let v1 = self
                            .get_parametered_value(v1, opcode.get_parameter_mode(0))
                            .unwrap();
                        self.increment_cursor();
                        let v2 = self.get_value_at_cursor().unwrap();
                        let v2 = self
                            .get_parametered_value(v2, opcode.get_parameter_mode(1))
                            .unwrap();
                        self.increment_cursor();
                        let v3 = self.get_value_at_cursor().unwrap();
                        self.increment_cursor();

                        self.set_value(v3 as usize, v1 * v2);
                    }
                    OpCode::Store => {
                        let input = self.pop_input().expect("Input stack is empty");
                        let output = self.get_value_at_cursor().unwrap();
                        self.increment_cursor();

                        self.set_value(output as usize, input);
                    }
                    OpCode::Show => {
                        let v1 = self.get_value_at_cursor().unwrap();
                        let v1 = self
                            .get_parametered_value(v1, opcode.get_parameter_mode(0))
                            .unwrap();
                        self.increment_cursor();

                        self.push_output(v1);
                    }
                    OpCode::Exit => {
                        break;
                    }
                }
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Interpreter, OpCode, ParameterMode, ParameteredOpCode};

    #[test]
    fn test_opcodes() {
        assert_eq!(OpCode::parse(3), OpCode::Store);
        assert_eq!(
            ParameteredOpCode::parse(1002),
            ParameteredOpCode {
                code: OpCode::Multiply,
                modes: vec![ParameterMode::Position, ParameterMode::Immediate]
            }
        );
    }

    #[test]
    fn test_simple() {
        // Day 2
        assert_eq!(
            Interpreter::run_and_dump("1,0,0,0,99"),
            "2,0,0,0,99".to_owned()
        );
        assert_eq!(
            Interpreter::run_and_dump("2,3,0,3,99"),
            "2,3,0,6,99".to_owned()
        );
        assert_eq!(
            Interpreter::run_and_dump("2,4,4,5,99,0"),
            "2,4,4,5,99,9801".to_owned()
        );
        assert_eq!(
            Interpreter::run_and_dump("1,1,1,4,99,5,6,0,99"),
            "30,1,1,4,2,5,6,0,99".to_owned()
        );

        // Day 5
        assert_eq!(
            Interpreter::run_and_dump("1002,4,3,4,33"),
            "1002,4,3,4,99".to_owned()
        );
        assert_eq!(
            Interpreter::run_and_dump("1101,100,-1,4,0"),
            "1101,100,-1,4,99".to_owned()
        );
        assert_eq!(
            Interpreter::run_and_dump_with_output("104,50,99"),
            ("104,50,99".to_owned(), "50".to_owned())
        );
    }
}
