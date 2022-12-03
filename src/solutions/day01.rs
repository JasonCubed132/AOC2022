use crate::utils::solver_types::{solve_linear, SolutionLinear};
use anyhow::{Result};
use itertools::Itertools;

/// Not yet implementd
pub struct Day1Solution {}

pub fn day01(input: &str) -> Result<f32> {
    solve_linear::<Day1Solution, _, _, _>(input)
}

impl SolutionLinear<Vec<Vec<i32>>, i32, i32> for Day1Solution {
    fn load(_input: &str) -> Result<Vec<Vec<i32>>> {
        let test: Vec<&str> = _input.lines().collect();
        println!("{test:?}");

        let mut inventories = Vec::new();
        let mut inventory = Vec::new();

        for line in _input.lines() {
            if line != "" {
                inventory.push(line.parse::<i32>().unwrap());
            } else {
                if inventory.len() != 0 {
                    inventories.push(inventory);
                    inventory = Vec::new();
                }
            }
        }

        if inventory.len() != 0 {
            inventories.push(inventory);
        }

        return Ok(inventories);
    }

    fn part1(_input: &mut Vec<Vec<i32>>) -> Result<i32> {
        Ok(_input
            .iter()
            .map(|x| x.iter().sum::<i32>())
            .max()
            .unwrap()
        )
    }

    fn part2(_input: &mut Vec<Vec<i32>>, _part_1_solution: i32) -> Result<i32> {
        Ok(_input
            .iter()
            .map(|x| x.iter().sum::<i32>())
            .sorted()
            .rev()
            .take(3)
            .sum::<i32>()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::Day1Solution;
    use crate::utils::solver_types::SolutionLinear;
    use rstest::rstest;

    #[rstest]
    #[case("1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
", 24000, 45000)]
    fn validate_linear(#[case] input: &str, #[case] expected_1: i32, #[case] expected_2: i32) {
        let mut input = Day1Solution::load(input).unwrap();
        let p1 = Day1Solution::part1(&mut input).unwrap();
        let p2 = Day1Solution::part2(&mut input, p1).unwrap();

        assert_eq!(expected_1, p1);
        assert_eq!(expected_2, p2);
    }
}