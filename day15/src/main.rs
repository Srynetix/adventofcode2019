use common::interpreter::{ExecutionState, Interpreter};
use std::collections::{HashMap, HashSet};

type Vector2D = euclid::default::Vector2D<i32>;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Tile {
    Wall,
    Empty,
    Oxygen,
    Way,
}

impl Tile {
    pub fn from_code(code: i64) -> Self {
        match code {
            0 => Self::Wall,
            1 => Self::Empty,
            2 => Self::Oxygen,
            9 => Self::Way,
            _ => panic!("unknown tile code: {}", code),
        }
    }

    pub fn to_code(self) -> i64 {
        match self {
            Self::Wall => 0,
            Self::Empty => 1,
            Self::Oxygen => 2,
            Self::Way => 9,
        }
    }

    pub fn to_ascii(self) -> String {
        let text = match self {
            Self::Wall => "█",
            Self::Empty => ".",
            Self::Oxygen => "O",
            Self::Way => "@",
        };

        text.to_owned()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn from_code(code: i64) -> Self {
        match code {
            1 => Self::North,
            2 => Self::South,
            3 => Self::West,
            4 => Self::East,
            _ => panic!("unknown direction code: {}", code),
        }
    }

    pub fn list() -> Vec<Self> {
        vec![Self::East, Self::West, Self::North, Self::South]
    }

    pub fn to_offset(self) -> Vector2D {
        match self {
            Self::North => Vector2D::new(0, 1),
            Self::South => Vector2D::new(0, -1),
            Self::East => Vector2D::new(1, 0),
            Self::West => Vector2D::new(-1, 0),
        }
    }

    pub fn invert(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East,
        }
    }

    pub fn to_code(self) -> i64 {
        match self {
            Self::North => 1,
            Self::South => 2,
            Self::West => 3,
            Self::East => 4,
        }
    }
}

pub struct Simulation {
    code: String,
}

impl Simulation {
    pub fn from_input(code: &str) -> Self {
        Self {
            code: code.to_owned(),
        }
    }

    pub fn get_rect(&self, tiles: &HashMap<Vector2D, Tile>) -> (Vector2D, Vector2D) {
        let mut top_left = Vector2D::new(i32::max_value(), i32::max_value());
        let mut bottom_right = Vector2D::new(i32::min_value(), i32::min_value());

        for coord in tiles.keys() {
            if coord.x < top_left.x {
                top_left.x = coord.x;
            }
            if coord.x > bottom_right.x {
                bottom_right.x = coord.x;
            }

            if coord.y < top_left.y {
                top_left.y = coord.y;
            }
            if coord.y > bottom_right.y {
                bottom_right.y = coord.y;
            }
        }

        (top_left, bottom_right + Vector2D::new(1, 1))
    }

    pub fn show_map(&self, tiles: &HashMap<Vector2D, Tile>, position: Vector2D) {
        let (top_left, bottom_right) = self.get_rect(tiles);
        let mut screen = String::new();

        for y in top_left.y..bottom_right.y {
            for x in top_left.x..bottom_right.x {
                if position.x == x && position.y == y {
                    screen.push('x');
                } else {
                    screen.push_str(
                        &tiles
                            .get(&Vector2D::new(x, y))
                            .copied()
                            .unwrap_or_else(|| Tile::Empty)
                            .to_ascii(),
                    );
                }
            }

            screen.push('\n');
        }

        std::thread::sleep(std::time::Duration::from_millis(50));
        // Shakes a little but it does the job
        print!("{}[2J", 27 as char);
        println!("{}", screen);
    }

