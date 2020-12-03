use advent_of_code_2015::run;
use color_eyre::eyre::{Report, Result};
use serde_json::Value;

fn part1(input: &Value) -> Result<i64> {
    Ok(match input {
        Value::Number(n) => n.as_i64().unwrap(),
        Value::Array(arr) => arr
            .iter()
            .map(part1)
            .try_fold(0, |acc, i| Ok::<_, Report>(acc + i?))?,
        Value::Object(map) => map
            .values()
            .map(part1)
            .try_fold(0, |acc, i| Ok::<_, Report>(acc + i?))?,
        _ => 0,
    })
}

fn part2(input: &Value) -> Result<i64> {
    Ok(match input {
        Value::Number(n) => n.as_i64().unwrap(),
        Value::Array(arr) => arr
            .iter()
            .map(part2)
            .try_fold(0, |acc, i| Ok::<_, Report>(acc + i?))?,
        Value::Object(map)
            if !map
                .values()
                .any(|v| matches!(v, Value::String(v) if v == "red")) =>
        {
            map.values()
                .map(part2)
                .try_fold(0, |acc, i| Ok::<_, Report>(acc + i?))?
        }
        _ => 0,
    })
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = std::fs::read_to_string("inputs/aoc_1512.txt")?;
    let values: Value = serde_json::from_str(&input)?;

    run("Day 12: JSAbacusFramework.io", &values, &[&part1, &part2])
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
            assert_eq!(part1(&test).unwrap(), expected, "{}", test);
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
            assert_eq!(part2(&test).unwrap(), expected, "{}", test);
        }
    }
}
