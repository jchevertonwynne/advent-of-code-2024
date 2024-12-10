use crate::{DayResult, IntoDayResult};
use anyhow::Result;

pub fn solve(input: &str) -> Result<DayResult> {
    let (mut p1_files, p2_files, mut p2_spaces) = parse(input);
    let mut forward = 0;
    while forward < p1_files.len() {
        if p1_files[forward].is_none() {
            let mut last = p1_files.pop().unwrap();
            while last.is_none() {
                last = p1_files.pop().unwrap();
            }
            if forward < p1_files.len() {
                p1_files[forward] = last;
            } else {
                p1_files.push(last);
            }
        }
        forward += 1;
    }
    let p1: u128 = p1_files
        .iter()
        .enumerate()
        .map(|(i, v)| i as u128 * v.expect("there is no spare space"))
        .sum();

    let mut p2_final = Vec::new();
    for mut file in p2_files.into_iter().rev() {
        if let Some((space_index, space)) = p2_spaces
            .iter()
            .cloned()
            .take_while(|space| space.starts_at < file.starts_at)
            .enumerate()
            .find(|(_, space)| space.size >= file.size)
        {
            p2_spaces.push(Space {
                starts_at: file.starts_at,
                size: file.size,
            });
            p2_spaces.remove(space_index);
            let new_space = Space {
                starts_at: space.starts_at + file.size,
                size: space.size - file.size,
            };
            if new_space.size > 0 {
                p2_spaces.push(new_space);
                p2_spaces.sort_unstable_by_key(|f| f.starts_at);
                let mut i = 0;
                while i < p2_spaces.len() - 2 {
                    if p2_spaces[i + 1].starts_at - p2_spaces[i].starts_at == p2_spaces[i].size {
                        let a = p2_spaces.remove(i);
                        let b = p2_spaces[i];
                        let c = Space {
                            starts_at: a.starts_at,
                            size: a.size + b.size,
                        };
                        p2_spaces[i] = c;
                    } else {
                        i += 1;
                    }
                }
            }
            file.starts_at = space.starts_at;
        }
        p2_final.push(file);
    }
    let p2: u128 = p2_final
        .iter()
        .flat_map(|v| (v.starts_at..v.starts_at + v.size).map(|i| i as u128 * v.id))
        .sum();

    (p1, p2).into_result()
}

fn parse(s: &str) -> (Vec<Option<u128>>, Vec<FileItem>, Vec<Space>) {
    let mut full_system = Vec::new();
    let mut blocks = Vec::new();
    let mut spaces = Vec::new();
    let mut id = 0;
    let mut is_file = true;

    for b in s.bytes() {
        if b == b'\n' {
            continue;
        }
        let n = (b - b'0') as usize;
        if is_file {
            blocks.push(FileItem {
                id,
                starts_at: full_system.len(),
                size: n,
            });
            full_system.reserve(n);
            for _ in 0..n {
                full_system.push(Some(id));
            }
        } else {
            id += 1;
            if n != 0 {
                spaces.push(Space {
                    starts_at: full_system.len(),
                    size: n,
                });
                full_system.reserve(n);
                for _ in 0..n {
                    full_system.push(None);
                }
            }
        }
        is_file = !is_file;
    }

    (full_system, blocks, spaces)
}

#[derive(Debug, Clone, Copy)]
struct FileItem {
    id: u128,
    starts_at: usize,
    size: usize,
}

#[derive(Debug, Clone, Copy)]
struct Space {
    starts_at: usize,
    size: usize,
}

#[cfg(test)]
mod tests {
    use crate::{days::day09::solve, IntoDayResult};

    #[test]
    fn works_for_example() {
        const INPUT: &str = include_str!("../../test_input/day09.txt");
        let solution = solve(INPUT).unwrap();
        assert_eq!((1_928, 2_858).into_day_result(), solution);
    }

    #[test]
    fn works_for_input() {
        const INPUT: &str =
            include_str!(concat!(std::env!("AOC_CACHE"), "/2024_", "day09", ".txt"));
        let solution = solve(INPUT).unwrap();
        assert_eq!(
            (6_607_511_583_593_u128, 6_636_608_781_232_u128).into_day_result(),
            solution
        );
    }
}
