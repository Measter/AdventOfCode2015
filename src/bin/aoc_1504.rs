use color_eyre::eyre::{eyre, Result};

use std::fmt::Write;

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

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = std::fs::read_to_string("inputs/aoc_1504.txt")?;

    aoc_lib::run(
        "Day 4: The Ideal Stocking Stuffer",
        input.as_str(),
        &|i| part(i, true),
        &|i| part(i, false),
    )
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
