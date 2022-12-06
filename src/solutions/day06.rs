
use crate::utils::solver_types::{solve_linear, SolutionLinear};
use anyhow::{anyhow, Result};
use itertools::Itertools;

pub struct Day6Solution {}

pub fn day06(input: &str) -> Result<f32> {
    solve_linear::<Day6Solution, _, _, _>(input)
}

impl SolutionLinear<Vec<char>, i32, i32> for Day6Solution {
    fn load(input: &str) -> Result<Vec<char>> {
        todo!()
    }

    fn part1(input: &mut Vec<char>) -> Result<i32> {
        todo!()
    }

    fn part2(input: &mut Vec<char>, part_1_solution: i32) -> Result<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::Day6Solution;
    use crate::utils::solver_types::SolutionLinear;
    use rstest::rstest;

    #[rstest]
    #[case("", 0, 0)]
    fn validate_linear(#[case] input: &str, #[case] expected_1: i32, #[case] expected_2: i32) {
        let mut input = Day6Solution::load(input).unwrap();
        let p1 = Day6Solution::part1(&mut input).unwrap();
        let p2 = Day6Solution::part2(&mut input, p1).unwrap();

        assert_eq!(expected_1, p1);
        assert_eq!(expected_2, p2);
    }
}
