#![allow(clippy::clippy::ptr_arg)]

use aoc_lib::parsers::unsigned_number;
use color_eyre::eyre::{eyre, Result};
use nom::bytes::complete::take_while1;

#[derive(Debug, Default, PartialEq)]
struct Sue {
    id: u16,
    children: Option<u8>,
    cats: Option<u8>,
    samoyeds: Option<u8>,
    pomeranians: Option<u8>,
    akitas: Option<u8>,
    vizslas: Option<u8>,
    goldfish: Option<u8>,
    trees: Option<u8>,
    cars: Option<u8>,
    perfumes: Option<u8>,
}

impl Sue {
    fn parse(line: &str) -> Result<Self> {
        use nom::{bytes::complete::tag, combinator::opt, multi::many0, sequence::tuple};

        let (rest, (_, id, _)) = tuple((tag("Sue "), unsigned_number::<u16>, tag(": ")))(line)
            .map_err(|e| eyre!("Error parsing ID: {}", e))?;

        let (_, facts) = many0(tuple((
            take_while1(|c: char| c.is_alphabetic()),
            tag(": "),
            unsigned_number::<u8>,
            opt(tag(", ")),
        )))(rest)
        .map_err(|e| eyre!("Error parse facts: {}", e))?;

        let mut sue = Sue {
            id: id?,
            ..Sue::default()
        };

        for (fact_name, _, fact_value, _) in facts {
            let fact = match fact_name {
                "children" => &mut sue.children,
                "cats" => &mut sue.cats,
                "samoyeds" => &mut sue.samoyeds,
                "pomeranians" => &mut sue.pomeranians,
                "akitas" => &mut sue.akitas,
                "vizslas" => &mut sue.vizslas,
                "goldfish" => &mut sue.goldfish,
                "trees" => &mut sue.trees,
                "cars" => &mut sue.cars,
                "perfumes" => &mut sue.perfumes,
                _ => return Err(eyre!("Unknown fact: {}", fact_name)),
            };

            *fact = Some(fact_value?);
        }

        Ok(sue)
    }
}

impl std::fmt::Display for Sue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Sue {}", self.id)
    }
}

fn part1(sues: &Vec<Sue>) -> Result<&Sue> {
    sues.iter()
        .filter(|s| !matches!(s.children, Some(v) if v != 3))
        .filter(|s| !matches!(s.cats, Some(v) if v != 7))
        .filter(|s| !matches!(s.samoyeds, Some(v) if v != 2))
        .filter(|s| !matches!(s.pomeranians, Some(v) if v != 3))
        .filter(|s| !matches!(s.akitas, Some(v) if v != 0))
        .filter(|s| !matches!(s.vizslas, Some(v) if v != 0))
        .filter(|s| !matches!(s.goldfish, Some(v) if v != 5))
        .filter(|s| !matches!(s.trees, Some(v) if v != 3))
        .filter(|s| !matches!(s.cars, Some(v) if v != 2))
        .find(|s| !matches!(s.perfumes, Some(v) if v != 1))
        .ok_or_else(|| eyre!("Unable to find result"))
}

fn part2(sues: &Vec<Sue>) -> Result<&Sue> {
    sues.iter()
        .filter(|s| !matches!(s.children, Some(v) if v != 3))
        .filter(|s| !matches!(s.cats, Some(v) if v <= 7))
        .filter(|s| !matches!(s.samoyeds, Some(v) if v != 2))
        .filter(|s| matches!(s.pomeranians, Some(v) if v <= 3) || s.pomeranians.is_none())
        .filter(|s| !matches!(s.akitas, Some(v) if v != 0))
        .filter(|s| !matches!(s.vizslas, Some(v) if v != 0))
        .filter(|s| matches!(s.goldfish, Some(v) if v <= 5) || s.goldfish.is_none())
        .filter(|s| !matches!(s.trees, Some(v) if v <= 3))
        .filter(|s| !matches!(s.cars, Some(v) if v != 2))
        .find(|s| !matches!(s.perfumes, Some(v) if v != 1))
        .ok_or_else(|| eyre!("Unable to find result"))
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = std::fs::read_to_string("inputs/aoc_1516.txt")?;
    let sues: Vec<_> = input
        .lines()
        .map(str::trim)
        .map(Sue::parse)
        .collect::<Result<_>>()?;

    aoc_lib::run("Day 16: Aunt Sue", &sues, &part1, &part2)
}

#[cfg(test)]
mod tests_1516 {
    use super::*;

    #[test]
    fn parse_test() {
        let input = "Sue 1: cars: 9, akitas: 3, goldfish: 0
        Sue 2: akitas: 9, children: 3, samoyeds: 9";

        let expected = [
            Sue {
                id: 1,
                cars: Some(9),
                akitas: Some(3),
                goldfish: Some(0),
                ..Sue::default()
            },
            Sue {
                id: 2,
                akitas: Some(9),
                children: Some(3),
                samoyeds: Some(9),
                ..Sue::default()
            },
        ];

        let actual: Vec<_> = input
            .lines()
            .map(str::trim)
            .map(Sue::parse)
            .collect::<Result<_>>()
            .unwrap();

        assert_eq!(actual, expected);
    }
}
