//! Day 1

fn part1(input_txt: &str) {
    let mut sum = 0;

    for line in input_txt.split('\n') {
        let mass: f32 = line.parse().unwrap();
        let fuel = (mass / 3.0).floor() - 2.0;
        sum += fuel as u32;
    }

    println!("[Part 1]");
    println!("{}", sum);
}

fn part2(input_txt: &str) {
    let mut sum = 0;

    for line in input_txt.split('\n') {
        let mut mass: f32 = line.parse().unwrap();

        loop {
            let fuel = (mass / 3.0).floor() - 2.0;
            if fuel < 0.0 {
                break;
            }

            sum += fuel as i32;
            mass = fuel;
        }
    }

    println!("[Part 2]");
    println!("{}", sum);
}

fn main() {
    let input_txt = include_str!("../input.txt");
    part1(&input_txt);
    part2(&input_txt);
}
