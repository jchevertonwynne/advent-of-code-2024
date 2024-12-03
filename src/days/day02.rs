use anyhow::Result;
use itertools::Itertools;

use crate::{DayResult, IntoDayResult};

pub fn solve(mut input: &str) -> Result<DayResult> {
    let mut report = Vec::new();
    let mut p1 = 0;
    let mut p2 = 0;

    while let Some(remaining_input) = parse_next_report(input, &mut report)? {
        input = remaining_input;
        match is_safe(report.iter().cloned()) {
            Ok(()) => {
                p1 += 1;
                p2 += 1;
            }
            Err(indices) => match indices {
                FailOptions::Single(index) => {
                    for i in (index - 1)..=(std::cmp::min(index + 1, report.len())) {
                        let copy = SkippingIterator::new(report.iter().cloned(), i);
                        if is_safe(copy).is_ok() {
                            p2 += 1;
                            break;
                        }
                    }
                }
                FailOptions::Multi(a, b) => {
                    'outer: for index in [a, b] {
                        for i in (index - 1)..=(std::cmp::min(index + 1, report.len())) {
                            let copy = SkippingIterator::new(report.iter().cloned(), i);
                            if is_safe(copy).is_ok() {
                                p2 += 1;
                                break 'outer;
                            }
                        }
                    }
                }
            },
        }
    }

    (p1, p2).into_result()
}

fn parse_next_report<'a>(s: &'a str, into: &mut Vec<usize>) -> Result<Option<&'a str>> {
    let Some(newline) = s.find('\n') else {
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

#[derive(Debug)]
enum FailOptions {
    Single(usize),
    Multi(usize, usize),
}

fn is_safe<I: Iterator<Item = usize> + Clone>(report: I) -> Result<(), FailOptions> {
    let mut fail_asc = None;
    let mut fail_desc = None;
    let mut fail_close = None;
    for ((_, a), (j, b)) in report.clone().enumerate().tuple_windows() {
        if a >= b {
            fail_asc = Some(j);
        }
        if a <= b {
            fail_desc = Some(j);
        }
        if !(1..=3).contains(&a.abs_diff(b)) {
            fail_close = Some(j);
        }
    }

    if let Some(fail_close) = fail_close {
        return Err(FailOptions::Single(fail_close));
    }

    match (fail_asc, fail_desc) {
        (None, None) | (None, Some(_)) | (Some(_), None) => Ok(()),
        (Some(fail_asc), Some(fail_desc)) => Err(FailOptions::Multi(fail_asc, fail_desc)),
    }
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
