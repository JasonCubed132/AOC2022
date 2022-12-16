use crate::utils::solver_types::{solve_linear, SolutionLinear};
use anyhow::{anyhow, Result};
use itertools::Itertools;

pub struct Day8Solution {}

pub fn day08(input: &str) -> Result<f32> {
    solve_linear::<Day8Solution, _, _, _>(input)
}

impl SolutionLinear<Vec<Vec<i32>>, i32, i32> for Day8Solution {
    fn load(input: &str) -> Result<Vec<Vec<i32>>> {
        //https://stackoverflow.com/questions/43983414/how-to-convert-a-rust-char-to-an-integer-so-that-1-becomes-1#43985962
        let forest = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|x| x.to_digit(10).expect("issue converting to int") as i32)
                    .collect_vec()
            })
            .collect_vec();
        Ok(forest)
    }

    fn part1(input: &mut Vec<Vec<i32>>) -> Result<i32> {
        todo!()
    }

    fn part2(input: &mut Vec<Vec<i32>>, _part_1_solution: i32) -> Result<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::Day8Solution;
    use crate::utils::solver_types::SolutionLinear;
    use rstest::rstest;

    #[rstest]
    #[case(
        "30373
25512
65332
33549
35390",
        21,
        0
    )]
    fn validate_linear(#[case] input: &str, #[case] expected_1: i32, #[case] expected_2: i32) {
        let mut input = Day8Solution::load(input).unwrap();
        let p1 = Day8Solution::part1(&mut input).unwrap();
        let p2 = Day8Solution::part2(&mut input, p1).unwrap();

        assert_eq!(expected_1, p1);
        assert_eq!(expected_2, p2);
    }
}
