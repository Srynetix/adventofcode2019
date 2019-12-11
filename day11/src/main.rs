use std::collections::HashMap;

use common::interpreter::{ExecutionState, Interpreter};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn rotate_right(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    pub fn rotate_left(self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Left => Self::Down,
            Self::Down => Self::Right,
            Self::Right => Self::Up,
        }
    }

    pub fn with_rotation_code(self, code: i64) -> Self {
        match code {
            0 => self.rotate_left(),
            1 => self.rotate_right(),
            _ => panic!("wrong code"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Black,
    White,
}

impl Color {
    pub fn to_code(self) -> i64 {
        match self {
            Self::Black => 0,
            Self::White => 1,
        }
    }

    pub fn from_code(code: i64) -> Self {
        match code {
            0 => Self::Black,
            1 => Self::White,
            _ => panic!("wrong code"),
        }
    }
}

impl Direction {
    pub fn to_offset(self) -> (i32, i32) {
        match self {
            Self::Up => (0, -1),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
            Self::Right => (1, 0),
        }
    }
}

#[derive(Debug)]
pub struct DrawSim {
    tiles: HashMap<(i32, i32), Color>,
    robot_position: (i32, i32),
    interpreter: Interpreter,
}

impl DrawSim {
    pub fn new(code: &str) -> Self {
        let robot_position = (0, 0);
        let interpreter = Interpreter::new(code);
        let tiles = HashMap::new();

        Self {
            robot_position,
            tiles,
            interpreter,
        }
    }

    pub fn paint_position(&mut self, color: Color) {
        self.tiles.insert(self.robot_position, color);
    }

    pub fn get_color_at_robot(&self) -> Color {
        self.get_color_at_position(self.robot_position)
    }

    pub fn get_color_at_position(&self, pos: (i32, i32)) -> Color {
        self.tiles.get(&pos).copied().unwrap_or(Color::Black)
    }

    pub fn move_robot(&mut self, direction: Direction) {
        let (ox, oy) = direction.to_offset();
        let (rx, ry) = self.robot_position;
        self.robot_position = (rx + ox, ry + oy);
    }

    pub fn run(&mut self, base_color: Color) {
        let mut direction = Direction::Up;
        let mut color = base_color;

        loop {
            let (new_color, new_direction, state) = self.step(color, direction);
            if let ExecutionState::Exit = state {
                break;
            }

            // Paint
            self.paint_position(new_color);
            self.move_robot(new_direction);
            direction = new_direction;
            color = self.get_color_at_robot();
        }
    }

    pub fn get_rect(&self) -> (i32, i32, i32, i32) {
        let mut x_min = i32::max_value();
        let mut x_max = i32::min_value();
        let mut y_min = i32::max_value();
        let mut y_max = i32::min_value();

        for (x, y) in self.tiles.keys().copied() {
            if x < x_min {
                x_min = x;
            }
            if x > x_max {
                x_max = x;
            }
            if y < y_min {
                y_min = y;
            }
            if y > y_max {
                y_max = y;
            }
        }

        (x_min, y_min, x_max - x_min, y_max - y_min + 1)
    }

    pub fn draw(&self) -> String {
        let mut string = String::new();
        let (ox, oy, w, h) = self.get_rect();

        for y in oy..oy + h {
            for x in ox..ox + w {
                let color = self.get_color_at_position((x, y));
                if color == Color::Black {
                    string.push_str("  ");
                } else {
                    string.push_str("██");
                }
            }

            if y != oy + h - 1 {
                string.push('\n');
            }
        }

        string
    }

    pub fn step(
        &mut self,
        color: Color,
        direction: Direction,
    ) -> (Color, Direction, ExecutionState) {
        self.interpreter.push_input(color.to_code());

        loop {
            let (_, state) = self.interpreter.step();
            match state {
                ExecutionState::Wait => {
                    return (
                        Color::from_code(self.interpreter.pop_output().unwrap()),
                        direction.with_rotation_code(self.interpreter.pop_output().unwrap()),
                        state,
                    );
                }
                ExecutionState::Exit => {
                    return (Color::Black, Direction::Up, state);
                }
                _ => (),
            }
        }
    }
}

fn part1(input_txt: &str) -> usize {
    let mut sim = DrawSim::new(input_txt);
    sim.run(Color::Black);
    sim.tiles.keys().count()
}

fn part2(input_txt: &str) -> String {
    let mut sim = DrawSim::new(input_txt);
    sim.run(Color::White);
    sim.draw()
}

fn main() {
    let input_txt = include_str!("../input.txt");

    println!("[Part 1]");
    let r = part1(&input_txt);
    println!("Result: {}", r);

    println!("[Part 2]");
    let r = part2(&input_txt);
    println!("Result:\n{}", r);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let input_txt = include_str!("../input.txt");
        let map = DrawSim::new(&input_txt);
        assert_eq!(map.robot_position, (0, 0));
    }

    #[test]
    fn test_results() {
        let input_txt = include_str!("../input.txt");
        let part2_result = include_str!("../part2_result.txt");
        assert_eq!(part1(&input_txt), 2_088);
        assert_eq!(part2(&input_txt), part2_result);
    }
}
