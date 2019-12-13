use std::collections::HashMap;

use common::Interpreter;

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
}

#[derive(Debug)]
pub struct Game {
    tiles: HashMap<Vector2D, Tile>,
}

impl Game {
    fn from_intcode(code: &str) -> Self {
        let mut interpreter = Interpreter::new(code);
        interpreter.run();

        Self::from_input(&interpreter.dump_output())
    }

    fn from_input(input: &str) -> Self {
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

            tiles.insert(Vector2D::new(x, y), Tile::from_tile_id(tile_id));

            cursor += 3;
            if cursor >= entries_count {
                break;
            }
        }

        Self { tiles }
    }

    pub fn get_tile(&self, x: i32, y: i32) -> Tile {
        let vec = Vector2D::new(x, y);
        self.tiles[&vec]
    }

    pub fn count_tiles(&self, tile: Tile) -> usize {
        self.tiles.iter().filter(|&(_, v)| *v == tile).count()
    }
}

fn part1(input_txt: &str) -> usize {
    let game = Game::from_intcode(input_txt);
    game.count_tiles(Tile::Block)
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
    fn test_parse() {
        let game = Game::from_input("1,2,3,6,5,4");
        assert_eq!(game.get_tile(1, 2), Tile::HorizontalPaddle);
        assert_eq!(game.get_tile(6, 5), Tile::Ball);
    }

    #[test]
    fn test_results() {
        let input_txt = include_str!("../input.txt");
        assert_eq!(part1(&input_txt), 253);
        // assert_eq!(part2(&input_txt), 0);
    }
}
