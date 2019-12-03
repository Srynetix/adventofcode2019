use std::collections::HashSet;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y
        }
    }

    fn cross(&self, other: Self) -> i32 {
        self.x * other.y - self.y * other.x
    }

    fn subtract(&self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }

    fn from_wire(wire: &str) -> Self {
        let direction = wire.chars().nth(0).unwrap();
        let amount: i32 = wire[1..].parse().unwrap();

        match direction {
            'R' => Self { x: amount, y: 0 },
            'L' => Self { x: -amount, y: 0 },
            'U' => Self { x: 0, y: amount },
            'D' => Self { x: 0, y: -amount },
            _ => unreachable!()
        }
    }

    fn manhattan_distance(&self, other: Self) -> u32 {
        (self.x - other.x).abs() as u32 + (self.y - other.y).abs() as u32
    }

    const fn zero() -> Self {
        Self {
            x: 0,
            y: 0
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Segment {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32
}

impl Segment {
    fn new(origin: Point, wire: &str) -> Self {
        let wire_point = Point::from_wire(wire);
        Self {
            x1: origin.x,
            y1: origin.y,
            x2: origin.x + wire_point.x,
            y2: origin.y + wire_point.y
        }
    }

    fn new_raw(x1: i32, y1: i32, x2: i32, y2: i32) -> Self {
        Self {
            x1,
            y1,
            x2,
            y2
        }
    }

    fn from_path(path: &str) -> Vec<Self> {
        let mut segments = vec![];
        let mut origin = Point::new(0, 0);
        for wire in path.split(',') {
            let segment = Self::new(origin, wire);
            origin = segment.target();
            segments.push(segment);
        }

        segments
    }

    fn origin(&self) -> Point {
        Point::new(self.x1, self.y1)
    }

    fn target(&self) -> Point {
        Point::new(self.x2, self.y2)
    }

    fn intersect(&self, other: Self) -> Option<Point> {
        let d: i32 = (other.y2 - other.y1) * (self.x2 - self.x1) - (other.x2 - other.x1) * (self.y2 - self.y1);
        let n_a: i32 = (other.x2 - other.x1) * (self.y1 - other.y1) - (other.y2 - other.y1) * (self.x1 - other.x1);
        let n_b: i32 = (self.x2 - self.x1) * (self.y1 - other.y1) - (self.y2 - self.y1) * (self.x1 - other.x1);
        if d == 0 {
            return None;
        }

        let ua: f32 = n_a as f32 / d as f32;
        let ub: f32 = n_b as f32 / d as f32;

        if ua >= 0.0 && ua <= 1.0 && ub >= 0.0 && ub <= 1.0 {
            let nx: f32 = self.x1 as f32 + (ua * (self.x2 - self.x1) as f32);
            let ny: f32 = self.y1 as f32 + (ua * (self.y2 - self.y1) as f32);

            Some(Point::new(nx as i32, ny as i32))
        } else {
            None
        }
    }
}

fn calculate_intersection_distance(first_path: &str, second_path: &str) -> u32 {
    let first_seg_path = Segment::from_path(first_path);
    let second_seg_path = Segment::from_path(second_path);

    let mut intersection_points = HashSet::new();
    for f_path in &first_seg_path {
        for s_path in &second_seg_path {
            let pts = f_path.intersect(s_path.clone());
            for p in pts {
                if p != Point::zero() {
                    intersection_points.insert(p);
                }
            }
        }
    }

    let origin = Point::new(0, 0);
    let mut closest_distance = u32::max_value();
    for p in intersection_points {
        let dist = origin.manhattan_distance(p);
        if dist < closest_distance {
            closest_distance = dist;
        }
    }

    closest_distance
}

fn part1(input_txt: &str) {
    let paths: Vec<&str> = input_txt.split('\n').collect();
    let path1 = paths[0];
    let path2 = paths[1];

    println!("[Part 1]");
    println!("Result: {}", calculate_intersection_distance(path1, path2));
}

fn main() {
    let input_txt = include_str!("../input.txt");
    part1(&input_txt);
}

#[cfg(test)]
mod tests {
    use super::{Point, Segment, calculate_intersection_distance};

    #[test]
    fn test_segments() {
        assert_eq!(
            Segment::from_path("R8,U5,L5,D3"),
            vec![
                Segment::new_raw(0, 0, 8, 0),
                Segment::new_raw(8, 0, 8, 5),
                Segment::new_raw(8, 5, 3, 5),
                Segment::new_raw(3, 5, 3, 2)
            ]
        );
    }

    #[test]
    fn test_intersect() {
        assert_eq!(
            Segment::new_raw(0, 0, 4, 0).intersect(Segment::new_raw(2, -2, 2, 2)),
            Some(Point::new(2, 0))
        )
    }

    #[test]
    fn test_intersection() {
        assert_eq!(
            calculate_intersection_distance(
                "R8,U5,L5,D3",
                "U7,R6,D4,L4"
            ),
            6
        );
        assert_eq!(
            calculate_intersection_distance(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72",
                "U62,R66,U55,R34,D71,R55,D58,R83"
            ),
            159
        );
        assert_eq!(
            calculate_intersection_distance(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
                "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            ),
            135
        );
    }
}
