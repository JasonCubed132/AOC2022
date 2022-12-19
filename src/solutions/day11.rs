use crate::utils::solver_types::{solve_linear, SolutionLinear};
use anyhow::{anyhow, Result};
use itertools::Itertools;
use regex::Regex;

pub struct Day11Solution {}

pub fn day11(input: &str) -> Result<f32> {
    solve_linear::<Day11Solution, _, _, _>(input)
}

#[derive(Debug, Clone)]
enum Val {
    Num(i32),
    Own,
}

impl Val {
    fn make(input: &str) -> Val {
        if input == "old" {
            return Val::Own;
        }
        let val = input.parse::<i32>().unwrap();
        Val::Num(val)
    }
}

#[derive(Debug, Clone)]
enum Op {
    Add(Val, Val),
    Sub(Val, Val),
    Mul(Val, Val),
}
impl Op {
    fn make(left: &str, op: &str, right: &str) -> Op {
        let left_parsed = Val::make(left);
        let right_parsed = Val::make(right);

        match op {
            "+" => return Op::Add(left_parsed, right_parsed),
            "-" => return Op::Sub(left_parsed, right_parsed),
            "*" => return Op::Mul(left_parsed, right_parsed),
            _ => {
                panic!();
            }
        }
    }
}

#[derive(Debug, Clone)]
struct throw_tester {
    is_div_by: i32,
    true_target: usize,
    false_target: usize,
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<i32>,
    op: Op,
    tester: throw_tester,
}

impl SolutionLinear<Vec<Monkey>, i32, i32> for Day11Solution {
    fn load(input: &str) -> Result<Vec<Monkey>> {
        let unparsed_monkeys = input.lines().chunks(7);

        let head_re = Regex::new(r"^Monkey (\d):$").unwrap();
        let items_re = Regex::new(r"^ *Starting items: (\d+(?:, \d+)*)$").unwrap();
        let op_re = Regex::new(r"^ *Operation: new = (old|\d+) (\+|\-|\*) (old|\d+)$").unwrap();
        let test_re = Regex::new(r"^ *Test: divisible by (\d+)$").unwrap();
        let test_true_re = Regex::new(r"^ *If true: throw to monkey (\d+)$").unwrap();
        let test_false_re = Regex::new(r"^ *If false: throw to monkey (\d+)$").unwrap();

        let mut parsed_monkeys: Vec<Monkey> = Vec::new();

        let mut idx = 0;

        for unparsed_monkey in unparsed_monkeys.into_iter() {
            let mut unparsed_monkey_vec = unparsed_monkey.collect_vec();

            if unparsed_monkey_vec.len() == 7 {
                if unparsed_monkey_vec[6] == "" {
                    unparsed_monkey_vec.pop();
                }
            }

            if unparsed_monkey_vec.len() != 6 {
                return Err(anyhow!("Unparsed monkey was not of length 6"));
            }

            let head_match = head_re.captures(unparsed_monkey_vec[0]).unwrap();
            let monkey_num = head_match[1].to_string().parse::<usize>().unwrap();
            assert!(idx == monkey_num);

            let items_match = items_re.captures(unparsed_monkey_vec[1]).unwrap();
            let items_str = items_match[1].to_string();
            let items = items_str
                .split(", ")
                .map(|x| x.parse::<i32>().unwrap())
                .collect_vec();

            let op_match = op_re.captures(unparsed_monkey_vec[2]).unwrap();
            let op = Op::make(&op_match[1], &op_match[2], &op_match[3]);

            let test_match = test_re.captures(unparsed_monkey_vec[3]).unwrap();
            let test_val = test_match[1].parse::<i32>().unwrap();

            let test_true_match = test_true_re.captures(unparsed_monkey_vec[4]).unwrap();
            let test_true_val = test_true_match[1].parse::<usize>().unwrap();

            let test_false_match = test_false_re.captures(unparsed_monkey_vec[5]).unwrap();
            let test_false_val = test_false_match[1].parse::<usize>().unwrap();

            let tester = throw_tester {
                is_div_by: test_val,
                true_target: test_true_val,
                false_target: test_false_val,
            };
            let monkey = Monkey {
                items: items,
                op: op,
                tester: tester,
            };
            parsed_monkeys.push(monkey);

            idx += 1;
        }
        println!("{parsed_monkeys:?}");

        Ok(parsed_monkeys)
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
    #[case(
        "Monkey 0:
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
    If false: throw to monkey 1",
        10605,
        0
    )]
    fn validate_linear(#[case] input: &str, #[case] expected_1: i32, #[case] expected_2: i32) {
        let mut input = Day11Solution::load(input).unwrap();
        let p1 = Day11Solution::part1(&mut input).unwrap();
        let p2 = Day11Solution::part2(&mut input, p1).unwrap();

        assert_eq!(expected_1, p1);
        assert_eq!(expected_2, p2);
    }
}
