use common::interpreter::{ExecutionState, Interpreter};
use itertools::Itertools;

#[derive(Debug, Default)]
pub struct AmplifierSystem;

impl AmplifierSystem {
    pub fn new() -> Self {
        Self::default()
    }

    /// Run interpreter for amp phase.
    /// Output the interpreter output value
    pub fn run_phase(&self, interpreter: &mut Interpreter, phase: i64, input: i64) -> i64 {
        // Reset interpreter state
        interpreter.reset_intepreter();

        // Push phase input
        interpreter.push_input(phase);
        // Push amp input
        interpreter.push_input(input);

        // Run interpreter
        interpreter.run();

        // Get output
        interpreter.pop_output().unwrap()
    }

    /// Run interpreter for phase sequence
    pub fn run_phase_sequence(&self, interpreter: &mut Interpreter, phase_sequence: &str) -> i64 {
        let seq: Vec<i64> = phase_sequence
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();
        let mut output = 0;

        for i in seq {
            output = self.run_phase(interpreter, i, output);
        }

        output
    }

    /// Run interpreter for feedback phase sequence
    pub fn run_feedback_phase_sequence(
        &self,
        interpreter: &mut Interpreter,
        phase_sequence: &str,
    ) -> i64 {
        let mut seq: Vec<i64> = phase_sequence
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();
        let mut interpreters: Vec<_> = (0..5).map(|_| interpreter.clone()).collect();

        // Initialization
        for interp in interpreters.iter_mut() {
            interp.reset_intepreter();
            interp.push_input(seq.remove(0));
        }

        // Last output
        let mut last_output = 0;

        // Run
        'outer: loop {
            for index in 0..5 {
                // Run interpreter
                {
                    let interp = interpreters.get_mut(index).unwrap();
                    interp.push_input(last_output);

                    'inner: loop {
                        let (_, state) = interp.step();
                        match state {
                            ExecutionState::Wait => {
                                last_output = interp.pop_output().unwrap();
                                break 'inner;
                            }
                            ExecutionState::Exit => {
                                last_output = interp.pop_output().unwrap();
                                // Last index?
                                if index == 4 {
                                    break 'outer;
                                } else {
                                    break 'inner;
                                }
                            }
                            ExecutionState::Next => (),
                        }
                    }
                }
            }
        }

        // Pop last output
        last_output
    }

    /// Find max thruster signal
    pub fn find_max_thruster_signal(&self, interpreter: &mut Interpreter) -> (i64, String) {
        let permutations: Vec<_> = (0..5).permutations(5).collect();
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

    pub fn find_max_feedback_thruster_signal(
        &self,
        interpreter: &mut Interpreter,
    ) -> (i64, String) {
        let permutations: Vec<_> = (5..10).permutations(5).collect();
        let mut max_value = 0;
        let mut max_permutation = String::new();

        for permutation in permutations {
            let phase_sequence: String = permutation.iter().map(|x| x.to_string()).join(",");
            let output = self.run_feedback_phase_sequence(interpreter, &phase_sequence);
            if output > max_value {
                max_value = output;
                max_permutation = phase_sequence;
            }
        }

        (max_value, max_permutation)
    }
}

fn part1(input_txt: &str) -> i64 {
    let mut interpreter = Interpreter::new(input_txt);
    let system = AmplifierSystem::new();
    let (result, _) = system.find_max_thruster_signal(&mut interpreter);
    result
}

fn part2(input_txt: &str) -> i64 {
    let mut interpreter = Interpreter::new(input_txt);
    let system = AmplifierSystem::new();
    let (result, _) = system.find_max_feedback_thruster_signal(&mut interpreter);
    result
}

fn main() {
    let input_txt = include_str!("../input.txt");

    println!("[Part 1]");
    let r = part1(&input_txt);
    println!("Result: {}", r);

    println!("[Part 2]");
    let r = part2(&input_txt);
    println!("Result: {}", r);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_amplifiers() {
        fn run_with_code(input_txt: &str) -> (i64, String) {
            let mut interpreter = Interpreter::new(input_txt);
            let system = AmplifierSystem::new();
            system.find_max_thruster_signal(&mut interpreter)
        }

        fn run_phase_sequence(input_txt: &str, seq: &str) -> i64 {
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

    #[test]
    fn test_feedback_amplifiers() {
        fn run_with_code(input_txt: &str) -> (i64, String) {
            let mut interpreter = Interpreter::new(input_txt);
            let system = AmplifierSystem::new();
            system.find_max_feedback_thruster_signal(&mut interpreter)
        }

        fn run_feedback_phase_sequence(input_txt: &str, seq: &str) -> i64 {
            let mut interpreter = Interpreter::new(input_txt);
            let system = AmplifierSystem::new();
            system.run_feedback_phase_sequence(&mut interpreter, seq)
        }

        assert_eq!(
            run_feedback_phase_sequence(
                "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,\
                 27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5",
                "9,8,7,6,5"
            ),
            139629729
        );
        assert_eq!(
            run_with_code(
                "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,\
                 27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"
            ),
            (139629729, "9,8,7,6,5".to_owned())
        );

        assert_eq!(
            run_with_code(
                "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,\
                 -5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,\
                 53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10"
            ),
            (18216, "9,7,8,5,6".to_owned())
        );
    }

    #[test]
    fn test_results() {
        let input_txt = include_str!("../input.txt");
        assert_eq!(part1(&input_txt), 437860);
        assert_eq!(part2(&input_txt), 49810599);
    }
}
