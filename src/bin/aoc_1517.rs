use color_eyre::eyre::Result;
use itertools::Itertools;

use std::num::ParseIntError;

fn part1(containers: &[u32], total_eggnog: u32) -> usize {
    let mut num_permutations = 0;

    for len in 1..=containers.len() {
        num_permutations += containers
            .iter()
            .combinations(len)
            .filter(|p| p.iter().map(|p| **p).sum::<u32>() == total_eggnog)
            .count();
    }

    num_permutations
}

fn part2(containers: &[u32], total_eggnog: u32) -> usize {
    for len in 1..=containers.len() {
        let count = containers
            .iter()
            .combinations(len)
            .filter(|p| p.iter().map(|p| **p).sum::<u32>() == total_eggnog)
            .count();

        if count > 0 {
            return count;
        }
    }

    0
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = std::fs::read_to_string("inputs/aoc_1517.txt")?;
    let containers: Vec<_> = input
        .lines()
        .map(str::trim)
        .map(str::parse)
        .collect::<Result<_, ParseIntError>>()?;

    let start = std::time::Instant::now();

    let part1 = part1(&containers, 150);
    let part2 = part2(&containers, 150);

    let elapsed = start.elapsed();

    println!("Part 1 output: {}", part1);
    println!("Part 2 output: {}", part2);

    println!("Elapsed: {}ms", elapsed.as_millis());

    Ok(())
}

#[cfg(test)]
mod tests_1517 {
    use super::*;

    #[test]
    fn part1_example() {
        let containers = [20, 15, 10, 5, 5];

        assert_eq!(4, part1(&containers, 25));
    }

    #[test]
    fn part2_example() {
        let containers = [20, 15, 10, 5, 5];

        assert_eq!(3, part2(&containers, 25));
    }
}
