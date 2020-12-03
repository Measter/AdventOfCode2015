#![allow(clippy::unnecessary_wraps)]

use advent_of_code_2015::run;
use color_eyre::eyre::Result;

fn banned_char(c: char) -> bool {
    c == 'i' || c == 'o' || c == 'l'
}

fn part1_validity(pswd: &str) -> bool {
    let has_banned_letters = pswd.matches(banned_char).count() != 0;

    let chars: Vec<_> = pswd.chars().collect();

    let has_triplet = chars
        .windows(3)
        .filter(|triplet| {
            matches!(triplet, [a, b, c] if {
                let a = *a as u8;
                let b = *b as u8;
                let c = *c as u8;

                a<b && b<c && b-a == 1 && c-a == 2
            })
        })
        .count()
        > 0;

    let mut seen_pairs = 0;

    let mut windows = chars.windows(2);
    while let Some([a, b]) = windows.next() {
        if a != b {
            continue;
        }

        seen_pairs += 1;
        windows.next();
    }

    !has_banned_letters && has_triplet && seen_pairs >= 2
}

fn part1_next_password(pswd: &str) -> Result<String> {
    let max_num = 26u64.pow(9) - 1;

    let decoded = pswd
        .bytes()
        .map(|b| (b - b'a') as u64)
        .fold(0, |acc, b| acc * 26 + b);

    let mut next_buf = String::with_capacity(8);

    'outer: for mut i in (decoded + 1)..=max_num {
        next_buf.clear();

        for _ in 0..8 {
            let rem = i % 26;
            i /= 26;
            let next_char = (rem as u8 + b'a') as char;
            if banned_char(next_char) {
                continue 'outer;
            }
            next_buf.push(next_char);
        }

        // Safe because we know we're only dealing with ascii.
        unsafe { next_buf.as_bytes_mut().reverse() };

        if part1_validity(&next_buf) {
            break;
        }
    }

    Ok(next_buf)
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = std::fs::read_to_string("inputs/aoc_1511.txt")?;

    run(
        "Day 11: Corporate Policy",
        input.as_str(),
        &[&part1_next_password, &|pswd| {
            part1_next_password(pswd).and_then(|pswd| part1_next_password(&*pswd))
        }],
    )
}

#[cfg(test)]
mod tests_1511 {
    use super::*;

    #[test]
    fn part1_validity_test() {
        let tests = [
            ("hijklmmn", false),
            ("abbceffg", false),
            ("abbcegjk", false),
            ("abcdefgh", false),
            ("abcdffaa", true),
            ("ghijklmn", false),
            ("ghjaabcc", true),
        ];

        for &(pswd, expected) in &tests {
            assert_eq!(part1_validity(pswd), expected, "{}", pswd);
        }
    }

    #[test]
    fn part1_next_password_test() {
        let tests = [("abcdefgh", "abcdffaa"), ("ghijklmn", "ghjaabcc")];

        for &(pswd, expected) in &tests {
            assert_eq!(part1_next_password(pswd).unwrap(), expected, "{}", pswd);
        }
    }
}
