use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct AsteroidMap {
    data: Vec<char>,
    asteroid_locations: Vec<(usize, usize)>,
    width: usize,
    height: usize,
}

impl AsteroidMap {
    pub fn from_input(input_txt: &str) -> Self {
        let lines: Vec<_> = input_txt.split('\n').collect();
        let width = lines[0].chars().count();
        let height = lines.len();
        let mut asteroid_locations = vec![];
        let mut data = vec![];

        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    asteroid_locations.push((x, y));
                }

                data.push(c);
            }
        }

        Self {
            data,
            width,
            height,
            asteroid_locations,
        }
    }

    pub fn get_char(&self, x: usize, y: usize) -> char {
        if x >= self.width || y >= self.height {
            ' '
        } else {
            self.data[x + y * self.width]
        }
    }

    /// Scan asteroids at point
    pub fn scan_point(&self, x: usize, y: usize) -> usize {
        if self.get_char(x, y) == '.' {
            return 0;
        }

        self.asteroid_locations
            .iter()
            .filter(|(ax, ay)| *ax != x || *ay != y)
            .map(|(ax, ay)| {
                let x = x as f32;
                let y = y as f32;
                let ax = *ax as f32;
                let ay = *ay as f32;
                (-(x - ax).atan2(y - ay) * 1000.0) as i32
            })
            .unique()
            .count()
    }

    pub fn better_position(&self) -> ((usize, usize), usize) {
        let mut better_pos = (0, 0);
        let mut better_count = 0;

        for (ax, ay) in &self.asteroid_locations {
            let count = self.scan_point(*ax, *ay);
            if count > better_count {
                better_count = count;
                better_pos = (*ax, *ay);
            }
        }

        (better_pos, better_count)
    }

    pub fn dump_scan(&self) -> String {
        let mut output = String::new();

        for y in 0..self.height {
            for x in 0..self.width {
                if self.get_char(x, y) == '.' {
                    output.push('.');
                } else {
                    output.push_str(&self.scan_point(x, y).to_string());
                }
            }

            if y != self.height - 1 {
                output.push('\n');
            }
        }

        output
    }

    pub fn dump(&self) -> String {
        let mut output = String::new();

        for y in 0..self.height {
            for x in 0..self.width {
                output.push(self.data[x + y * self.width]);
            }

            if y != self.height - 1 {
                output.push('\n');
            }
        }

        output
    }
}

fn part1(input_txt: &str) -> usize {
    let map = AsteroidMap::from_input(input_txt);
    let (_, count) = map.better_position();
    count
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
    fn test_small() {
        let map = ".#..#\n\
                   .....\n\
                   #####\n\
                   ....#\n\
                   ...##";

        let ast_map = AsteroidMap::from_input(map);
        assert_eq!(ast_map.dump(), map);

        assert_eq!(ast_map.scan_point(0, 0), 0);
        assert_eq!(ast_map.scan_point(1, 0), 7);
        assert_eq!(ast_map.scan_point(4, 2), 5);

        assert_eq!(
            ast_map.dump_scan(),
            ".7..7\n\
             .....\n\
             67775\n\
             ....7\n\
             ...87"
        );

        assert_eq!(ast_map.better_position(), ((3, 4), 8));
    }

    #[test]
    fn test_medium() {
        assert_eq!(
            AsteroidMap::from_input(
                "......#.#.\n\
                 #..#.#....\n\
                 ..#######.\n\
                 .#.#.###..\n\
                 .#..#.....\n\
                 ..#....#.#\n\
                 #..#....#.\n\
                 .##.#..###\n\
                 ##...#..#.\n\
                 .#....####"
            )
            .better_position(),
            ((5, 8), 33)
        );

        assert_eq!(
            AsteroidMap::from_input(
                "#.#...#.#.\n\
                 .###....#.\n\
                 .#....#...\n\
                 ##.#.#.#.#\n\
                 ....#.#.#.\n\
                 .##..###.#\n\
                 ..#...##..\n\
                 ..##....##\n\
                 ......#...\n\
                 .####.###."
            )
            .better_position(),
            ((1, 2), 35)
        );

        assert_eq!(
            AsteroidMap::from_input(
                ".#..#..###\n\
                 ####.###.#\n\
                 ....###.#.\n\
                 ..###.##.#\n\
                 ##.##.#.#.\n\
                 ....###..#\n\
                 ..#.#..#.#\n\
                 #..#.#.###\n\
                 .##...##.#\n\
                 .....#.#.."
            )
            .better_position(),
            ((6, 3), 41)
        );
    }

    #[test]
    fn test_big() {
        assert_eq!(
            AsteroidMap::from_input(
                ".#..##.###...#######\n\
                 ##.############..##.\n\
                 .#.######.########.#\n\
                 .###.#######.####.#.\n\
                 #####.##.#.##.###.##\n\
                 ..#####..#.#########\n\
                 ####################\n\
                 #.####....###.#.#.##\n\
                 ##.#################\n\
                 #####.##.###..####..\n\
                 ..######..##.#######\n\
                 ####.##.####...##..#\n\
                 .#####..#.######.###\n\
                 ##...#.##########...\n\
                 #.##########.#######\n\
                 .####.#.###.###.#.##\n\
                 ....##.##.###..#####\n\
                 .#.#.###########.###\n\
                 #.#.#.#####.####.###\n\
                 ###.##.####.##.#..##"
            )
            .better_position(),
            ((11, 13), 210)
        );
    }

    #[test]
    fn test_results() {
        let input_txt = include_str!("../input.txt");
        assert_eq!(part1(&input_txt), 329);
        // assert_eq!(part2(&input_txt), 76_642);
    }
}
