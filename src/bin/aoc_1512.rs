use color_eyre::eyre::Result;
use serde_json::Value;

fn part1(input: &Value) -> i64 {
    match input {
        Value::Number(n) => n.as_i64().unwrap(),
        Value::Array(arr) => arr.iter().map(part1).sum(),
        Value::Object(map) => map.values().map(part1).sum(),
        _ => 0,
    }
}

fn part2(input: &Value) -> i64 {
    match input {
        Value::Number(n) => n.as_i64().unwrap(),
        Value::Array(arr) => arr.iter().map(part2).sum(),
        Value::Object(map)
            if !map
                .values()
                .any(|v| matches!(v, Value::String(v) if v == "red")) =>
        {
            map.values().map(part2).sum()
        }
        _ => 0,
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = std::fs::read_to_string("inputs/aoc_1512.txt")?;
    let values: Value = serde_json::from_str(&input)?;

    let start = std::time::Instant::now();

    let part1 = part1(&values);
    let part2 = part2(&values);

    let elapsed = start.elapsed();

    println!("Part 1 output: {}", part1);
    println!("Part 2 output: {}", part2);

    println!("Elapsed: {}us", elapsed.as_micros());

    Ok(())
}

#[cfg(test)]
mod tests_1512 {
    use super::*;

    #[test]
    fn part1_example() {
        let tests = [
            ("[1,2,3]", 6),
            (r#"{"a":2,"b":4}"#, 6),
            ("[[[3]]]", 3),
            (r#"{"a":{"b":4},"c":-1}"#, 3),
            (r#"{"a":[-1,1]}"#, 0),
            (r#"[-1,{"a":1}]"#, 0),
            ("[]", 0),
            ("{}", 0),
        ];

        for &(test, expected) in &tests {
            let test: Value = serde_json::from_str(test).unwrap();
            assert_eq!(part1(&test), expected, "{}", test);
        }
    }

    #[test]
    fn part2_example() {
        let tests = [
            ("[1,2,3]", 6),
            (r#"[1,{"c":"red","b":2},3]"#, 4),
            (r#"{"d":"red","e":[1,2,3,4],"f":5}"#, 0),
            (r#"[1,"red",5]"#, 6),
        ];

        for &(test, expected) in &tests {
            let test: Value = serde_json::from_str(test).unwrap();
            assert_eq!(part2(&test), expected, "{}", test);
        }
    }
}
