use aoc_lib::{misc::ArrWindows, Bench, BenchResult, Day};
use color_eyre::eyre::{eyre, Result};

pub const DAY: Day = Day {
    day: 11,
    name: "Corporate Policy",
    part_1: run_part1,
    part_2: Some(run_part2),
    other: &[],
};

fn run_part1(input: &str, b: Bench) -> BenchResult {
    b.bench(|| part1_next_password(input))
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    b.bench(|| part1_next_password(input).and_then(|pswd| part1_next_password(&pswd)))
}

fn banned_char(c: char) -> bool {
    c == 'i' || c == 'o' || c == 'l'
}

fn part1_validity(char_buffer: &[char]) -> bool {
    let has_banned_letters = char_buffer.iter().any(|&c| banned_char(c));

    if has_banned_letters {
        return false;
    }

    let has_triplet = ArrWindows::new(char_buffer)
        .map(|&[a, b, c]| [a as u8, b as u8, c as u8])
        .any(|[a, b, c]| a < b && b < c && b - a == 1 && c - a == 2);

    if !has_triplet {
        return false;
    }

    let mut seen_pairs = 0;

    let mut windows = ArrWindows::new(char_buffer);
    while let Some([a, b]) = windows.next() {
        if a != b {
            continue;
        }

        seen_pairs += 1;
        if seen_pairs == 2 {
            break;
        }
        windows.next();
    }

    seen_pairs >= 2
}

fn part1_next_password(pswd: &str) -> Result<String> {
    let max_num = 26u64.pow(9) - 1;

    let decoded = pswd
        .bytes()
        .map(|b| (b - b'a') as u64)
        .fold(0, |acc, b| acc * 26 + b);

    let mut char_buffer = Vec::with_capacity(8);

    'outer: for mut i in (decoded + 1)..=max_num {
        char_buffer.clear();

        for _ in 0..8 {
            let rem = i % 26;
            i /= 26;
            let next_char = (rem as u8 + b'a') as char;
            if banned_char(next_char) {
                continue 'outer;
            }
            char_buffer.push(next_char);
        }

        char_buffer.reverse();

        if part1_validity(&char_buffer) {
            return Ok(char_buffer.into_iter().collect());
        }
    }

    Err(eyre!("No next password found"))
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

        let mut char_buf = Vec::new();

        for &(pswd, expected) in &tests {
            char_buf.clear();
            char_buf.extend(pswd.chars());
            assert_eq!(part1_validity(&char_buf), expected, "{}", pswd);
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
