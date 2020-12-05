#![allow(clippy::unnecessary_wraps)]

use color_eyre::eyre::{eyre, Report, Result};

use std::collections::HashSet;

struct MoveList(Vec<Move>);

enum Move {
    North,
    South,
    East,
    West,
}

impl std::str::FromStr for MoveList {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(MoveList(
            s.chars()
                .map(|c| match c {
                    '^' => Ok(Move::North),
                    'v' => Ok(Move::South),
                    '<' => Ok(Move::West),
                    '>' => Ok(Move::East),
                    _ => Err(eyre!("Invalid character: {}", c)),
                })
                .collect::<Result<_, _>>()?,
        ))
    }
}

fn part1(input: &MoveList) -> Result<usize> {
    Ok(input
        .0
        .iter()
        .scan((0, 0), |(x, y), mv| {
            match mv {
                Move::North => *y += 1,
                Move::South => *y -= 1,
                Move::East => *x += 1,
                Move::West => *x -= 1,
            }

            Some((*x, *y))
        })
        .chain(std::iter::once((0, 0))) // Need to add the origin
        .collect::<HashSet<(i32, i32)>>()
        .len())
}

fn part2(input: &MoveList) -> Result<usize> {
    let init = [(0, 0), (0, 0)];
    Ok(input
        .0
        .chunks_exact(2)
        .scan(init, |coords, moves| {
            for ((x, y), mv) in coords.iter_mut().zip(moves) {
                match mv {
                    Move::North => *y += 1,
                    Move::South => *y -= 1,
                    Move::East => *x += 1,
                    Move::West => *x -= 1,
                }
            }

            Some(std::iter::once(coords[0]).chain(std::iter::once(coords[1])))
        })
        .flatten()
        .chain(std::iter::once((0, 0))) // Need to add the origin
        .collect::<HashSet<(i32, i32)>>()
        .len())
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = std::fs::read_to_string("inputs/aoc_1503.txt")?;
    let moves: MoveList = input.parse()?;

    aoc_lib::run(
        "Day 3: Perfectly Spherical Houses in a Vacuum",
        &moves,
        &part1,
        &part2,
    )
}

#[cfg(test)]
mod tests_1503 {
    use super::*;

    #[test]
    fn part1_examples() {
        let vals = [(">", 2), ("^>v<", 4), ("^v^v^v^v^v", 2)];

        for &(input, output) in &vals {
            let moves = input.parse().unwrap();
            assert_eq!(output, part1(&moves).unwrap());
        }
    }

    #[test]
    fn part2_examples() {
        let vals = [("^v", 3), ("^>v<", 3), ("^v^v^v^v^v", 11)];

        for &(input, output) in &vals {
            let moves = input.parse().unwrap();
            assert_eq!(output, part2(&moves).unwrap());
        }
    }
}
