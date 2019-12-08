pub struct SpaceImageLayer {
    data: String,
}

impl SpaceImageLayer {
    pub fn new(data: &str) -> Self {
        Self {
            data: data.to_owned(),
        }
    }

    pub fn count_digits(&self, digit: char) -> usize {
        self.data.matches(digit).count()
    }

    pub fn get_data(&self) -> &str {
        &self.data
    }
}

pub struct SpaceImage {
    layers: Vec<SpaceImageLayer>,
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
            layers.push(SpaceImageLayer::new(stream));
        }

        Self { layers }
    }

    pub fn find_layer_with_least_digit(&self, digit: char) -> Option<&SpaceImageLayer> {
        if self.layers.is_empty() {
            None
        } else {
            let mut selected_layer = &self.layers[0];
            let mut least_digits = usize::max_value();
            for layer in &self.layers[1..] {
                let value = layer.count_digits(digit);
                if value < least_digits {
                    least_digits = value;
                    selected_layer = layer;
                }
            }

            Some(selected_layer)
        }
    }

    pub fn get_layers(&self) -> &[SpaceImageLayer] {
        &self.layers
    }
}

fn part1(input_txt: &str) -> usize {
    let image = SpaceImage::from_str(input_txt, 25, 6);
    let layer = image.find_layer_with_least_digit('0').unwrap();
    layer.count_digits('1') * layer.count_digits('2')
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
    fn test_small_image() {
        let image = SpaceImage::from_str("123456789012", 3, 2);
        assert_eq!(image.get_layers().len(), 2);
        assert_eq!(image.get_layers()[0].get_data(), "123456");
        assert_eq!(image.get_layers()[1].get_data(), "789012");
    }

    #[test]
    fn test_results() {
        let input_txt = include_str!("../input.txt");
        assert_eq!(part1(&input_txt), 1690);
        // assert_eq!(part2(&input_txt), 0);
    }
}
