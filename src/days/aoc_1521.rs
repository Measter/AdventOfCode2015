use aoc_lib::{Bench, BenchResult, Day, ParseResult, UserError};
use color_eyre::{
    eyre::{eyre, Result},
    Report,
};
use itertools::Itertools;

use std::iter;

pub const DAY: Day = Day {
    day: 21,
    name: "RPG Simulator 20XX",
    part_1: run_part1,
    part_2: Some(run_part2),
    other: &[("Parse", run_parse)],
};

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let boss = Actor::parse(input).map_err(UserError)?;

    let weapons = Equipment::get_weapons();
    let armor = Equipment::get_armor();
    let rings = Equipment::get_rings();

    b.bench(|| part1(&boss, &weapons, &armor, &rings))
}
fn run_part2(input: &str, b: Bench) -> BenchResult {
    let boss = Actor::parse(input).map_err(UserError)?;

    let weapons = Equipment::get_weapons();
    let armor = Equipment::get_armor();
    let rings = Equipment::get_rings();

    b.bench(|| part2(&boss, &weapons, &armor, &rings))
}
fn run_parse(input: &str, b: Bench) -> BenchResult {
    b.bench(|| {
        let data = Actor::parse(input)?;
        Ok::<_, Report>(ParseResult(data))
    })
}

#[derive(Debug)]
struct Equipment {
    #[allow(unused)]
    name: &'static str,
    cost: u16,
    damage: i16,
    armor: i16,
}

impl Equipment {
    fn parse(input: &'static str) -> Equipment {
        let mut parts = input.split_whitespace();

        let name = parts.next();

        let cost = (&mut parts).map(str::parse).next();

        let mut parts = parts.map(str::parse);
        let damage = parts.next();
        let armor = parts.next();

        if let (Some(name), Some(Ok(cost)), Some(Ok(damage)), Some(Ok(armor))) =
            (name, cost, damage, armor)
        {
            Equipment {
                name,
                cost,
                damage,
                armor,
            }
        } else {
            panic!("Invalid equipment");
        }
    }

    fn get_weapons() -> Vec<Equipment> {
        let vals = "Dagger        8     4       0
        Shortsword   10     5       0
        Warhammer    25     6       0
        Longsword    40     7       0
        Greataxe     74     8       0";

        vals.lines().map(str::trim).map(Self::parse).collect()
    }

    fn get_armor() -> Vec<Equipment> {
        let vals = "Leather      13     0       1
        Chainmail    31     0       2
        Splintmail   53     0       3
        Bandedmail   75     0       4
        Platemail   102     0       5";

        vals.lines().map(str::trim).map(Self::parse).collect()
    }

    fn get_rings() -> Vec<Equipment> {
        let vals = "Damage+1    25     1       0
        Damage+2    50     2       0
        Damage+3   100     3       0
        Defense+1   20     0       1
        Defense+2   40     0       2
        Defense+3   80     0       3";

        vals.lines().map(str::trim).map(Self::parse).collect()
    }

    const NULL_EQUIPMENT: Equipment = Equipment {
        name: "Blank Ring",
        cost: 0,
        damage: 0,
        armor: 0,
    };
}

#[derive(Debug, Copy, Clone)]
struct Actor {
    hp: i16,
    damage: i16,
    armor: i16,
}

impl Actor {
    fn parse(input: &str) -> Result<Actor> {
        let mut lines = input.lines().map(str::trim);

        let (_, hp) = lines
            .next()
            .and_then(|l| l.split_once(": "))
            .ok_or_else(|| eyre!("Invalid boss input"))?;

        let (_, damage) = lines
            .next()
            .and_then(|l| l.split_once(": "))
            .ok_or_else(|| eyre!("Invalid boss input"))?;

        let (_, armor) = lines
            .next()
            .and_then(|l| l.split_once(": "))
            .ok_or_else(|| eyre!("Invalid boss input"))?;

        Ok(Actor {
            hp: hp.parse()?,
            damage: damage.parse()?,
            armor: armor.parse()?,
        })
    }

    fn can_defeat(&self, opponent: &Self) -> bool {
        let self_damage = (self.damage - opponent.armor).max(1);
        let opponent_damage = (opponent.damage - self.armor).max(1);

        let self_death_time = match (self.hp / opponent_damage, self.hp % opponent_damage) {
            (r, 0) => r,
            (r, _) => r + 1,
        };

        let opponent_death_time = match (opponent.hp / self_damage, opponent.hp % self_damage) {
            (r, 0) => r,
            (r, _) => r + 1,
        };

        self_death_time >= opponent_death_time
    }
}

fn part1(
    boss: &Actor,
    weapons: &[Equipment],
    armor: &[Equipment],
    rings: &[Equipment],
) -> Result<u16> {
    let rings_iter = iter::once((&Equipment::NULL_EQUIPMENT, &Equipment::NULL_EQUIPMENT))
        .chain(rings.iter().map(|r| (r, &Equipment::NULL_EQUIPMENT)))
        .chain(rings.iter().tuple_combinations());

    let equipment = weapons
        .iter()
        .cartesian_product(iter::once(&Equipment::NULL_EQUIPMENT).chain(armor))
        .cartesian_product(rings_iter);

    let mut cost = u16::MAX;

    for ((weapon, armor), (ring_a, ring_b)) in equipment {
        let player = Actor {
            hp: 100,
            damage: weapon.damage + ring_a.damage + ring_b.damage,
            armor: armor.armor + ring_a.armor + ring_b.armor,
        };

        if player.can_defeat(boss) {
            cost = cost.min(weapon.cost + armor.cost + ring_a.cost + ring_b.cost);
        }
    }

    Ok(cost)
}

fn part2(
    boss: &Actor,
    weapons: &[Equipment],
    armor: &[Equipment],
    rings: &[Equipment],
) -> Result<u16> {
    let rings_iter = std::iter::once((&Equipment::NULL_EQUIPMENT, &Equipment::NULL_EQUIPMENT))
        .chain(rings.iter().map(|r| (r, &Equipment::NULL_EQUIPMENT)))
        .chain(rings.iter().tuple_combinations());

    let equipment = weapons
        .iter()
        .cartesian_product(iter::once(&Equipment::NULL_EQUIPMENT).chain(armor))
        .cartesian_product(rings_iter);

    let mut cost = 0;

    for ((weapon, armor), (ring_a, ring_b)) in equipment {
        let player = Actor {
            hp: 100,
            damage: weapon.damage + ring_a.damage + ring_b.damage,
            armor: armor.armor + ring_a.armor + ring_b.armor,
        };

        if !player.can_defeat(boss) {
            cost = cost.max(weapon.cost + armor.cost + ring_a.cost + ring_b.cost);
        }
    }

    Ok(cost)
}

#[cfg(test)]
mod tests_1521 {
    use super::*;

    #[test]
    fn part1_example() {
        let boss = Actor {
            hp: 12,
            damage: 7,
            armor: 2,
        };

        let player = Actor {
            hp: 8,
            damage: 5,
            armor: 5,
        };

        assert!(player.can_defeat(&boss));
    }
}
