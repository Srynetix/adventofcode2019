use colored::Colorize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpaceColor {
    Black,
    White,
    Transparent,
    Unknown(char),
}

impl SpaceColor {
    pub fn from_char(c: char) -> Self {
        match c {
            '0' => Self::Black,
            '1' => Self::White,
            '2' => Self::Transparent,
            _ => Self::Unknown(c),
        }
    }

    pub fn to_char(self) -> char {
        match self {
            Self::Black => '0',
            Self::White => '1',
            Self::Transparent => '2',
            Self::Unknown(c) => c,
        }
    }

    pub fn draw(self) -> String {
        match self {
            Self::Black => "██".black().to_string(),
            Self::White => "██".white().to_string(),
            Self::Transparent => "  ".to_owned(),
            Self::Unknown(_) => "??".red().to_string(),
        }
    }
}

pub struct SpaceImageLayer {
    data: Vec<SpaceColor>,
    width: usize,
    height: usize,
}

impl SpaceImageLayer {
    pub fn new(data: &str, width: usize, height: usize) -> Self {
        Self {
            data: data.chars().map(SpaceColor::from_char).collect(),
            width,
            height,
        }
    }

    pub fn push_color(&mut self, color: SpaceColor) {
        self.data.push(color);
    }

    pub fn count_color(&self, color: SpaceColor) -> usize {
        self.data.iter().filter(|&x| *x == color).count()
    }

    pub fn get_color_at_idx(&self, idx: usize) -> SpaceColor {
        self.data[idx]
    }

    pub fn get_data(&self) -> &[SpaceColor] {
        &self.data
    }

    pub fn get_as_str(&self) -> String {
        self.data.iter().copied().map(SpaceColor::to_char).collect()
    }

    pub fn draw(&self) -> String {
        let mut output = String::new();

        for y in 0..self.height {
            for x in 0..self.width {
                output.push_str(&self.get_color_at_idx(x + y * self.width).draw());
            }

            if y != self.height - 1 {
                output.push('\n');
            }
        }

        output
    }
}

pub struct SpaceImage {
    layers: Vec<SpaceImageLayer>,
    width: usize,
    height: usize,
}

impl SpaceImage {
    pub fn from_str(input_str: &str, width: usize, height: usize) -> Self {
        let layer_size = width * height;
        let layer_count = input_str.len() / layer_size;
        let mut layers = vec![];

        for layer_id in 0..layer_count {
            let min_range = layer_id * layer_size;
            let max_range = (layer_id + 1) * layer_size;
            let stream = &input_str[min_range..max_range];
            layers.push(SpaceImageLayer::new(stream, width, height));
        }

        Self {
            layers,
            width,
            height,
        }
    }

    pub fn find_layer_with_least_color(&self, color: SpaceColor) -> Option<&SpaceImageLayer> {
        if self.layers.is_empty() {
            None
        } else {
            let mut selected_layer = &self.layers[0];
            let mut least_color = usize::max_value();
            for layer in &self.layers[1..] {
                let value = layer.count_color(color);
                if value < least_color {
                    least_color = value;
                    selected_layer = layer;
                }
            }

            Some(selected_layer)
        }
    }

    pub fn blend_color_at_idx(&self, idx: usize) -> SpaceColor {
        for layer in &self.layers {
            let color = layer.get_color_at_idx(idx);
            if color != SpaceColor::Transparent {
                return color;
            }
        }

        SpaceColor::Transparent
    }

    pub fn flatten_image(&self) -> SpaceImageLayer {
        let mut layer = SpaceImageLayer::new("", self.width, self.height);

        for idx in 0..self.width * self.height {
            let color = self.blend_color_at_idx(idx);
            layer.push_color(color);
        }

        layer
    }

    pub fn get_layers(&self) -> &[SpaceImageLayer] {
        &self.layers
    }
}

fn part1(input_txt: &str) -> usize {
    let image = SpaceImage::from_str(input_txt, 25, 6);
    let layer = image
        .find_layer_with_least_color(SpaceColor::Black)
        .unwrap();
    layer.count_color(SpaceColor::White) * layer.count_color(SpaceColor::Transparent)
}

fn part2(input_txt: &str) -> String {
    let image = SpaceImage::from_str(input_txt, 25, 6);
    let layer = image.flatten_image();
    println!("{}", layer.draw());
    layer.get_as_str()
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
    fn test_small_image() {
        let image = SpaceImage::from_str("123456789012", 3, 2);
        assert_eq!(image.get_layers().len(), 2);
        assert_eq!(image.get_layers()[0].get_as_str(), "123456");
        assert_eq!(image.get_layers()[1].get_as_str(), "789012");
    }

    #[test]
    fn test_blend() {
        let image = SpaceImage::from_str("0222112222120000", 2, 2);
        let layer = image.flatten_image();
        assert_eq!(layer.get_as_str(), "0110");
    }

    #[test]
    fn test_results() {
        let input_txt = include_str!("../input.txt");
        assert_eq!(part1(&input_txt), 1690);
        assert_eq!(part2(&input_txt), "111101110011110100101110000010100100001010010100100010010010001001001011100010001110001000100101001010000100001000010010100101111010000111100110011100");
        // Text: ZPZUB
    }
}
