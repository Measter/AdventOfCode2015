use aoc_lib::{Bench, BenchResult, Day, NoError};
use itertools::Itertools;
use itoa::Buffer;

pub const DAY: Day = Day {
    day: 10,
    name: "Elves Look, Elves Say",
    part_1: run_part1,
    part_2: Some(run_part2),
    other: &[],
};

fn run_part1(input: &str, b: Bench) -> BenchResult {
    b.bench(|| Ok::<_, NoError>(looksay(input.to_owned(), 40).len()))
}
fn run_part2(input: &str, b: Bench) -> BenchResult {
    b.bench(|| Ok::<_, NoError>(looksay(input.to_owned(), 50).len()))
}

fn looksay(input: String, iterations: usize) -> String {
    let mut buf_a = input;
    let mut buf_b = String::new();
    let mut fmt_buf = Buffer::new();

    for _ in 0..iterations {
        for (ch, run) in &buf_a.chars().group_by(|c| *c) {
            buf_b.push_str(fmt_buf.format(run.count()));
            buf_b.push(ch);
        }

        std::mem::swap(&mut buf_a, &mut buf_b);
        buf_b.clear();
    }

    buf_a
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
