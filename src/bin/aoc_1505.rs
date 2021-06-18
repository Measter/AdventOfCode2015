use aoc_lib::TracingAlloc;
use color_eyre::eyre::Result;

#[global_allocator]
static ALLOC: TracingAlloc = TracingAlloc::new();

fn part1(input: &str) -> bool {
    let vowels: &[char] = &['a', 'e', 'i', 'o', 'u'];
    let has_three_vowels = input.matches(vowels).count() >= 3;

    let has_invalid_strings = input.contains("ab")
        || input.contains("cd")
        || input.contains("pq")
        || input.contains("xy");

    // We know the length is 16 chars.
    let chars = {
        let mut chars = ['\0'; 16];
        chars
            .iter_mut()
            .zip(input.chars())
            .for_each(|(dst, src)| *dst = src);
        chars
    };
    let has_double_char = chars
        .windows(2)
        .any(|pair| matches!(pair, [a, b] if a == b));

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

    let sep_letters = chars
        .windows(3)
        .any(|trio| matches!(trio, [a, _, b] if a == b));

    let mut has_two_pairs = false;

    for (idx, _) in chars.windows(2).enumerate() {
        let has_valids =
            input[idx + 2..]
                .match_indices(&input[idx..idx + 2])
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

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = aoc_lib::input(2015, 5).open()?;
    let (p1_res, p1_bench) = aoc_lib::bench(&ALLOC, "Part 1", || {
        Ok::<_, ()>(
            input
                .lines()
                .map(str::trim)
                .map(part1)
                .filter(|i| *i)
                .count(),
        )
    })?;
    let (p2_res, p2_bench) = aoc_lib::bench(&ALLOC, "Part 2", || {
        Ok::<_, ()>(
            input
                .lines()
                .map(str::trim)
                .map(part2)
                .filter(|i| *i)
                .count(),
        )
    })?;

    aoc_lib::display_results(
        "Day 5: Doesn't He Have Intern-Elves For This?",
        [(&p1_res, p1_bench), (&p2_res, p2_bench)],
    );

    Ok(())
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
            assert_eq!(part1(&input), output, "{}", input);
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
            assert_eq!(part2(&input), output, "{}", input);
        }
    }
}
