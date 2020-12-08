use aoc_lib::TracingAlloc;
use color_eyre::eyre::{eyre, Result};

#[global_allocator]
static ALLOC: TracingAlloc = TracingAlloc::new();

fn part1(num_presents: usize) -> Result<usize> {
    let mut houses = vec![0; (num_presents / 10) + 1];

    for elf in 1..=(num_presents / 10) {
        (1..)
            .map(|m| m * elf)
            .take_while(|&house| house <= (num_presents / 10))
            .for_each(|house| houses[house] += elf * 10);
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

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = std::fs::read_to_string("inputs/aoc_1520.txt")?;
    let input = input.trim().parse::<usize>()?;

    aoc_lib::run(
        &ALLOC,
        "Day 20: Infinite Elves and Infinite Houses",
        input,
        &|i| part1(i),
        &|i| part2(i),
    )
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
