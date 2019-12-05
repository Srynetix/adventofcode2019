use aocutils::Interpreter;

fn part1(input_txt: &str) {
    let mut interpreter = Interpreter::new(input_txt);
    println!("[Part 1]");
    interpreter.push_input(1);
    interpreter.run();
    println!("Result: {}", interpreter.pop_output().unwrap());
}

fn part2(input_txt: &str) {
    let mut interpreter = Interpreter::new(input_txt);
    println!("[Part 2]");
    interpreter.push_input(5);
    interpreter.run();
    println!("Result: {}", interpreter.pop_output().unwrap());
}

fn main() {
    let input_txt = include_str!("../input.txt");
    part1(&input_txt);
    part2(&input_txt);
}
