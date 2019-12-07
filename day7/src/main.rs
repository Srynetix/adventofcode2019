use common::Interpreter;
use itertools::Itertools;

#[derive(Debug, Default)]
pub struct AmplifierSystem;

impl AmplifierSystem {
    pub fn new() -> Self {
        Self::default()
    }

    /// Run interpreter for amp phase.
    /// Output the interpreter output value
    pub fn run_phase(&self, interpreter: &mut Interpreter, phase: i32, input: i32) -> i32 {
        // Reset interpreter state
        interpreter.reset_intepreter();

        // Push phase input
        interpreter.push_input(phase);
        // Push previous amp input
        interpreter.push_input(input);

        // Run interpreter
        let _ = interpreter.run();

        // Get output
        interpreter.pop_output().unwrap()
    }

    /// Run interpreter for phase sequence
    pub fn run_phase_sequence(&self, interpreter: &mut Interpreter, phase_sequence: &str) -> i32 {
        let seq: Vec<i32> = phase_sequence
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();
        let mut output = 0;

        for i in seq {
            output = self.run_phase(interpreter, i, output);
        }

        output
    }

    /// Find max thruster signal
    pub fn find_max_thruster_signal(
        &self,
        interpreter: &mut Interpreter,
        amp_count: usize,
    ) -> (i32, String) {
        let permutations: Vec<_> = (0..amp_count).permutations(amp_count).collect();
        let mut max_value = 0;
        let mut max_permutation = String::new();

        for permutation in permutations {
            let phase_sequence: String = permutation.iter().map(|x| x.to_string()).join(",");
            let output = self.run_phase_sequence(interpreter, &phase_sequence);
            if output > max_value {
                max_value = output;
                max_permutation = phase_sequence;
            }
        }

        (max_value, max_permutation)
    }
}

fn part1(input_txt: &str) {
    println!("[Part 1]");
    let mut interpreter = Interpreter::new(input_txt);
    let system = AmplifierSystem::new();
    let (result, _) = system.find_max_thruster_signal(&mut interpreter, 5);
    println!("Result: {}", result);
}

fn main() {
    let input_txt = include_str!("../input.txt");
    part1(&input_txt);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_amplifiers() {
        fn run_with_code(input_txt: &str) -> (i32, String) {
            let mut interpreter = Interpreter::new(input_txt);
            let system = AmplifierSystem::new();
            system.find_max_thruster_signal(&mut interpreter, 5)
        }

        fn run_phase_sequence(input_txt: &str, seq: &str) -> i32 {
            let mut interpreter = Interpreter::new(input_txt);
            let system = AmplifierSystem::new();
            system.run_phase_sequence(&mut interpreter, seq)
        }

        assert_eq!(
            run_phase_sequence(
                "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0",
                "4,3,2,1,0"
            ),
            43210
        );

        assert_eq!(
            run_with_code("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"),
            (43210, "4,3,2,1,0".to_owned())
        );

        assert_eq!(
            run_with_code(
                "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,\
                 23,4,23,99,0,0"
            ),
            (54321, "0,1,2,3,4".to_owned())
        );

        assert_eq!(
            run_with_code(
                "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,\
                 7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"
            ),
            (65210, "1,0,4,3,2".to_owned())
        );
    }
}
