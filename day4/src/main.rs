/// Check if input is in range
fn check_range(input: u32, min_range: u32, max_range: u32) -> bool {
    input >= min_range && input <= max_range
}

/// Check if input has same adjacents digits and that it never decreases
fn check_digits(input: u32) -> bool {
    let mut inp = input;
    let mut last_digit = None;
    let mut same_digits = false;

    while inp > 0 {
        let digit = (inp % 10) as i32;
        if let Some(l) = last_digit {
            // Same digit? Can be valid.
            if digit == l {
                same_digits = true;
            }

            // Next digit is bigger than last? Invalid.
            if digit > l {
                return false;
            }
        }

        last_digit = Some(digit);
        inp /= 10;
    }

    same_digits
}

/// Check if input has same adjacents digits (but not more than 2)
/// and that it never decreases
fn check_digits_non_repeated(input: u32) -> bool {
    let mut inp = input;
    let mut last_digit = None;
    let mut last_repeated_count = 0;
    let mut same_digits = false;

    while inp > 0 {
        let digit = (inp % 10) as i32;
        if let Some(l) = last_digit {
            // Same digit? Can be valid.
            if digit == l {
                last_repeated_count += 1;
            } else {
                if last_repeated_count == 1 {
                    same_digits = true;
                }

                last_repeated_count = 0;
            }

            // Next digit is bigger than last? Invalid.
            if digit > l {
                return false;
            }
        }

        last_digit = Some(digit);
        inp /= 10;
    }

    // Handle last digit
    if last_repeated_count == 1 {
        same_digits = true;
    }

    same_digits
}

/// Check if an input is valid
fn check_valid_input(input: u32, min_range: u32, max_range: u32) -> bool {
    check_range(input, min_range, max_range) && check_digits(input)
}

/// Check if an input is valid (non-repeated)
fn check_valid_input_non_repeated(input: u32, min_range: u32, max_range: u32) -> bool {
    check_range(input, min_range, max_range) && check_digits_non_repeated(input)
}

/// Count valid passwords in range
fn count_valid_passwords(min_range: u32, max_range: u32) -> u32 {
    let mut count = 0;
    for x in min_range..=max_range {
        if check_valid_input(x, min_range, max_range) {
            count += 1;
        }
    }

    count
}

/// Count valid passwords in range, non-repeated
fn count_valid_passwords_non_repeated(min_range: u32, max_range: u32) -> u32 {
    let mut count = 0;
    for x in min_range..=max_range {
        if check_valid_input_non_repeated(x, min_range, max_range) {
            count += 1;
        }
    }

    count
}

fn part1(input_txt: &str) -> u32 {
    let entries: Vec<u32> = input_txt.split('-').map(|x| x.parse().unwrap()).collect();
    count_valid_passwords(entries[0], entries[1])
}

fn part2(input_txt: &str) -> u32 {
    let entries: Vec<u32> = input_txt.split('-').map(|x| x.parse().unwrap()).collect();
    count_valid_passwords_non_repeated(entries[0], entries[1])
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
    fn test_valid() {
        assert!(check_valid_input(111_111, 100_000, 999_999));
        assert!(check_valid_input(123_345, 100_000, 999_999));
        assert!(!check_valid_input(223_450, 100_000, 999_999));
        assert!(!check_valid_input(123_789, 100_000, 999_999));
    }

    #[test]
    fn test_valid_non_repeated() {
        assert!(!check_valid_input_non_repeated(111_111, 100_000, 999_999));
        assert!(check_valid_input_non_repeated(112_345, 100_000, 999_999));
        assert!(check_valid_input_non_repeated(123_345, 100_000, 999_999));
        assert!(!check_valid_input_non_repeated(223_450, 100_000, 999_999));
        assert!(!check_valid_input_non_repeated(123_789, 100_000, 999_999));
        assert!(check_valid_input_non_repeated(112_233, 100_000, 999_999));
        assert!(!check_valid_input_non_repeated(123_444, 100_000, 999_999));
        assert!(check_valid_input_non_repeated(111_122, 100_000, 999_999));
        assert!(check_valid_input_non_repeated(667_899, 100_000, 999_999));
        assert!(!check_valid_input_non_repeated(124_444, 100_000, 999_999));
    }

    #[test]
    fn test_password_count() {
        assert_eq!(count_valid_passwords(100_000, 100_010), 0);
        assert_eq!(count_valid_passwords(100_000, 111_111), 1);
        assert_eq!(count_valid_passwords(100_000, 111_112), 2);
    }

    #[test]
    fn test_password_count_non_repeated() {
        assert_eq!(count_valid_passwords_non_repeated(100_000, 100_010), 0);
        assert_eq!(count_valid_passwords_non_repeated(100_000, 111_122), 1);
        assert_eq!(count_valid_passwords_non_repeated(100_000, 111_133), 2);
        assert_eq!(count_valid_passwords_non_repeated(100_000, 111_144), 3);
        assert_eq!(count_valid_passwords_non_repeated(100_000, 111_155), 4);
        assert_eq!(count_valid_passwords_non_repeated(100_000, 111_166), 5);
        assert_eq!(count_valid_passwords_non_repeated(100_000, 111_177), 6);
        assert_eq!(count_valid_passwords_non_repeated(100_000, 111_223), 9);
    }

    #[test]
    fn test_results() {
        let input_txt = include_str!("../input.txt");
        assert_eq!(part1(&input_txt), 1169);
        assert_eq!(part2(&input_txt), 757);
    }
}
