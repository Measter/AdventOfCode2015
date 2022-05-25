use aoc_lib::{misc::ArrWindows, Bench, BenchResult, Day, ParseResult, UserError};
use color_eyre::{eyre::Result, Report};
use itertools::Itertools;

use std::collections::{HashMap, HashSet};

pub const DAY: Day = Day {
    day: 9,
    name: "All in a Single Night",
    part_1: run_part1,
    part_2: Some(run_part2),
    other: &[("Parse", run_parse)],
};
fn run_part1(input: &str, b: Bench) -> BenchResult {
    let map = Map::parse(input).map_err(UserError)?;
    b.bench(|| Map::shortest(&map))
}
fn run_part2(input: &str, b: Bench) -> BenchResult {
    let map = Map::parse(input).map_err(UserError)?;
    b.bench(|| Map::longest(&map))
}
fn run_parse(input: &str, b: Bench) -> BenchResult {
    b.bench(|| {
        let data = Map::parse(input)?;
        Ok::<_, Report>(ParseResult(data))
    })
}

#[derive(Debug, PartialEq)]
struct Map<'a> {
    distances: HashMap<(&'a str, &'a str), u32>,
    locations: HashSet<&'a str>,
}

impl<'a> Map<'a> {
    fn parse(input: &str) -> Result<Map> {
        use nom::{
            bytes::complete::{tag, take_till1, take_while1},
            sequence::tuple,
        };

        let mut locations = HashSet::new();
        let mut distances = HashMap::new();

        for line in input.lines().map(str::trim) {
            let (_, (start, _, finish, _, distance)) = tuple::<_, _, (), _>((
                take_till1(char::is_whitespace),
                tag(" to "),
                take_till1(char::is_whitespace),
                tag(" = "),
                take_while1(|c: char| c.is_ascii_digit()),
            ))(line)?;

            locations.insert(start);
            locations.insert(finish);
            distances.insert((start, finish), distance.parse()?);
            distances.insert((finish, start), distance.parse()?);
        }

        Ok(Map {
            distances,
            locations,
        })
    }

    fn shortest(&self) -> Result<u32> {
        let mut min_distance = u32::MAX;

        for route in self.locations.iter().permutations(self.locations.len()) {
            let route_distance = ArrWindows::new(&route)
                .map(|&[a, b]| self.distances[&(*a, *b)])
                .sum();

            min_distance = min_distance.min(route_distance);
        }

        Ok(min_distance)
    }

    fn longest(&self) -> Result<u32> {
        let mut max_distance = 0;

        for route in self.locations.iter().permutations(self.locations.len()) {
            let route_distance = ArrWindows::new(&route)
                .map(|&[a, b]| self.distances[&(*a, *b)])
                .sum();

            max_distance = max_distance.max(route_distance);
        }

        Ok(max_distance)
    }
}

#[cfg(test)]
mod tests_1509 {
    use super::*;

    #[test]
    fn parse_test() {
        let input = "London to Dublin = 464
        London to Belfast = 518
        Dublin to Belfast = 141";

        let expected = Map {
            locations: {
                let mut set = HashSet::new();
                set.insert("London");
                set.insert("Dublin");
                set.insert("Belfast");
                set
            },
            distances: {
                let mut map = HashMap::new();
                map.insert(("London", "Dublin"), 464);
                map.insert(("Dublin", "London"), 464);
                map.insert(("London", "Belfast"), 518);
                map.insert(("Belfast", "London"), 518);
                map.insert(("Dublin", "Belfast"), 141);
                map.insert(("Belfast", "Dublin"), 141);
                map
            },
        };

        assert_eq!(expected, Map::parse(input).unwrap());
    }

    #[test]
    fn part1_example() {
        let input = "London to Dublin = 464
        London to Belfast = 518
        Dublin to Belfast = 141";

        let map = Map::parse(input).unwrap();

        assert_eq!(605, map.shortest().unwrap());
    }
}
