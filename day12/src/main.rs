use itertools::Itertools;

pub type Vector3D = euclid::default::Vector3D<i32>;

#[derive(Debug, Clone)]
pub struct Moon {
    position: Vector3D,
    velocity: Vector3D,
}

impl Moon {
    pub fn from_input(input: &str) -> Self {
        let coords = &input[1..input.len() - 1]
            .split(", ")
            .map(|x| {
                x.split('=')
                    .skip(1)
                    .map(|y| y.parse::<i32>().unwrap())
                    .next()
                    .unwrap()
            })
            .collect::<Vec<i32>>();

        Self {
            position: Vector3D::new(coords[0], coords[1], coords[2]),
            velocity: Vector3D::default(),
        }
    }

    pub fn apply_gravity(&mut self, other_moon: &mut Self) {
        if self.position.x < other_moon.position.x {
            self.velocity.x += 1;
            other_moon.velocity.x -= 1;
        } else if self.position.x > other_moon.position.x {
            self.velocity.x -= 1;
            other_moon.velocity.x += 1;
        }

        if self.position.y < other_moon.position.y {
            self.velocity.y += 1;
            other_moon.velocity.y -= 1;
        } else if self.position.y > other_moon.position.y {
            self.velocity.y -= 1;
            other_moon.velocity.y += 1;
        }

        if self.position.z < other_moon.position.z {
            self.velocity.z += 1;
            other_moon.velocity.z -= 1;
        } else if self.position.z > other_moon.position.z {
            self.velocity.z -= 1;
            other_moon.velocity.z += 1;
        }
    }

    pub fn integrate_velocity(&mut self) {
        self.position += self.velocity;
    }

    pub fn compute_total_energy(&self) -> usize {
        let pot = self.position.x.abs() + self.position.y.abs() + self.position.z.abs();
        let kin = self.velocity.x.abs() + self.velocity.y.abs() + self.velocity.z.abs();
        (pot * kin) as usize
    }
}

#[derive(Debug)]
pub struct MoonSim {
    moons: Vec<Moon>,
}

impl MoonSim {
    pub fn from_input(input: &str) -> Self {
        Self {
            moons: input.split('\n').map(Moon::from_input).collect(),
        }
    }

    pub fn step(&mut self) {
        let combinations: Vec<Vec<usize>> = (0..self.moons.len()).combinations(2).collect();
        for combination in combinations {
            let mut moon1 = self.moons[combination[0]].clone();
            let mut moon2 = self.moons[combination[1]].clone();

            moon1.apply_gravity(&mut moon2);

            self.moons[combination[0]] = moon1;
            self.moons[combination[1]] = moon2;
        }

        for moon in &mut self.moons {
            moon.integrate_velocity();
        }
    }

    pub fn step_for(&mut self, steps: usize) {
        for _ in 0..steps {
            self.step();
        }
    }

    pub fn compute_total_energy(&self) -> usize {
        self.moons.iter().map(|x| x.compute_total_energy()).sum()
    }
}

fn part1(input_txt: &str) -> usize {
    let mut sim = MoonSim::from_input(input_txt);
    sim.step_for(1_000);
    sim.compute_total_energy()
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

    fn example1() -> &'static str {
        "<x=-1, y=0, z=2>\n\
         <x=2, y=-10, z=-7>\n\
         <x=4, y=-8, z=8>\n\
         <x=3, y=5, z=-1>"
    }

    fn assert_expr(moon: &Moon, px: i32, py: i32, pz: i32, vx: i32, vy: i32, vz: i32) {
        assert_eq!(moon.position, Vector3D::new(px, py, pz));
        assert_eq!(moon.velocity, Vector3D::new(vx, vy, vz));
    }

    #[test]
    fn test_parse() {
        let sim = MoonSim::from_input(example1());
        assert_eq!(sim.moons.len(), 4);
        assert_expr(&sim.moons[0], -1, 0, 2, 0, 0, 0);
        assert_expr(&sim.moons[1], 2, -10, -7, 0, 0, 0);
        assert_expr(&sim.moons[2], 4, -8, 8, 0, 0, 0);
        assert_expr(&sim.moons[3], 3, 5, -1, 0, 0, 0);
    }

    #[test]
    fn test_step() {
        let mut sim = MoonSim::from_input(example1());

        // First step
        sim.step();
        assert_expr(&sim.moons[0], 2, -1, 1, 3, -1, -1);
        assert_expr(&sim.moons[1], 3, -7, -4, 1, 3, 3);
        assert_expr(&sim.moons[2], 1, -7, 5, -3, 1, -3);
        assert_expr(&sim.moons[3], 2, 2, 0, -1, -3, 1);

        // 9 more steps
        sim.step_for(9);
        assert_expr(&sim.moons[0], 2, 1, -3, -3, -2, 1);
        assert_expr(&sim.moons[1], 1, -8, 0, -1, 1, 3);
        assert_expr(&sim.moons[2], 3, -6, 1, 3, 2, -3);
        assert_expr(&sim.moons[3], 2, 0, 4, 1, -1, -1);

        assert_eq!(sim.moons[0].compute_total_energy(), 36);
        assert_eq!(sim.moons[1].compute_total_energy(), 45);
        assert_eq!(sim.moons[2].compute_total_energy(), 80);
        assert_eq!(sim.moons[3].compute_total_energy(), 18);
        assert_eq!(sim.compute_total_energy(), 179);
    }

    #[test]
    fn test_results() {
        let input_txt = include_str!("../input.txt");
        assert_eq!(part1(&input_txt), 8_960);
        // assert_eq!(part2(&input_txt), 0);
    }
}
