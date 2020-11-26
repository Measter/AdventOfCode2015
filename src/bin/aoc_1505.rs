use color_eyre::eyre::Result;

fn part1(input: &str) -> bool {
    let vowels: &[char] = &['a', 'e', 'i', 'o', 'u'];
    let has_three_vowels = input.matches(vowels).count() >= 3;

    let has_invalid_strings = input.contains("ab")
        || input.contains("cd")
        || input.contains("pq")
        || input.contains("xy");

    let chars: Vec<_> = input.chars().collect();
    let has_double_char = chars
        .windows(2)
        .any(|pair| matches!(pair, [a, b] if a == b));

    has_three_vowels && !has_invalid_strings && has_double_char
}

fn part2(input: &str) -> bool {
    let chars: Vec<_> = input.chars().collect();

    let sep_letters = chars
        .windows(3)
        .any(|trio| matches!(trio, [a, _, b] if a == b));

    let mut has_two_pairs = false;

    for (idx, _) in chars.windows(2).enumerate() {
        let valids = input
            .match_indices(&input[idx..idx + 2])
            .filter(|&(m_idx, _)| {
                let diff = idx.max(m_idx) - idx.min(m_idx);
                idx != m_idx && diff >= 2
            })
            .count();

        if valids > 0 {
            has_two_pairs = true;
            break;
        }
    }

    sep_letters && has_two_pairs
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = std::fs::read_to_string("inputs/aoc_1505.txt")?;

    let start = std::time::Instant::now();

    let part_1 = input
        .lines()
        .map(|line| part1(line.trim()))
        .filter(|i| *i)
        .count();

    let part_2 = input
        .lines()
        .map(|line| part2(line.trim()))
        .filter(|i| *i)
        .count();

    let elapsed = start.elapsed();

    println!("Part 1 output: {}", part_1);
    println!("Part 2 output: {}", part_2);

    println!("Elapsed: {}us", elapsed.as_micros());

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
