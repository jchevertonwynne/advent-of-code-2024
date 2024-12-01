use std::collections::HashMap;

use anyhow::Context;
use fxhash::FxHashMap;

use crate::{DayResult, IntoDayResult};

pub fn solve(input: &str) -> anyhow::Result<DayResult> {
    let (mut a, mut b) = parse_inputs(input)?;

    a.sort_unstable();
    b.sort_unstable();

    let mut p1 = 0;
    for (&a, &b) in a.iter().zip(b.iter()) {
        p1 += a.abs_diff(b)
    }

    let counts: HashMap<usize, usize, _> = {
        let mut counts = FxHashMap::default();
        for b in b {
            *counts.entry(b).or_default() += 1_usize;
        }
        counts
    };
    let mut p2: usize = 0;
    for a in a {
        let count = counts.get(&a).cloned().unwrap_or_default();
        let mult = a * count;
        p2 += mult;
    }

    (p1, p2).into_result()
}

fn parse_inputs(s: &str) -> anyhow::Result<(Vec<usize>, Vec<usize>)> {
    s.lines()
        .try_fold((vec![], vec![]), |(mut a, mut b), line| {
            let mut line = line.split_whitespace();
            a.push(
                line.next()
                    .context("failed to get next from split")?
                    .parse()
                    .context("failed to parse usize")?,
            );
            b.push(
                line.next()
                    .context("failed to get next from split")?
                    .parse()
                    .context("failed to parse usize")?,
            );

            Ok::<_, anyhow::Error>((a, b))
        })
}

#[cfg(test)]
mod tests {
    use crate::{days::day01::solve, IntoDayResult};

    #[test]
    fn works_for_example() {
        const INPUT: &str = include_str!("../../test_input/day01.txt");
        let solution = solve(INPUT).unwrap();
        assert_eq!((11, 31).into_day_result(), solution);
    }

    #[test]
    fn works_for_input() {
        const INPUT: &str =
            include_str!(concat!(std::env!("AOC_CACHE"), "/2024_", "day01", ".txt"));
        let solution = solve(INPUT).unwrap();
        assert_eq!((1_603_498, 25_574_739).into_day_result(), solution);
    }
}
