use crate::utils::solver_types::{solve_linear, SolutionLinear};
use anyhow::{anyhow, Result};
use itertools::Itertools;

pub struct Day2Solution {}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    fn score(&self) -> i32 {
        match *self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn to_rps_result(&self) -> RPSResult {
        match *self {
            Self::Rock => RPSResult::Lose,
            Self::Paper => RPSResult::Draw,
            Self::Scissors => RPSResult::Win,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum RPSResult {
    Lose,
    Draw,
    Win,
}

impl RPSResult {
    fn score(&self) -> i32 {
        match *self {
            Self::Lose => 0,
            Self::Draw => 3,
            Self::Win => 6,
        }
    }
}

pub fn day02(input: &str) -> Result<f32> {
    solve_linear::<Day2Solution, _, _, _>(input)
}

pub fn str_to_rps(input: &str) -> Result<RPS> {
    match input {
        "A" | "X" => Ok(RPS::Rock),
        "B" | "Y" => Ok(RPS::Paper),
        "C" | "Z" => Ok(RPS::Scissors),
        _ => Err(anyhow!("Unknown value {input}")),
    }
}

pub fn get_your_result(you: RPS, them: RPS) -> RPSResult {
    match (you, them) {
        (RPS::Rock, RPS::Scissors) | (RPS::Paper, RPS::Rock) | (RPS::Scissors, RPS::Paper) => {
            RPSResult::Win
        }
        (RPS::Rock, RPS::Rock) | (RPS::Paper, RPS::Paper) | (RPS::Scissors, RPS::Scissors) => {
            RPSResult::Draw
        }
        (RPS::Rock, RPS::Paper) | (RPS::Paper, RPS::Scissors) | (RPS::Scissors, RPS::Rock) => {
            RPSResult::Lose
        }
    }
}

pub fn get_your_required_move(expected_result: RPSResult, them: RPS) -> RPS {
    match (expected_result, them) {
        (RPSResult::Win, RPS::Scissors)
        | (RPSResult::Draw, RPS::Rock)
        | (RPSResult::Lose, RPS::Paper) => RPS::Rock,
        (RPSResult::Win, RPS::Rock)
        | (RPSResult::Draw, RPS::Paper)
        | (RPSResult::Lose, RPS::Scissors) => RPS::Paper,
        (RPSResult::Win, RPS::Paper)
        | (RPSResult::Draw, RPS::Scissors)
        | (RPSResult::Lose, RPS::Rock) => RPS::Scissors,
    }
}

impl SolutionLinear<Vec<(RPS, RPS)>, i32, i32> for Day2Solution {
    fn load(input: &str) -> Result<Vec<(RPS, RPS)>> {
        let mut moves: Vec<(RPS, RPS)> = Vec::new();
        for line in input.lines() {
            let items = line.split(" ").collect_vec();
            moves.push((str_to_rps(items[0]).unwrap(), str_to_rps(items[1]).unwrap()));
        }
        Ok(moves)
    }

    fn part1(input: &mut Vec<(RPS, RPS)>) -> Result<i32> {
        // let total = input.iter().fold(0, |acc, (them, you)| {
        //     let t1 = acc + you.score();
        //     let t2 = t1 + get_your_result(*you, *them).score();
        //     t2
        // });

        // let mut total = 0;
        //
        // for turn in input {
        //     let (them, you) = turn;
        //     total += you.score();
        //     total += get_your_result(*you, *them).score();
        // }

        let total = input
            .iter()
            .map(|(them, you)| you.score() + get_your_result(*you, *them).score())
            .sum::<i32>();

        Ok(total)
    }

    fn part2(input: &mut Vec<(RPS, RPS)>, _part_1_solution: i32) -> Result<i32> {
        // let mut total = 0;

        // for turn in input {
        //     let (them, unconverted_result) = turn;
        //     let result = unconverted_result.to_rps_result();

        //     let your_move = get_your_required_move(result, *them);
        //     total += your_move.score();
        //     total += result.score();
        // }

        let total = input
            .iter()
            .map(|(them, unconverted_result)| {
                let result = unconverted_result.to_rps_result();
                let your_move = get_your_required_move(result, *them);
                your_move.score() + result.score()
            })
            .sum::<i32>();

        Ok(total)
    }
}

#[cfg(test)]
mod tests {
    use super::Day2Solution;
    use crate::utils::solver_types::SolutionLinear;
    use rstest::rstest;

    #[rstest]
    #[case("A Y
B X
C Z
", 15, 12)]
    fn validate_linear(#[case] input: &str, #[case] expected_1: i32, #[case] expected_2: i32) {
        let mut input = Day2Solution::load(input).unwrap();
        let p1 = Day2Solution::part1(&mut input).unwrap();
        let p2 = Day2Solution::part2(&mut input, p1).unwrap();

        assert_eq!(expected_1, p1);
        assert_eq!(expected_2, p2);
    }
}
