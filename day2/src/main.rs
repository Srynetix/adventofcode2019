use common::Interpreter;

fn part1(input_txt: &str) {
    let mut interpreter = Interpreter::new(input_txt);
    println!("[Part 1]");
    interpreter.restore_alarm_state();
    interpreter.run();
    println!("Result: {}", interpreter.get_value(0).unwrap());
}

fn part2(input_txt: &str, answer: i32) {
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
