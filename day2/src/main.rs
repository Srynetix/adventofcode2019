#![allow(dead_code)]

struct Interpreter {
    data: Vec<u32>,
    initial: Vec<u32>,
    cursor: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum OpCode {
    Add = 1,
    Multiply = 2,
    Exit = 99,
}

impl OpCode {
    fn parse(code: u32) -> Option<Self> {
        match code {
            1 => Some(Self::Add),
            2 => Some(Self::Multiply),
            99 => Some(Self::Exit),
            _ => None,
        }
    }
}

impl Interpreter {
    /// Create intepreter from input text
    fn new(input_txt: &str) -> Self {
        let data: Vec<u32> = input_txt.split(',').map(|x| x.parse().unwrap()).collect();

        Self {
            initial: data.clone(),
            data,
            cursor: 0,
        }
    }

    /// Get value at position
    fn get_value(&self, position: usize) -> Option<u32> {
        self.data.get(position).cloned()
    }

    /// Set value at position
    fn set_value(&mut self, position: usize, value: u32) {
        self.data[position] = value;
    }

    /// Get cursor value
    fn get_value_at_cursor(&self) -> Option<u32> {
        self.get_value(self.cursor)
    }

    /// Increment cursor
    fn increment_cursor(&mut self) {
        self.cursor += 1;
    }

    /// Restore 1202 program alarm state
    fn restore_alarm_state(&mut self) {
        self.set_input_values(12, 2);
    }

    /// Set input values
    fn set_input_values(&mut self, noun: u32, verb: u32) {
        self.data[1] = noun;
        self.data[2] = verb;
    }

    /// Restore data
    fn reset_intepreter(&mut self) {
        self.data = self.initial.clone();
        self.cursor = 0;
    }

    /// Dump intepreter data
    fn dump(&self) -> String {
        let str_dump: Vec<String> = self.data.iter().map(|x| x.to_string()).collect();
        str_dump.join(",")
    }

    /// Run intepreter on initial data
    fn run(&mut self) {
        loop {
            let opcode = self.get_value_at_cursor().and_then(OpCode::parse);
            self.increment_cursor();

            if let Some(opcode) = opcode {
                match opcode {
                    OpCode::Add => {
                        let v1 = self.get_value_at_cursor().unwrap();
                        let v1 = self.get_value(v1 as usize).unwrap();
                        self.increment_cursor();
                        let v2 = self.get_value_at_cursor().unwrap();
                        let v2 = self.get_value(v2 as usize).unwrap();
                        self.increment_cursor();
                        let v3 = self.get_value_at_cursor().unwrap();
                        self.increment_cursor();

                        self.set_value(v3 as usize, v1 + v2);
                    }
                    OpCode::Multiply => {
                        let v1 = self.get_value_at_cursor().unwrap();
                        let v1 = self.get_value(v1 as usize).unwrap();
                        self.increment_cursor();
                        let v2 = self.get_value_at_cursor().unwrap();
                        let v2 = self.get_value(v2 as usize).unwrap();
                        self.increment_cursor();
                        let v3 = self.get_value_at_cursor().unwrap();
                        self.increment_cursor();

                        self.set_value(v3 as usize, v1 * v2);
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

fn part1(input_txt: &str) {
    let mut interpreter = Interpreter::new(input_txt);
    println!("[Part 1]");
    interpreter.restore_alarm_state();
    interpreter.run();
    println!("Result: {}", interpreter.get_value(0).unwrap());
}

fn part2(input_txt: &str, answer: u32) {
    let mut interpreter = Interpreter::new(input_txt);
    println!("[Part 2]");

    for inp1 in 0..99 {
        for inp2 in 0..99 {
            interpreter.reset_intepreter();
            interpreter.set_input_values(inp1, inp2);
            interpreter.run();
            let first_value = interpreter.get_value(0).unwrap();
            if first_value == answer {
                println!("Result: 100 * {} + {} = {}", inp1, inp2, 100 * inp1 + inp2);
                return;
            }
        }
    }
}

fn main() {
    let input_txt = include_str!("../input.txt");
    part1(&input_txt);
    part2(&input_txt, 19_690_720);
}

#[cfg(test)]
mod tests {
    use super::Interpreter;

    fn run_and_dump(input_txt: &str) -> String {
        let mut interpreter = Interpreter::new(input_txt);
        interpreter.run();
        interpreter.dump()
    }

    #[test]
    fn test_simple() {
        assert_eq!(run_and_dump("1,0,0,0,99"), "2,0,0,0,99".to_owned());
        assert_eq!(run_and_dump("2,3,0,3,99"), "2,3,0,6,99".to_owned());
        assert_eq!(run_and_dump("2,4,4,5,99,0"), "2,4,4,5,99,9801".to_owned());
        assert_eq!(
            run_and_dump("1,1,1,4,99,5,6,0,99"),
            "30,1,1,4,2,5,6,0,99".to_owned()
        );
    }
}
