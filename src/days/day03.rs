use nom::{
    branch::alt, bytes::complete::tag, character::complete::u64 as nom_u64, combinator::map,
    sequence::tuple, IResult,
};

use crate::{DayResult, IntoDayResult};

pub fn solve(mut input: &str) -> anyhow::Result<DayResult> {
    let mut p1 = 0;
    let mut p2 = 0;
    let mut enabled = true;

    while !input.is_empty() {
        let Ok((rem, parsed)) = parse_next(input) else {
            input = &input[1..];
            continue;
        };
        input = rem;
        match parsed {
            ParseResult::Do => enabled = true,
            ParseResult::Dont => enabled = false,
            ParseResult::Mul(a, b) => {
                let sum = a * b;
                p1 += sum;
                if enabled {
                    p2 += sum;
                }
            }
        }
    }

    (p1, p2).into_result()
}

enum ParseResult {
    Do,
    Dont,
    Mul(u64, u64),
}

fn parse_next(s: &str) -> IResult<&str, ParseResult> {
    alt((
        map(tag("do()"), |_| ParseResult::Do),
        map(tag("don't()"), |_| ParseResult::Dont),
        map(
            tuple((tag("mul("), nom_u64, tag(","), nom_u64, tag(")"))),
            |(_, a, _, b, _)| ParseResult::Mul(a, b),
        ),
    ))(s)
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
