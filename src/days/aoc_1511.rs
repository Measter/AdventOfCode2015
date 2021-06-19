use aoc_lib::{day, misc::ArrWindows, Bench, BenchResult};
use color_eyre::eyre::Result;

day! {
    day 11: "Corporate Policy"
    1: run_part1
    2: run_part2
}

fn run_part1(input: &str, b: Bench) -> BenchResult {
    b.bench(|| part1_next_password(input))
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    b.bench(|| part1_next_password(input).and_then(|pswd| part1_next_password(&pswd)))
}

fn banned_char(c: char) -> bool {
    c == 'i' || c == 'o' || c == 'l'
}

fn part1_validity(pswd: &str, char_buffer: &mut Vec<char>) -> bool {
    let has_banned_letters = pswd.matches(banned_char).count() != 0;

    char_buffer.clear();
    char_buffer.extend(pswd.chars());

    let has_triplet = ArrWindows::new(&char_buffer).any(|&[a, b, c]| {
        let a = a as u8;
        let b = b as u8;
        let c = c as u8;

        a < b && b < c && b - a == 1 && c - a == 2
    });

    let mut seen_pairs = 0;

    let mut windows = char_buffer.windows(2);
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

    !has_banned_letters && has_triplet && seen_pairs >= 2
}

fn part1_next_password(pswd: &str) -> Result<String> {
    let max_num = 26u64.pow(9) - 1;

    let decoded = pswd
        .bytes()
        .map(|b| (b - b'a') as u64)
        .fold(0, |acc, b| acc * 26 + b);

    let mut next_buf = String::with_capacity(8);
    let mut char_buffer = Vec::new();

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

        // SAFETY: We know our input is ASCII.
        unsafe { next_buf.as_bytes_mut().reverse() };

        if part1_validity(&next_buf, &mut char_buffer) {
            break;
        }
    }

    Ok(next_buf)
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
            assert_eq!(part1_validity(pswd, &mut char_buf), expected, "{}", pswd);
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
