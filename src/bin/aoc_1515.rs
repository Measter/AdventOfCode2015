use color_eyre::eyre::{eyre, Result};

#[derive(Debug, PartialEq)]
struct Ingredient {
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl Ingredient {
    fn parse(line: &str) -> Result<Ingredient> {
        use nom::{
            bytes::complete::{tag, take_while1},
            combinator::{map, opt},
            sequence::tuple,
            IResult,
        };

        fn number(input: &str) -> IResult<&str, Result<i32, std::num::ParseIntError>> {
            map(
                tuple((
                    map(opt(tag("-")), |o: Option<&str>| o.is_some()),
                    take_while1(|c: char| c.is_ascii_digit()),
                )),
                |(is_neg, num)| num.parse::<i32>().map(|n| if is_neg { -n } else { n }),
            )(input)
        }

        let (rm, (name, _, capacity, _, durability)) = tuple((
            take_while1(char::is_alphabetic),
            tag(": capacity "),
            number,
            tag(", durability "),
            number,
        ))(line)
        .map_err(|e| eyre!("Error parsing input: {}", e))?;

        let (_, (_, flavor, _, texture, _, calories)) = tuple((
            tag(", flavor "),
            number,
            tag(", texture "),
            number,
            tag(", calories "),
            number,
        ))(rm)
        .map_err(|e| eyre!("Error parsing input: {}", e))?;

        Ok(Self {
            name: name.to_owned(),
            flavor: flavor?,
            capacity: capacity?,
            durability: durability?,
            texture: texture?,
            calories: calories?,
        })
    }
}

fn get_score(ingredients: &[Ingredient], teaspoons: &[u32], cal_func: impl Fn(i32) -> bool) -> i32 {
    let (capacity, durability, flavor, texture, calories) = ingredients
        .iter()
        .zip(teaspoons)
        .map(|(i, t)| {
            (
                i.capacity * *t as i32,
                i.durability * *t as i32,
                i.flavor * *t as i32,
                i.texture * *t as i32,
                i.calories * *t as i32,
            )
        })
        .fold((0, 0, 0, 0, 0), |acc, i| {
            (
                acc.0 + i.0,
                acc.1 + i.1,
                acc.2 + i.2,
                acc.3 + i.3,
                acc.4 + i.4,
            )
        });

    capacity.max(0) * durability.max(0) * flavor.max(0) * texture.max(0) * cal_func(calories) as i32
}

fn next_teaspoons(tsps: &mut [u32], max_teaspoons: u32) {
    // The sum of all digits MUST be equal to `max_teaspoons` at all times.
    // This means that a digit will always "roll over" to its minimum value on every increment.
    // Rolling over means that the next digit up will increment.

    // The trivial cases, only one thing to do in both.
    if tsps.is_empty() {
        return;
    }
    if let [cur_digit] = tsps {
        *cur_digit = max_teaspoons;
        return;
    }

    // Check for leading 0s, so that we start counting correctly in the rollover.
    let zero_prefix_count = tsps.iter().take_while(|t| **t == 0).count();

    if zero_prefix_count == 0 || zero_prefix_count == tsps.len() {
        // No leading zeroes (e.g. [30, 30, 40]).
        let (cur_digit, higher) = tsps.split_first_mut().unwrap();

        let sum_high: u32 = higher.iter().sum();

        if sum_high + *cur_digit < max_teaspoons {
            // If the current digit has enough room to be incremented, we should do so here.
            *cur_digit += 1;
        } else {
            // Otherwise send that increment up to the remaining set of digits, making sure not to increase
            // the total number of maximum teaspoons.
            next_teaspoons(higher, (sum_high + 1).min(max_teaspoons));
            *cur_digit = (max_teaspoons - sum_high).saturating_sub(1);
        }
    } else {
        // At least 1 leading zero (e.g. [0, 30, 70]).
        // Split into [0] and [30, 70].
        let (left, right) = tsps.split_at_mut(zero_prefix_count);

        if right.len() == 1 && right[0] == max_teaspoons {
            // There's nothing we can do in this case, as the next digit can't be incremented.
            return;
        }

        let right_teaspoons = right.iter().sum();
        if max_teaspoons != right_teaspoons {
            // We know that the sum of the right side is less than the max, so the remainder should go into
            // the least significant digit.
            left[0] = max_teaspoons - right_teaspoons;
        } else {
            // Here the right side is the same as the max, so we need to roll over the least sig. digit
            // in the right side.
            // Because of the roll-over, that digit now becomes 0. The value it did have is now split between
            // the least sig. digit of the left side, and the remainder of the right.
            let new_left_val = right[0] - 1;
            left[0] = new_left_val;

            right[0] = 0;
            let right_max_teaspoons = right_teaspoons - new_left_val;
            next_teaspoons(&mut right[1..], right_max_teaspoons);
        }
    }
}

fn cookie_search(
    ingredients: &[Ingredient],
    max_teaspoons: u32,
    cal_func: impl Fn(i32) -> bool,
) -> i32 {
    let mut num_teaspoons = vec![0; ingredients.len()];
    num_teaspoons[0] = max_teaspoons;

    let mut max_score = 0;
    let mut hit_max = false;

    loop {
        let score = get_score(&ingredients, &num_teaspoons, &cal_func);
        max_score = max_score.max(score);

        next_teaspoons(&mut num_teaspoons, max_teaspoons);

        if matches!(&num_teaspoons[..], [start @ .., last] if *last == max_teaspoons && start.iter().all(|t| *t == 0))
        {
            if hit_max {
                break;
            } else {
                hit_max = true;
            }
        }
    }

    max_score
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = std::fs::read_to_string("inputs/aoc_1515.txt")?;
    let ingredients: Vec<_> = input
        .lines()
        .map(str::trim)
        .map(Ingredient::parse)
        .collect::<Result<_>>()
        .unwrap();

    println!(
        "Part 1 output: {}",
        cookie_search(&ingredients, 100, |_| true)
    );
    println!(
        "Part 2 output: {}",
        cookie_search(&ingredients, 100, |c| c == 500)
    );

    Ok(())
}

#[cfg(test)]
mod tests_1515 {
    use super::*;

