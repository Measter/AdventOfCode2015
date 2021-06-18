use aoc_lib::TracingAlloc;
use color_eyre::eyre::{eyre, Result};

use std::collections::HashMap;

#[global_allocator]
static ALLOC: TracingAlloc = TracingAlloc::new();

#[derive(Debug, PartialEq, Clone, Copy)]
enum Input<'a> {
    Number(u16),
    Wire(&'a str),
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Component<'a> {
    Const {
        input: Input<'a>,
        output: &'a str,
    },
    And {
        input_a: Input<'a>,
        input_b: Input<'a>,
        output: &'a str,
    },
    Or {
        input_a: Input<'a>,
        input_b: Input<'a>,
        output: &'a str,
    },
    LShift {
        input_a: Input<'a>,
        input_b: Input<'a>,
        output: &'a str,
    },
    RShift {
        input_a: Input<'a>,
        input_b: Input<'a>,
        output: &'a str,
    },
    Not {
        input: Input<'a>,
        output: &'a str,
    },
}

#[derive(Debug, PartialEq, Clone)]
struct Circuit<'a> {
    wires: HashMap<&'a str, Option<u16>>,
    components: Vec<Component<'a>>,
}

impl<'a> Circuit<'a> {
    fn get_wire_val(&self, key: Input) -> Option<u16> {
        match key {
            Input::Number(n) => Some(n),
            Input::Wire(key) => self.wires.get(key).copied().flatten(),
        }
    }

    fn evaluate(&mut self, mut exit: impl FnMut(&Self) -> bool) {
        while exit(self) {
            for &comp in &self.components {
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

    fn parse_circuit(input: &'a str) -> Result<Circuit<'a>> {
        use nom::{
            branch::alt,
            bytes::complete::{tag, take_till1, take_while1},
            sequence::{separated_pair, tuple},
        };

        let mut wires = HashMap::new();
        let mut components = Vec::new();

        for line in input.lines().map(str::trim) {
            let (_, (component, output)) = separated_pair::<_, _, _, _, (), _, _, _>(
                take_till1(|c| c == '-'),
                tag("->"),
                take_while1(|_| true),
            )(line)?;

            let (_, component) = alt::<_, _, (), _>((
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

            let output = output.trim();
            wires.entry(output).or_default();

            let mut parse_input = |input: &'a str| {
                if let Ok(c) = input.parse() {
                    Input::Number(c)
                } else {
                    wires.entry(input).or_default();
                    Input::Wire(input)
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

        Ok(Circuit { wires, components })
    }
}

fn part_1(mut circuit: Circuit) -> Result<u16> {
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

fn part_2(mut circuit: Circuit) -> Result<u16> {
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

    let input = aoc_lib::input(2015, 7).open()?;
    let (circuit, parse_bench) =
        aoc_lib::bench(&ALLOC, "Parse", || Circuit::parse_circuit(&input))?;

    let (p1_res, p1_bench) = aoc_lib::bench(&ALLOC, "Part 1", || part_1(circuit.clone()))?;
    let (p2_res, p2_bench) = aoc_lib::bench(&ALLOC, "Part 2", || part_2(circuit.clone()))?;

    aoc_lib::display_results(
        "Day 7: Some Assembly Required",
        [(&"", parse_bench), (&p1_res, p1_bench), (&p2_res, p2_bench)],
    );

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
            ("d", Some(72)),
            ("e", Some(507)),
            ("f", Some(492)),
            ("g", Some(114)),
            ("h", Some(65412)),
            ("i", Some(65079)),
            ("x", Some(123)),
            ("y", Some(456)),
        ]
        .iter()
        .cloned()
        .collect();

        let mut circuit = Circuit::parse_circuit(circuit_str).unwrap();

        let mut counter = 0;
        circuit.evaluate(|c| {
            counter += 1;
            c.wires.values().any(Option::is_none) && counter < 100
        });

        assert_eq!(expected, circuit.wires);
    }
}
