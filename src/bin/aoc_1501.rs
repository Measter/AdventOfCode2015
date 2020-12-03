#![allow(clippy::unnecessary_wraps)]

use advent_of_code_2015::run;
use color_eyre::eyre::{eyre, Result};

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

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = std::fs::read_to_string("inputs/aoc_1501.txt")?;

    run("Day 1: Not Quite Lisp", input.as_str(), &[&part1, &part2])
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
        assert_eq!(1, part2(&")").unwrap());

        assert_eq!(5, part2(&"()())").unwrap());
    }
}
