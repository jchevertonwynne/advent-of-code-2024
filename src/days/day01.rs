use crate::{DayResult, IntoDayResult};

pub fn solve(_input: &str) -> anyhow::Result<DayResult> {
    ().into_result()
}

#[cfg(test)]
mod tests {
    use crate::{days::day01::solve, IntoDayResult};

    #[ignore]
    #[test]
    fn works_for_example() {
        const INPUT: &str = include_str!("../../test_input/day01.txt");
        let solution = solve(INPUT).unwrap();
        assert_eq!(
            ().into_day_result(),
            solution
        );
    }

    #[ignore]
    #[test]
    fn works_for_input() {
        const INPUT: &str = include_str!(concat!(std::env!("AOC_CACHE"), "/2024_", "day01", ".txt"));
        let solution = solve(INPUT).unwrap();
        assert_eq!(
            ().into_day_result(),
            solution
        );
    }
}