
use crate::utils::solver_types::{solve_linear, SolutionLinear};
use anyhow::{anyhow, Result};
use itertools::Itertools;
use std::collections::HashSet;

pub struct Day3Solution {}

pub fn item_to_priority(input: char) -> Result<i32> {
    if input >= 'a' && input <= 'z' {
        Ok(input as i32 - 'a' as i32 + 1)
    } else if input >= 'A' && input <= 'Z' {
        Ok(input as i32 - 'A' as i32 + 27)
    } else {
        Err(anyhow!("Blah {input}"))
    }
}

pub fn day03(input: &str) -> Result<f32> {
    solve_linear::<Day3Solution, _, _, _>(input)
}

impl SolutionLinear<Vec<(Vec<char>, Vec<char>)>, i32, i32> for Day3Solution {
    fn load(input: &str) -> Result<Vec<(Vec<char>, Vec<char>)>> {
        let mut output: Vec<(Vec<char>, Vec<char>)> = Vec::new();
        for line in input.lines() {
            let stuff = line.chars().collect_vec();
            let half = stuff.len() / 2;
            output.push(
                (
                    stuff[0..half].to_vec(),
                    stuff[half..stuff.len()].to_vec()
                )
            );
        }
        Ok(output)
    }

    fn part1(input: &mut Vec<(Vec<char>, Vec<char>)>) -> Result<i32> {
        let mut total = 0;
        for (bag_a, bag_b) in input {
            // println!("{bag_a:?} {bag_b:?}");
            let bag_a_hash: HashSet<char> = bag_a.iter().copied().collect();
            let bag_b_hash: HashSet<char> = bag_b.iter().copied().collect();
            let common: Vec<char> = bag_a_hash.intersection(&bag_b_hash).copied().collect();
            // println!("{common:?}");
            for item in common {
                let val = item_to_priority(item).unwrap();
                // println!("{item} {val}");
                total += val;
            }
        }
        Ok(total)
    }

    fn part2(input: &mut Vec<(Vec<char>, Vec<char>)>, part_1_solution: i32) -> Result<i32> {
        let processed_input: Vec<HashSet<char>> = input.iter().map(|(bag_a, bag_b)| {
            let bag_a_hash: HashSet<char> = bag_a.iter().copied().collect();
            let bag_b_hash: HashSet<char> = bag_b.iter().copied().collect();
            let common: HashSet<char> = bag_a_hash.union(&bag_b_hash).copied().collect();
            common
        }).collect();

        let mut index = 0;
        let mut total = 0;
        loop {
            // let a = &processed_input[index];
            // println!("{a:?}");
            // let a = &processed_input[index+1];
            // println!("{a:?}");
            // let a = &processed_input[index+2];
            // println!("{a:?}");
            let common_a_b: HashSet<char> = processed_input[index].intersection(&processed_input[index+1]).copied().collect();
            // println!("{common_a_b:?}");
            let common_a_b_c: Vec<char> = common_a_b.intersection(&processed_input[index+2]).copied().collect();
            // println!("{common_a_b_c:?}");
            if common_a_b_c.len() != 1 {
                return Err(anyhow!("More or less than 1 item in common between three elves at index {index}"));
            }
            total += item_to_priority(common_a_b_c[0]).unwrap();

            if index + 3 >= processed_input.len() { break };
            index += 3;
        }
        Ok(total)
    }
}

#[cfg(test)]
mod tests {
    use super::Day3Solution;
    use crate::utils::solver_types::SolutionLinear;
    use rstest::rstest;

    #[rstest]
    #[case("[1,2,3]", 6, 18)]
    #[case("[0,7,13,20,1,100]", 141, 846)]
    #[case("[6000]", 6000, 6000)]
    fn validate_linear(#[case] input: &str, #[case] expected_1: i32, #[case] expected_2: i32) {
        let mut input = Day3Solution::load(input).unwrap();
        let p1 = Day3Solution::part1(&mut input).unwrap();
        let p2 = Day3Solution::part2(&mut input, p1).unwrap();

        assert_eq!(expected_1, p1);
        assert_eq!(expected_2, p2);
    }
}