    #[test]
    fn parse_test() {
        let input = "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
        Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";

        let expected = [
            Ingredient {
                name: "Butterscotch".to_owned(),
                capacity: -1,
                durability: -2,
                flavor: 6,
                texture: 3,
                calories: 8,
            },
            Ingredient {
                name: "Cinnamon".to_owned(),
                capacity: 2,
                durability: 3,
                flavor: -2,
                texture: -1,
                calories: 3,
            },
        ];

        let actual: Vec<_> = input
            .lines()
            .map(str::trim)
            .map(Ingredient::parse)
            .collect::<Result<_>>()
            .unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn score_test() {
        let ingrediants = [
            Ingredient {
                name: "Butterscotch".to_owned(),
                capacity: -1,
                durability: -2,
                flavor: 6,
                texture: 3,
                calories: 8,
            },
            Ingredient {
                name: "Cinnamon".to_owned(),
                capacity: 2,
                durability: 3,
                flavor: -2,
                texture: -1,
                calories: 3,
            },
        ];
        let teaspoons = [44, 56];

        let expected = 62842880;
        let actual = get_score(&ingrediants, &teaspoons, |_| true);
        assert_eq!(expected, actual);
    }

    #[test]
    fn next_teaspoons_test() {
        let tests: &mut [(_, &[_])] = &mut [
            (vec![100, 0], &[99, 1]),
            (vec![45, 55], &[44, 56]),
            (vec![99, 0, 1], &[98, 1, 1]),
            (vec![100, 0, 0], &[99, 1, 0]),
            (vec![99, 1, 0], &[98, 2, 0]),
            (vec![1, 99, 0], &[0, 100, 0]),
            (vec![0, 100, 0], &[99, 0, 1]),
            (vec![0, 0, 100], &[0, 0, 100]),
            (vec![0, 99, 1, 0], &[98, 0, 2, 0]),
        ];

        for (i, (test, expected)) in tests.iter_mut().enumerate() {
            next_teaspoons(test, 100);
            assert_eq!(test, expected, "{}", i);
        }
    }

    #[test]
    fn part1_example() {
        let input = "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
        Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";

        let ingredients: Vec<_> = input
            .lines()
            .map(str::trim)
            .map(Ingredient::parse)
            .collect::<Result<_>>()
            .unwrap();

        let expected = 62842880;

        assert_eq!(expected, cookie_search(&ingredients, 100, |_| true));
    }

    #[test]
    fn part2_example() {
        let input = "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
        Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";

        let ingredients: Vec<_> = input
            .lines()
            .map(str::trim)
            .map(Ingredient::parse)
            .collect::<Result<_>>()
            .unwrap();

        let expected = 57600000;

        assert_eq!(expected, cookie_search(&ingredients, 100, |c| c == 500));
    }
}
