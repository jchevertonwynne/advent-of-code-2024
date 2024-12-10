use crate::{DayResult, IntoDayResult};
use anyhow::Result;
use fxhash::FxHashSet;

pub fn solve(input: &str) -> Result<DayResult> {
    let (world, starts) = parse_world(input);

    let world: &[Vec<u8>] = &world;
    let starts: &[Coord] = &starts;
    let mut p1 = 0;
    let mut p2 = 0;
    let mut reachable = FxHashSet::default();
    for &start in starts {
        reachable.clear();
        p2 += recurse(start, &mut reachable, world);
        p1 += reachable.len();
    }

    (p1, p2).into_result()
}

fn recurse(pos: Coord, reachable: &mut FxHashSet<Coord>, world: &[Vec<u8>]) -> usize {
    let score = world[pos.y as usize][pos.x as usize];
    if score == 9 {
        reachable.insert(pos);
        return 1;
    }

    let mut res = 0;
    for dir in [
        Coord { x: 1, y: 0 },
        Coord { x: -1, y: 0 },
        Coord { x: 0, y: 1 },
        Coord { x: 0, y: -1 },
    ] {
        let new_pos = pos + dir;
        if is_out_of_bounds(new_pos, world) {
            continue;
        }
        let new_score = world[new_pos.y as usize][new_pos.x as usize];
        if new_score > score && new_score - score == 1 {
            res += recurse(new_pos, reachable, world);
        }
    }

    res
}

fn is_out_of_bounds(new: Coord, world: &[Vec<u8>]) -> bool {
    if new.x < 0 || new.x >= world[0].len() as isize {
        return true;
    }
    if new.y < 0 || new.y >= world.len() as isize {
        return true;
    }
    false
}

fn parse_world(s: &str) -> (Vec<Vec<u8>>, Vec<Coord>) {
    let mut starts: Vec<Coord> = vec![];
    let mut world: Vec<Vec<u8>> = vec![];

    for (y, line) in s.lines().enumerate() {
        let mut score_line = Vec::with_capacity(world.first().map(|f| f.len()).unwrap_or(0));
        for (x, b) in line.bytes().enumerate() {
            let val = if b == b'.' { 255 } else { b - b'0' };
            score_line.push(val);
            if val == 0 {
                starts.push(Coord {
                    x: x as isize,
                    y: y as isize,
                });
            }
        }
        world.push(score_line);
    }
    (world, starts)
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Coord {
    x: isize,
    y: isize,
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
    use crate::{days::day10::solve, IntoDayResult};

    #[test]
    fn works_for_example() {
        const INPUT: &str = include_str!("../../test_input/day10.txt");
        let solution = solve(INPUT).unwrap();
        assert_eq!((36, 81).into_day_result(), solution);
    }

    #[test]
    fn works_for_input() {
        const INPUT: &str =
            include_str!(concat!(std::env!("AOC_CACHE"), "/2024_", "day10", ".txt"));
        let solution = solve(INPUT).unwrap();
        assert_eq!((754, 1_609).into_day_result(), solution);
    }
}
