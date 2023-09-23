use aoc_lib::{Bench, BenchResult, Day, ParseResult, UserError};
use chumsky::Parser;
use color_eyre::{
    eyre::{eyre, Result},
    Report,
};

use std::{cell::RefCell, collections::HashMap};

pub const DAY: Day = Day {
    day: 7,
    name: "Some Assembly Required",
    part_1: run_part1,
    part_2: Some(run_part2),
    other: &[("Parse", run_parse)],
};
fn run_part1(input: &str, b: Bench) -> BenchResult {
    let circuit = Circuit::parse_circuit(input).map_err(UserError)?;
    b.bench(|| part_1(circuit.clone()))
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    let circuit = Circuit::parse_circuit(input).map_err(UserError)?;
    b.bench(|| part_2(circuit.clone()))
}

fn run_parse(input: &str, b: Bench) -> BenchResult {
    b.bench(|| {
        let data = Circuit::parse_circuit(input)?;
        Ok::<_, Report>(ParseResult(data))
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct WireId(u16);

#[derive(Debug, PartialEq, Clone, Copy)]
enum Input {
    Number(u16),
    Wire(WireId),
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Component {
    Const {
        input: Input,
        output: WireId,
    },
    And {
        input_a: Input,
        input_b: Input,
        output: WireId,
    },
    Or {
        input_a: Input,
        input_b: Input,
        output: WireId,
    },
    LShift {
        input_a: Input,
        input_b: Input,
        output: WireId,
    },
    RShift {
        input_a: Input,
        input_b: Input,
        output: WireId,
    },
    Not {
        input: Input,
        output: WireId,
    },
}

#[derive(Debug, PartialEq, Clone)]
struct Circuit {
    wires: Vec<Option<u16>>,
    components: Vec<Component>,
    a_id: WireId,
    b_id: WireId,
}

impl Circuit {
    fn get_wire_val(&self, key: Input) -> Option<u16> {
        match key {
            Input::Number(n) => Some(n),
            Input::Wire(key) => self.wires[key.0 as usize],
        }
    }

    fn evaluate(&mut self, mut exit: impl FnMut(&Self) -> bool) {
        while exit(self) {
            for &comp in &self.components {
                use Component::*;
                match comp {
                    Const { input, output } => {
                        if let (Some(val), output @ None) =
                            (self.get_wire_val(input), &mut self.wires[output.0 as usize])
                        {
                            *output = Some(val);
                        }
                    }

                    Not { input, output } => {
                        if let (Some(val), output @ None) =
                            (self.get_wire_val(input), &mut self.wires[output.0 as usize])
                        {
                            *output = Some(!val);
                        }
                    }

                    And {
                        input_a,
                        input_b,
                        output,
                    } => {
                        if let (Some(a), Some(b), output @ None) = (
                            self.get_wire_val(input_a),
                            self.get_wire_val(input_b),
                            &mut self.wires[output.0 as usize],
                        ) {
                            *output = Some(a & b);
                        }
                    }

                    Or {
                        input_a,
                        input_b,
                        output,
                    } => {
                        if let (Some(a), Some(b), output @ None) = (
                            self.get_wire_val(input_a),
                            self.get_wire_val(input_b),
                            &mut self.wires[output.0 as usize],
                        ) {
                            *output = Some(a | b);
                        }
                    }

                    LShift {
                        input_a,
                        input_b,
                        output,
                    } => {
                        if let (Some(a), Some(b), output @ None) = (
                            self.get_wire_val(input_a),
                            self.get_wire_val(input_b),
                            &mut self.wires[output.0 as usize],
                        ) {
                            *output = Some(a << b);
                        }
                    }

                    RShift {
                        input_a,
                        input_b,
                        output,
                    } => {
                        if let (Some(a), Some(b), output @ None) = (
                            self.get_wire_val(input_a),
                            self.get_wire_val(input_b),
                            &mut self.wires[output.0 as usize],
                        ) {
                            *output = Some(a >> b);
                        }
                    }
                }
            }
        }
    }

    fn parse_circuit(input: &str) -> Result<Circuit> {
        let wire_ids = RefCell::new(HashMap::new());

        let get_wire_id = |s: &str| -> WireId {
            let mut wire_ids = wire_ids.borrow_mut();
            if let Some(id) = wire_ids.get(s) {
                return *id;
            }

            let id = WireId(wire_ids.len() as u16);
            wire_ids.insert(s.to_owned(), id);
            id
        };

        let mut components = Vec::new();
        let a_id = get_wire_id("a");
        let b_id = get_wire_id("b");

        for line in input.lines().map(str::trim) {
            let (input, output) = line
                .split_once(" -> ")
                .ok_or_else(|| eyre!("Unable to parse `{line:?}`"))?;

            fn component_parser<'a, 'b: 'a>(
                output: WireId,
                id_gen: &'b impl Fn(&'a str) -> WireId,
            ) -> impl Parser<'a, &'a str, Component> {
                use chumsky::{
                    primitive::just,
                    text::{ident, int},
                };

                let num_constant = int(10).from_str::<u16>().unwrapped().map(Input::Number);
                let wire_name = ident().map(id_gen).map(Input::Wire);
                let input = num_constant.or(wire_name);

                let not = just("NOT ")
                    .ignore_then(input)
                    .map(move |input| Component::Not { input, output });

                let and = input.then_ignore(just("AND").padded()).then(input).map(
                    move |(input_a, input_b)| Component::And {
                        input_a,
                        input_b,
                        output,
                    },
                );

                let or = input.then_ignore(just("OR").padded()).then(input).map(
                    move |(input_a, input_b)| Component::Or {
                        input_a,
                        input_b,
                        output,
                    },
                );

                let lshift = input.then_ignore(just("LSHIFT").padded()).then(input).map(
                    move |(input_a, input_b)| Component::LShift {
                        input_a,
                        input_b,
                        output,
                    },
                );

                let rshift = input.then_ignore(just("RSHIFT").padded()).then(input).map(
                    move |(input_a, input_b)| Component::RShift {
                        input_a,
                        input_b,
                        output,
                    },
                );

                not.or(and)
                    .or(or)
                    .boxed()
                    .or(lshift)
                    .or(rshift)
                    .boxed()
                    .or(input.map(move |input| Component::Const { input, output }))
            }

            let output_id = get_wire_id(output.trim());
            let component = component_parser(output_id, &get_wire_id)
                .parse(input.trim())
                .into_output()
                .ok_or_else(|| eyre!("Unable to parse `{line:?}`"))?;

            components.push(component);
        }

        let wire_ids = wire_ids.into_inner();
        Ok(Circuit {
            wires: vec![None; wire_ids.len()],
            components,
            a_id,
            b_id,
        })
    }
}

fn part_1(mut circuit: Circuit) -> Result<u16> {
    let mut counter = 0;
    circuit.evaluate(|c| {
        counter += 1;
        c.wires.iter().any(Option::is_none) && counter < 1000
    });

    circuit.wires[circuit.a_id.0 as usize].ok_or_else(|| eyre!("Wire not found: a"))
}

fn part_2(mut circuit: Circuit) -> Result<u16> {
    circuit.wires[circuit.b_id.0 as usize] = Some(46065);

    let mut counter = 0;
    circuit.evaluate(|c| {
        counter += 1;
        c.wires.iter().any(Option::is_none) && counter < 1000
    });

    circuit.wires[circuit.a_id.0 as usize].ok_or_else(|| eyre!("Wire not found: a"))
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

        let expected: Vec<Option<u16>> = vec![
            None,        // a
            None,        // b
            Some(123),   // x
            Some(456),   // y
            Some(72),    // d
            Some(507),   // e
            Some(492),   // f
            Some(114),   // g
            Some(65412), // h
            Some(65079), // i
        ];

        let mut circuit = Circuit::parse_circuit(circuit_str).unwrap();

        let mut counter = 0;
        circuit.evaluate(|c| {
            counter += 1;
            c.wires.iter().any(Option::is_none) && counter < 100
        });

        assert_eq!(expected, circuit.wires);
    }
}
