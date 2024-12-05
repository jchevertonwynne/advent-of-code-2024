use crate::{DayResult, IntoDayResult};
use anyhow::{Context, Result};
use fxhash::FxHashMap;

pub fn solve(_input: &str) -> Result<DayResult> {
    let (rules_str, updates_str) = _input.split_once("\n\n").context("no double newline")?;
    let mut rules_first = std::array::from_fn::<_, 100, _>(|_| Vec::new());
    let mut rules_second = std::array::from_fn::<_, 100, _>(|_| Vec::new());
    for rule in rules_str.lines() {
        let (first, second) = rule.split_once("|").context("no vertical bar")?;
        let first = first.parse()?;
        let second = second.parse()?;
        let rule = Rule { first, second };
        rules_first[first].push(rule);
        rules_second[second].push(rule);
    }

    let mut updates = Vec::new();
    for update in updates_str.lines() {
        let mut u = Vec::<usize>::new();
        for num in update.split(",") {
            u.push(num.parse()?);
        }
        updates.push(u);
    }

    let mut to_left = FxHashMap::default();
    let mut ordered = Vec::new();
    let mut p1 = 0;
    let mut p2 = 0;
    for update in updates.iter() {
        let mut contains = [None; 100];
        for (i, u) in update.iter().enumerate() {
            contains[*u] = Some(i);
        }
        if is_success(&rules_first, &rules_second, update, &contains) {
            p1 += update[update.len() / 2];
        } else {
            ordered.clear();
            for &u in update {
                let mut count = 0;
                for r in &rules_second[u] {
                    if contains[r.first].is_some() {
                        count += 1;
                    }
                }
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

fn is_success(
    rules_first: &[Vec<Rule>],
    rules_second: &[Vec<Rule>],
    update: &[usize],
    contains: &[Option<usize>; 100],
) -> bool {
    for (i, &u) in update.iter().enumerate() {
        for r in &rules_second[u] {
            if contains[r.first].map(|n| n > i).unwrap_or(false) {
                return false;
            }
        }
        for r in &rules_first[u] {
            if contains[r.second].map(|n| n < i).unwrap_or(false) {
                return false;
            }
        }
    }

    true
}

struct CurrIter<'a, T> {
    index: usize,
    src: &'a [T],
}

impl<'a, T> Iterator for CurrIter<'a, T> {
    type Item = (&'a [T], &'a T, &'a [T]);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.src.len() {
            return None;
        }
        let before = &self.src[0..self.index];
        let curr = &self.src[self.index];
        let after = &self.src[self.index + 1..];
        self.index += 1;
        Some((before, curr, after))
    }
}

#[derive(Debug, Clone, Copy)]
struct Rule {
    first: usize,
    second: usize,
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
