use crate::{DayResult, IntoDayResult};
use anyhow::{bail, Result};

pub fn solve(input: &str) -> Result<DayResult> {
    let lines = Container::new(input)?;

    let p1 = solve_p1(&lines);
    let p2 = solve_p2(&lines);

    (p1, p2).into_result()
}

struct Container<'a> {
    source: &'a [u8],
    width: usize,
    height: usize,
}

impl Container<'_> {
    fn new(source: &str) -> Result<Container> {
        let source = source.as_bytes();
        let Some(width) = source.iter().position(|&b| b == b'\n') else {
            bail!("expected a newline")
        };

        let height = source.len() / (width + 1);

        Ok(Container {
            source,
            width,
            height,
        })
    }

    fn get(&self, x: usize, y: usize) -> Option<u8> {
        self.source.get(y * (self.width + 1) + x).cloned()
    }
}

fn solve_p1(lines: &Container) -> i32 {
    (0..lines.height)
        .flat_map(|y| {
            (0..lines.width).flat_map(move |x| {
                [
                    (1, 0),
                    (-1, 0),
                    (0, 1),
                    (0, -1),
                    (1, 1),
                    (1, -1),
                    (-1, 1),
                    (-1, -1),
                ]
                .into_iter()
                .map(move |(dx, dy)| check(lines, x, y, dx, dy, "XMAS") as i32)
            })
        })
        .sum()
}

fn solve_p2(lines: &Container) -> i32 {
    (0..lines.height)
        .flat_map(|y| {
            (0..lines.width).map(move |x| {
                let leading_mas =
                    || check(lines, x + 2, y + 2, -1, -1, "MAS") || check(lines, x, y, 1, 1, "MAS");
                let trailing_mas =
                    || check(lines, x + 2, y, -1, 1, "MAS") || check(lines, x, y + 2, 1, -1, "MAS");
                (leading_mas() && trailing_mas()) as i32
            })
        })
        .sum()
}

fn check(lines: &Container, x: usize, y: usize, dx: isize, dy: isize, s: &str) -> bool {
    s.as_bytes()
        .iter()
        .scan((true, x, y), |(cont, x, y), &b| {
            if !*cont {
                return Some(None);
            }
            let res = lines.get(*x, *y).map(|b2| b == b2);
            match x.checked_add_signed(dx) {
                Some(nx) => *x = nx,
                None => *cont = false,
            }
            match y.checked_add_signed(dy) {
                Some(ny) => *y = ny,
                None => *cont = false,
            }
            Some(res)
        })
        .all(|ok| ok.unwrap_or_default())
}

#[cfg(test)]
mod tests {
    use crate::{days::day04::solve, IntoDayResult};

    #[test]
    fn works_for_example() {
        const INPUT: &str = include_str!("../../test_input/day04.txt");
        let solution = solve(INPUT).unwrap();
        assert_eq!((18, 9).into_day_result(), solution);
    }

    #[test]
    fn works_for_input() {
        const INPUT: &str =
            include_str!(concat!(std::env!("AOC_CACHE"), "/2024_", "day04", ".txt"));
        let solution = solve(INPUT).unwrap();
        assert_eq!((2_401, 1_822).into_day_result(), solution);
    }
}
