use aoc_lib::{Bench, BenchResult, Day, ParseResult, UserError};
use chumsky::{IterParser, Parser};
use color_eyre::{
    eyre::{eyre, Result},
    Report,
};

pub const DAY: Day = Day {
    day: 16,
    name: "Aunt Sue",
    part_1: run_part1,
    part_2: Some(run_part2),
    other: &[("Parse", run_parse)],
};

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let sues: Vec<_> = input
        .lines()
        .map(str::trim)
        .map(Sue::parse)
        .collect::<Result<_, _>>()
        .map_err(UserError)?;

    b.bench(|| part1(&sues))
}
fn run_part2(input: &str, b: Bench) -> BenchResult {
    let sues: Vec<_> = input
        .lines()
        .map(str::trim)
        .map(Sue::parse)
        .collect::<Result<_, _>>()
        .map_err(UserError)?;

    b.bench(|| part2(&sues))
}
fn run_parse(input: &str, b: Bench) -> BenchResult {
    b.bench(|| {
        let data: Vec<_> = input
            .lines()
            .map(str::trim)
            .map(Sue::parse)
            .collect::<Result<_, _>>()?;
        Ok::<_, Report>(ParseResult(data))
    })
}

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
        fn parse_sue<'a>() -> impl Parser<'a, &'a str, Sue> {
            use chumsky::{
                primitive::just,
                text::{ident, int},
            };

            let id_num = int(10).from_str::<u16>().unwrapped();
            let value_num = int(10).from_str::<u8>().unwrapped();

            let id = just("Sue ").ignore_then(id_num).then_ignore(just(":"));
            let fact = ident().then_ignore(just(": ")).then(value_num);

            id.then(
                fact.padded()
                    .separated_by(just(","))
                    .at_least(1)
                    .collect::<Vec<_>>(),
            )
            .map(|(id, facts)| {
                facts.into_iter().fold(
                    Sue {
                        id,
                        ..Default::default()
                    },
                    |mut sue, (fact_name, fact_value)| {
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
                            _ => unreachable!(),
                        };

                        *fact = Some(fact_value);
                        sue
                    },
                )
            })
        }

        parse_sue()
            .parse(line)
            .into_output()
            .ok_or_else(|| eyre!("Failed to parse line `{line:?}`"))
    }
}

impl std::fmt::Display for Sue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Sue {}", self.id)
    }
}

fn part1(sues: &[Sue]) -> Result<&Sue> {
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

fn part2(sues: &[Sue]) -> Result<&Sue> {
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
