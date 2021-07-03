use aoc_lib::{day, Bench, BenchResult, NoError};
use itertools::Itertools;

day! {
    day 10: "Elves Look, Elves Say"
    1: run_part1
    2: run_part2
}

fn run_part1(input: &str, b: Bench) -> BenchResult {
    b.bench(|| Ok::<_, NoError>(looksay(input.to_owned(), 40).len()))
}
fn run_part2(input: &str, b: Bench) -> BenchResult {
    b.bench(|| Ok::<_, NoError>(looksay(input.to_owned(), 50).len()))
}

fn looksay(input: String, iterations: usize) -> String {
    let mut buf_a = input;
    let mut buf_b = String::new();

    for _ in 0..iterations {
        for (ch, run) in &buf_a.chars().group_by(|c| *c) {
            itoa::fmt(&mut buf_b, run.count()).unwrap();
            buf_b.push(ch);
            //write!(&mut buf_b, "{}{}", run.count(), ch).unwrap();
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
