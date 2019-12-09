use common::Interpreter;

fn part1(input_txt: &str) -> i64 {
    let mut interpreter = Interpreter::new(input_txt);
    interpreter.push_input(1);
    interpreter.run();
    interpreter.get_output_stream().last().copied().unwrap()
}

fn part2(input_txt: &str) -> i64 {
    let mut interpreter = Interpreter::new(input_txt);
    interpreter.push_input(5);
    interpreter.run();
    interpreter.pop_output().unwrap()
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
    fn test_results() {
        let input_txt = include_str!("../input.txt");
        assert_eq!(part1(&input_txt), 12428642);
        assert_eq!(part2(&input_txt), 918655);
    }
}
