use aoc_lib::TracingAlloc;
use color_eyre::eyre::Result;
use itertools::Itertools;

use std::fmt::Write;

#[global_allocator]
static ALLOC: TracingAlloc = TracingAlloc::new();

fn looksay(input: String, iterations: usize) -> String {
    let mut buf_a = input;
    let mut buf_b = String::new();

    for _ in 0..iterations {
        for (ch, run) in &buf_a.chars().group_by(|c| *c) {
            write!(&mut buf_b, "{}{}", run.count(), ch).unwrap();
        }

        std::mem::swap(&mut buf_a, &mut buf_b);
        buf_b.clear();
    }

    buf_a
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = std::fs::read_to_string("inputs/aoc_1510.txt")?;

    aoc_lib::run(
        &ALLOC,
        "Day 10: Elves Look, Elves Say",
        input.as_str(),
        &|i| Ok(looksay(i.to_owned(), 40).len()),
        &|i| Ok(looksay(i.to_owned(), 50).len()),
    )
}

#[cfg(test)]
mod tests_1510 {
    use super::*;

    #[test]
    fn part1_example() {
        let tests = [
            ("1", "11", 1),
            ("11", "21", 1),
            ("21", "1211", 1),
            ("1211", "111221", 1),
            ("111221", "312211", 1),
            ("1", "312211", 5),
        ];

        for (i, &(start, end, iters)) in tests.iter().enumerate() {
            assert_eq!(looksay(start.to_string(), iters), end, "{}", i);
        }
    }
}
