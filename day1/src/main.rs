//! Day 1

/// Calculate fuel
fn calculate_fuel(mass: i32) -> i32 {
    ((mass as f32 / 3.0).floor() - 2.0) as i32
}

/// Calculate fuel recursive
fn calculate_fuel_recursive(mass: i32) -> i32 {
    let mut sum = 0;
    let mut mass = mass;

    loop {
        let fuel = calculate_fuel(mass);
        if fuel < 0 {
            break;
        }

        sum += fuel;
        mass = fuel;
    }

    sum
}

fn part1(input_txt: &str) {
    let mut sum = 0;

    for line in input_txt.split('\n') {
        let mass: i32 = line.parse().unwrap();
        sum += calculate_fuel(mass);
    }

    println!("[Part 1]");
    println!("{}", sum);
}

fn part2(input_txt: &str) {
    let mut sum = 0;

    for line in input_txt.split('\n') {
        let mass: i32 = line.parse().unwrap();
        sum += calculate_fuel_recursive(mass);
    }

    println!("[Part 2]");
    println!("{}", sum);
}

fn main() {
    let input_txt = include_str!("../input.txt");
    part1(&input_txt);
    part2(&input_txt);
}

#[cfg(test)]
mod tests {
    use super::{calculate_fuel, calculate_fuel_recursive};

    #[test]
    fn test_part1() {
        assert_eq!(calculate_fuel(12), 2);
        assert_eq!(calculate_fuel(14), 2);
        assert_eq!(calculate_fuel(1969), 654);
        assert_eq!(calculate_fuel(100756), 33583);
    }

    #[test]
    fn test_part2() {
        assert_eq!(calculate_fuel_recursive(14), 2);
        assert_eq!(calculate_fuel_recursive(1969), 966);
        assert_eq!(calculate_fuel_recursive(100756), 50346);
    }
}
