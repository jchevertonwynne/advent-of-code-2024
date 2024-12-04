use crate::{DayResult, IntoDayResult};
use anyhow::Result;

pub fn solve(input: &str) -> Result<DayResult> {
    let lines = input.lines().map(str::as_bytes).collect::<Vec<_>>();

    let p1 = solve_p1(&lines);
    let p2 = solve_p2(lines);

    (p1, p2).into_result()
}

fn solve_p1(lines: &[&[u8]]) -> i32 {
    let mut hori_r = 0;
    let mut hori_l = 0;
    for line in lines {
        for x in 0..lines[0].len() - 3 {
            let mut success = true;
            for (i, &b) in b"XMAS".iter().enumerate() {
                if line[x + i] != b {
                    success = false;
                    break;
                }
            }
            if success {
                hori_r += 1;
            }
            let mut success = true;
            for (i, &b) in b"XMAS".iter().enumerate() {
                if line[x + 3 - i] != b {
                    success = false;
                    break;
                }
            }
            if success {
                hori_l += 1;
            }
        }
    }

    let mut vert_up = 0;
    let mut vert_down = 0;
    for x in 0..lines[0].len() {
        for y in 0..lines.len() - 3 {
            let mut success = true;
            for (i, &b) in b"XMAS".iter().enumerate() {
                if lines[y + i][x] != b {
                    success = false;
                    break;
                }
            }
            if success {
                vert_up += 1;
            }
            let mut success = true;
            for (i, &b) in b"XMAS".iter().enumerate() {
                if lines[y + 3 - i][x] != b {
                    success = false;
                    break;
                }
            }
            if success {
                vert_down += 1;
            }
        }
    }

    let mut diag_ur = 0;
    let mut diag_ul = 0;
    let mut diag_dr = 0;
    let mut diag_dl = 0;

    for x in 0..lines[0].len() - 3 {
        for y in 0..lines.len() - 3 {
            let mut success = true;
            for (i, &b) in b"XMAS".iter().enumerate() {
                if lines[y + i][x + i] != b {
                    success = false;
                    break;
                }
            }
            if success {
                diag_dr += 1;
            }
            let mut success = true;
            for (i, &b) in b"XMAS".iter().enumerate() {
                if lines[y + i][x + 3 - i] != b {
                    success = false;
                    break;
                }
            }
            if success {
                diag_dl += 1;
            }
            let mut success = true;
            for (i, &b) in b"XMAS".iter().enumerate() {
                if lines[y + 3 - i][x + i] != b {
                    success = false;
                    break;
                }
            }
            if success {
                diag_ur += 1;
            }
            let mut success = true;
            for (i, &b) in b"XMAS".iter().enumerate() {
                if lines[y + 3 - i][x + 3 - i] != b {
                    success = false;
                    break;
                }
            }
            if success {
                diag_ul += 1;
            }
        }
    }

    vert_up + vert_down + hori_r + hori_l + diag_ur + diag_ul + diag_dr + diag_dl
}

fn solve_p2(lines: Vec<&[u8]>) -> i32 {
    let mut p2 = 0;
    for x in 0..lines[0].len() - 2 {
        for y in 0..lines.len() - 2 {
            let mut success = true;
            for (i, &b) in b"MAS".iter().enumerate() {
                if lines[y + i][x + i] != b {
                    success = false;
                    break;
                }
            }
            if success {
                let mut success = true;
                for (i, &b) in b"MAS".iter().enumerate() {
                    if lines[y + i][x + 2 - i] != b {
                        success = false;
                        break;
                    }
                }
                if success {
                    p2 += 1;
                }
                let mut success = true;
                for (i, &b) in b"MAS".iter().enumerate() {
                    if lines[y + 2 - i][x + i] != b {
                        success = false;
                        break;
                    }
                }
                if success {
                    p2 += 1;
                }
            }
            let mut success = true;
            for (i, &b) in b"MAS".iter().enumerate() {
                if lines[y + 2 - i][x + 2 - i] != b {
                    success = false;
                    break;
                }
            }
            if success {
                let mut success = true;
                for (i, &b) in b"MAS".iter().enumerate() {
                    if lines[y + i][x + 2 - i] != b {
                        success = false;
                        break;
                    }
                }
                if success {
                    p2 += 1;
                }
                let mut success = true;
                for (i, &b) in b"MAS".iter().enumerate() {
                    if lines[y + 2 - i][x + i] != b {
                        success = false;
                        break;
                    }
                }
                if success {
                    p2 += 1;
                }
            }
        }
    }

    p2
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
