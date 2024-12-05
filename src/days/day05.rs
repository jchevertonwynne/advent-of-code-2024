use crate::{DayResult, IntoDayResult};
use anyhow::{Context, Result};
use fxhash::FxHashMap as HashMap;

pub fn solve(input: &str) -> Result<DayResult> {
    let (rules, updates) = parse_input(input)?;

    let mut to_left = HashMap::default();
    let mut ordered = Vec::new();
    let mut p1 = 0;
    let mut p2 = 0;
    for update in updates.iter() {
        let mut contains = [None; 100];
        for (i, u) in update.iter().enumerate() {
            contains[*u] = Some(i);
        }
        if is_success(&rules, update, &contains) {
            p1 += update[update.len() / 2];
        } else {
            ordered.clear();
            for &u in update {
                let count = rules
                    .iter_second(u)
                    .filter(|&first| contains[first].is_some())
                    .count();
                to_left.insert(u, count);
            }
            for tl in to_left.drain() {
                ordered.push(tl);
            }
            ordered.sort_unstable_by(|a, b| a.1.cmp(&b.1));
            p2 += ordered[ordered.len() / 2].0;
        }
    }

    (p1, p2).into_result()
}

fn parse_input(input: &str) -> Result<(RuleSets, Vec<Vec<usize>>), anyhow::Error> {
    let (rules_str, updates_str) = input.split_once("\n\n").context("no double newline")?;
    let rules = RuleSets::new(rules_str)?;
    let updates = parse_updates(updates_str)?;

    Ok((rules, updates))
}

fn parse_updates(updates_str: &str) -> Result<Vec<Vec<usize>>, anyhow::Error> {
    let mut updates = Vec::new();

    for update_str in updates_str.lines() {
        let mut update = Vec::<usize>::new();
        for num in update_str.split(",") {
            update.push(num.parse()?);
        }
        updates.push(update);
    }

    Ok(updates)
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
    first: [u128; 100],
    second: [u128; 100],
}

impl RuleSets {
    fn new(rules_str: &str) -> Result<Self> {
        let mut rules_first = [0; 100];
        let mut rules_second = [0; 100];

        for rule in rules_str.lines() {
            let (first, second) = rule.split_once("|").context("no vertical bar")?;
            let first: usize = first.parse()?;
            let second: usize = second.parse()?;
            rules_first[first] |= 1 << second;
            rules_second[second] |= 1 << first;
        }

        Ok(Self {
            first: rules_first,
            second: rules_second,
        })
    }

    fn iter_first(&self, i: usize) -> BitIter {
        BitIter::new(self.first[i])
    }

    fn iter_second(&self, i: usize) -> BitIter {
        BitIter::new(self.second[i])
    }
}

struct BitIter {
    inner: u128,
}

impl BitIter {
    fn new(inner: u128) -> Self {
        Self { inner }
    }
}

impl Iterator for BitIter {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let place = self.inner.trailing_zeros();
        if place == 128 {
            return None;
        }
        self.inner &= !(1_u128 << place);

        Some(place as usize)
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
