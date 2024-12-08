use crate::{DayResult, IntoDayResult};
use anyhow::Result;
use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;
use num::integer::gcd;

pub fn solve(input: &str) -> Result<DayResult> {
    let (sensor_types, x, y) = parse(input)?;

    let mut locs = FxHashSet::default();
    for sensors in sensor_types.values() {
        for ab in sensors.iter().combinations(2) {
            let a = *ab[0];
            let b = *ab[1];
            let diff = a - b;

            let in_bounds = |c: Coord| c.x >= 0 && c.x < x && c.y >= 0 && c.y < y;

            let an_1 = a + diff;
            if in_bounds(an_1) {
                locs.insert(an_1);
            }

            let an_2 = b - diff;
            if in_bounds(an_2) {
                locs.insert(an_2);
            }
        }
    }
    let p1 = locs.len();

    locs.clear();
    for sensors in sensor_types.values() {
        for ab in sensors.iter().combinations(2) {
            let a = *ab[0];
            let b = *ab[1];
            let diff = a - b;

            let lcm = gcd(diff.x, diff.y);
            let diff = Coord {
                x: diff.x / lcm,
                y: diff.y / lcm,
            };

            let in_bounds = |c: Coord| c.x >= 0 && c.x < x && c.y >= 0 && c.y < y;

            let mut a_anti = b + diff;
            while in_bounds(a_anti) {
                locs.insert(a_anti);
                a_anti = a_anti + diff;
            }

            let mut b_anti = a - diff;
            while in_bounds(b_anti) {
                locs.insert(b_anti);
                b_anti = b_anti - diff;
            }
        }
    }
    let p2 = locs.len();

    (p1, p2).into_result()
}

fn parse(s: &str) -> Result<(FxHashMap<char, Vec<Coord>>, isize, isize)> {
    let mut rx = 0;
    let mut ry = 0;
    let mut coords = FxHashMap::<char, Vec<Coord>>::default();

    for (y, line) in s.lines().enumerate() {
        ry = y;
        for (x, c) in line.chars().enumerate() {
            rx = x;
            if c == '.' {
                continue;
            }
            let coord = Coord {
                x: x as isize,
                y: y as isize,
            };
            coords.entry(c).or_default().push(coord);
        }
    }

    Ok((coords, rx as isize + 1, ry as isize + 1))
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Coord {
    x: isize,
    y: isize,
}

impl std::ops::Sub for Coord {
    type Output = Coord;

    fn sub(self, rhs: Self) -> Self::Output {
        let Coord { x: x1, y: y1 } = self;
        let Coord { x: x2, y: y2 } = rhs;
        Coord {
            x: x1 - x2,
            y: y1 - y2,
        }
    }
}

impl std::ops::Add for Coord {
    type Output = Coord;

    fn add(self, rhs: Self) -> Self::Output {
        let Coord { x: x1, y: y1 } = self;
        let Coord { x: x2, y: y2 } = rhs;
        Coord {
            x: x1 + x2,
            y: y1 + y2,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{days::day08::solve, IntoDayResult};

    #[test]
    fn works_for_example() {
        const INPUT: &str = include_str!("../../test_input/day08.txt");
        let solution = solve(INPUT).unwrap();
        assert_eq!((14, 34).into_day_result(), solution);
    }

    #[test]
    fn works_for_input() {
        const INPUT: &str =
            include_str!(concat!(std::env!("AOC_CACHE"), "/2024_", "day08", ".txt"));
        let solution = solve(INPUT).unwrap();
        assert_eq!((301, 1_019).into_day_result(), solution);
    }
}
