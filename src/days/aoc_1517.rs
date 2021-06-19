use aoc_lib::{day, Bench, BenchError, BenchResult};
use color_eyre::eyre::{eyre, Result};
use itertools::Itertools;

use std::num::ParseIntError;

day! {
    day 17: "No Such Thing as Too Much"
    1: run_part1
    2: run_part2
}

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let containers: Vec<_> = input
        .lines()
        .map(str::trim)
        .map(str::parse)
        .collect::<Result<_, ParseIntError>>()
        .map_err(|e| BenchError::UserError(e.into()))?;

    b.bench(|| part1(&containers, 150))
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    let containers: Vec<_> = input
        .lines()
        .map(str::trim)
        .map(str::parse)
        .collect::<Result<_, ParseIntError>>()
        .map_err(|e| BenchError::UserError(e.into()))?;

    b.bench(|| part2(&containers, 150))
}

fn part1(containers: &[u32], total_eggnog: u32) -> Result<usize> {
    let mut num_permutations = 0;

    for len in 1..=containers.len() {
        num_permutations += containers
            .iter()
            .combinations(len)
            .filter(|p| p.iter().map(|p| **p).sum::<u32>() == total_eggnog)
            .count();
    }

    if num_permutations == 0 {
        Err(eyre!("No result found"))
    } else {
        Ok(num_permutations)
    }
}

fn part2(containers: &[u32], total_eggnog: u32) -> Result<usize> {
    for len in 1..=containers.len() {
        let count = containers
            .iter()
            .combinations(len)
            .filter(|p| p.iter().map(|p| **p).sum::<u32>() == total_eggnog)
            .count();

        if count > 0 {
            return Ok(count);
        }
    }

    Err(eyre!("No result found"))
}

#[cfg(test)]
mod tests_1517 {
    use super::*;

    #[test]
    fn part1_example() {
        let containers = [20, 15, 10, 5, 5];

        assert_eq!(4, part1(&containers, 25).unwrap());
    }

    #[test]
    fn part2_example() {
        let containers = [20, 15, 10, 5, 5];

        assert_eq!(3, part2(&containers, 25).unwrap());
    }
}
