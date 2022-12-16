use std::collections::HashSet;

use crate::utils::solver_types::{solve_linear, SolutionLinear};
use anyhow::{anyhow, Result};
use itertools::Itertools;

pub struct Day6Solution {}

pub fn day06(input: &str) -> Result<f32> {
    solve_linear::<Day6Solution, _, _, _>(input)
}

fn generic_solution(input: &mut Vec<char>, len_of_set: usize) -> Result<i32> {
    for index in 0..(input.len() - len_of_set) {
        let set: HashSet<char> = input[index..index + len_of_set]
            .into_iter()
            .copied()
            .collect();
        if set.len() == len_of_set {
            let result: i32 = (index + len_of_set).try_into().unwrap();
            return Ok(result);
        }
    }

    Err(anyhow!("Unique set not found"))
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
        generic_solution(input, 4)
    }

    fn part2(input: &mut Vec<char>, _part_1_solution: i32) -> Result<i32> {
        generic_solution(input, 14)
    }
}

#[cfg(test)]
mod tests {
    use super::Day6Solution;
    use crate::utils::solver_types::SolutionLinear;
    use rstest::rstest;

    #[rstest]
    #[case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7, 19)]
    #[case("bvwbjplbgvbhsrlpgdmjqwftvncz", 5, 23)]
    #[case("nppdvjthqldpwncqszvftbrmjlhg", 6, 23)]
    #[case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10, 29)]
    #[case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11, 26)]
    fn validate_linear(#[case] input: &str, #[case] expected_1: i32, #[case] expected_2: i32) {
        let mut input = Day6Solution::load(input).unwrap();
        let p1 = Day6Solution::part1(&mut input).unwrap();
        let p2 = Day6Solution::part2(&mut input, p1).unwrap();

        assert_eq!(expected_1, p1);
        assert_eq!(expected_2, p2);
    }
}
