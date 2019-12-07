use common::Interpreter;

fn part1(input_txt: &str) -> i32 {
    let mut interpreter = Interpreter::new(input_txt);
    interpreter.restore_alarm_state();
    interpreter.run();
    interpreter.get_value(0).unwrap()
}

fn part2(input_txt: &str, answer: i32) -> i32 {
    let mut interpreter = Interpreter::new(input_txt);

    for inp1 in 0..99 {
        for inp2 in 0..99 {
            interpreter.reset_intepreter();
            interpreter.set_input_values(inp1, inp2);
            interpreter.run();
            let first_value = interpreter.get_value(0).unwrap();
            if first_value == answer {
                return 100 * inp1 + inp2;
            }
        }
    }

    0
}

fn main() {
    let input_txt = include_str!("../input.txt");

    println!("[Part 1]");
    let r = part1(&input_txt);
    println!("Result: {}", r);

    println!("[Part 2]");
    let r = part2(&input_txt, 19_690_720);
    println!("Result: {}", r);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_results() {
        let input_txt = include_str!("../input.txt");
        assert_eq!(part1(&input_txt), 3765464);
        assert_eq!(part2(&input_txt, 19_690_720), 7610);
    }
}
