//! Interpreter module

mod opcode;
mod parameter_mode;

pub use opcode::{OpCode, ParameteredOpCode};
pub use parameter_mode::ParameterMode;

/// Interpreter
#[derive(Debug)]
pub struct Interpreter {
    data: Vec<i32>,
    initial: Vec<i32>,
    cursor: usize,
    input_stream: Vec<i32>,
    output_stream: Vec<i32>,
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

    /// Run with input/output
    pub fn run_with_input_output(input_txt: &str, input: &[i32]) -> String {
        let mut interpreter = Self::new(input_txt);
        for i in input {
            interpreter.push_input(*i);
        }

        interpreter.run();

        interpreter.dump_output()
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

    /// Set cursor value
    pub fn set_cursor_value(&mut self, value: usize) {
        self.cursor = value;
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
                    OpCode::JumpIfTrue => {
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

                        if v1 != 0 {
                            self.set_cursor_value(v2 as usize);
                        }
                    }
                    OpCode::JumpIfFalse => {
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

                        if v1 == 0 {
                            self.set_cursor_value(v2 as usize);
                        }
                    }
                    OpCode::LessThan => {
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

                        if v1 < v2 {
                            self.set_value(v3 as usize, 1);
                        } else {
                            self.set_value(v3 as usize, 0);
                        }
                    }
                    OpCode::Equals => {
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

                        if v1 == v2 {
                            self.set_value(v3 as usize, 1);
                        } else {
                            self.set_value(v3 as usize, 0);
                        }
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

    #[test]
    fn test_jumps_and_conditions() {
        // Equals 8 (pos)
        assert_eq!(
            Interpreter::run_with_input_output("3,9,8,9,10,9,4,9,99,-1,8", &vec![8]),
            "1".to_owned()
        );
        assert_eq!(
            Interpreter::run_with_input_output("3,9,8,9,10,9,4,9,99,-1,8", &vec![7]),
            "0".to_owned()
        );
        // Less than 8 (pos)
        assert_eq!(
            Interpreter::run_with_input_output("3,9,7,9,10,9,4,9,99,-1,8", &vec![8]),
            "0".to_owned()
        );
        assert_eq!(
            Interpreter::run_with_input_output("3,9,7,9,10,9,4,9,99,-1,8", &vec![7]),
            "1".to_owned()
        );
        // Equals 8 (imm)
        assert_eq!(
            Interpreter::run_with_input_output("3,3,1108,-1,8,3,4,3,99", &vec![8]),
            "1".to_owned()
        );
        assert_eq!(
            Interpreter::run_with_input_output("3,3,1108,-1,8,3,4,3,99", &vec![7]),
            "0".to_owned()
        );
        // Less than 8 (imm)
        assert_eq!(
            Interpreter::run_with_input_output("3,3,1107,-1,8,3,4,3,99", &vec![8]),
            "0".to_owned()
        );
        assert_eq!(
            Interpreter::run_with_input_output("3,3,1107,-1,8,3,4,3,99", &vec![7]),
            "1".to_owned()
        );

        // Jump (pos)
        assert_eq!(
            Interpreter::run_with_input_output(
                "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9",
                &vec![0]
            ),
            "0".to_owned()
        );
        assert_eq!(
            Interpreter::run_with_input_output(
                "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9",
                &vec![5]
            ),
            "1".to_owned()
        );
        // Jump (imm)
        assert_eq!(
            Interpreter::run_with_input_output("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", &vec![0]),
            "0".to_owned()
        );
        assert_eq!(
            Interpreter::run_with_input_output("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", &vec![5]),
            "1".to_owned()
        );

        // Full
        let full_code = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,\
                         1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,\
                         46,1101,1000,1,20,4,20,1105,1,46,98,99";

        assert_eq!(
            Interpreter::run_with_input_output(full_code, &vec![7]),
            "999".to_owned()
        );
        assert_eq!(
            Interpreter::run_with_input_output(full_code, &vec![8]),
            "1000".to_owned()
        );
        assert_eq!(
            Interpreter::run_with_input_output(full_code, &vec![9]),
            "1001".to_owned()
        );
    }
}
