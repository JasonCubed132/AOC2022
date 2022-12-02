
use crate::utils::solver_types::{solve_linear, SolutionLinear};
use anyhow::{anyhow, Result};
use itertools::Itertools;

pub struct Day2Solution {}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub enum RPS {
    Rock,
    Paper,
    Scissors
}

impl RPS {
    fn score(&self) -> i32 {
        match *self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3
        }
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum RPSResult {
    Lose,
    Draw,
    Win
}

impl RPSResult {
    fn score(&self) -> i32 {
        match *self {
            Self::Lose => 0,
            Self::Draw => 3,
            Self::Win => 6
        }
    }
}

pub fn day02(input: &str) -> Result<f32> {
    solve_linear::<Day2Solution, _, _, _>(input)
}

pub fn str_to_rps(input: &str) -> Result<RPS> {
    match input {
        "A" | "X" => {
            Ok(RPS::Rock)
        },
        "B" | "Y" => {
            Ok(RPS::Paper)
        },
        "C" | "Z" => {
            Ok(RPS::Scissors)
        },
        _ => {
            Err(anyhow!("Unknown value {input}"))
        }
    }
}

pub fn rps_to_rps_result(input: RPS) -> RPSResult {
    match input {
        RPS::Rock => {
            RPSResult::Lose
        },
        RPS::Paper => {
            RPSResult::Draw
        },
        RPS::Scissors => {
            RPSResult::Win
        }
    }
}

pub fn str_to_rps_result(input: &str) -> Result<RPSResult> {
    match input {
        "X" => {
            Ok(RPSResult::Lose)
        },
        "Y" => {
            Ok(RPSResult::Draw)
        },
        "Z" => {
            Ok(RPSResult::Win)
        },
        _ => {
            Err(anyhow!("Unknown value {input}"))
        }
    }
}

pub fn get_your_result(you: RPS, them: RPS) -> RPSResult {
    match you {
        RPS::Rock => {
            match them {
                RPS::Rock => { RPSResult::Draw },
                RPS::Paper => { RPSResult::Lose },
                RPS::Scissors => { RPSResult::Win }
            }
        },
        RPS::Paper => {
            match them {
                RPS::Rock => { RPSResult::Win },
                RPS::Paper => { RPSResult::Draw },
                RPS::Scissors => { RPSResult::Lose}
            }
        },
        RPS::Scissors => {
            match them {
                RPS::Rock => { RPSResult::Lose },
                RPS::Paper => { RPSResult::Win },
                RPS::Scissors => { RPSResult::Draw }
            }
        }
    }
}

pub fn get_your_required_move(expected_result: RPSResult, them: RPS) -> RPS {
    match expected_result {
        RPSResult::Win => {
            match them {
                RPS::Rock => { RPS::Paper },
                RPS::Paper => { RPS::Scissors },
                RPS::Scissors => { RPS::Rock }
            }
        },
        RPSResult::Draw => {
            them
        },
        RPSResult::Lose => {
            match them {
                RPS::Rock => { RPS::Scissors },
                RPS::Paper => { RPS::Rock },
                RPS::Scissors => { RPS::Paper }
            }
        }
    }
}

impl SolutionLinear<Vec<[RPS; 2]>, i32, i32> for Day2Solution {
    fn load(input: &str) -> Result<Vec<[RPS; 2]>> {
        let mut moves: Vec<[RPS; 2]> = Vec::new();
        for line in input.lines() {
            let items = line.split(" ").collect_vec();
            moves.push(
                [
                    str_to_rps(items[0]).unwrap(), 
                    str_to_rps(items[1]).unwrap()
                ]
            );
        }
        Ok(moves)
    }

    fn part1(input: &mut Vec<[RPS; 2]>) -> Result<i32> {
        let mut total = 0;

        for turn in input {
            let you = turn[1];
            let them = turn[0];
            total += you.score();
            total += get_your_result(you, them).score();
        }

        Ok(total)
    }

    fn part2(input: &mut Vec<[RPS; 2]>, part_1_solution: i32) -> Result<i32> {
        let mut total = 0;

        for turn in input {
            let result = rps_to_rps_result(turn[1]);
            let them = turn[0];

            let your_move = get_your_required_move(result, them);
            total += your_move.score();
            total += result.score();
        }

        Ok(total)
    }
}

#[cfg(test)]
mod tests {
    use super::Day2Solution;
    use crate::utils::solver_types::SolutionLinear;
    use rstest::rstest;

    #[rstest]
    #[case("[1,2,3]", 6, 18)]
    #[case("[0,7,13,20,1,100]", 141, 846)]
    #[case("[6000]", 6000, 6000)]
    fn validate_linear(#[case] input: &str, #[case] expected_1: usize, #[case] expected_2: usize) {
        // let mut input = Day2Solution::load(input).unwrap();
        // let p1 = Day2Solution::part1(&mut input).unwrap();
        // let p2 = Day2Solution::part2(&mut input, p1).unwrap();

        // assert_eq!(expected_1, p1);
        // assert_eq!(expected_2, p2);
    }
}
