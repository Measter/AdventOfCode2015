#![allow(clippy::unnecessary_wraps)]

use color_eyre::eyre::{eyre, Result};
use itertools::Itertools;

use std::collections::{BTreeSet, HashMap};

#[derive(Debug, PartialEq, Clone)]
struct People<'a> {
    happiness: HashMap<(&'a str, &'a str), i32>,
    people: Vec<&'a str>,
}

impl<'a> People<'a> {
    fn parse(input: &'a str) -> Result<Self> {
        use nom::{
            bytes::complete::{tag, take_till1, take_while1},
            sequence::tuple,
        };

        let mut people = BTreeSet::new();
        let mut happiness = HashMap::new();

        for line in input.lines().map(str::trim) {
            let (_, (first, _, dir, _, mag, _, second)) = tuple::<_, _, (), _>((
                take_till1(char::is_whitespace),
                tag(" would "),
                take_till1(char::is_whitespace),
                tag(" "),
                take_while1(|c: char| c.is_ascii_digit()),
                tag(" happiness units by sitting next to "),
                take_till1(|c: char| c == '.'),
            ))(line)?;

            let happiness_change = mag.parse::<i32>()?
                * match dir {
                    "gain" => 1,
                    "lose" => -1,
                    _ => return Err(eyre!("Invalid magnitude: {}", dir)),
                };

            people.insert(first);
            happiness.insert((first, second), happiness_change);
        }

        Ok(People {
            people: people.into_iter().collect(),
            happiness,
        })
    }

    fn biggest_happiness(&self) -> Result<i32> {
        let mut max_change = 0;

        for mut arrangement in self.people.iter().permutations(self.people.len()) {
            // Circular table, so put the first on the end.
            let first = arrangement[0];
            arrangement.push(first);

            let happiness: i32 = arrangement
                .windows(2)
                .map(|pair| {
                    self.happiness[&(*pair[0], *pair[1])] + self.happiness[&(*pair[1], *pair[0])]
                })
                .sum();

            max_change = max_change.max(happiness);
        }

        Ok(max_change)
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = std::fs::read_to_string("inputs/aoc_1513.txt")?;
    let part1_table = People::parse(&input)?;

    let mut part2_table = part1_table.clone();
    part2_table.people.push("Self");
    for p in &part2_table.people {
        part2_table.happiness.insert(("Self", p), 0);
        part2_table.happiness.insert((p, "Self"), 0);
    }

    aoc_lib::run(
        "Day 13: Knights of the Dinner Table",
        (&part1_table, &part2_table),
        &|(table, _)| table.biggest_happiness(),
        &|(_, table)| table.biggest_happiness(),
    )
}

#[cfg(test)]
mod tests_1513 {
    use super::*;

    #[test]
    fn parse_test() {
        let input = "Alice would gain 54 happiness units by sitting next to Bob.
        Alice would lose 79 happiness units by sitting next to Carol.
        Alice would lose 2 happiness units by sitting next to David.
        Bob would gain 83 happiness units by sitting next to Alice.
        Bob would lose 7 happiness units by sitting next to Carol.
        Bob would lose 63 happiness units by sitting next to David.
        Carol would lose 62 happiness units by sitting next to Alice.
        Carol would gain 60 happiness units by sitting next to Bob.
        Carol would gain 55 happiness units by sitting next to David.
        David would gain 46 happiness units by sitting next to Alice.
        David would lose 7 happiness units by sitting next to Bob.
        David would gain 41 happiness units by sitting next to Carol.";

        let expected = People {
            people: vec!["Alice", "Bob", "Carol", "David"],
            happiness: {
                let mut map = HashMap::new();
                map.insert(("Alice", "Bob"), 54);
                map.insert(("Alice", "Carol"), -79);
                map.insert(("Alice", "David"), -2);
                map.insert(("Bob", "Alice"), 83);
                map.insert(("Bob", "Carol"), -7);
                map.insert(("Bob", "David"), -63);
                map.insert(("Carol", "Alice"), -62);
                map.insert(("Carol", "Bob"), 60);
                map.insert(("Carol", "David"), 55);
                map.insert(("David", "Alice"), 46);
                map.insert(("David", "Bob"), -7);
                map.insert(("David", "Carol"), 41);
                map
            },
        };

        let actual = People::parse(input).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn part1_example() {
        let input = "Alice would gain 54 happiness units by sitting next to Bob.
        Alice would lose 79 happiness units by sitting next to Carol.
        Alice would lose 2 happiness units by sitting next to David.
        Bob would gain 83 happiness units by sitting next to Alice.
        Bob would lose 7 happiness units by sitting next to Carol.
        Bob would lose 63 happiness units by sitting next to David.
        Carol would lose 62 happiness units by sitting next to Alice.
        Carol would gain 60 happiness units by sitting next to Bob.
        Carol would gain 55 happiness units by sitting next to David.
        David would gain 46 happiness units by sitting next to Alice.
        David would lose 7 happiness units by sitting next to Bob.
        David would gain 41 happiness units by sitting next to Carol.";

        let table = People::parse(input).unwrap();
        let actual = table.biggest_happiness();

        assert_eq!(330, actual.unwrap());
    }
}
