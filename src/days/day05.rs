use crate::{DayResult, IntoDayResult};
use anyhow::{Context, Result};

pub fn solve(input: &str) -> Result<DayResult> {
    let (rules, updates) = parse_rulesets(input)?;

    let mut p1 = 0;
    let mut p2 = 0;
    let mut update = Vec::new();
    let mut counts = [0; 100];
    for update_str in updates.lines() {
        update.clear();
        for num in update_str.split(",") {
            update.push(num.parse::<usize>()?);
        }

        let mut contains = [None; 100];
        for (i, u) in update.iter().enumerate() {
            contains[*u] = Some(i);
        }

        if is_success(&rules, &update, &contains) {
            p1 += update[update.len() / 2];
        } else {
            for &u in &update {
                let count = rules
                    .iter_second(u)
                    .filter(|&first| contains[first].is_some())
                    .count();
                counts[u] = count;
            }
            update.sort_unstable_by_key(|&i| counts[i]);
            p2 += update[update.len() / 2];
        }
    }

    (p1, p2).into_result()
}

fn parse_rulesets(input: &str) -> Result<(RuleSets, &str)> {
    let (rules_str, updates_str) = input.split_once("\n\n").context("no double newline")?;
    let rules = RuleSets::new(rules_str)?;

    Ok((rules, updates_str))
}

fn is_success(rules: &RuleSets, update: &[usize], contains: &[Option<usize>; 100]) -> bool {
    update.iter().enumerate().all(|(i, &u)| {
        for first in rules.iter_second(u) {
            if contains[first].map(|n| n > i).unwrap_or(false) {
                return false;
            }
        }
        for second in rules.iter_first(u) {
            if contains[second].map(|n| n < i).unwrap_or(false) {
                return false;
            }
        }

        true
    })
}

struct RuleSets {
    first: [Vec<usize>; 100],
    second: [Vec<usize>; 100],
}

impl RuleSets {
    fn new(rules_str: &str) -> Result<Self> {
        let mut rules_first = std::array::from_fn(|_| Vec::new());
        let mut rules_second = std::array::from_fn(|_| Vec::new());

        for rule in rules_str.lines() {
            let (first, second) = rule.split_once("|").context("no vertical bar")?;
            let first: usize = first.parse()?;
            let second: usize = second.parse()?;
            rules_first[first].push(second);
            rules_second[second].push(first);
        }

        Ok(Self {
            first: rules_first,
            second: rules_second,
        })
    }

    fn iter_first(&self, i: usize) -> impl Iterator<Item = usize> + '_ {
        self.first[i].iter().cloned()
    }

    fn iter_second(&self, i: usize) -> impl Iterator<Item = usize> + '_ {
        self.second[i].iter().cloned()
    }
}

#[cfg(test)]
mod tests {
    use crate::{days::day05::solve, IntoDayResult};

    #[test]
    fn works_for_example() {
        const INPUT: &str = include_str!("../../test_input/day05.txt");
        let solution = solve(INPUT).unwrap();
        assert_eq!((143, 123).into_day_result(), solution);
    }

    #[test]
    fn works_for_input() {
        const INPUT: &str =
            include_str!(concat!(std::env!("AOC_CACHE"), "/2024_", "day05", ".txt"));
        let solution = solve(INPUT).unwrap();
        assert_eq!((4_774, 6_004).into_day_result(), solution);
    }
}
