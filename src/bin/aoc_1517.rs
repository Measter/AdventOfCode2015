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

    let input = aoc_lib::input(2015, 17).open()?;
    let (containers, parse_bench) = aoc_lib::bench(&ALLOC, "Parse", || {
        input
            .lines()
            .map(str::trim)
            .map(str::parse)
            .collect::<Result<Vec<_>, ParseIntError>>()
    })?;

    let (p1_res, p1_bench) = aoc_lib::bench(&ALLOC, "Part 1", || part1(&containers, 150))?;
    let (p2_res, p2_bench) = aoc_lib::bench(&ALLOC, "Part 2", || part2(&containers, 150))?;

    aoc_lib::display_results(
        "Day 17: No Such Thing as Too Much",
        [(&"", parse_bench), (&p1_res, p1_bench), (&p2_res, p2_bench)],
    );

    Ok(())
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
