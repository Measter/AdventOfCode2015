use std::ops::{Index, IndexMut};

use aoc_lib::{Bench, BenchResult, Day, NoError, UserError};
use color_eyre::eyre::{eyre, Context, Result};

pub const DAY: Day = Day {
    day: 23,
    name: "Opening the Turing Lock",
    part_1: run_part1,
    part_2: Some(run_part2),
    other: &[],
};

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let instructions: Vec<_> = input
        .lines()
        .map(str::trim)
        .map(Instruction::parse)
        .collect::<Result<_, _>>()
        .map_err(UserError)?;

    b.bench(|| {
        let mut computer = Computer::default();
        computer.run_program(&instructions);
        Ok::<_, NoError>(computer.registers.b)
    })
}
fn run_part2(input: &str, b: Bench) -> BenchResult {
    let instructions: Vec<_> = input
        .lines()
        .map(str::trim)
        .map(Instruction::parse)
        .collect::<Result<_, _>>()
        .map_err(UserError)?;

    b.bench(|| {
        let mut computer = Computer::default();
        computer.registers.a = 1;
        computer.run_program(&instructions);
        Ok::<_, NoError>(computer.registers.b)
    })
}

const SEPARATORS: &[char] = &[' ', ','];
const OFFSET_PREFIX: &[char] = &['-', '+'];

#[derive(Debug, Copy, Clone, PartialEq)]
enum Register {
    A,
    B,
}

impl Register {
    fn parse(input: &str) -> Result<Register> {
        match input.trim() {
            "a" => Ok(Register::A),
            "b" => Ok(Register::B),
            c => Err(eyre!("Unknown register: {}", c)),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Instruction {
    Half(Register),
    Triple(Register),
    Increment(Register),
    Jump(isize),
    JumpIfEven(Register, isize),
    JumpIfOne(Register, isize),
}

impl Instruction {
    fn parse(input: &str) -> Result<Instruction> {
        let mut parts = input.trim().split(SEPARATORS).filter(|s| !s.is_empty());

        let instr_name = parts.next();
        let op_a = parts.next();
        let op_b = parts.next();

        let dir = |d| -> isize {
            match d {
                "+" => 1,
                "-" => -1,
                _ => unreachable!(),
            }
        };

        let instr = match (instr_name, op_a, op_b) {
            (Some("hlf"), Some(reg), None) => Instruction::Half(Register::parse(reg)?),
            (Some("tpl"), Some(reg), None) => Instruction::Triple(Register::parse(reg)?),
            (Some("inc"), Some(reg), None) => Instruction::Increment(Register::parse(reg)?),
            (Some("jmp"), Some(offset), None) if offset.starts_with(OFFSET_PREFIX) => {
                Instruction::Jump(
                    dir(&offset[..1])
                        * offset[1..]
                            .parse::<isize>()
                            .with_context(|| eyre!("Error parsing instruction: {}", input))?,
                )
            }
            (Some("jie"), Some(reg), Some(offset)) if offset.starts_with(OFFSET_PREFIX) => {
                Instruction::JumpIfEven(
                    Register::parse(reg)?,
                    dir(&offset[..1])
                        * offset[1..]
                            .parse::<isize>()
                            .with_context(|| eyre!("Error parsing instruction: {}", input))?,
                )
            }
            (Some("jio"), Some(reg), Some(offset)) if offset.starts_with(OFFSET_PREFIX) => {
                Instruction::JumpIfOne(
                    Register::parse(reg)?,
                    dir(&offset[..1])
                        * offset[1..]
                            .parse::<isize>()
                            .with_context(|| eyre!("Error parsing instruction: {}", input))?,
                )
            }

            _ => return Err(eyre!("Unknown instruction: {}", input)),
        };

        Ok(instr)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
struct RegisterFile {
    a: u64,
    b: u64,
}

impl Index<Register> for RegisterFile {
    type Output = u64;

    fn index(&self, index: Register) -> &Self::Output {
        match index {
            Register::A => &self.a,
            Register::B => &self.b,
        }
    }
}

impl IndexMut<Register> for RegisterFile {
    fn index_mut(&mut self, index: Register) -> &mut Self::Output {
        match index {
            Register::A => &mut self.a,
            Register::B => &mut self.b,
        }
    }
}

#[derive(Debug, Clone, Default)]
struct Computer {
    registers: RegisterFile,
}

impl Computer {
    fn run_program(&mut self, program: &[Instruction]) {
        let mut pc: usize = 0;

        while let Some(&instr) = program.get(pc) {
            match instr {
                Instruction::Half(reg) => self.registers[reg] /= 2,
                Instruction::Triple(reg) => self.registers[reg] *= 3,
                Instruction::Increment(reg) => self.registers[reg] += 1,
                Instruction::Jump(offset) => {
                    pc = (pc as isize + offset) as usize;
                    continue;
                }
                Instruction::JumpIfEven(reg, offset) if self.registers[reg] % 2 == 0 => {
                    pc = (pc as isize + offset) as usize;
                    continue;
                }
                Instruction::JumpIfEven(_, _) => {}
                Instruction::JumpIfOne(reg, offset) if self.registers[reg] == 1 => {
                    pc = (pc as isize + offset) as usize;
                    continue;
                }
                Instruction::JumpIfOne(_, _) => {}
            }

            pc += 1;
        }
    }
}

#[cfg(test)]
mod tests_1523 {
    use super::*;

    #[test]
    fn instruction_parse_test() {
        let tests = [
            ("hlf a", Instruction::Half(Register::A)),
            ("hlf b", Instruction::Half(Register::B)),
            ("tpl a", Instruction::Triple(Register::A)),
            ("inc b", Instruction::Increment(Register::B)),
            ("jmp +2", Instruction::Jump(2)),
            ("jmp -2", Instruction::Jump(-2)),
            ("jie a, +2", Instruction::JumpIfEven(Register::A, 2)),
            ("jio b, -2", Instruction::JumpIfOne(Register::B, -2)),
        ];

        for &(instr, expected) in &tests {
            assert_eq!(Instruction::parse(instr).unwrap(), expected);
        }
    }

    #[test]
    fn part1_example() {
        let input = "inc a
        jio a, +2
        tpl a
        inc a";

        let program: Vec<_> = input
            .lines()
            .map(str::trim)
            .map(Instruction::parse)
            .collect::<Result<_>>()
            .unwrap();

        let mut computer = Computer::default();
        computer.run_program(&program);

        assert_eq!(computer.registers.a, 2);
    }
}
