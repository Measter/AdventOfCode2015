use aoc_lib::{Bench, BenchResult, Day, ParseResult, UserError};
use color_eyre::{
    eyre::{eyre, Result},
    Report,
};

pub const DAY: Day = Day {
    day: 18,
    name: "Like a GIF For Your Yard",
    part_1: run_part1,
    part_2: Some(run_part2),
    other: &[("Parse", run_parse)],
};

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let light_array = LightArray::parse(input).map_err(UserError)?;
    b.bench(|| run_gol(light_array.clone(), false))
}
fn run_part2(input: &str, b: Bench) -> BenchResult {
    let light_array = LightArray::parse(input).map_err(UserError)?;
    b.bench(|| run_gol(light_array.clone(), true))
}
fn run_parse(input: &str, b: Bench) -> BenchResult {
    b.bench(|| {
        let data = LightArray::parse(input)?;
        Ok::<_, Report>(ParseResult(data))
    })
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum LightState {
    On,
    Off,
}

impl LightState {
    fn parse(input: char) -> Result<LightState> {
        match input {
            '#' => Ok(LightState::On),
            '.' => Ok(LightState::Off),
            _ => Err(eyre!("Invalid character: {}", input)),
        }
    }
}

#[derive(Debug, Clone)]
struct LightArray {
    array: Vec<LightState>,
    buf: Vec<LightState>,
    size: usize,
}

impl LightArray {
    fn parse(input: &str) -> Result<LightArray> {
        let size = input.find(['\n', '\r']).unwrap_or(input.len());

        let array: Vec<_> = input
            .lines()
            .map(str::trim)
            .flat_map(str::chars)
            .map(LightState::parse)
            .collect::<Result<_>>()?;

        if array.len() != size * size {
            Err(eyre!("Input must be a square grid"))
        } else {
            Ok(LightArray {
                buf: vec![LightState::Off; array.len()],
                array,
                size,
            })
        }
    }

    fn get_neighbours(x: usize, y: usize, size: usize) -> [Option<usize>; 8] {
        let neighbour = |rel_x: isize, rel_y: isize| -> Option<usize> {
            let size = size as isize;
            let new_x = (x as isize) + rel_x;
            let new_y = (y as isize) + rel_y;

            if new_x < 0 || new_x >= size || new_y < 0 || new_y >= size {
                None
            } else {
                Some((new_y * size + new_x) as usize)
            }
        };

        [
            neighbour(-1, -1),
            neighbour(0, -1),
            neighbour(1, -1),
            neighbour(-1, 0),
            neighbour(1, 0),
            neighbour(-1, 1),
            neighbour(0, 1),
            neighbour(1, 1),
        ]
    }

    fn apply_stuck(&mut self) {
        // Corners should be stuck on.
        self.array[0] = LightState::On;
        self.array[self.size - 1] = LightState::On;
        self.array[self.size * (self.size - 1)] = LightState::On;
        self.array[self.size * self.size - 1] = LightState::On;
    }

    fn step(&mut self, stuck: bool) {
        let mut buffer = std::mem::take(&mut self.buf);

        let rows = self
            .array
            .chunks_exact(self.size)
            .zip(buffer.chunks_exact_mut(self.size))
            .enumerate();

        for (y, (light, buf)) in rows {
            let lights = light.iter().zip(buf).enumerate();
            for (x, (light, buf)) in lights {
                let neighbours = LightArray::get_neighbours(x, y, self.size);

                let live_count: u8 = neighbours
                    .iter()
                    .filter_map(|i| *i)
                    .map(|idx| (self.array[idx] == LightState::On) as u8)
                    .sum();

                *buf = match (light, live_count) {
                    (LightState::On, 2..=3) | (LightState::Off, 3) => LightState::On,
                    _ => LightState::Off,
                };
            }
        }

        std::mem::swap(&mut self.array, &mut self.buf);
        self.array = buffer;

        if stuck {
            self.apply_stuck();
        }
    }
}

fn run_gol(mut array: LightArray, stuck: bool) -> Result<usize> {
    if stuck {
        array.apply_stuck();
    }

    for _ in 0..100 {
        array.step(stuck);
    }

    Ok(array
        .array
        .iter()
        .map(|&l| (l == LightState::On) as usize)
        .sum())
}

#[cfg(test)]
mod tests_1518 {
    use super::*;

    #[test]
    fn parse_test() {
        let input = "#.\r\n.#";

        let expected = [
            LightState::On,
            LightState::Off,
            LightState::Off,
            LightState::On,
        ];
        let actual = LightArray::parse(input).unwrap();

        assert_eq!(&*actual.array, expected);
    }

    #[test]
    fn part1_example() {
        let expected_input = "......
        ......
        ..##..
        ..##..
        ......
        ......";

        let LightArray {
            array: expected, ..
        } = LightArray::parse(expected_input).unwrap();

        let input = ".#.#.#
        ...##.
        #....#
        ..#...
        #.#..#
        ####..";

        let mut array = LightArray::parse(input).unwrap();

        for _ in 0..4 {
            array.step(false);
        }

        assert_eq!(array.array, expected);
    }

    #[test]
    fn apply_stuck_test() {
        let expected_input = "#....#
        ......
        ..##..
        ..##..
        ......
        #....#";

        let LightArray {
            array: expected, ..
        } = LightArray::parse(expected_input).unwrap();

        let input = "......
        ......
        ..##..
        ..##..
        ......
        ......";

        let mut array = LightArray::parse(input).unwrap();
        array.apply_stuck();

        assert_eq!(array.array, expected);
    }

    #[test]
    fn part2_example() {
        let expected_input = "##.###
        .##..#
        .##...
        .##...
        #.#...
        ##...#";

        let LightArray {
            array: expected, ..
        } = LightArray::parse(expected_input).unwrap();

        let input = "##.#.#
        ...##.
        #....#
        ..#...
        #.#..#
        ####.#";

        let mut array = LightArray::parse(input).unwrap();
        array.apply_stuck();

        for _ in 0..5 {
            array.step(true);
        }

        assert_eq!(array.array, expected);
    }
}
