use core::panic;
use std::ops::Add;

use crate::{DayResult, IntoDayResult};
use anyhow::Result;
use fxhash::FxHashSet;

pub fn solve(input: &str) -> Result<DayResult> {
    let game = Game::parse(input);
    let mut game_p1 = game.clone();
    let start_pos = game.position;
    loop {
        if !game_p1.step_p1() {
            break;
        }
    }
    let p1 = game_p1.visited.len();
    let mut p2 = 0;
    for visited in game_p1.visited {
        if visited == start_pos {
            continue;
        }
        let mut game_p2 = game.clone();
        game_p2.world[visited.y][visited.x] = '#';
        loop {
            match game_p2.step_p2() {
                Some(true) => {}
                Some(false) => break,
                None => {
                    p2 += 1;
                    break;
                }
            }
        }
    }

    (p1, p2).into_result()
}

#[derive(Debug, Clone)]
struct Game {
    world: Vec<Vec<char>>,
    position: Coord,
    curr_dir: DxDy,
    visited: FxHashSet<Coord>,
    visited_with_dir: FxHashSet<(Coord, DxDy)>,
}

impl Game {
    fn parse(s: &str) -> Self {
        let mut world = Vec::new();
        let mut position = Coord { x: 0, y: 0 };
        for (j, line) in s.lines().enumerate() {
            let mut row = Vec::new();
            for (i, mut c) in line.chars().enumerate() {
                if c == '^' {
                    position = Coord { x: i, y: j };
                    c = '.';
                }
                row.push(c);
            }
            world.push(row);
        }
        let curr_dir = DxDy { x: 0, y: -1 };
        let visited = FxHashSet::from_iter([position]);
        let visited_with_dir = FxHashSet::from_iter([(position, curr_dir)]);
        Self {
            world,
            position,
            curr_dir,
            visited,
            visited_with_dir,
        }
    }

    fn step_p1(&mut self) -> bool {
        let Some(mut new_pos) = self.position + self.curr_dir else {
            return false;
        };
        // check if we are oob and therefore done
        if new_pos.x >= self.world[0].len() {
            return false;
        }
        if new_pos.y >= self.world.len() {
            return false;
        }

        // check if wall and rotate if needed
        while self.world[new_pos.y][new_pos.x] == '#' {
            self.curr_dir = self.curr_dir.right();
            let Some(new_pos_2) = self.position + self.curr_dir else {
                return false;
            };
            new_pos = new_pos_2;

            // check if we are oob and therefore done
            if new_pos.x >= self.world[0].len() {
                return false;
            }
            if new_pos.y >= self.world.len() {
                return false;
            }
        }

        // apply move
        self.position = new_pos;
        self.visited.insert(self.position);
        true
    }

    fn step_p2(&mut self) -> Option<bool> {
        let Some(mut new_pos) = self.position + self.curr_dir else {
            return Some(false);
        };
        // check if we are oob and therefore done
        if new_pos.x >= self.world[0].len() {
            return Some(false);
        }
        if new_pos.y >= self.world.len() {
            return Some(false);
        }

        // check if wall and rotate if needed
        while self.world[new_pos.y][new_pos.x] == '#' {
            self.curr_dir = self.curr_dir.right();
            if !self.visited_with_dir.insert((self.position, self.curr_dir)) {
                return None;
            }

            let Some(new_pos_2) = self.position + self.curr_dir else {
                return Some(false);
            };
            // check if we are oob and therefore done
            if new_pos_2.x >= self.world[0].len() {
                return Some(false);
            }
            if new_pos_2.y >= self.world.len() {
                return Some(false);
            }

            new_pos = new_pos_2;
        }

        // apply move
        self.position = new_pos;
        if !self.visited_with_dir.insert((self.position, self.curr_dir)) {
            return None;
        }

        Some(true)
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Coord {
    x: usize,
    y: usize,
}

impl Add<DxDy> for Coord {
    type Output = Option<Coord>;

    fn add(self, rhs: DxDy) -> Self::Output {
        let Coord { x: x1, y: y1 } = self;
        let DxDy { x: x2, y: y2 } = rhs;
        let x = x1.checked_add_signed(x2)?;
        let y = y1.checked_add_signed(y2)?;
        Some(Self { x, y })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Hash)]
struct DxDy {
    x: isize,
    y: isize,
}

impl DxDy {
    fn right(self) -> Self {
        let DxDy { x, y } = self;
        DxDy { x: -y, y: x }
    }
}

#[cfg(test)]
mod tests {
    use crate::{days::day06::solve, IntoDayResult};

    #[ignore]
    #[test]
    fn works_for_example() {
        const INPUT: &str = include_str!("../../test_input/day06.txt");
        let solution = solve(INPUT).unwrap();
        assert_eq!(().into_day_result(), solution);
    }

    #[ignore]
    #[test]
    fn works_for_input() {
        const INPUT: &str =
            include_str!(concat!(std::env!("AOC_CACHE"), "/2024_", "day06", ".txt"));
        let solution = solve(INPUT).unwrap();
        assert_eq!(().into_day_result(), solution);
    }
}
