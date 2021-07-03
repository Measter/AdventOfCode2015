use aoc_lib::{day, Bench, BenchResult, UserError};
use color_eyre::eyre::{eyre, Result};

day! {
    day 20: "Infinite Elves and Infinite Houses"
    1: run_part1
    2: run_part2
}

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let input = input.trim().parse::<usize>().map_err(UserError)?;
    b.bench(|| part1(input))
}
fn run_part2(input: &str, b: Bench) -> BenchResult {
    let input = input.trim().parse::<usize>().map_err(UserError)?;
    b.bench(|| part2(input))
}

fn part1(num_presents: usize) -> Result<usize> {
    let num_presents = num_presents / 10;
    let mut houses = vec![0; num_presents + 1];

    for elf in 1..=(num_presents) {
        (1..)
            .map(|m| m * elf)
            .take_while(|&house| house <= num_presents)
            .for_each(|house| houses[house] += elf);
    }

    houses
        .iter()
        .enumerate()
        .find(|(_, h)| **h >= num_presents)
        .map(|(house, _)| house)
        .ok_or_else(|| eyre!("Unable to find result"))
}

fn house_presents_part2(house: usize) -> usize {
    let mut num_presents = 0;

    for div in (1..=50).filter(|&e| house % e == 0).map(|e| house / e) {
        num_presents += div;
    }

    num_presents * 11
}

fn part2(num_presents: usize) -> Result<usize> {
    (1..)
        .find(|&h| house_presents_part2(h) >= num_presents)
        .ok_or_else(|| eyre!("Unable to find result"))
}

#[cfg(test)]
mod tests_1520 {
    use super::*;

    #[test]
    fn part1_example() {
        let tests = [(1, 10), (2, 30), (3, 40), (4, 70), (6, 120), (8, 150)];

        for (i, (expected, num_presents)) in tests.iter().enumerate() {
            assert_eq!(part1(*num_presents).unwrap(), *expected, "{}", i);
        }
    }
}
