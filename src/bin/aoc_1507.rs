use color_eyre::eyre::{eyre, Result};

use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum Input {
    Number(u16),
    Wire(String),
}

#[derive(Debug, PartialEq)]
enum Component {
    Const {
        input: Input,
        output: String,
    },
    And {
        input_a: Input,
        input_b: Input,
        output: String,
    },
    Or {
        input_a: Input,
        input_b: Input,
        output: String,
    },
    LShift {
        input_a: Input,
        input_b: Input,
        output: String,
    },
    RShift {
        input_a: Input,
        input_b: Input,
        output: String,
    },
    Not {
        input: Input,
        output: String,
    },
}

#[derive(Debug, PartialEq)]
struct Circuit {
    wires: HashMap<String, Option<u16>>,
    components: Vec<Component>,
}

impl Circuit {
    fn get_wire_val(&self, key: &Input) -> Option<u16> {
        match key {
            Input::Number(n) => Some(*n),
            Input::Wire(key) => self.wires.get(key).copied().flatten(),
        }
    }

    fn evaluate(&mut self, mut exit: impl FnMut(&Self) -> bool) {
        while exit(self) {
            for comp in &self.components {
                use Component::*;
                match comp {
                    Const { input, output } => {
                        if let (Some(val), Some(output @ None)) =
                            (self.get_wire_val(input), self.wires.get_mut(output))
                        {
                            *output = Some(val);
                        }
                    }

                    Not { input, output } => {
                        if let (Some(val), Some(output @ None)) =
                            (self.get_wire_val(input), self.wires.get_mut(output))
                        {
                            *output = Some(!val);
                        }
                    }

                    And {
                        input_a,
                        input_b,
                        output,
                    } => {
                        if let (Some(a), Some(b), Some(output @ None)) = (
                            self.get_wire_val(input_a),
                            self.get_wire_val(input_b),
                            self.wires.get_mut(output),
                        ) {
                            *output = Some(a & b);
                        }
                    }

                    Or {
                        input_a,
                        input_b,
                        output,
                    } => {
                        if let (Some(a), Some(b), Some(output @ None)) = (
                            self.get_wire_val(input_a),
                            self.get_wire_val(input_b),
                            self.wires.get_mut(output),
                        ) {
                            *output = Some(a | b);
                        }
                    }

                    LShift {
                        input_a,
                        input_b,
                        output,
                    } => {
                        if let (Some(a), Some(b), Some(output @ None)) = (
                            self.get_wire_val(input_a),
                            self.get_wire_val(input_b),
                            self.wires.get_mut(output),
                        ) {
                            *output = Some(a << b);
                        }
                    }

                    RShift {
                        input_a,
                        input_b,
                        output,
                    } => {
                        if let (Some(a), Some(b), Some(output @ None)) = (
                            self.get_wire_val(input_a),
                            self.get_wire_val(input_b),
                            self.wires.get_mut(output),
                        ) {
                            *output = Some(a >> b);
                        }
                    }
                }
            }
        }
    }

