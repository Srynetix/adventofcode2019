use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chemical {
    value: i32,
    name: String,
}

impl Chemical {
    pub fn new(value: i32, name: &str) -> Self {
        Self {
            value,
            name: name.to_owned(),
        }
    }

    pub fn is_ore(&self) -> bool {
        self.name == "ORE"
    }

    pub fn from_input(input: &str) -> Self {
        let entry: Vec<&str> = input.split(' ').collect();
        let value = entry
            .get(0)
            .and_then(|x| x.parse::<i32>().ok())
            .unwrap_or_else(|| panic!("invalid chemical value {:?}", entry));
        let name = entry
            .get(1)
            .map(|x| x.to_string())
            .unwrap_or_else(|| panic!("invalid chemical name: {:?}", entry));

        Self { value, name }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reaction {
    input: Vec<Chemical>,
    output: Chemical,
}

impl Reaction {
    pub fn new(input: Vec<Chemical>, output: Chemical) -> Self {
        Self { input, output }
    }

    pub fn from_input(input: &str) -> Self {
        let parts: Vec<&str> = input.split(" => ").collect();
        let input: Vec<_> = parts[0].split(", ").map(Chemical::from_input).collect();
        let output = Chemical::from_input(parts[1]);

        Self { input, output }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Simulation {
    reactions: Vec<Reaction>,
}

impl Simulation {
    pub fn new(reactions: Vec<Reaction>) -> Self {
        Self { reactions }
    }

    pub fn from_input(input: &str) -> Self {
        let reactions: Vec<_> = input.split('\n').map(Reaction::from_input).collect();
        Self { reactions }
    }

    pub fn find_reaction_for(&self, var_name: &str) -> Reaction {
        self.reactions
            .iter()
            .find(|&x| x.output.name == var_name)
            .cloned()
            .unwrap_or_else(|| panic!("unknown output variable: {}", var_name))
    }

    pub fn find_fuel_reaction(&self) -> Reaction {
        self.find_reaction_for("FUEL")
    }

    pub fn calculate_fuel(&self) -> i32 {
        let mut needed = Vec::new();
        let mut remaining = HashMap::new();
        let mut ore = 0;
        needed.push(("FUEL".to_owned(), 1));

        while !needed.is_empty() {
            let (needed_name, mut needed_quantity) = needed.remove(0);

            // Use quantity from remaining chemicals
            let remaining_quantity = remaining.entry(needed_name.clone()).or_insert(0);
            let remaining_quantity_to_use = if *remaining_quantity < needed_quantity {
                *remaining_quantity
            } else {
                needed_quantity
            };

            // Remove remaining quantity from remaining and from needed quantity
            needed_quantity -= remaining_quantity_to_use;
            *remaining_quantity -= remaining_quantity_to_use;

            // Handle ore
            if &needed_name == "ORE" {
                ore += needed_quantity;
                continue;
            }

            if needed_quantity > 0 {
                // Get reaction & calculate coef
                let reaction = self.find_reaction_for(&needed_name);
                let div = ((needed_quantity - 1) / reaction.output.value) + 1;
                *remaining_quantity = reaction.output.value * div - needed_quantity;

                // Iterate on input chemicals from reaction
                for chemical in &reaction.input {
                    needed.push((chemical.name.clone(), chemical.value * div));
                }
            }
        }

        ore
    }
}

fn part1(input_txt: &str) -> i32 {
    Simulation::from_input(input_txt).calculate_fuel()
}

fn part2(input_txt: &str) -> i32 {
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
        "10 ORE => 10 A\n\
         1 ORE => 1 B\n\
         7 A, 1 B => 1 C\n\
         7 A, 1 C => 1 D\n\
         7 A, 1 D => 1 E\n\
         7 A, 1 E => 1 FUEL"
    }

    fn example2() -> &'static str {
        "9 ORE => 2 A\n\
         8 ORE => 3 B\n\
         7 ORE => 5 C\n\
         3 A, 4 B => 1 AB\n\
         5 B, 7 C => 1 BC\n\
         4 C, 1 A => 1 CA\n\
         2 AB, 3 BC, 4 CA => 1 FUEL"
    }

    fn example3() -> &'static str {
        "157 ORE => 5 NZVS\n\
         165 ORE => 6 DCFZ\n\
         44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL\n\
         12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ\n\
         179 ORE => 7 PSHF\n\
         177 ORE => 5 HKGWZ\n\
         7 DCFZ, 7 PSHF => 2 XJWVT\n\
         165 ORE => 2 GPVTF\n\
         3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT"
    }

    fn example4() -> &'static str {
        "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG\n\
         17 NVRVD, 3 JNWZP => 8 VPVL\n\
         53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL\n\
         22 VJHF, 37 MNCFX => 5 FWMGM\n\
         139 ORE => 4 NVRVD\n\
         144 ORE => 7 JNWZP\n\
         5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC\n\
         5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV\n\
         145 ORE => 6 MNCFX\n\
         1 NVRVD => 8 CXFTF\n\
         1 VJHF, 6 MNCFX => 4 RFSQX\n\
         176 ORE => 6 VJHF"
    }

    fn example5() -> &'static str {
        "171 ORE => 8 CNZTR\n\
         7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL\n\
         114 ORE => 4 BHXH\n\
         14 VRPVC => 6 BMBT\n\
         6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL\n\
         6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT\n\
         15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW\n\
         13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW\n\
         5 BMBT => 4 WPTQ\n\
         189 ORE => 9 KTJDG\n\
         1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP\n\
         12 VRPVC, 27 CNZTR => 2 XDBXC\n\
         15 KTJDG, 12 BHXH => 5 XCVML\n\
         3 BHXH, 2 VRPVC => 7 MZWV\n\
         121 ORE => 7 VRPVC\n\
         7 XCVML => 6 RJRHP\n\
         5 BHXH, 4 VRPVC => 5 LTCX"
    }

    #[test]
    fn test_parse() {
        assert_eq!(Chemical::from_input("5 ABC"), Chemical::new(5, "ABC"));

        assert_eq!(
            Reaction::from_input("1 A, 2 B => 3 C"),
            Reaction::new(
                vec![Chemical::new(1, "A"), Chemical::new(2, "B")],
                Chemical::new(3, "C")
            )
        );

        assert_eq!(
            Simulation::from_input("10 A => 20 B\n3 B => 2 C"),
            Simulation::new(vec![
                Reaction::new(vec![Chemical::new(10, "A")], Chemical::new(20, "B")),
                Reaction::new(vec![Chemical::new(3, "B")], Chemical::new(2, "C"))
            ])
        );
    }

    #[test]
    fn test_resolution_small() {
        assert_eq!(Simulation::from_input(example1()).calculate_fuel(), 31);
        assert_eq!(Simulation::from_input(example2()).calculate_fuel(), 165);
    }

    #[test]
    fn test_resolution_large() {
        assert_eq!(Simulation::from_input(example3()).calculate_fuel(), 13312);
        assert_eq!(Simulation::from_input(example4()).calculate_fuel(), 180697);
        assert_eq!(Simulation::from_input(example5()).calculate_fuel(), 2210736);
    }

    #[test]
    fn test_results() {
        let input_txt = include_str!("../input.txt");
        assert_eq!(part1(&input_txt), 443_537);
        // assert_eq!(part2(&input_txt), 0);
    }
}
