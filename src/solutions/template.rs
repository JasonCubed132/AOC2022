
use crate::utils::solver_types::{solve_linear, SolutionLinear};
use anyhow::{anyhow, Result};
use itertools::Itertools;

pub struct TemplateSolution {}

pub fn template(input: &str) -> Result<f32> {
    solve_linear::<TemplateSolution, _, _, _>(input)
}

impl SolutionLinear<_, _, _> for TemplateSolution {
    fn load(input: &str) -> Result<_> {
        todo!()
    }

    fn part1(input: &mut _) -> Result<_> {
        todo!()
    }

    fn part2(input: &mut _, part_1_solution: _) -> Result<_> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::TemplateSolution;
    use crate::utils::solver_types::SolutionLinear;
    use rstest::rstest;

    #[rstest]
    #[case("[1,2,3]", 6, 18)]
    #[case("[0,7,13,20,1,100]", 141, 846)]
    #[case("[6000]", 6000, 6000)]
    fn validate_linear(#[case] input: &str, #[case] expected_1: usize, #[case] expected_2: usize) {
        let mut input = Day2Solution::load(input).unwrap();
        let p1 = Day2Solution::part1(&mut input).unwrap();
        let p2 = Day2Solution::part2(&mut input, p1).unwrap();

        assert_eq!(expected_1, p1);
        assert_eq!(expected_2, p2);
    }
}
