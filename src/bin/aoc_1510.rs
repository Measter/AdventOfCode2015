use color_eyre::eyre::Result;
use itertools::Itertools;

use std::fmt::Write;

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

    let start = std::time::Instant::now();

    let part1 = looksay(input.to_owned(), 40).len();
    let part2 = looksay(input, 50).len();

    let elapsed = start.elapsed();

    println!("Part 1 output: {:?}", part1);
    println!("Part 2 output: {:?}", part2);

    println!("Elapsed: {}ms", elapsed.as_millis());

    Ok(())
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
