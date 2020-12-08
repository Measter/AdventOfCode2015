#![allow(clippy::unnecessary_wraps)]

use aoc_lib::TracingAlloc;
use color_eyre::eyre::{eyre, Result};

use std::collections::{HashMap, HashSet};

#[global_allocator]
static ALLOC: TracingAlloc = TracingAlloc::new();

fn parse_input(input: &str) -> Result<(HashMap<&str, Vec<&str>>, &str)> {
    let mut mappings = HashMap::new();

    let mut lines = input.lines().map(str::trim);
    for line in (&mut lines).take_while(|l| !l.is_empty()) {
        use nom::{
            bytes::complete::{tag, take_while1},
            sequence::tuple,
        };

        let (_, (from, _, to)) = tuple::<_, _, (), _>((
            take_while1(char::is_alphabetic),
            tag(" => "),
            take_while1(char::is_alphabetic),
        ))(line)
        .map_err(|_| eyre!("Error parsing mapping"))?;

        mappings.entry(from).or_insert_with(Vec::new).push(to);
    }

    let input = lines.next().ok_or_else(|| eyre!("Expected input"))?;

    Ok((mappings, input))
}

fn part1(mappings: &HashMap<&str, Vec<&str>>, input: &str) -> Result<usize> {
    let mut seen = HashSet::new();

    for (from, tos) in mappings {
        for &to in tos {
            for (idx, _) in input.match_indices(from) {
                let mut result = String::with_capacity(input.len() + 20);

                result.push_str(&input[..idx]);
                result.push_str(to);
                result.push_str(&input[idx + from.len()..]);

                seen.insert(result);
            }
        }
    }

    Ok(seen.len())
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = std::fs::read_to_string("inputs/aoc_1519.txt")?;
    let (mappings, input) = parse_input(&input)?;

    aoc_lib::run(
        &ALLOC,
        "Day 19: Medicine for Rudolph",
        (&mappings, input),
        &|(m, i)| part1(m, i),
        &|_| Ok("Not implemented"),
    )
}

#[cfg(test)]
mod tests_1519 {
    use super::*;
    use maplit::hashmap;

    #[test]
    fn parse_test() {
        let input = "H => HO
        H => OH
        O => HH
        
        HOH";

        let expected_mapping = hashmap![
            "H" => vec!["HO", "OH"],
            "O" => vec!["HH"]
        ];

        let expected_input = "HOH";

        let (actual_mapping, actual_input) = parse_input(input).unwrap();

        assert_eq!(actual_input, expected_input);
        assert_eq!(actual_mapping, expected_mapping);
    }

    #[test]
    fn part1_example() {
        let mappings = hashmap![
            "H" => vec!["HO", "OH"],
            "O" => vec!["HH"]
        ];

        let tests = [("HOH", 4), ("HOHOHO", 7)];

        for &(test, expected) in &tests {
            assert_eq!(part1(&mappings, test).unwrap(), expected, "{}", test);
        }
    }
}
