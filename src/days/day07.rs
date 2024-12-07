use crate::{DayResult, IntoDayResult};
use anyhow::Result;
use nom::{bytes::complete::tag, combinator::map, sequence::tuple, IResult};

pub fn solve(input: &str) -> Result<DayResult> {
    let mut entries = parse_entries(input)?;
    let mut p1 = 0;
    let mut p2 = 0;
    for e in entries.iter_mut() {
        if e.can_be_solved_p1() {
            p1 += e.goal;
        }
        if e.can_be_solved_p2() {
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

impl Math {
    fn can_be_solved_p1(&mut self) -> bool {
        can_be_solved_p1(self.goal, &mut self.numbers)
    }
    fn can_be_solved_p2(&mut self) -> bool {
        can_be_solved_p2(self.goal, &mut self.numbers)
    }
}

fn can_be_solved_p1(goal: u128, numbers: &mut [u128]) -> bool {
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

    let added = a + b;
    let multiplied = a * b;

    rem[0] = added;
    let first = can_be_solved_p1(goal, rem);
    rem[0] = b;
    if first {
        return true;
    }

    rem[0] = multiplied;
    let second = can_be_solved_p1(goal, rem);
    rem[0] = b;
    if second {
        return true;
    }

    false
}

fn can_be_solved_p2(goal: u128, numbers: &mut [u128]) -> bool {
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

    rem[0] = a + b;
    let first = can_be_solved_p2(goal, rem);
    rem[0] = b;
    if first {
        return true;
    }

    rem[0] = a * b;
    let second = can_be_solved_p2(goal, rem);
    rem[0] = b;
    if second {
        return true;
    }

    rem[0] = concat(a, b);
    let second = can_be_solved_p2(goal, rem);
    rem[0] = b;
    if second {
        return true;
    }

    false
}

fn concat(a: u128, b: u128) -> u128 {
    let mut _b = b;
    let mut count = 0;
    while _b != 0 {
        _b /= 10;
        count += 1;
    }
    a * (10_u128.pow(count)) + b
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
