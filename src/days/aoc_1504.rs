use aoc_lib::{day, Bench, BenchResult};
use color_eyre::eyre::{eyre, Result};

use std::fmt::Write;

day! {
    day 4: "THe Ideal Stocking Stuffer"
    1: run_part1
    2: run_part2
}

fn run_part1(input: &str, b: Bench) -> BenchResult {
    b.bench(|| part(input, true))
}
fn run_part2(input: &str, b: Bench) -> BenchResult {
    b.bench(|| part(input, false))
}

fn part(input: &str, is_five: bool) -> Result<u32> {
    let mut buf = String::new();

    for i in 0..u32::MAX {
        let mut ctx = md5::Context::new();
        ctx.consume(input);

        write!(&mut buf, "{}", i).unwrap();
        ctx.consume(&buf);

        let digest = ctx.compute();

        if (is_five && matches!(digest.0, [0, 0, 0..=0xF, ..])) || matches!(digest.0, [0, 0, 0, ..])
        {
            return Ok(i);
        }

        buf.clear();
    }

    Err(eyre!("Unable to find result"))
}

#[cfg(test)]
mod tests_1504 {
    use super::*;

    #[test]
    fn part1_examples() {
        let vals = [("abcdef", 609043), ("pqrstuv", 1048970)];

        for &(input, output) in &vals {
            assert_eq!(output, part(input, true).unwrap());
        }
    }
}
