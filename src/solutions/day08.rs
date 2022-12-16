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
        //https://stackoverflow.com/questions/67829405/how-to-convert-a-u32-constant-to-i32-constant-without-unsafe-in-rust
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
        let height = input.len();
        let width = input[0].len();

        let mut visible_map = vec![vec![0; width]; height];

        // Fill initial
        for i in 0..width {
            visible_map[0][i] = 1;
            visible_map[height - 1][i] = 1;
        }

        for i in 1..height - 1 {
            visible_map[i][0] = 1;
            visible_map[i][width - 1] = 1;
        }

        // Scan down
        let mut height_map = vec![vec![0; width]; height];
        for i in 0..width {
            height_map[0][i] = input[0][i];
        }

        for i in 1..height {
            for j in 0..width {
                if input[i][j] > height_map[i - 1][j] {
                    height_map[i][j] = input[i][j];
                    visible_map[i][j] = 1;
                } else {
                    height_map[i][j] = height_map[i - 1][j];
                }
            }
        }

        // Scan up
        let mut height_map = vec![vec![0; width]; height];
        for i in 0..width {
            height_map[height - 1][i] = input[height - 1][i];
        }

        for pos_i in 1..height {
            let i = height - (pos_i + 1);
            for j in 0..width {
                if input[i][j] > height_map[i + 1][j] {
                    height_map[i][j] = input[i][j];
                    visible_map[i][j] = 1;
                } else {
                    height_map[i][j] = height_map[i + 1][j];
                }
            }
        }

        // Scan right
        let mut height_map = vec![vec![0; width]; height];
        for i in 0..height {
            height_map[i][0] = input[i][0];
        }

        for j in 1..width {
            for i in 0..height {
                if input[i][j] > height_map[i][j - 1] {
                    height_map[i][j] = input[i][j];
                    visible_map[i][j] = 1;
                } else {
                    height_map[i][j] = height_map[i][j - 1];
                }
            }
        }

        // Scan left
        let mut height_map = vec![vec![0; width]; height];
        for i in 0..height {
            height_map[i][width - 1] = input[i][width - 1];
        }

        for pos_j in 1..width {
            let j = width - (pos_j + 1);
            for i in 0..height {
                if input[i][j] > height_map[i][j + 1] {
                    height_map[i][j] = input[i][j];
                    visible_map[i][j] = 1;
                } else {
                    height_map[i][j] = height_map[i][j + 1];
                }
            }
        }

        let result = visible_map
            .iter()
            .map(|x| x.iter().sum::<i32>())
            .sum::<i32>();

        println!("P1 result {result}");
        Ok(result)
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
