use std::ops::{Add, Mul};

use crate::{DayResult, IntoDayResult};
use anyhow::Result;
use nom::{bytes::complete::tag, combinator::map, sequence::tuple, IResult};

pub fn solve(input: &str) -> Result<DayResult> {
    let mut entries = parse_entries(input)?;
    let mut p1 = 0;
    let mut p2 = 0;
    for e in entries.iter_mut() {
        if can_be_solved(e.goal, &mut e.numbers, (Add::add, Mul::mul)) {
            p1 += e.goal;
        }
        if can_be_solved(e.goal, &mut e.numbers, (Add::add, Mul::mul, concat)) {
            p2 += e.goal;
        }
    }
    (p1, p2).into_result()
}

fn parse_entries(s: &str) -> Result<Vec<Math>> {
    s.lines()
        .map(|line| {
            parse_line(line)
                .map(|(_, m)| m)
                .map_err(|err| anyhow::anyhow!("{err}"))
        })
        .collect()
}

fn parse_line(s: &str) -> IResult<&str, Math> {
    map(
        tuple((
            nom::character::complete::u128,
            tag(": "),
            nom::multi::separated_list1(tag(" "), nom::character::complete::u128),
        )),
        |(goal, _, numbers)| Math { goal, numbers },
    )(s)
}

#[derive(Debug)]
struct Math {
    goal: u128,
    numbers: Vec<u128>,
}

fn can_be_solved(goal: u128, numbers: &mut [u128], applicable: impl Applicable) -> bool {
    let [a, rem @ ..] = numbers else {
        unreachable!("should not be 0 len");
    };
    let a = *a;
    let Some(b) = rem.first().cloned() else {
        return a == goal;
    };

    if a > goal {
        return false;
    }

    applicable.apply(goal, rem, a, b)
}

fn concat(a: u128, b: u128) -> u128 {
    if b < 10 {
        return a * 10 + b;
    } else if b < 100 {
        return a * 100 + b;
    } else if b < 1000 {
        return a * 1000 + b;
    }
    unreachable!("lmao")
}

trait Applicable {
    fn apply(self, goal: u128, numbers: &mut [u128], a: u128, b: u128) -> bool;
}

macro_rules! applicable_logic_impl {
    ($self:tt, $goal:tt, $numbers:tt, $a:tt, $b:tt, $f:tt) => {
        $numbers[0] = $f($a, $b);
        let first = can_be_solved($goal, $numbers, $self);
        $numbers[0] = $b;
        if first {
            return true;
        }
    };
}

impl<F1, F2> Applicable for (F1, F2)
where
    F1: Fn(u128, u128) -> u128 + Copy,
    F2: Fn(u128, u128) -> u128 + Copy,
{
    fn apply(self, goal: u128, numbers: &mut [u128], a: u128, b: u128) -> bool {
        let (f1, f2) = self;
        applicable_logic_impl!(self, goal, numbers, a, b, f1);
        applicable_logic_impl!(self, goal, numbers, a, b, f2);
        false
    }
}

impl<F1, F2, F3> Applicable for (F1, F2, F3)
where
    F1: Fn(u128, u128) -> u128 + Copy,
    F2: Fn(u128, u128) -> u128 + Copy,
    F3: Fn(u128, u128) -> u128 + Copy,
{
    fn apply(self, goal: u128, numbers: &mut [u128], a: u128, b: u128) -> bool {
        let (f1, f2, f3) = self;
        applicable_logic_impl!(self, goal, numbers, a, b, f1);
        applicable_logic_impl!(self, goal, numbers, a, b, f2);
        applicable_logic_impl!(self, goal, numbers, a, b, f3);
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::{days::day07::solve, IntoDayResult};

    #[test]
    fn works_for_example() {
        const INPUT: &str = include_str!("../../test_input/day07.txt");
        let solution = solve(INPUT).unwrap();
        assert_eq!((3_749, 11_387).into_day_result(), solution);
    }

    #[test]
    fn works_for_input() {
        const INPUT: &str =
            include_str!(concat!(std::env!("AOC_CACHE"), "/2024_", "day07", ".txt"));
        let solution = solve(INPUT).unwrap();
        assert_eq!(
            (1_620_690_235_709_u128, 145_397_611_075_341_u128).into_day_result(),
            solution
        );
    }
}
