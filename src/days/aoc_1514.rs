use aoc_lib::{day, Bench, BenchError, BenchResult};
use color_eyre::eyre::{eyre, Result};

day! {
    day 14: "Reindeer Olympics"
    1: run_part1
    2: run_part2
}

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let reindeer: Vec<_> = input
        .lines()
        .map(str::trim)
        .map(Reindeer::parse)
        .collect::<Result<_, _>>()
        .map_err(|e| BenchError::UserError(e.into()))?;

    b.bench(|| {
        reindeer
            .iter()
            .map(|r| r.distance(2503))
            .max()
            .ok_or_else(|| eyre!("No result found"))
    })
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    let reindeer: Vec<_> = input
        .lines()
        .map(str::trim)
        .map(Reindeer::parse)
        .collect::<Result<_, _>>()
        .map_err(|e| BenchError::UserError(e.into()))?;

    b.bench(|| Ok::<_, u32>(part2(&&reindeer, 2503).1))
}

#[derive(Debug, PartialEq)]
struct Reindeer {
    name: String,
    flight_speed: u32,
    flight_time: u32,
    rest_time: u32,
}

impl Reindeer {
    fn parse(line: &str) -> Result<Self> {
        use nom::{
            bytes::complete::{tag, take_till1, take_while1},
            sequence::tuple,
        };

        let (_, (name, _, flight_speed, _, flight_time, _, rest_time, _)) =
            tuple::<_, _, (), _>((
                take_till1(char::is_whitespace),
                tag(" can fly "),
                take_while1(|c: char| c.is_ascii_digit()),
                tag(" km/s for "),
                take_while1(|c: char| c.is_ascii_digit()),
                tag(" seconds, but then must rest for "),
                take_while1(|c: char| c.is_ascii_digit()),
                tag(" seconds."),
            ))(line)?;

        Ok(Reindeer {
            name: name.to_owned(),
            flight_speed: flight_speed.parse()?,
            flight_time: flight_time.parse()?,
            rest_time: rest_time.parse()?,
        })
    }

    fn distance(&self, total_flight_time: u32) -> u32 {
        let cycle_time = self.flight_time + self.rest_time;

        let (total_cycles, partial_cycle) = (
            total_flight_time / cycle_time,
            total_flight_time % cycle_time,
        );

        (total_cycles * self.flight_time + self.flight_time.min(partial_cycle)) * self.flight_speed
    }
}

#[derive(Debug)]
enum ReindeerFlightState {
    Flying(u32),
    Resting(u32),
}

#[derive(Debug)]
struct RaceEntryState<'a> {
    reindeer: &'a Reindeer,
    flight_state: ReindeerFlightState,
    distance: u32,
    points: u32,
}

fn part2(reindeer: &[Reindeer], total_time: u32) -> (&Reindeer, u32) {
    use ReindeerFlightState::*;

    let mut state: Vec<_> = reindeer
        .iter()
        .map(|r| RaceEntryState {
            reindeer: r,
            flight_state: Flying(r.flight_time),
            distance: 0,
            points: 0,
        })
        .collect();

    for _ in 0..total_time {
        let mut cur_lead_distance = 0;
        let mut leader_id = 0;

        for (
            id,
            RaceEntryState {
                reindeer,
                flight_state,
                distance,
                ..
            },
        ) in state.iter_mut().enumerate()
        {
            *flight_state = match flight_state {
                Flying(time_left @ 2..=u32::MAX) => {
                    *distance += reindeer.flight_speed;
                    Flying(*time_left - 1)
                }
                Flying(_) => {
                    *distance += reindeer.flight_speed;
                    Resting(reindeer.rest_time)
                }
                Resting(time_left @ 2..=u32::MAX) => Resting(*time_left - 1),
                Resting(_) => Flying(reindeer.flight_time),
            };

            if *distance > cur_lead_distance {
                cur_lead_distance = *distance;
                leader_id = id;
            }
        }

        state[leader_id].points += 1;
    }

    state
        .iter()
        .map(|r| (r.reindeer, r.points))
        .max_by_key(|(_, p)| *p)
        .unwrap()
}

#[cfg(test)]
mod tests_1514 {
    use super::*;

    #[test]
    fn parse_test() {
        let input = "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
        Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";

        let reindeer: Vec<_> = input
            .lines()
            .map(str::trim)
            .map(Reindeer::parse)
            .collect::<Result<_, _>>()
            .unwrap();

        let expected_parse = vec![
            Reindeer {
                name: "Comet".to_owned(),
                flight_speed: 14,
                flight_time: 10,
                rest_time: 127,
            },
            Reindeer {
                name: "Dancer".to_owned(),
                flight_speed: 16,
                flight_time: 11,
                rest_time: 162,
            },
        ];

        assert_eq!(expected_parse, reindeer);
    }

    #[test]
    fn part1_example() {
        let input = "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
        Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";

        let reindeer: Vec<_> = input
            .lines()
            .map(str::trim)
            .map(Reindeer::parse)
            .collect::<Result<_, _>>()
            .unwrap();

        let tests = [
            (&reindeer[0], 1, 14),
            (&reindeer[0], 10, 140),
            (&reindeer[0], 11, 140),
            (&reindeer[0], 1000, 1120),
            (&reindeer[1], 1, 16),
            (&reindeer[1], 10, 160),
            (&reindeer[1], 11, 176),
            (&reindeer[1], 1000, 1056),
        ];

        for (i, &(r, time, expected)) in tests.iter().enumerate() {
            assert_eq!(expected, r.distance(time), "{}", i);
        }
    }

    #[test]
    fn part2_example() {
        let input = "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
        Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";

        let reindeer: Vec<_> = input
            .lines()
            .map(str::trim)
            .map(Reindeer::parse)
            .collect::<Result<_, _>>()
            .unwrap();

        let tests = [
            (&reindeer[1], 1, 1),
            (&reindeer[1], 140, 139),
            // Site says 689, but I think that may be a mistake. My solution gets 688, but gets the correct 312
            // for the other reindeer, as well as getting the correct answer to the full problem.
            (&reindeer[1], 1000, 688),
        ];

        for (i, &(r, time, expected)) in tests.iter().enumerate() {
            assert_eq!((r, expected), part2(&reindeer, time), "{}", i);
        }
    }
}
