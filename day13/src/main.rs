use std::collections::HashMap;
use std::env;
use std::io::{stdout, Write};
use std::time::Instant;

use colored::Colorize;
use crossterm::{cursor, style, terminal, ExecutableCommand, QueueableCommand};

use common::interpreter::{ExecutionState, Interpreter};

pub type Vector2D = euclid::default::Vector2D<i32>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Empty,
    Wall,
    Block,
    HorizontalPaddle,
    Ball,
}

impl Tile {
    pub fn from_tile_id(tile_id: i32) -> Self {
        match tile_id {
            0 => Self::Empty,
            1 => Self::Wall,
            2 => Self::Block,
            3 => Self::HorizontalPaddle,
            4 => Self::Ball,
            _ => panic!("wrong tile id: {}", tile_id),
        }
    }

    pub fn to_tile_id(self) -> i32 {
        match self {
            Tile::Empty => 0,
            Tile::Wall => 1,
            Tile::Block => 2,
            Tile::HorizontalPaddle => 3,
            Tile::Ball => 4,
        }
    }

    pub fn to_ascii(self) -> String {
        match self {
            Tile::Empty => " ".to_owned(),
            Tile::Wall => "█".magenta().to_string(),
            Tile::Block => "█".green().to_string(),
            Tile::HorizontalPaddle => "█".blue().to_string(),
            Tile::Ball => "█".yellow().to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JoystickMovement {
    Neutral,
    Left,
    Right,
}

impl JoystickMovement {
    pub fn to_code(self) -> i64 {
        match self {
            Self::Neutral => 0,
            Self::Left => -1,
            Self::Right => 1,
        }
    }
}

#[derive(Debug, Default)]
pub struct Game {
    tiles: HashMap<Vector2D, Tile>,
    score: i32,
}

impl Game {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn play(&mut self, code: &str, with_ui: bool) -> i32 {
        let mut interpreter = Interpreter::new(code);
        // Play for free!
        interpreter.set_value(0, 2);

        let mut stdout = stdout();
        let start = Instant::now();

        if with_ui {
            println!("Running game with UI ...");
        } else {
            println!("Running game without UI ...");
        }

        'game: loop {
            let (_, state) = interpreter.step();
            match state {
                ExecutionState::Wait => {
                    // Read input from interpreter
                    self.read_input(&interpreter.dump_output());
                    // Reimport new data in interpreter output
                    interpreter.set_output_values(self.dump_tiles());

                    // Show game
                    if with_ui {
                        self.print_screen(&mut stdout);
                    }

                    // Move the paddle depending on the ball position
                    let movement = self.process_joystick_input();
                    interpreter.push_input(movement.to_code());
                }
                ExecutionState::Exit => {
                    // Read last input from interpreter
                    self.read_input(&interpreter.dump_output());
                    break 'game;
                }
                _ => (),
            }
        }
        println!("Game over: {} milliseconds", start.elapsed().as_millis());

        // Score
        self.score
    }

    pub fn get_single_tile_position(&self, tile: Tile) -> Vector2D {
        self.tiles
            .iter()
            .filter_map(|(k, v)| if *v == tile { Some(*k) } else { None })
            .next()
            .unwrap()
    }

    pub fn get_ball_position(&self) -> Vector2D {
        self.get_single_tile_position(Tile::Ball)
    }

    pub fn get_paddle_position(&self) -> Vector2D {
        self.get_single_tile_position(Tile::HorizontalPaddle)
    }

    pub fn process_joystick_input(&self) -> JoystickMovement {
        let ball_position = self.get_ball_position();
        let paddle_position = self.get_paddle_position();

        if ball_position.x < paddle_position.x {
            JoystickMovement::Left
        } else if ball_position.x > paddle_position.x {
            JoystickMovement::Right
        } else {
            JoystickMovement::Neutral
        }
    }

    pub fn from_idle_intcode(code: &str) -> Self {
        let mut interpreter = Interpreter::new(code);
        interpreter.run();

        Self::from_input(&interpreter.dump_output())
    }

    pub fn from_input(input: &str) -> Self {
        let mut game = Self::new();
        game.read_input(input);
        game
    }

    pub fn read_input(&mut self, input: &str) {
        let mut tiles = HashMap::new();
        let entries: Vec<_> = input
            .split(',')
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        let entries_count = entries.len();

        let mut cursor = 0;
        loop {
            let x = entries[cursor];
            let y = entries[cursor + 1];
            let tile_id = entries[cursor + 2];

            if x == -1 && y == 0 {
                // Score update
                self.score = tile_id;
            } else {
                // New tile
                tiles.insert(Vector2D::new(x, y), Tile::from_tile_id(tile_id));
            }

            cursor += 3;
            if cursor >= entries_count {
                break;
            }
        }

        self.tiles = tiles;
    }

    pub fn get_screen_rect(&self) -> (Vector2D, Vector2D) {
        let mut top_left = Vector2D::new(i32::max_value(), i32::max_value());
        let mut bottom_right = Vector2D::new(i32::min_value(), i32::min_value());

        for coord in self.tiles.keys() {
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

    pub fn dump_screen(&self) -> String {
        let (top_left, bottom_right) = self.get_screen_rect();
        let mut screen = String::new();

        for y in top_left.y..bottom_right.y {
            for x in top_left.x..bottom_right.x {
                screen.push_str(&self.get_tile(x, y).to_ascii());
            }

            screen.push('\n');
        }

        screen
    }

    pub fn dump_tiles(&self) -> Vec<i64> {
        self.tiles.iter().fold(vec![], |mut ve, (k, v)| {
            ve.push(k.x.into());
            ve.push(k.y.into());
            ve.push(v.to_tile_id().into());
            ve
        })
    }

    pub fn print_screen(&self, stdout: &mut std::io::Stdout) {
        let (top_left, bottom_right) = self.get_screen_rect();

        stdout
            .execute(terminal::Clear(terminal::ClearType::All))
            .unwrap();
        for y in top_left.y..bottom_right.y {
            for x in top_left.x..bottom_right.x {
                stdout
                    .queue(cursor::MoveTo(x as u16, y as u16))
                    .unwrap()
                    .queue(style::Print(self.get_tile(x, y).to_ascii()))
                    .unwrap();
            }
        }
        stdout
            .queue(cursor::MoveTo(
                top_left.x as u16,
                (bottom_right.y + 1) as u16,
            ))
            .unwrap()
            .queue(style::Print(format!("Score: {}", self.score)))
            .unwrap();
        stdout.flush().unwrap();
    }

    pub fn get_tile(&self, x: i32, y: i32) -> Tile {
        let vec = Vector2D::new(x, y);
        self.tiles.get(&vec).copied().unwrap_or(Tile::Empty)
    }

    pub fn count_tiles(&self, tile: Tile) -> usize {
        self.tiles.iter().filter(|&(_, v)| *v == tile).count()
    }
}

fn part1(input_txt: &str) -> usize {
    let game = Game::from_idle_intcode(input_txt);
    game.count_tiles(Tile::Block)
}

fn part2(input_txt: &str, with_ui: bool) -> i32 {
    let mut game = Game::new();
    game.play(input_txt, with_ui)
}

fn main() {
    let input_txt = include_str!("../input.txt");
    let args: Vec<String> = env::args().collect();
    let with_ui = &args.get(1).cloned().unwrap_or_else(|| "".to_owned()) == "ui";

    println!("[Part 1]");
    let r = part1(&input_txt);
    println!("Result: {}", r);

    println!("[Part 2]");
    let r = part2(&input_txt, with_ui);
    println!("Result: {}", r);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let game = Game::from_input("1,2,3,6,5,4");
        assert_eq!(game.get_tile(1, 2), Tile::HorizontalPaddle);
        assert_eq!(game.get_tile(6, 5), Tile::Ball);
    }

    #[test]
    fn test_results() {
        let input_txt = include_str!("../input.txt");
        assert_eq!(part1(&input_txt), 253);
        // Execution time: ~30 seconds
        assert_eq!(part2(&input_txt, false), 12_263);
    }
}
