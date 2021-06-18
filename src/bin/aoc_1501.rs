#![allow(clippy::unnecessary_wraps)]

use aoc_lib::TracingAlloc;
use color_eyre::eyre::{eyre, Result};

#[global_allocator]
static ALLOC: TracingAlloc = TracingAlloc::new();

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

    let input = aoc_lib::input(2015, 1).open()?;
    let (p1_res, p1_bench) = aoc_lib::bench(&ALLOC, "Part 1", || part1(&input))?;
    let (p2_res, p2_bench) = aoc_lib::bench(&ALLOC, "Part 2", || part2(&input))?;

    aoc_lib::display_results(
        "Day 1: Not Quite Lisp",
        [(&p1_res, p1_bench), (&p2_res, p2_bench)],
    );

    Ok(())
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
