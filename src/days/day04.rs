use crate::{DayResult, IntoDayResult};
use anyhow::Result;

pub fn solve(input: &str) -> Result<DayResult> {
    let lines = input.lines().map(str::as_bytes).collect::<Vec<_>>();

    let p1 = solve_p1(&lines);
    let p2 = solve_p2(&lines);

    (p1, p2).into_result()
}

fn solve_p1(lines: &[&[u8]]) -> i32 {
    (0..lines[0].len())
        .map(|x| {
            (0..lines.len())
                .map(|y| {
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
                    .map(|(dx, dy)| check(lines, x, y, dx, dy, "XMAS") as i32)
                    .sum::<i32>()
                })
                .sum::<i32>()
        })
        .sum()
}

fn solve_p2(lines: &[&[u8]]) -> i32 {
    (0..lines[0].len())
        .map(|x| {
            (0..lines.len())
                .map(|y| {
                    let leading_mas = || {
                        check(lines, x + 2, y + 2, -1, -1, "MAS") || check(lines, x, y, 1, 1, "MAS")
                    };
                    let trailing_mas = || {
                        check(lines, x + 2, y, -1, 1, "MAS") || check(lines, x, y + 2, 1, -1, "MAS")
                    };

                    (leading_mas() && trailing_mas()) as i32
                })
                .sum::<i32>()
        })
        .sum()
}

fn check(lines: &[&[u8]], x: usize, y: usize, dx: isize, dy: isize, s: &str) -> bool {
    let x = x as isize;
    let y = y as isize;
    s.as_bytes()
        .iter()
        .scan((x, y), |(x, y), &b| {
            let res = lines
                .get(*y as usize)
                .and_then(|line| line.get(*x as usize))
                .map(|&b2| b == b2);
            *x += dx;
            *y += dy;
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
