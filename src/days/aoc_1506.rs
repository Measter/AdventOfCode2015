#![allow(clippy::unnecessary_wraps)]

use aoc_lib::{Bench, BenchResult, Day, ParseResult, UserError};
use color_eyre::{
    eyre::{eyre, Result},
    Report,
};

pub const DAY: Day = Day {
    day: 6,
    name: "Probably a Fire Hazard",
    part_1: run_part1,
    part_2: Some(run_part2),
    other: &[("Parse", run_parse)],
};

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let instructions: Vec<_> = input
        .lines()
        .map(Instruction::parse)
        .collect::<Result<_, _>>()
        .map_err(UserError)?;

    b.bench(|| part(&instructions, Operation::apply_part1))
}
fn run_part2(input: &str, b: Bench) -> BenchResult {
    let instructions: Vec<_> = input
        .lines()
        .map(Instruction::parse)
        .collect::<Result<_, _>>()
        .map_err(UserError)?;

    b.bench(|| part(&instructions, Operation::apply_part2))
}
fn run_parse(input: &str, b: Bench) -> BenchResult {
    b.bench(|| {
        let data: Vec<_> = input
            .lines()
            .map(Instruction::parse)
            .collect::<Result<_, _>>()?;
        Ok::<_, Report>(ParseResult(data))
    })
}

#[derive(Debug, Clone, PartialEq)]
struct Instruction {
    op: Operation,
    top: usize,
    left: usize,
    bottom: usize,
    right: usize,
}

impl Instruction {
    fn parse(line: &str) -> Result<Instruction> {
        use nom::{
            branch::alt,
            bytes::complete::{tag, take_while1},
            sequence::separated_pair,
        };

        let (rest, op_type) =
            alt::<_, _, (), _>((tag("toggle"), tag("turn on"), tag("turn off")))(line)?;

        let (rest, (top, left)) = separated_pair::<_, _, _, _, (), _, _, _>(
            take_while1(char::is_numeric),
            tag(","),
            take_while1(char::is_numeric),
        )(rest.trim())?;

        let (rest, _) = tag::<_, _, ()>(" through ")(rest)?;

        let (_, (bottom, right)) = separated_pair::<_, _, _, _, (), _, _, _>(
            take_while1(char::is_numeric),
            tag(","),
            take_while1(char::is_numeric),
        )(rest.trim())?;

        let top = top.parse()?;
        let left = left.parse()?;
        let bottom = bottom.parse()?;
        let right = right.parse()?;

        let op = match op_type {
            "turn off" => Operation::Off,
            "turn on" => Operation::On,
            "toggle" => Operation::Toggle,
            _ => return Err(eyre!("Invalid operation type: {}", op_type)),
        };

        Ok(Instruction {
            op,
            top,
            left,
            bottom,
            right,
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Operation {
    Toggle,
    On,
    Off,
}

impl Operation {
    fn apply_part1(self, value: &mut bool) {
        use Operation::*;
        match self {
            Toggle => *value = !*value,
            On => *value = true,
            Off => *value = false,
        }
    }

    fn apply_part2(self, value: &mut u64) {
        use Operation::*;
        match self {
            Toggle => *value += 2,
            On => *value += 1,
            Off => *value = value.saturating_sub(1),
        }
    }
}

fn part<T: Copy + Default>(input: &[Instruction], f: fn(Operation, &mut T)) -> Result<u64>
where
    u64: From<T>,
{
    let mut light_array = vec![T::default(); 1000 * 1000];

    for inst in input {
        light_array
            .chunks_exact_mut(1000)
            .skip(inst.top)
            .take(inst.bottom - inst.top + 1)
            .for_each(|row| {
                row.iter_mut()
                    .skip(inst.left)
                    .take(inst.right - inst.left + 1)
                    .for_each(|light| f(inst.op, light))
            });
    }

    Ok(light_array.into_iter().map(u64::from).sum())
}

#[cfg(test)]
mod tests_1506 {
    use super::*;

    #[test]
    fn parse_test() {
        let vals = [
            (
                "turn on 0,0 through 999,999",
                Instruction {
                    op: Operation::On,
                    top: 0,
                    left: 0,
                    bottom: 999,
                    right: 999,
                },
            ),
            (
                "toggle 0,0 through 999,0",
                Instruction {
                    op: Operation::Toggle,
                    top: 0,
                    left: 0,
                    bottom: 999,
                    right: 0,
                },
            ),
            (
                "turn off 499,499 through 500,500",
                Instruction {
                    op: Operation::Off,
                    top: 499,
                    left: 499,
                    bottom: 500,
                    right: 500,
                },
            ),
        ];

        for (input, output) in &vals {
            assert_eq!(&Instruction::parse(input).unwrap(), output, "{}", input);
        }
    }
}