    fn parse_circuit(input: &str) -> nom::IResult<&str, Circuit> {
        use nom::{
            branch::alt,
            bytes::complete::{tag, take_till1, take_while1},
            sequence::{separated_pair, tuple},
        };

        let mut wires = HashMap::new();
        let mut components = Vec::new();

        for line in input.lines().map(str::trim) {
            let (_, (component, output)) =
                separated_pair(take_till1(|c| c == '-'), tag("->"), take_while1(|_| true))(line)?;

            let (_, component) = alt((
                tuple((
                    take_till1(|c| c == ' '),
                    tag(" AND "),
                    take_while1(|_| true),
                )),
                tuple((take_till1(|c| c == ' '), tag(" OR "), take_while1(|_| true))),
                tuple((
                    take_till1(|c| c == ' '),
                    tag(" RSHIFT "),
                    take_while1(|_| true),
                )),
                tuple((
                    take_till1(|c| c == ' '),
                    tag(" LSHIFT "),
                    take_while1(|_| true),
                )),
                tuple((tag("NOT "), take_till1(char::is_whitespace), tag(""))),
                tuple((
                    take_while1(|c: char| c.is_ascii_alphanumeric()),
                    tag(""),
                    tag(""),
                )),
            ))(component.trim())?;

            let output = output.trim().to_owned();
            wires.entry(output.clone()).or_default();

            let mut parse_input = |input: &str| {
                if let Ok(c) = input.parse() {
                    Input::Number(c)
                } else {
                    wires.entry(input.to_owned()).or_default();
                    Input::Wire(input.to_owned())
                }
            };

            let component = match component {
                (a, " RSHIFT ", b) => Component::RShift {
                    input_a: parse_input(a),
                    input_b: parse_input(b),
                    output,
                },
                (a, " OR ", b) => Component::Or {
                    input_a: parse_input(a),
                    input_b: parse_input(b),
                    output,
                },
                (a, " AND ", b) => Component::And {
                    input_a: parse_input(a),
                    input_b: parse_input(b),
                    output,
                },
                (a, " LSHIFT ", b) => Component::LShift {
                    input_a: parse_input(a),
                    input_b: parse_input(b),
                    output,
                },

                ("NOT ", a, _) => Component::Not {
                    input: parse_input(a),
                    output,
                },
                (a, _, _) => Component::Const {
                    input: parse_input(a),
                    output,
                },
            };

            components.push(component);
        }

        Ok(("", Circuit { wires, components }))
    }
}

fn part_1(input: &str) -> Result<u16> {
    let (_, mut circuit) =
        Circuit::parse_circuit(&input).map_err(|e| eyre!("Parse Error: {}", e))?;
    let mut counter = 0;
    circuit.evaluate(|c| {
        counter += 1;
        c.wires.values().any(Option::is_none) && counter < 1000
    });

    circuit
        .wires
        .get("a")
        .copied()
        .flatten()
        .ok_or_else(|| eyre!("Wire not found: a"))
}

fn part_2(input: &str) -> Result<u16> {
    let (_, mut circuit) =
        Circuit::parse_circuit(&input).map_err(|e| eyre!("Parse Error: {}", e))?;

    *circuit.wires.get_mut("b").unwrap() = Some(46065);

    let mut counter = 0;
    circuit.evaluate(|c| {
        counter += 1;
        c.wires.values().any(Option::is_none) && counter < 1000
    });

    circuit
        .wires
        .get("a")
        .copied()
        .flatten()
        .ok_or_else(|| eyre!("Wire not found: a"))
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = std::fs::read_to_string("inputs/aoc_1507.txt")?;

    let start = std::time::Instant::now();

    let part_1 = part_1(&input)?;
    let part_2 = part_2(&input)?;

    let elapsed = start.elapsed();

    println!("Part 1 output: {:?}", part_1);
    println!("Part 2 output: {:?}", part_2);

    println!("Elapsed: {}us", elapsed.as_micros());

    Ok(())
}

#[cfg(test)]
mod tests_1507 {
    use super::*;

    #[test]
    fn part1_example() {
        let circuit_str = "123 -> x
        456 -> y
        x AND y -> d
        x OR y -> e
        x LSHIFT 2 -> f
        y RSHIFT 2 -> g
        NOT x -> h
        NOT y -> i";

        let expected: HashMap<_, _> = [
            ("d".to_owned(), Some(72)),
            ("e".to_owned(), Some(507)),
            ("f".to_owned(), Some(492)),
            ("g".to_owned(), Some(114)),
            ("h".to_owned(), Some(65412)),
            ("i".to_owned(), Some(65079)),
            ("x".to_owned(), Some(123)),
            ("y".to_owned(), Some(456)),
        ]
        .iter()
        .cloned()
        .collect();

        let (_, mut circuit) = Circuit::parse_circuit(circuit_str).unwrap();

        let mut counter = 0;
        circuit.evaluate(|c| {
            counter += 1;
            c.wires.values().any(Option::is_none) && counter < 100
        });

        assert_eq!(expected, circuit.wires);
    }
}
