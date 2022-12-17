
use crate::utils::solver_types::{solve_linear, SolutionLinear};
use anyhow::{anyhow, Result};
use itertools::Itertools;

pub struct Day11Solution {}

pub fn day11(input: &str) -> Result<f32> {
    solve_linear::<Day11Solution, _, _, _>(input)
}

enum Op {
    Add(i32),
    Sub(i32),
    Mul(i32)
}

struct throw_tester {
    is_div_by: i32,
    true_target: usize,
    false_target: usize
}

struct Monkey {
    items: Vec<i32>,
    op: Op,
    tester: throw_tester
}

impl SolutionLinear<Vec<Monkey>, i32, i32> for Day11Solution {
    fn load(input: &str) -> Result<Vec<Monkey>> {
        let unparsed_monkeys = input.lines().chunks(7);

        for mut unparsed_monkey in unparsed_monkeys.into_iter() {
            for line in unparsed_monkey.join("\n").lines() {
                println!("{line:?}");
            }

            println!("-------------");
        }
        
        todo!()
    }

    fn part1(input: &mut Vec<Monkey>) -> Result<i32> {
        todo!()
    }

    fn part2(input: &mut Vec<Monkey>, _part_1_solution: i32) -> Result<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::Day11Solution;
    use crate::utils::solver_types::SolutionLinear;
    use rstest::rstest;

    #[rstest]
    #[case("Monkey 0:
Starting items: 79, 98
Operation: new = old * 19
Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
Starting items: 54, 65, 75, 74
Operation: new = old + 6
Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
Starting items: 79, 60, 97
Operation: new = old * old
Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
Starting items: 74
Operation: new = old + 3
Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1", 10605, 0)]
    fn validate_linear(#[case] input: &str, #[case] expected_1: i32, #[case] expected_2: i32) {
        let mut input = Day11Solution::load(input).unwrap();
        let p1 = Day11Solution::part1(&mut input).unwrap();
        let p2 = Day11Solution::part2(&mut input, p1).unwrap();

        assert_eq!(expected_1, p1);
        assert_eq!(expected_2, p2);
    }
}
