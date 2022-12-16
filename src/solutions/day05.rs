use std::vec;

use crate::utils::solver_types::{solve_linear, SolutionLinear};
use anyhow::Result;
use itertools::Itertools;
use regex::Regex;
use transpose::transpose;

pub struct Day5Solution {}

pub fn day05(input: &str) -> Result<f32> {
    solve_linear::<Day5Solution, _, _, _>(input)
}

impl SolutionLinear<(Vec<Vec<char>>, Vec<(i32, i32, i32)>), String, String> for Day5Solution {
    fn load(input: &str) -> Result<(Vec<Vec<char>>, Vec<(i32, i32, i32)>)> {
        let lines: Vec<&str> = input.lines().collect();
        let parts = lines.split(|x| x.to_string().eq("")).collect_vec();

        // Initial parse
        let mut stacks: Vec<Vec<char>> = Vec::new();
        let lines_num = parts[0].len();
        for line in parts[0] {
            let stack_chars = (**line).chars().collect_vec();
            let items = stack_chars.chunks(4).collect_vec();
            let mut stack: Vec<char> = Vec::new();

            for item in items {
                stack.push(item[1]);
            }

            stacks.push(stack);
        }

        let mut flat_stacks: Vec<char> = Vec::new();
        for stack in stacks {
            for item in stack {
                flat_stacks.push(item);
            }
        }

        let lines_width = flat_stacks.len() / lines_num;
        let mut transposed_flat_stacks: Vec<char> = vec!['_'; flat_stacks.len()];

        transpose(
            flat_stacks.as_slice(),
            transposed_flat_stacks.as_mut_slice(),
            lines_width,
            lines_num,
        );

        let transposed_stacks = transposed_flat_stacks.chunks(lines_num).collect_vec();

        let mut output_stacks: Vec<Vec<char>> = Vec::new();
        for tranposed_stack in transposed_stacks {
            let mut vec_stack = tranposed_stack.to_vec();
            vec_stack.reverse();
            while vec_stack[vec_stack.len() - 1] == ' ' {
                vec_stack.pop();
            }
            output_stacks.push(vec_stack);
        }

        let mut instructions: Vec<(i32, i32, i32)> = Vec::new();

        let instruction_regex = Regex::new(r"^move ([0-9]+) from ([0-9]+) to ([0-9]+)$").unwrap();
        for line in parts[1] {
            let result = instruction_regex.captures(line).unwrap();

            let count = &result[1].parse::<i32>().unwrap();
            let start = &result[2].parse::<i32>().unwrap();
            let end = &result[3].parse::<i32>().unwrap();

            instructions.push((*count, *start, *end));
        }

        Ok((output_stacks, instructions))
    }

    fn part1(input: &mut (Vec<Vec<char>>, Vec<(i32, i32, i32)>)) -> Result<String> {
        let (mut stacks, instructions) = input.clone();

        for (count, start, end) in instructions {
            let start_idx: usize = (start - 1).try_into().unwrap();
            let end_idx: usize = (end - 1).try_into().unwrap();
            for _ in 0..count {
                let item = stacks[start_idx].pop().unwrap();
                stacks[end_idx].push(item);
            }
        }

        let mut tops: String = String::new();

        for stack in stacks {
            tops += &stack[stack.len() - 1].to_string();
        }

        Ok(tops)
    }

    fn part2(
        input: &mut (Vec<Vec<char>>, Vec<(i32, i32, i32)>),
        _part_1_solution: String,
    ) -> Result<String> {
        let (mut stacks, instructions) = input.clone();

        for (count, start, end) in instructions {
            let start_idx: usize = (start - 1).try_into().unwrap();
            let end_idx: usize = (end - 1).try_into().unwrap();
            let mut items: Vec<char> = Vec::new();
            for _ in 0..count {
                let item = stacks[start_idx].pop().unwrap();
                items.push(item);
            }
            items.reverse();
            for item in items {
                stacks[end_idx].push(item);
            }
        }

        let mut tops: String = String::new();

        for stack in stacks {
            tops += &stack[stack.len() - 1].to_string();
        }

        Ok(tops)
    }
}

#[cfg(test)]
mod tests {
    use super::Day5Solution;
    use crate::utils::solver_types::SolutionLinear;
    use rstest::rstest;

    #[rstest]
    #[case(
        "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
",
        "CMZ",
        "MCD"
    )]
    fn validate_linear(
        #[case] input: &str,
        #[case] expected_1: String,
        #[case] expected_2: String,
    ) {
        let mut input = Day5Solution::load(input).unwrap();
        let p1 = Day5Solution::part1(&mut input).unwrap();
        let p1_clone = p1.clone();
        let p2 = Day5Solution::part2(&mut input, p1_clone).unwrap();

        assert_eq!(expected_1, p1);
        assert_eq!(expected_2, p2);
    }
}
