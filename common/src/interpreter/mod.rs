//! Interpreter module

mod opcode;
mod parameter_mode;

pub use opcode::{OpCode, Register};
pub use parameter_mode::ParameterMode;

/// Execution state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionState {
    /// Continuing
    Next,
    /// Quitting
    Exit,
    /// Waiting
    Wait,
}

/// Interpreter
#[derive(Debug, Clone)]
pub struct Interpreter {
    data: Vec<i64>,
    initial: Vec<i64>,
    cursor: usize,
    input_stream: Vec<i64>,
    output_stream: Vec<i64>,
    debug: bool,
    relative_base: i64,
}

impl Interpreter {
    /// Create intepreter from input text
    pub fn new(input_txt: &str) -> Self {
        let data: Vec<i64> = input_txt.split(',').map(|x| x.parse().unwrap()).collect();

        Self {
            initial: data.clone(),
            data,
            cursor: 0,
            output_stream: vec![],
            input_stream: vec![],
            debug: false,
            relative_base: 0,
        }
    }

    /// Set debug mode
    pub fn set_debug_mode(&mut self, value: bool) {
        self.debug = value;
    }

    /// Push input value
    pub fn push_input(&mut self, input: i64) {
        self.input_stream.push(input);
    }

    /// Pop input
    pub fn pop_input(&mut self) -> Option<i64> {
        if self.input_stream.is_empty() {
            None
        } else {
            Some(self.input_stream.remove(0))
        }
    }

    /// Push output value
    pub fn push_output(&mut self, value: i64) {
        self.output_stream.push(value);
    }

    /// Set output values
    pub fn set_output_values(&mut self, values: Vec<i64>) {
        self.output_stream = values;
    }

    /// Pop output
    pub fn pop_output(&mut self) -> Option<i64> {
        if self.output_stream.is_empty() {
            None
        } else {
            Some(self.output_stream.remove(0))
        }
    }

