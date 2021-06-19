use aoc_lib::{day, Bench, BenchResult};
use color_eyre::eyre::{eyre, Report, Result};

day! {
   day 2: "I Was Told There Would Be No Math"
   1: run_part1
   2: run_part2
}

fn run_part1(input: &str, b: Bench) -> BenchResult {
    b.bench(|| part(input, Box::paper))
}
fn run_part2(input: &str, b: Bench) -> BenchResult {
    b.bench(|| part(input, Box::ribbon))
}

#[derive(Debug, Copy, Clone)]
struct Box {
    width: u32,
    height: u32,
    length: u32,
}

impl Box {
    fn paper(self) -> u32 {
        let side_a_area = self.width * self.height;
        let side_b_area = self.width * self.length;
        let side_c_area = self.height * self.length;

        let min_side = side_a_area.min(side_b_area).min(side_c_area);

        2 * (side_a_area + side_b_area + side_c_area) + min_side
    }

    fn ribbon(self) -> u32 {
        let side_a = self.width.min(self.height).min(self.length);
        let side_b = if side_a == self.width {
            self.height.min(self.length)
        } else if side_a == self.height {
            self.width.min(self.length)
        } else {
            self.height.min(self.width)
        };

        let len = 2 * (side_a + side_b);

        let volume = self.height * self.width * self.length;

        len + volume
    }
}

impl std::str::FromStr for Box {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.splitn(3, 'x').map(str::parse::<u32>);
        let parts = (
            parts.next().transpose()?,
            parts.next().transpose()?,
            parts.next().transpose()?,
        );

        if let (Some(width), Some(height), Some(length)) = parts {
            Ok(Box {
                width,
                height,
                length,
            })
        } else {
            Err(eyre!("Invalid Box definition: {}", s))
        }
    }
}

fn part(input: &str, f: fn(Box) -> u32) -> Result<u32> {
    let mut total = 0;

    for b in input.lines().map(str::parse::<Box>) {
        total += f(b?);
    }

    Ok(total)
}

#[cfg(test)]
mod tests_1502 {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(58, "2x3x4".parse::<Box>().map(Box::paper).unwrap());
        assert_eq!(43, "1x1x10".parse::<Box>().map(Box::paper).unwrap());
    }

    #[test]
    fn part2_examples() {
        assert_eq!(34, "2x3x4".parse::<Box>().map(Box::ribbon).unwrap());
        assert_eq!(14, "1x1x10".parse::<Box>().map(Box::ribbon).unwrap());
    }
}
