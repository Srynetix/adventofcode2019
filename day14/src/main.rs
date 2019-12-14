#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variable {
    value: i32,
    name: String,
}

impl Variable {
    pub fn new(value: i32, name: &str) -> Self {
        Self {
            value,
            name: name.to_owned(),
        }
    }

    pub fn from_input(input: &str) -> Self {
        let entry: Vec<&str> = input.split(' ').collect();
        let value = entry.get(0).and_then(|x| x.parse::<i32>().ok()).unwrap();
        let name = entry.get(1).map(|x| x.to_string()).unwrap();

        Self { value, name }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Equation {
    input: Vec<Variable>,
    output: Variable,
}

impl Equation {
    pub fn new(input: Vec<Variable>, output: Variable) -> Self {
        Self { input, output }
    }

    pub fn from_input(input: &str) -> Self {
        let parts: Vec<&str> = input.split(" => ").collect();
        let input: Vec<_> = parts[0].split(", ").map(Variable::from_input).collect();
        let output = Variable::from_input(parts[1]);

        Self { input, output }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Problem {
    equations: Vec<Equation>,
}

impl Problem {
    pub fn new(equations: Vec<Equation>) -> Self {
        Self { equations }
    }

    pub fn from_input(input: &str) -> Self {
        let equations: Vec<_> = input.split('\n').map(Equation::from_input).collect();
        Self { equations }
    }

    pub fn resolve_fuel(&self) -> i32 {
        
        0
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(Variable::from_input("5 ABC"), Variable::new(5, "ABC"));

        assert_eq!(
            Equation::from_input("1 A, 2 B => 3 C"),
            Equation::new(
                vec![Variable::new(1, "A"), Variable::new(2, "B")],
                Variable::new(3, "C")
            )
        );

        assert_eq!(
            Problem::from_input("10 A => 20 B\n3 B => 2 C"),
            Problem::new(vec![
                Equation::new(vec![Variable::new(10, "A")], Variable::new(20, "B")),
                Equation::new(vec![Variable::new(3, "B")], Variable::new(2, "C"))
            ])
        );
    }
}
