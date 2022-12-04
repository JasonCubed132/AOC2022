use crate::utils::solver_types::{solve_linear, SolutionLinear};
use anyhow::{anyhow, Result};
use itertools::Itertools;

pub struct Day4Solution {}

pub fn day04(input: &str) -> Result<f32> {
    solve_linear::<Day4Solution, _, _, _>(input)
}

#[derive(Clone, Copy, Debug)]
pub struct Range {
    lower: i32,
    upper: i32,
}

impl Range {
    pub fn new(lower: i32, upper: i32) -> Self {
        Self { lower, upper }
    }

    pub fn test_contains_range(self, other: Range) -> bool {
        self.lower >= other.lower && self.upper <= other.upper
    }

    pub fn test_overlaps_range(self, other: Range) -> bool {
        (self.lower >= other.lower && self.lower <= other.upper)
            || (self.upper >= other.lower && self.upper <= other.upper)
    }
}

impl SolutionLinear<Vec<(Range, Range)>, i32, i32> for Day4Solution {
    fn load(input: &str) -> Result<Vec<(Range, Range)>> {
        let mut range_pairs: Vec<(Range, Range)> = Vec::new();
        for line in input.lines() {
            let raw_pairs: Vec<Range> = line
                .split(",")
                .map(|x| {
                    let parsed = x
                        .split("-")
                        .map(|y| y.parse::<i32>().unwrap())
                        .collect_vec();

                    if parsed.len() != 2 {
                        panic!("Got other than 2 numbers from range split");
                    }

                    Range::new(parsed[0], parsed[1])
                })
                .collect_vec();

            if raw_pairs.len() != 2 {
                return Err(anyhow!("Got other than 2 ranges"));
            }

            range_pairs.push((raw_pairs[0], raw_pairs[1]));
        }

        Ok(range_pairs)
    }

    fn part1(input: &mut Vec<(Range, Range)>) -> Result<i32> {
        let mut count = 0;

        for (range_1, range_2) in input {
            if range_1.test_contains_range(*range_2) || range_2.test_contains_range(*range_1) {
                count += 1;
            }
        }

        Ok(count)
    }

    fn part2(input: &mut Vec<(Range, Range)>, _part_1_solution: i32) -> Result<i32> {
        let mut count = 0;

        for (range_1, range_2) in input {
            if range_1.test_overlaps_range(*range_2) || range_2.test_overlaps_range(*range_1) {
                count += 1;
            }
        }

        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use super::Day4Solution;
    use crate::utils::solver_types::SolutionLinear;
    use rstest::rstest;

    #[rstest]
    #[case(
        "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8",
        2,
        4
    )]
    fn validate_linear(#[case] input: &str, #[case] expected_1: i32, #[case] expected_2: i32) {
        let mut input = Day4Solution::load(input).unwrap();
        let p1 = Day4Solution::part1(&mut input).unwrap();
        let p2 = Day4Solution::part2(&mut input, p1).unwrap();

        assert_eq!(expected_1, p1);
        assert_eq!(expected_2, p2);
    }
}