    /// Run and dump
    pub fn run_and_dump(input_txt: &str) -> String {
        let mut interpreter = Self::new(input_txt);
        interpreter.set_debug_mode(true);
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
    pub fn run_with_input_output(input_txt: &str, input: &[i64]) -> String {
        let mut interpreter = Self::new(input_txt);
        for i in input {
            interpreter.push_input(*i);
        }

        interpreter.run();

        interpreter.dump_output()
    }

    /// Allocate memory
    pub fn allocate_memory(&mut self, up_to: usize) {
        let data_len = self.data.len();
        if up_to < data_len {
            panic!(
                "Cannot allocate up to {}: current memory already is up to {}",
                up_to, data_len
            );
        }

        if self.debug {
            println!("Allocating memory from {} to {} ...", data_len, up_to);
        }

        for _ in data_len..=up_to {
            self.data.push(0);
        }
    }

    /// Get value at position
    pub fn get_value(&self, position: usize) -> i64 {
        // If position does not exist in memory,
        // return default memory: 0
        if position >= self.data.len() {
            0
        } else {
            self.data[position]
        }
    }

    /// Set value at position
    pub fn set_value(&mut self, position: usize, value: i64) {
        if position >= self.data.len() {
            self.allocate_memory(position);
        }

        self.data[position] = value;
    }

    /// Read register
    pub fn read_register(&self, reg: Register) -> i64 {
        match reg.mode {
            ParameterMode::Position => self.get_value(reg.value as usize),
            ParameterMode::Immediate => reg.value,
            ParameterMode::Relative => self.get_value((self.relative_base + reg.value) as usize),
        }
    }

    /// Read output register
    pub fn read_output_register(&self, reg: Register) -> i64 {
        match reg.mode {
            ParameterMode::Position => reg.value,
            ParameterMode::Immediate => reg.value,
            ParameterMode::Relative => self.relative_base + reg.value,
        }
    }

    /// Adjust relative base
    pub fn adjust_relative_base(&mut self, offset: i64) {
        self.relative_base += offset;
    }

    /// Increment cursor
    pub fn increment_cursor(&mut self) {
        self.cursor += 1;
    }

    /// Advance cursor from opcode
    pub fn advance_cursor(&mut self, amount: usize) {
        self.cursor += amount;
    }

    /// Restore 1202 program alarm state
    pub fn restore_alarm_state(&mut self) {
        self.set_input_values(12, 2);
    }

    /// Set input values
    pub fn set_input_values(&mut self, noun: i64, verb: i64) {
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
        self.input_stream.clear();
        self.output_stream.clear();
        self.relative_base = 0;
    }

    /// Get stream at cursor
    pub fn get_stream_at_cursor(&self) -> &[i64] {
        &self.data[self.cursor as usize..]
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

    // Clear output
    pub fn clear_output(&mut self) {
        self.output_stream.clear();
    }

    /// Get input stream
    pub fn get_input_stream(&self) -> &[i64] {
        &self.input_stream
    }

    /// Get output stream
    pub fn get_output_stream(&self) -> &[i64] {
        &self.output_stream
    }

    /// Run step
    pub fn step(&mut self) -> (OpCode, ExecutionState) {
        let code_stream = self.get_stream_at_cursor();
        if code_stream.is_empty() {
            return (OpCode::Exit, ExecutionState::Exit);
        }

        if self.debug {
            println!("Reading stream {:?}", code_stream);
        }

        let (opcode, count) = OpCode::parse(code_stream);
        if self.debug {
            println!("Opcode: {:?}", opcode.dump());
        }

        match opcode {
            OpCode::Add(r1, r2, r3) => {
                let v1 = self.read_register(r1);
                let v2 = self.read_register(r2);
                let v3 = self.read_output_register(r3);
                self.set_value(v3 as usize, v1 + v2);
                self.advance_cursor(count);
            }
            OpCode::Multiply(r1, r2, r3) => {
                let v1 = self.read_register(r1);
                let v2 = self.read_register(r2);
                let v3 = self.read_output_register(r3);
                self.set_value(v3 as usize, v1 * v2);
                self.advance_cursor(count);
            }
            OpCode::Store(r) => {
                if let Some(input) = self.pop_input() {
                    if self.debug {
                        println!("Getting input {}", input);
                    }
                    let output = self.read_output_register(r);
                    self.set_value(output as usize, input);
                    self.advance_cursor(count);
                } else {
                    if self.debug {
                        println!("[WAITING]");
                    }
                    return (opcode, ExecutionState::Wait);
                }
            }
            OpCode::Show(r) => {
                let v = self.read_register(r);
                self.push_output(v);
                if self.debug {
                    println!("Outputting: {}", v);
                }
                self.advance_cursor(count);
            }
            OpCode::JumpIfTrue(ri, ro) => {
                let i = self.read_register(ri);
                if i != 0 {
                    let o = self.read_register(ro);
                    self.set_cursor_value(o as usize);
                } else {
                    self.advance_cursor(count);
                }
            }
            OpCode::JumpIfFalse(ri, ro) => {
                let i = self.read_register(ri);
                if i == 0 {
                    let o = self.read_register(ro);
                    self.set_cursor_value(o as usize);
                } else {
                    self.advance_cursor(count);
                }
            }
            OpCode::LessThan(r1, r2, r3) => {
                let v1 = self.read_register(r1);
                let v2 = self.read_register(r2);
                let v3 = self.read_output_register(r3);
                if v1 < v2 {
                    self.set_value(v3 as usize, 1);
                } else {
                    self.set_value(v3 as usize, 0);
                }
                self.advance_cursor(count);
            }
            OpCode::Equals(r1, r2, r3) => {
                let v1 = self.read_register(r1);
                let v2 = self.read_register(r2);
                let v3 = self.read_output_register(r3);
                if v1 == v2 {
                    self.set_value(v3 as usize, 1);
                } else {
                    self.set_value(v3 as usize, 0);
                }
                self.advance_cursor(count);
            }
            OpCode::AdjustRelativeBase(r) => {
                let v = self.read_register(r);
                self.adjust_relative_base(v);
                self.advance_cursor(count);
            }
            OpCode::Exit => {
                return (opcode, ExecutionState::Exit);
            }
        }

        (opcode, ExecutionState::Next)
    }

    /// Run interpreter on initial data
    pub fn run(&mut self) -> String {
        let mut output = String::new();
        if self.debug {
            println!("Interpreter input: {:?}", self.get_input_stream());
        }

        loop {
            let (opcode, state) = self.step();
            output.push_str(&opcode.dump());
            output.push('\n');

            match state {
                ExecutionState::Next => (),
                ExecutionState::Exit => break,
                ExecutionState::Wait => break,
            }
        }

        if self.debug {
            println!("Interpreter output: {:?}", self.get_output_stream());
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_trace() {
        let code = "3,3,1105,-1,9,1101,0,0,12,4,12,99,1";
        let trace = "STORE 3\n\
                     JMPT [8], [9]\n\
                     SHOW 12\n\
                     EXIT\n";

        let mut interpreter = Interpreter::new(code);
        interpreter.push_input(8);
        assert_eq!(interpreter.run(), trace.to_owned());
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

    #[test]
    fn test_relative() {
        fn run_and_full_output(input_txt: &str) -> String {
            let mut interp = Interpreter::new(input_txt);
            interp.run();
            interp.dump_output()
        }

        assert_eq!(
            run_and_full_output("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99"),
            "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99".to_owned()
        );

        // Big number (16 digits)
        assert_eq!(
            run_and_full_output("1102,34915192,34915192,7,4,7,99,0")
                .chars()
                .count(),
            16
        );

        assert_eq!(
            run_and_full_output("104,1125899906842624,99"),
            "1125899906842624".to_owned()
        );
    }
}
