use color_eyre::eyre::Result;

fn part1(input: &str) -> (usize, usize) {
    let mut rendered = String::with_capacity(input.len());

    let mut input_rem = input;
    loop {
        match input_rem.as_bytes() {
            [b'"', ..] => {
                input_rem = &input_rem[1..];
            }
            [b'\\', b'\\', ..] => {
                rendered.push('\\');
                input_rem = &input_rem[2..];
            }
            [b'\\', b'"', ..] => {
                rendered.push('"');
                input_rem = &input_rem[2..];
            }
            [b'\\', b'x', a, b, ..] if a.is_ascii_hexdigit() && b.is_ascii_hexdigit() => {
                let a = match a.to_ascii_lowercase() {
                    b'0'..=b'9' => a - b'0',
                    b'a'..=b'f' => a - b'a',
                    _ => *a,
                };

                let b = match b.to_ascii_lowercase() {
                    b'0'..=b'9' => b - b'0',
                    b'a'..=b'f' => b - b'a',
                    _ => *b,
                };

                let ch = (a << 4 | b) as char;
                rendered.push(ch);
                input_rem = &input_rem[4..];
            }
            [_, ..] => {
                let ch = input_rem.chars().next().unwrap(); // We know it's not empty.
                rendered.push(ch);
                input_rem = &input_rem[ch.len_utf8()..];
            }
            [] => break,
        }
    }

    (input.len(), rendered.chars().count())
}

fn part2(input: &str) -> (usize, usize) {
    let mut rendered = String::new();
    rendered.push('"');

    for c in input.chars() {
        match c {
            '"' => rendered.push_str(r#"\""#),
            '\\' => rendered.push_str(r#"\\"#),
            _ => rendered.push(c),
        }
    }

    rendered.push('"');

    (input.len(), rendered.len())
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = std::fs::read_to_string("inputs/aoc_1508.txt")?;

    aoc_lib::run(
        "Day 8: Matchsticks",
        input.as_str(),
        &|i| {
            Ok(i.lines()
                .map(str::trim)
                .map(part1)
                .map(|(code, rendered)| code - rendered)
                .sum::<usize>())
        },
        &|i| {
            Ok(i.lines()
                .map(str::trim)
                .map(part2)
                .map(|(code, rendered)| rendered - code)
                .sum::<usize>())
        },
    )
}

#[cfg(test)]
mod tests_1508 {
    use super::*;

    #[test]
    fn part1_example() {
        let tests = [
            (r#""""#, 2, 0),
            (r#""abc""#, 5, 3),
            (r#""aaa\"aaa""#, 10, 7),
            (r#""\x27""#, 6, 1),
        ];

        for &(input, expected_code, expected_string) in &tests {
            let (code, string) = part1(input);
            assert_eq!(code, expected_code, "{}", input);
            assert_eq!(string, expected_string, "{}", input);
        }
    }

    #[test]
    fn part2_example() {
        let tests = [
            (r#""""#, 2, 6),
            (r#""abc""#, 5, 9),
            (r#""aaa\"aaa""#, 10, 16),
            (r#""\x27""#, 6, 11),
        ];

        for &(input, expected_code, expected_string) in &tests {
            let (code, string) = part2(input);
            assert_eq!(code, expected_code, "{}", input);
            assert_eq!(string, expected_string, "{}", input);
        }
    }
}
