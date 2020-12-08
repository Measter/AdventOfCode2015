use aoc_lib::TracingAlloc;
use color_eyre::eyre::{eyre, Result};
use itertools::Itertools;

use std::num::ParseIntError;

#[global_allocator]
static ALLOC: TracingAlloc = TracingAlloc::new();

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

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = std::fs::read_to_string("inputs/aoc_1517.txt")?;
    let containers: Vec<_> = input
        .lines()
        .map(str::trim)
        .map(str::parse)
        .collect::<Result<_, ParseIntError>>()?;

    aoc_lib::run(
        &ALLOC,
        "Day 17: No Such Thing as Too Much",
        &containers,
        &|c| part1(c, 150),
        &|c| part2(c, 150),
    )
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
