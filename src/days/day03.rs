use anyhow::Context;
use regex::Regex;

use crate::{DayResult, IntoDayResult};

pub fn solve(input: &str) -> anyhow::Result<DayResult> {
    let re = Regex::new(r#"(mul\((\d+),(\d+)\)|do\(\)|don't\(\))"#)?;
    let mut p1 = 0;
    let mut p2 = 0;
    let mut enabled = true;
    for found in re.captures_iter(input) {
        let full = found.get(0).context("should match")?;
        match full.as_str() {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ => {
                let a: usize = found
                    .get(2)
                    .context("should be a first int")?
                    .as_str()
                    .parse()?;
                let b: usize = found
                    .get(3)
                    .context("should be a first int")?
                    .as_str()
                    .parse()?;
                p1 += a * b;
                if enabled {
                    p2 += a * b
                }
            }
        }
    }
    (p1, p2).into_result()
}

#[cfg(test)]
mod tests {
    use crate::{days::day03::solve, IntoDayResult};

    #[test]
    fn works_for_example() {
        const INPUT: &str = include_str!("../../test_input/day03.txt");
        let solution = solve(INPUT).unwrap();
        assert_eq!((161, 48).into_day_result(), solution);
    }

    #[test]
    fn works_for_input() {
        const INPUT: &str =
            include_str!(concat!(std::env!("AOC_CACHE"), "/2024_", "day03", ".txt"));
        let solution = solve(INPUT).unwrap();
        assert_eq!((166_630_675, 93_465_710).into_day_result(), solution);
    }
}
