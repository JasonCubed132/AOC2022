use crate::utils::solver_types::{solve_linear, SolutionLinear};
use anyhow::{anyhow, Result};
use itertools::Itertools;

/// Not yet implementd
pub struct Day1Solution {}

pub fn day01(input: &str) -> Result<f32> {
    solve_linear::<Day1Solution, _, _, _>(input)
}

impl SolutionLinear<Vec<Vec<i32>>, String, String> for Day1Solution {
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

    fn part1(_input: &mut Vec<Vec<i32>>) -> Result<String> {
        Ok(_input
            .iter()
            .map(|x| x.iter().sum::<i32>())
            .max()
            .unwrap()
            .to_string())
    }

    fn part2(_input: &mut Vec<Vec<i32>>, _part_1_solution: String) -> Result<String> {
        Ok(_input
            .iter()
            .map(|x| x.iter().sum::<i32>())
            .sorted()
            .rev()
            .take(3)
            .sum::<i32>()
            .to_string())
    }
}

#[cfg(test)]
mod tests {}
