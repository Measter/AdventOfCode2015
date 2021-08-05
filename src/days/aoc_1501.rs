#![allow(clippy::unnecessary_wraps)]

use aoc_lib::{day, Bench, BenchResult};
use color_eyre::eyre::{eyre, Result};

day! {
    day 1: "Not Quite Lisp"
    1: run_part1
    2: run_part2
}

fn run_part1(input: &str, b: Bench) -> BenchResult {
    b.bench(|| part1(input))
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    b.bench(|| part2(input))
}

fn part1(input: &str) -> Result<i64> {
    Ok(input
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        })
        .sum())
}

fn part2(input: &str) -> Result<i64> {
    input
        .chars()
        .zip(1..)
        .scan(0, |floor, (c, position)| {
            *floor += match c {
                '(' => 1,
                ')' => -1,
                _ => 0,
            };

            Some((*floor, position))
        })
        .skip_while(|(floor, _)| *floor > -1)
        .map(|(_, pos)| pos)
        .next()
        .ok_or_else(|| eyre!("Unable to find result"))
}

#[cfg(test)]
mod tests_1501 {
    use super::*;

    #[test]
    fn part1_examples() {
        let tests = [
            (0, "(())"),
            (0, "()()"),
            (3, "((("),
            (3, "(()(()("),
            (3, "))((((("),
            (-1, "())"),
            (-1, "))("),
            (-3, ")))"),
            (-3, ")())())"),
        ];

        for (i, (expected, test)) in tests.iter().enumerate() {
            assert_eq!(part1(test).unwrap(), *expected, "{}", i);
        }
    }

    #[test]
    fn part2_examples() {
        assert_eq!(1, part2(")").unwrap());

        assert_eq!(5, part2("()())").unwrap());
    }
}
