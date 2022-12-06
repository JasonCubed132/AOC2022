
use std::collections::HashSet;

use crate::utils::solver_types::{solve_linear, SolutionLinear};
use anyhow::{anyhow, Result};
use itertools::Itertools;

pub struct Day6Solution {}

pub fn day06(input: &str) -> Result<f32> {
    solve_linear::<Day6Solution, _, _, _>(input)
}

impl SolutionLinear<Vec<char>, i32, i32> for Day6Solution {
    fn load(input: &str) -> Result<Vec<char>> {
        let lines = input.lines().collect_vec();
        if lines.len() != 1 {
            return Err(anyhow!("Expected 1 line"));
        }

        let output = lines[0].chars().collect_vec();
        if output.len() < 4 {
            return Err(anyhow!("Not enough elements"));
        }

        Ok(output)
    }

    fn part1(input: &mut Vec<char>) -> Result<i32> {
        let mut set: HashSet<char> = HashSet::new();

        let mut start_index = 0;
        for i in 0..3 {
            set.insert(input[i]);
        }
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
    #[case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7, 0)]
    #[case("bvwbjplbgvbhsrlpgdmjqwftvncz", 5, 0)]
    #[case("nppdvjthqldpwncqszvftbrmjlhg", 6, 0)]
    #[case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10, 0)]
    #[case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11, 0)]
    fn validate_linear(#[case] input: &str, #[case] expected_1: i32, #[case] expected_2: i32) {
        let mut input = Day6Solution::load(input).unwrap();
        let p1 = Day6Solution::part1(&mut input).unwrap();
        let p2 = Day6Solution::part2(&mut input, p1).unwrap();

        assert_eq!(expected_1, p1);
        assert_eq!(expected_2, p2);
    }
}
