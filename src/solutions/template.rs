
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
    #[case("", 0, 0)]
    fn validate_linear(#[case] input: &str, #[case] expected_1: _, #[case] expected_2: _) {
        let mut input = TemplateSolution::load(input).unwrap();
        let p1 = TemplateSolution::part1(&mut input).unwrap();
        let p2 = TemplateSolution::part2(&mut input, p1).unwrap();

        assert_eq!(expected_1, p1);
        assert_eq!(expected_2, p2);
    }
}
