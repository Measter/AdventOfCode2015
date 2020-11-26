use color_eyre::eyre::{eyre, Result};

fn part1(input: &str) -> i64 {
    input
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        })
        .sum()
}

fn part2(input: &str) -> Option<i64> {
    input
        .chars()
        .zip(1..)
        .scan(0, |floor, (c, position)| {
            *floor += match c {
                '(' => 1,
                ')' => -1,
                _ => 0,
            };

            Some((*floor, position))
        })
        .skip_while(|(floor, _)| *floor > -1)
        .map(|(_, pos)| pos)
        .next()
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = std::fs::read_to_string("inputs/aoc_1501.txt")?;

    let start = std::time::Instant::now();

    let part_1: i64 = part1(&input);
    let part_2 = part2(&input).ok_or_else(|| eyre!("Part 2 result not found"))?;

    let elapsed = start.elapsed();

    println!("Part 1 output: {}", part_1);
    println!("Part 2 output: {}", part_2);

    println!("Elapsed: {}us", elapsed.as_micros());

    Ok(())
}

#[cfg(test)]
mod tests_1501 {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(0, part1("(())"));
        assert_eq!(0, part1("()()"));

        assert_eq!(3, part1("((("));
        assert_eq!(3, part1("(()(()("));

        assert_eq!(3, part1("))((((("));

        assert_eq!(-1, part1("())"));
        assert_eq!(-1, part1("))("));

        assert_eq!(-3, part1(")))"));
        assert_eq!(-3, part1(")())())"));
    }

    #[test]
    fn part2_examples() {
        assert_eq!(Some(1), part2(")"));

        assert_eq!(Some(5), part2("()())"));
    }
}
