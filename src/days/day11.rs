use crate::{DayResult, IntoDayResult};
use anyhow::{Context, Result};
use fxhash::FxHashMap;

pub fn solve(input: &str) -> Result<DayResult> {
    let mut stones: FxHashMap<u64, u64> = FxHashMap::default();
    for stone in input
        .lines()
        .next()
        .context("there is a line")?
        .split(" ")
        .map(|n| n.parse())
    {
        let stone = stone?;
        *stones.entry(stone).or_default() += 1;
    }
    let mut new_stones = FxHashMap::default();

    blink(25, &mut stones, &mut new_stones);
    let p1: u64 = stones.values().sum();
    blink(50, &mut stones, &mut new_stones);
    let p2: u64 = stones.values().sum();

    (p1, p2).into_result()
}

fn blink(range: usize, stones: &mut FxHashMap<u64, u64>, new_stones: &mut FxHashMap<u64, u64>) {
    for _ in 0..range {
        for (stone, count) in stones.drain() {
            if stone == 0 {
                *new_stones.entry(1).or_default() += count;
            } else if let Some((a, b)) = split_in_half(stone) {
                *new_stones.entry(a).or_default() += count;
                *new_stones.entry(b).or_default() += count;
            } else {
                *new_stones.entry(stone * 2024).or_default() += count;
            }
        }
        std::mem::swap(stones, new_stones);
    }
}

fn split_in_half(n: u64) -> Option<(u64, u64)> {
    if n < 10 {
        return None;
    } else if n < 100 {
        return Some((n / 10, n % 10));
    } else if n < 1_000 {
        return None;
    } else if n < 10_000 {
        return Some((n / 100, n % 100));
    } else if n < 100_000 {
        return None;
    } else if n < 1_000_000 {
        return Some((n / 1_000, n % 1_000));
    } else if n < 10_000_000 {
        return None;
    } else if n < 100_000_000 {
        return Some((n / 10_000, n % 10_000));
    } else if n < 1_000_000_000 {
        return None;
    } else if n < 10_000_000_000 {
        return Some((n / 100_000, n % 100_000));
    } else if n < 100_000_000_000 {
        return None;
    } else if n < 1_000_000_000_000 {
        return Some((n / 1_000_000, n % 1_000_000));
    } else if n < 10_000_000_000_000 {
        return None;
    } else if n < 100_000_000_000_000 {
        return Some((n / 10_000_000, n % 10_000_000));
    } else if n < 1_000_000_000_000_000 {
        return None;
    } else if n < 10_000_000_000_000_000 {
        return Some((n / 100_000_000, n % 100_000_000));
    } else if n < 100_000_000_000_000_000 {
        return None;
    } else if n < 1_000_000_000_000_000_000 {
        return Some((n / 1_000_000_000, n % 1_000_000_000));
    }
    unreachable!("add more cases please")
}

#[cfg(test)]
mod tests {
    use crate::{days::day11::solve, IntoDayResult};

    #[test]
    fn works_for_example() {
        const INPUT: &str = include_str!("../../test_input/day11.txt");
        let solution = solve(INPUT).unwrap();
        assert_eq!((55_312, 65_601_038_650_482_u64).into_day_result(), solution);
    }

    #[test]
    fn works_for_input() {
        const INPUT: &str =
            include_str!(concat!(std::env!("AOC_CACHE"), "/2024_", "day11", ".txt"));
        let solution = solve(INPUT).unwrap();
        assert_eq!(
            (183_620, 220_377_651_399_268_u64).into_day_result(),
            solution
        );
    }
}
