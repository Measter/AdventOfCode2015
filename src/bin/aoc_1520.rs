use color_eyre::eyre::{eyre, Result};
use itertools::Itertools;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum IsPrime {
    Prime,
    NotPrime,
}

#[derive(Debug, Copy, Clone)]
enum SieveState {
    Two,
    Other(usize),
    Finished,
}

#[derive(Debug, Clone)]
pub struct Sieve {
    sieve: Vec<IsPrime>,
    state: SieveState,
}

impl Sieve {
    pub fn new(len: usize) -> Sieve {
        assert!(len > 2);

        Sieve {
            sieve: vec![IsPrime::Prime; len / 2],
            state: SieveState::Two,
        }
    }
}

impl Iterator for Sieve {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        use self::IsPrime::*;
        use self::SieveState::*;
        use std::iter::repeat;

        match self.state {
            Finished => None,
            Two => {
                self.state = Other(3);
                Some(2)
            }
            Other(cur) => {
                let len = self.sieve.len();
                repeat(cur)
                    .zip(cur..)
                    .map(|(a, b)| a * b)
                    .filter(|&i| i % 2 == 1)
                    .map(|i| i / 2)
                    .take_while(|&i| i < len)
                    .for_each(|i| self.sieve[i] = NotPrime);

                let next = self
                    .sieve
                    .iter()
                    .enumerate()
                    .skip(cur / 2 + 1)
                    .find(|&(_, &p)| p == Prime);

                match next {
                    None => {
                        self.state = Finished;
                        Some(cur)
                    }
                    Some((i, _)) => {
                        self.state = Other(i * 2 + 1);
                        Some(cur)
                    }
                }
            }
        }
    }
}

// Could be done better by using a list of prime numbers.
struct PrimeFactors<'a> {
    num: u32,
    primes: &'a [u32],
}

impl<'a> PrimeFactors<'a> {
    fn of(num: u32, primes: &'a [u32]) -> Self {
        Self { num, primes }
    }
}

impl Iterator for PrimeFactors<'_> {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (&next, rest) = self.primes.split_first()?;
            if next > self.num {
                return None;
            }

            if self.num % next == 0 {
                self.num /= next;
                return Some(next);
            }

            self.primes = rest;
        }
    }
}

fn house_presents_part1(house: u32, primes: &[u32]) -> u32 {
    match house {
        1 => 10,
        _ => {
            let mut num_presents = 1;

            // Sum of divisors.
            for (p, power) in &PrimeFactors::of(house, primes).group_by(|&i| i) {
                let a = power.count() as u32;

                num_presents *= (p.pow(a + 1) - 1) / (p - 1);
            }

            num_presents * 10
        }
    }
}

fn part1(num_presents: u32, primes: &[u32]) -> Option<u32> {
    (1..)
        .inspect(|i| {
            if i % 100_000 == 0 {
                println!("{}", i)
            }
        })
        .find(|&h| house_presents_part1(h, primes) >= num_presents)
}

fn house_presents_part2(house: u32) -> u32 {
    let mut num_presents = 0;

    for div in (1..=50).filter(|&e| house % e == 0).map(|e| house / e) {
        num_presents += div;
    }

    num_presents * 11
}

fn part2(num_presents: u32) -> Option<u32> {
    (1..)
        .inspect(|i| {
            if i % 100_000 == 0 {
                println!("{}", i)
            }
        })
        .find(|&h| house_presents_part2(h) >= num_presents)
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = std::fs::read_to_string("inputs/aoc_1520.txt")?;
    let input = input.trim().parse::<u32>()?;

    let start = std::time::Instant::now();

    let primes: Vec<u32> = Sieve::new(input as usize).map(|p| p as u32).collect();

    let part1 = part1(input, &primes).ok_or_else(|| eyre!("Unable to find answer for part 1"))?;
    let part2 = part2(input).ok_or_else(|| eyre!("Unable to find answer for part 1"))?;

    let elapsed = start.elapsed();

    println!("Part 1 output: {}", part1);
    println!("Part 2 output: {}", part2);

    println!("Elapsed: {}ms", elapsed.as_millis());

    Ok(())
}

#[cfg(test)]
mod tests_1520 {
    use super::*;

    #[test]
    fn part1_example() {
        let tests = [
            (1, 10),
            (2, 30),
            (3, 40),
            (4, 70),
            (5, 60),
            (6, 120),
            (7, 80),
            (8, 150),
            (9, 130),
        ];

        let primes: Vec<u32> = Sieve::new(10).map(|p| p as u32).collect();

        for &(house, expected) in &tests {
            assert_eq!(house_presents_part1(house, &primes), expected, "{}", house);
        }
    }
}