    pub fn run(
        &mut self,
        stop_at_oxygen: bool,
        debug: bool,
    ) -> (Vec<Direction>, HashMap<Vector2D, Tile>) {
        let mut path = vec![];
        let mut oxygen_path = vec![];
        let mut to_visit = vec![];
        let mut visited = HashSet::new();
        let mut tiles = HashMap::new();
        let mut interpreter = Interpreter::new(&self.code);
        let mut position = Vector2D::new(0, 0);
        let mut backtrack: Option<Direction> = None;

        // Initial position
        visited.insert(position.clone());
        to_visit.push(Direction::North);
        tiles.insert(position.clone(), Tile::Empty);

        'simulation: loop {
            // Let's go !
            let direction = if let Some(mvmt) = backtrack {
                mvmt.invert()
            } else if let Some(mvmt) = to_visit.pop() {
                mvmt
            } else {
                break 'simulation;
            };

            // Move!
            if backtrack.is_none() {
                path.push(direction);
            } else {
                backtrack = None;
            }

            position += direction.to_offset();
            visited.insert(position);
            interpreter.push_input(direction.to_code());

            'internal: loop {
                let (_, state) = interpreter.step();
                match state {
                    ExecutionState::Exit => {
                        break 'simulation;
                    }
                    ExecutionState::Wait => {
                        break 'internal;
                    }
                    _ => (),
                }
            }

            ///////////////////////////////////////////////
            // Waiting for input / Handling previous output

            // Movement
            let output_tile = interpreter
                .pop_output()
                .map(Tile::from_code)
                .expect("empty interpreter output");
            tiles.insert(position.clone(), output_tile);
            if debug {
                self.show_map(&tiles, position + direction.invert().to_offset());
            }

            match output_tile {
                Tile::Wall => {
                    // Cannot pass, move back!
                    let dir = path.pop().unwrap();
                    position += dir.invert().to_offset();
                }
                Tile::Oxygen => {
                    // Ok!
                    oxygen_path = path.clone();
                    if stop_at_oxygen {
                        println!("Oxygen found");
                        break;
                    }
                }
                _ => (),
            }

            // Check for other movements
            for dir in Direction::list() {
                let tgt = position + dir.to_offset();
                if !visited.contains(&tgt) {
                    to_visit.push(dir);
                    break;
                }
            }

            if to_visit.is_empty() {
                // Go back
                backtrack = path.pop();
            }
        }

        let mut show_map_tiles = tiles.clone();
        let mut path_pos = Vector2D::new(0, 0);
        // Compute path
        for p in &oxygen_path {
            show_map_tiles.insert(path_pos.clone(), Tile::Way);
            path_pos += p.to_offset();
        }

        show_map_tiles.insert(Vector2D::new(0, 0), Tile::Oxygen);
        self.show_map(&show_map_tiles, position);

        (oxygen_path, tiles)
    }

    pub fn fill_oxygen(&self, tiles: &mut HashMap<Vector2D, Tile>) -> usize {
        let mut oxygen_points: Vec<Vector2D> = vec![];
        let mut remaining_tiles: Vec<Vector2D> = vec![];

        for (pos, tile) in tiles.iter() {
            if *tile == Tile::Oxygen {
                oxygen_points.push(pos.clone());
            } else if *tile == Tile::Empty {
                remaining_tiles.push(pos.clone());
            }
        }

        let mut time = 0;
        while !remaining_tiles.is_empty() {
            let mut next_oxygen_points = vec![];
            while !oxygen_points.is_empty() {
                let point = oxygen_points.remove(0);
                for dir in Direction::list() {
                    let tgt = point + dir.to_offset();
                    if *tiles.get(&tgt).expect("tile should exist") == Tile::Empty {
                        // Remove remaining tile
                        tiles.insert(tgt, Tile::Oxygen);
                        next_oxygen_points.push(tgt.clone());

                        if let Some(x) = remaining_tiles.iter().position(|&x| x == tgt) {
                            remaining_tiles.remove(x);
                        }
                    }
                }
            }

            oxygen_points = next_oxygen_points;
            time += 1;
        }

        time
    }
}

fn part1(input_txt: &str) -> usize {
    let mut sim = Simulation::from_input(input_txt);
    let (path, _) = sim.run(true, false);
    path.len()
}

fn part2(input_txt: &str) -> usize {
    let mut sim = Simulation::from_input(input_txt);
    let (_, mut tiles) = sim.run(false, false);
    sim.fill_oxygen(&mut tiles)
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
        assert_eq!(part1(&input_txt), 224);
        assert_eq!(part2(&input_txt), 284);
    }
}
