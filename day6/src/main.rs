use std::collections::HashMap;

#[derive(Debug)]
pub struct OrbitGraph {
    nodes: HashMap<String, String>,
}

impl OrbitGraph {
    pub fn new(input_txt: &str) -> Self {
        let mut nodes = HashMap::new();
        for line in input_txt.split('\n') {
            let entry: Vec<&str> = line.split(')').collect();
            nodes.insert(entry[1].to_owned(), entry[0].to_owned());
        }

        Self { nodes }
    }

    /// List orbits at point
    pub fn list_orbits_at_point(&self, point: &str) -> Vec<String> {
        let mut orbits = vec![];
        let mut stack = vec![];
        stack.push(point);

        while !stack.is_empty() {
            let pt = stack.pop().unwrap();
            if let Some(source) = self.nodes.get(pt) {
                orbits.push(source.to_owned());
                stack.push(source);
            }
        }

        orbits
    }

    /// Count orbits at point
    pub fn count_orbits_at_point(&self, point: &str) -> usize {
        self.list_orbits_at_point(point).len()
    }

    /// Count total orbits
    pub fn count_total_orbits(&self) -> usize {
        let mut counter = 0;
        for key in self.nodes.keys() {
            counter += self.count_orbits_at_point(key);
        }

        counter
    }

    /// List transfers to target
    pub fn list_transfers_to_target(&self, source: &str, target: &str) -> Vec<String> {
        let source_path = self.list_orbits_at_point(source);
        let target_path = self.list_orbits_at_point(target);
        let mut path = vec![];
        let mut intersection = "";

        // Searching for intersection ...
        for x in &source_path[1..] {
            path.push(x.clone());
            if target_path.contains(x) {
                // Intersection found!
                intersection = x;
                break;
            }
        }

        if intersection == "" {
            // No intersection found, no path
            return vec![];
        }

        // Searching for path to target ...
        let mut dst_path: Vec<String> = vec![];
        for x in &target_path {
            if x == intersection {
                // Intersection found, just reverse dst_path and add it to path
                for y in dst_path.iter().rev() {
                    path.push(y.clone());
                }
                break;
            }

            // Store in path
            dst_path.push(x.clone());
        }

        path
    }

    /// Count transfers to target
    pub fn count_transfers_to_target(&self, source: &str, target: &str) -> usize {
        self.list_transfers_to_target(source, target).len()
    }
}

fn part1(input_txt: &str) -> usize {
    let graph = OrbitGraph::new(input_txt);
    graph.count_total_orbits()
}

fn part2(input_txt: &str) -> usize {
    let graph = OrbitGraph::new(input_txt);
    graph.count_transfers_to_target("YOU", "SAN")
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

    fn input_part1() -> &'static str {
        "COM)B\n\
         B)C\n\
         C)D\n\
         D)E\n\
         E)F\n\
         B)G\n\
         G)H\n\
         D)I\n\
         E)J\n\
         J)K\n\
         K)L"
    }

    fn input_part2() -> &'static str {
        "COM)B\n\
         B)C\n\
         C)D\n\
         D)E\n\
         E)F\n\
         B)G\n\
         G)H\n\
         D)I\n\
         E)J\n\
         J)K\n\
         K)L\n\
         K)YOU\n\
         I)SAN"
    }

    #[test]
    fn test_orbits() {
        fn orbits_eq(point: &str, validation: &str) {
            let graph = OrbitGraph::new(input_part1());
            assert_eq!(
                graph.list_orbits_at_point(point).join(","),
                validation.to_owned()
            );
        }

        orbits_eq("B", "COM");
        orbits_eq("D", "C,B,COM");
        orbits_eq("L", "K,J,E,D,C,B,COM");
        orbits_eq("COM", "");
    }

    #[test]
    fn test_orbits_count() {
        fn orbits_count_eq(point: &str, validation: usize) {
            let graph = OrbitGraph::new(input_part1());
            assert_eq!(graph.count_orbits_at_point(point), validation);
        }

        orbits_count_eq("B", 1);
        orbits_count_eq("D", 3);
        orbits_count_eq("L", 7);
        orbits_count_eq("COM", 0);
    }

    #[test]
    fn test_orbits_total_count() {
        let graph = OrbitGraph::new(input_part1());
        assert_eq!(graph.count_total_orbits(), 42);
    }

    #[test]
    fn test_transfers() {
        let graph = OrbitGraph::new(input_part2());
        assert_eq!(
            graph.list_transfers_to_target("YOU", "SAN").join(","),
            "J,E,D,I".to_owned()
        );
    }

    #[test]
    fn test_transfers_count() {
        let graph = OrbitGraph::new(input_part2());
        assert_eq!(graph.count_transfers_to_target("YOU", "SAN"), 4);
    }

    #[test]
    fn test_results() {
        let input_txt = include_str!("../input.txt");
        assert_eq!(part1(&input_txt), 147807);
        assert_eq!(part2(&input_txt), 229);
    }
}
