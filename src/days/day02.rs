use itertools::Itertools;
use nom::FindSubstring;

use crate::{DayResult, IntoDayResult};

pub fn solve(mut input: &str) -> anyhow::Result<DayResult> {
    let mut report = Vec::new();
    let mut p1 = 0;
    let mut p2 = 0;
    while let Some(remaining_input) = parse_next_report(input, &mut report)? {
        input = remaining_input;
        if is_safe(report.iter().cloned()) {
            p1 += 1;
            p2 += 1;
        } else {
            for i in 0..report.len() {
                let copy = SkippingIterator::new(report.iter().cloned(), i);
                if is_safe(copy) {
                    p2 += 1;
                    break;
                }
            }
        }
    }
    (p1, p2).into_result()
}

fn parse_next_report<'a>(s: &'a str, into: &mut Vec<usize>) -> anyhow::Result<Option<&'a str>> {
    let Some(newline) = s.find_substring("\n") else {
        return Ok(None);
    };

    into.clear();

    for num_str in s[..newline].split_whitespace() {
        into.push(num_str.parse()?);
    }

    Ok(Some(&s[newline + 1..]))
}

#[derive(Clone)]
struct SkippingIterator<I> {
    iterated: usize,
    to_skip: usize,
    iter: I,
}

impl<I> SkippingIterator<I> {
    fn new(iter: I, to_skip: usize) -> Self {
        Self {
            iterated: 0,
            to_skip,
            iter,
        }
    }
}

impl<I> Iterator for SkippingIterator<I>
where
    I: Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next = self.iter.next();
        if self.iterated == self.to_skip {
            next = self.iter.next();
            self.iterated += 1;
        }
        self.iterated += 1;
        next
    }
}

fn is_safe<I: Iterator<Item = usize> + Clone>(report: I) -> bool {
    let all_increasing = {
        let report = report.clone();
        move || report.clone().tuple_windows().all(|(a, b)| a < b)
    };
    let all_decreasing = {
        let report = report.clone();
        move || report.clone().tuple_windows().all(|(a, b)| a > b)
    };
    let all_close = || {
        report.tuple_windows().all(|(a, b)| {
            let diff = a.abs_diff(b);
            (1..=3).contains(&diff)
        })
    };
    (all_increasing() || all_decreasing()) && all_close()
}

#[cfg(test)]
mod tests {
    use crate::{days::day02::solve, IntoDayResult};

    #[test]
    fn works_for_example() {
        const INPUT: &str = include_str!("../../test_input/day02.txt");
        let solution = solve(INPUT).unwrap();
        assert_eq!((2, 4).into_day_result(), solution);
    }

    #[test]
    fn works_for_input() {
        const INPUT: &str =
            include_str!(concat!(std::env!("AOC_CACHE"), "/2024_", "day02", ".txt"));
        let solution = solve(INPUT).unwrap();
        assert_eq!((257, 328).into_day_result(), solution);
    }
}
