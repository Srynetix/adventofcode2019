pub fn base_pattern() -> &'static [i32] {
    &[0, 1, 0, -1]
}

pub fn parse_input(input: &str) -> Vec<i32> {
    input
        .chars()
        .map(|x| {
            x.to_digit(10)
                .map(|x| x as i32)
                .expect("input should be a number")
        })
        .collect()
}

pub fn pmod(a: i32, b: i32) -> i32 {
    let r = a % b;
    if r < 0 {
        -r
    } else {
        r
    }
}

pub fn prepare_pattern(pattern: &[i32], offset: usize) -> Vec<i32> {
    let mut output = vec![];
    for p in pattern {
        for _ in 0..=offset {
            output.push(*p);
        }
    }

    let x = output.remove(0);
    output.push(x);
    output
}

pub fn fft_phase(input: Vec<i32>, pattern: &[i32]) -> Vec<i32> {
    let mut output = vec![];
    let mut px: usize = 0;

    for idx in 0..input.len() {
        let pattern = prepare_pattern(pattern, idx);

        let mut out = 0;
        for inp in &input {
            let p = pattern[px];
            let v = p * inp;

            out += v;
            px = (px + 1) % pattern.len();
        }

        out = pmod(out, 10);
        output.push(out);

        // New offset
        px = 0;
    }

    output
}

pub fn fft_phases(input: Vec<i32>, pattern: &[i32], count: usize) -> String {
    let mut output = input.clone();
    for _ in 0..count {
        output = fft_phase(output, pattern);
    }

    output.iter().map(|x| x.to_string()).collect()
}

fn part1(input_txt: &str) -> String {
    fft_phases(parse_input(input_txt), base_pattern(), 100)[0..8].to_owned()
}

fn part2(_input_txt: &str) -> usize {
    0
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
    fn test_pattern() {
        assert_eq!(prepare_pattern(base_pattern(), 0), vec![1, 0, -1, 0]);
        assert_eq!(
            prepare_pattern(base_pattern(), 1),
            vec![0, 1, 1, 0, 0, -1, -1, 0]
        );
        assert_eq!(
            prepare_pattern(base_pattern(), 2),
            vec![0, 0, 1, 1, 1, 0, 0, 0, -1, -1, -1, 0]
        );
    }

    #[test]
    fn test_samples() {
        assert_eq!(
            fft_phases(parse_input("12345678"), base_pattern(), 1),
            "48226158".to_owned()
        );

        assert_eq!(
            fft_phases(parse_input("12345678"), base_pattern(), 2),
            "34040438".to_owned()
        );

        assert_eq!(
            fft_phases(parse_input("12345678"), base_pattern(), 4),
            "01029498".to_owned()
        );
    }

    #[test]
    fn test_samples_large() {
        assert_eq!(
            &fft_phases(
                parse_input("80871224585914546619083218645595"),
                base_pattern(),
                100
            )[0..8],
            "24176176".to_owned()
        );

        assert_eq!(
            &fft_phases(
                parse_input("19617804207202209144916044189917"),
                base_pattern(),
                100
            )[0..8],
            "73745418"
        );

        assert_eq!(
            &fft_phases(
                parse_input("69317163492948606335995924319873"),
                base_pattern(),
                100
            )[0..8],
            "52432133"
        );
    }

    #[test]
    fn test_results() {
        let input_txt = include_str!("../input.txt");
        assert_eq!(part1(&input_txt), "82525123".to_owned());
        // assert_eq!(part2(&input_txt), 0);
    }
}
