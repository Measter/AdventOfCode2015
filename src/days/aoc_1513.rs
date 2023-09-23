use aoc_lib::{misc::ArrWindows, Bench, BenchResult, Day, ParseResult, UserError};
use chumsky::Parser;
use color_eyre::{
    eyre::{eyre, Result},
    Report,
};
use itertools::Itertools;

use std::collections::{BTreeSet, HashMap};

pub const DAY: Day = Day {
    day: 13,
    name: "Knights of the Dinner Table",
    part_1: run_part1,
    part_2: Some(run_part2),
    other: &[("Parse", run_parse)],
};

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let mut table = People::parse(input).map_err(UserError)?;
    table.build_fast_lookup();
    b.bench(|| table.biggest_happiness())
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    let mut table = People::parse(input).map_err(UserError)?;
    table.people.push("Self");
    for p in &table.people {
        table.happiness.insert(("Self", p), 0);
        table.happiness.insert((p, "Self"), 0);
    }
    table.build_fast_lookup();
    b.bench(|| table.biggest_happiness())
}

fn run_parse(input: &str, b: Bench) -> BenchResult {
    b.bench(|| {
        let data = People::parse(input)?;
        Ok::<_, Report>(ParseResult(data))
    })
}

#[derive(Debug, PartialEq, Clone)]
struct People<'a> {
    happiness: HashMap<(&'a str, &'a str), i32>,
    people: Vec<&'a str>,
    fast_lookup: Vec<i32>,
}

impl<'a> People<'a> {
    fn parse(input: &'a str) -> Result<Self> {
        let mut people = BTreeSet::new();
        let mut happiness = HashMap::new();

        for line in input.lines().map(str::trim) {
            fn parse_line<'a>() -> impl Parser<'a, &'a str, (&'a str, i32, i32, &'a str)> {
                use chumsky::{
                    primitive::just,
                    text::{ident, int},
                };

                let mag = int(10).from_str::<i32>().unwrapped();
                let dir = just("gain").to(1).or(just("lose").to(-1));

                ident()
                    .then_ignore(just("would").padded())
                    .then(dir)
                    .boxed()
                    .then(mag.padded())
                    .then_ignore(just("happiness units by sitting next to "))
                    .then(ident())
                    .boxed()
                    .then_ignore(just("."))
                    .map(|(((a, b), c), d)| (a, b, c, d))
            }

            let (first, dir, mag, second) = parse_line()
                .parse(line)
                .into_output()
                .ok_or_else(|| eyre!("Failed to parse `{line:?}`"))?;

            people.insert(first);
            happiness.insert((first, second), dir * mag);
        }

        Ok(People {
            people: people.into_iter().collect(),
            happiness,
            fast_lookup: Vec::new(),
        })
    }

    // Saves us doing the hash lookup later.
    fn build_fast_lookup(&mut self) {
        self.fast_lookup
            .resize(self.people.len() * self.people.len(), 0);

        let range = 0..self.people.len();
        for (a, b) in (range.clone()).cartesian_product(range) {
            let a_name = self.people[a];
            let b_name = self.people[b];

            if a != b {
                let a_val = self.happiness[&(a_name, b_name)];
                let b_val = self.happiness[&(b_name, a_name)];

                self.fast_lookup[a * self.people.len() + b] = a_val + b_val;
            }
        }
    }

    fn biggest_happiness(&self) -> Result<i32> {
        let mut max_change = 0;

        let range = 0..self.people.len();
        for mut arrangement in range.permutations(self.people.len()) {
            // Circular table, so put the first on the end.
            let first = arrangement[0];
            arrangement.push(first);

            let happiness: i32 = ArrWindows::new(&arrangement)
                .map(|&[a, b]| self.fast_lookup[a * self.people.len() + b])
                .sum();

            max_change = max_change.max(happiness);
        }

        Ok(max_change)
    }
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
            fast_lookup: Vec::new(),
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

        let mut table = People::parse(input).unwrap();
        table.build_fast_lookup();
        let actual = table.biggest_happiness();

        assert_eq!(330, actual.unwrap());
    }
}
