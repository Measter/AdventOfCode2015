use aoc_lib::{misc::ArrWindows, Bench, BenchResult, Day, NoError};

pub const DAY: Day = Day {
    day: 5,
    name: "Doesn't He Have Intern-Elves For This?",
    part_1: run_part1,
    part_2: Some(run_part2),
    other: &[],
};

fn run_part1(input: &str, b: Bench) -> BenchResult {
    b.bench(|| {
        Ok::<_, NoError>(
            input
                .lines()
                .map(str::trim)
                .map(part1)
                .filter(|i| *i)
                .count(),
        )
    })
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    b.bench(|| {
        Ok::<_, NoError>(
            input
                .lines()
                .map(str::trim)
                .map(part2)
                .filter(|i| *i)
                .count(),
        )
    })
}

fn part1(input: &str) -> bool {
    let vowels: &[char] = &['a', 'e', 'i', 'o', 'u'];
    let has_three_vowels = input.matches(vowels).count() >= 3;

    // We know the length is 16 chars.
    let chars = {
        let mut chars = ['\0'; 16];
        chars
            .iter_mut()
            .zip(input.chars())
            .for_each(|(dst, src)| *dst = src);
        chars
    };
    let mut has_invalid_strings = false;
    let mut has_double_char = false;
    for pair in ArrWindows::new(&chars) {
        has_invalid_strings |= matches!(pair, ['a', 'b'] | ['c', 'd'] | ['p', 'q'] | ['x', 'y']);
        has_double_char |= pair[0] == pair[1];
    }

    has_three_vowels && !has_invalid_strings && has_double_char
}

fn part2(input: &str) -> bool {
    // We know the length is 16 chars.
    let chars = {
        let mut chars = ['\0'; 16];
        chars
            .iter_mut()
            .zip(input.chars())
            .for_each(|(dst, src)| *dst = src);
        chars
    };

    let sep_letters = ArrWindows::new(&chars).any(|[a, _, b]| a == b);

    let mut has_two_pairs = false;

    for (idx, pair) in ArrWindows::<_, 2>::new(&chars).enumerate() {
        let has_valids = ArrWindows::new(&chars[idx + 2..])
            .enumerate()
            .filter(|&(_, pair2)| pair == pair2)
            .any(|(mut m_idx, _)| {
                m_idx += idx + 2; // Need to account for the offset start of string.
                let diff = idx.max(m_idx) - idx.min(m_idx);
                idx != m_idx && diff >= 2
            });

        if has_valids {
            has_two_pairs = true;
            break;
        }
    }

    sep_letters && has_two_pairs
}

#[cfg(test)]
mod tests_1505 {
    use super::*;

    #[test]
    fn part1_examples() {
        let vals = [
            ("ugknbfddgicrmopn", true),
            ("aaa", true),
            ("jchzalrnumimnmhp", false),
            ("haegwjzuvuyypxyu", false),
            ("dvszwmarrgswjxmb", false),
        ];

        for &(input, output) in &vals {
            assert_eq!(part1(input), output, "{}", input);
        }
    }

    #[test]
    fn part2_examples() {
        let vals = [
            ("qjhvhtzxzqqjkmpb", true),
            ("xxyxx", true),
            ("uurcxstgmygtbstg", false),
            ("ieodomkazucvgmuy", false),
        ];

        for &(input, output) in &vals {
            assert_eq!(part2(input), output, "{}", input);
        }
    }
}
