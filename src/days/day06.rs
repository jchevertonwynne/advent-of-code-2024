use std::{ops::Add, vec};

use crate::{DayResult, IntoDayResult};
use anyhow::Result;

pub fn solve(input: &str) -> Result<DayResult> {
    let GameInfo {
        mut world,
        position,
    } = GameInfo::parse(input);
    let mut followed_path = vec![];
    let mut distances = distances(&world);

    let mut followed_path_dedup = Vec::new();
    let p1 = solve_p1(
        position,
        &distances,
        &mut followed_path,
        &mut followed_path_dedup,
    );
    let p2 = solve_p2(
        position,
        &mut distances,
        &mut followed_path,
        &followed_path_dedup,
        &mut world,
    );

    (p1, p2).into_result()
}

fn solve_p1(
    position: Coord,
    distances: &[Vec<Distances>],
    followed_path: &mut Vec<Coord>,
    followed_path_dedup: &mut Vec<Coord>,
) -> i32 {
    solve_p1_impl(position, distances, followed_path);
    let mut seen = distances
        .iter()
        .map(|l| l.iter().map(|_| false).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut p1 = 0;
    for c in followed_path {
        if !seen[c.y][c.x] {
            p1 += 1;
            followed_path_dedup.push(*c);
        }
        seen[c.y][c.x] = true;
    }

    p1
}

fn solve_p1_impl(mut position: Coord, distances: &[Vec<Distances>], visited: &mut Vec<Coord>) {
    visited.clear();
    visited.push(position);

    let mut curr_dir = DxDy { x: 0, y: -1 };
    loop {
        let mut dist = distances[position.y][position.x].distance(curr_dir.dir());
        while dist == 0 {
            curr_dir = curr_dir.right();
            dist = distances[position.y][position.x].distance(curr_dir.dir());
        }
        let jump = curr_dir * dist;
        let Some(new_position) = position + jump else {
            let mut c = position;
            loop {
                let Some(cn) = c + curr_dir else { return };
                visited.push(cn);
                c = cn;
            }
        };
        if new_position.x >= distances[0].len() {
            let mut c = position;
            loop {
                let cn = (c + curr_dir).unwrap();
                visited.push(cn);
                if cn == new_position {
                    visited.pop();
                    break;
                }
                c = cn;
            }
            return;
        }
        if new_position.y >= distances.len() {
            let mut c = position;
            loop {
                let cn = (c + curr_dir).unwrap();
                visited.push(cn);
                if cn == new_position {
                    visited.pop();
                    break;
                }
                c = cn;
            }
            return;
        }
        let mut c = position;
        loop {
            let cn = (c + curr_dir).unwrap();
            visited.push(cn);
            if cn == new_position {
                break;
            }
            c = cn;
        }
        position = new_position;
    }
}

fn solve_p2(
    position: Coord,
    distances: &mut [Vec<Distances>],
    followed_path: &mut Vec<Coord>,
    followed_path_dedup: &[Coord],
    world: &mut [Vec<bool>],
) -> usize {
    let mut seen_directional = distances
        .iter()
        .map(|d| {
            d.iter()
                .map(|_| DirectionalVisited {
                    up: false,
                    down: false,
                    left: false,
                    right: false,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut p2 = 0;
    for &v in followed_path_dedup {
        if v == position {
            continue;
        }

        for vis in followed_path.drain(..) {
            seen_directional[vis.y][vis.x] = DirectionalVisited {
                up: false,
                down: false,
                left: false,
                right: false,
            };
        }

        world[v.y][v.x] = true;
        reallocate_distances(world, v, distances);

        if solve_p2_solver(position, distances, followed_path, &mut seen_directional) {
            p2 += 1;
        }

        world[v.y][v.x] = false;
        reallocate_distances(world, v, distances);
    }

    p2
}

fn reallocate_distances(world: &[Vec<bool>], v: Coord, distances: &mut [Vec<Distances>]) {
    allocate_distances_hori(world, v.y, distances);
    allocate_distances_vert(world, v.x, distances);
}

fn allocate_distances_vert(world: &[Vec<bool>], x: usize, distances: &mut [Vec<Distances>]) {
    let mut d = 1;
    for j in 0..world.len() {
        if world[j][x] {
            distances[j][x].down = 0;
            d = 0
        } else {
            distances[j][x].down = d;
            d += 1;
        }
    }
    let mut d = 1;
    for j in (0..world.len()).rev() {
        if world[j][x] {
            distances[j][x].up = 0;
            d = 0
        } else {
            distances[j][x].up = d;
            d += 1;
        }
    }
}

fn allocate_distances_hori(world: &[Vec<bool>], y: usize, distances: &mut [Vec<Distances>]) {
    let mut d = 1;
    for (i, &wall) in world[y].iter().enumerate() {
        if wall {
            distances[y][i].left = 0;
            d = 0
        } else {
            distances[y][i].left = d;
            d += 1;
        }
    }
    let mut d = 1;
    for (i, &b) in world[y].iter().enumerate().rev() {
        if b {
            distances[y][i].right = 0;
            d = 0
        } else {
            distances[y][i].right = d;
            d += 1;
        }
    }
}

fn solve_p2_solver(
    mut position: Coord,
    distances: &[Vec<Distances>],
    followed_path: &mut Vec<Coord>,
    seen: &mut [Vec<DirectionalVisited>],
) -> bool {
    let mut curr_dir = DxDy { x: 0, y: -1 };
    followed_path.clear();
    followed_path.push(position);
    *seen[position.y][position.x].seen(curr_dir.dir()) = true;

    loop {
        let mut dist = distances[position.y][position.x].distance(curr_dir.dir());
        while dist == 0 {
            curr_dir = curr_dir.right();
            dist = distances[position.y][position.x].distance(curr_dir.dir());
        }

        let jump = curr_dir * dist;
        let Some(new_position) = position + jump else {
            return false;
        };
        if new_position.x >= distances[0].len() {
            return false;
        }
        if new_position.y >= distances.len() {
            return false;
        }

        let dir = curr_dir.dir();
        position = new_position;
        followed_path.push(position);
        if std::mem::replace(seen[position.y][position.x].seen(dir), true) {
            return true;
        }
    }
}

fn distances(world: &[Vec<bool>]) -> Vec<Vec<Distances>> {
    let mut res = vec![vec![Distances::default(); world[0].len()]; world.len()];

    for j in 0..world.len() {
        allocate_distances_hori(world, j, &mut res);
    }
    for i in 0..world[0].len() {
        allocate_distances_vert(world, i, &mut res);
    }

    res
}

struct GameInfo {
    world: Vec<Vec<bool>>,
    position: Coord,
}

impl GameInfo {
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
                row.push(c == '#');
            }
            world.push(row);
        }

        Self { world, position }
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

#[derive(Debug, Default, Clone, Copy)]
struct Distances {
    up: usize,
    down: usize,
    left: usize,
    right: usize,
}

impl Distances {
    fn distance(&self, dir: Dirs) -> usize {
        match dir {
            Dirs::Up => self.up,
            Dirs::Down => self.down,
            Dirs::Left => self.left,
            Dirs::Right => self.right,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct DirectionalVisited {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

impl DirectionalVisited {
    fn seen(&mut self, dir: Dirs) -> &mut bool {
        match dir {
            Dirs::Up => &mut self.up,
            Dirs::Down => &mut self.down,
            Dirs::Left => &mut self.left,
            Dirs::Right => &mut self.right,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Hash)]
struct DxDy {
    x: isize,
    y: isize,
}

impl std::ops::Mul<usize> for DxDy {
    type Output = DxDy;

    fn mul(self, rhs: usize) -> Self::Output {
        let rhs = rhs as isize;
        let Self { x, y } = self;
        Self {
            x: x * rhs,
            y: y * rhs,
        }
    }
}

impl DxDy {
    fn right(self) -> Self {
        let DxDy { x, y } = self;
        DxDy { x: -y, y: x }
    }

    fn dir(self) -> Dirs {
        match self {
            DxDy { x: 0, y: 1 } => Dirs::Up,
            DxDy { x: 0, y: -1 } => Dirs::Down,
            DxDy { x: 1, y: 0 } => Dirs::Right,
            DxDy { x: -1, y: 0 } => Dirs::Left,
            _ => unreachable!("please dont"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Dirs {
    Up,
    Down,
    Left,
    Right,
}

#[cfg(test)]
mod tests {
    use crate::{days::day06::solve, IntoDayResult};

    #[test]
    fn works_for_example() {
        const INPUT: &str = include_str!("../../test_input/day06.txt");
        let solution = solve(INPUT).unwrap();
        assert_eq!((41, 6).into_day_result(), solution);
    }

    #[test]
    fn works_for_input() {
        const INPUT: &str =
            include_str!(concat!(std::env!("AOC_CACHE"), "/2024_", "day06", ".txt"));
        let solution = solve(INPUT).unwrap();
        assert_eq!((5_516, 2_008).into_day_result(), solution);
    }
}
