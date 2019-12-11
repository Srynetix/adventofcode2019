use common::interpreter::{ExecutionState, Interpreter};

use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn rotate_right(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up
        }
    }

    pub fn rotate_left(&self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Left => Self::Down,
            Self::Down => Self::Right,
            Self::Right => Self::Up
        }
    }

    pub fn with_rotation_code(&self, code: i64) -> Self {
        match code {
            0 => self.rotate_left(),
            1 => self.rotate_right(),
            _ => panic!("wrong code")
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Black,
    White,
}

impl Color {
    pub fn to_code(&self) -> i64 {
        match self {
            Self::Black => 0,
            Self::White => 1
        }
    }

    pub fn from_code(code: i64) -> Self {
        match code {
            0 => Self::Black,
            1 => Self::White,
            _ => panic!("wrong code")
        }
    }
}

impl Direction {
    pub fn to_offset(&self) -> (i32, i32) {
        match self {
            Self::Up => (0, 1),
            Self::Down => (0, -1),
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

    pub fn get_color_position(&self) -> Color {
        self.tiles.get(&self.robot_position).copied().unwrap_or(Color::Black)
    }

    pub fn move_robot(&mut self, direction: &Direction) {
        let (ox, oy) = direction.to_offset();
        let (rx, ry) = self.robot_position;
        self.robot_position = (rx + ox, ry + oy);
    }

    pub fn run(&mut self) {
        let mut direction = Direction::Up;
        let mut color = Color::Black;

        loop {
            let (new_color, new_direction, state) = self.step(color, direction);
            match state {
                ExecutionState::Exit => break,
                _ => ()
            }

            // Paint
            self.paint_position(new_color);
            self.move_robot(&new_direction);
            direction = new_direction;
            color = self.get_color_position();
        }
    }

    pub fn step(&mut self, color: Color, direction: Direction) -> (Color, Direction, ExecutionState) {
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
    sim.run();
    sim.tiles.keys().count()
}

fn part2(input_txt: &str) -> usize {
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
    fn test_creation() {
        let input_txt = include_str!("../input.txt");
        let map = DrawSim::new(&input_txt);
        assert_eq!(map.robot_position, (0, 0));
    }

    #[test]
    fn test_results() {
        let input_txt = include_str!("../input.txt");
        assert_eq!(part1(&input_txt), 2_088);
        // assert_eq!(part2(&input_txt), 0);
    }
}
