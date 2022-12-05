use crate::utils::solver_types::{solve_linear, SolutionLinear};
use anyhow::{anyhow, Result};
use itertools::Itertools;
use regex::Regex;

pub struct Day5Solution {}

pub fn day05(input: &str) -> Result<f32> {
    solve_linear::<Day5Solution, _, _, _>(input)
}

impl SolutionLinear<(Vec<Vec<char>>, Vec<(i32, i32, i32)>), String, String> for Day5Solution {
    fn load(input: &str) -> Result<(Vec<Vec<char>>, Vec<(i32, i32, i32)>)> {
        let lines: Vec<&str> = input.lines().collect();
        let parts = lines.split(|x| x.to_string().eq("")).collect_vec();
        println!("{parts:?}");

        let stack_regex = Regex::new(r"^(?:\[([A-Z])\]| ( ) )(?: (?:\[([A-Z])\]| ( ) ))*$").unwrap();

        for line in parts[0] {
            println!("{line:?}");
            let result = stack_regex.captures_iter(line);
            for item in result {
                println!("{item:?}");
            }
        }
        todo!()
    }

    fn part1(input: &mut (Vec<Vec<char>>, Vec<(i32, i32, i32)>)) -> Result<String> {
        todo!()
    }

    fn part2(input: &mut (Vec<Vec<char>>, Vec<(i32, i32, i32)>), _part_1_solution: String) -> Result<String> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::Day5Solution;
    use crate::utils::solver_types::SolutionLinear;
    use rstest::rstest;

    #[rstest]
    #[case("    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
", "CMZ", "")]
    fn validate_linear(#[case] input: &str, #[case] expected_1: String, #[case] expected_2: String) {
        let mut input = Day5Solution::load(input).unwrap();
        let p1 = Day5Solution::part1(&mut input).unwrap();
        let p1_clone = p1.clone();
        let p2 = Day5Solution::part2(&mut input, p1_clone).unwrap();

        assert_eq!(expected_1, p1);
        assert_eq!(expected_2, p2);
    }
}
