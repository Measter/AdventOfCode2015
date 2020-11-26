use color_eyre::eyre::Result;

use std::fmt::Write;

fn part(input: &str, is_five: bool) -> u32 {
    let mut buf = String::new();

    for i in 0..u32::MAX {
        let mut ctx = md5::Context::new();
        ctx.consume(input);

        write!(&mut buf, "{}", i).unwrap();
        ctx.consume(&buf);

        let digest = ctx.compute();

        if (is_five && matches!(digest.0, [0, 0, 0..=0xF, ..])) || matches!(digest.0, [0, 0, 0, ..])
        {
            return i;
        }

        buf.clear();
    }

    0
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = std::fs::read_to_string("inputs/aoc_1504.txt")?;

    let start = std::time::Instant::now();

    let part_1 = part(&input, true);
    let part_2 = part(&input, false);

    let elapsed = start.elapsed();

    println!("Part 1 output: {}", part_1);
    println!("Part 2 output: {}", part_2);

    println!("Elapsed: {}ms", elapsed.as_millis());

    Ok(())
}

#[cfg(test)]
mod tests_1504 {
    use super::*;

    #[test]
    fn part1_examples() {
        let vals = [("abcdef", 609043), ("pqrstuv", 1048970)];

        for &(input, output) in &vals {
            assert_eq!(output, part(input, true));
        }
    }
}
