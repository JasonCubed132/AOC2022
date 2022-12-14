use crate::utils::solver_types::{solve_linear, SolutionLinear};
use anyhow::{anyhow, Result};
use itertools::Itertools;

pub struct Day10Solution {}

pub fn day10(input: &str) -> Result<f32> {
    solve_linear::<Day10Solution, _, _, _>(input)
}

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Addx(i32),
    Noop,
}

impl Instruction {
    fn parse(input: &str) -> Result<Instruction> {
        let parts = input.split(" ").collect_vec();
        match parts[0] {
            "addx" => {
                let num = parts[1].parse::<i32>()?;
                return Ok(Instruction::Addx(num));
            }
            "noop" => {
                return Ok(Instruction::Noop);
            }
            _ => Err(anyhow!("Unknown seq {0}", parts[0])),
        }
    }
}

impl SolutionLinear<Vec<Instruction>, i32, String> for Day10Solution {
    fn load(input: &str) -> Result<Vec<Instruction>> {
        let mut output: Vec<Instruction> = Vec::new();

        for line in input.lines() {
            output.push(Instruction::parse(line)?)
        }

        Ok(output)
    }

    fn part1(input: &mut Vec<Instruction>) -> Result<i32> {
        let mut pc = 0;
        let mut clock_cycle = 0;
        let mut x = 1;
        let mut instr_in_progress = false;
        let mut signal_strength = 0;

        // Each loop should be one clock cycle
        loop {
            clock_cycle += 1;

            if clock_cycle % 40 == 20 {
                signal_strength += (clock_cycle) * x;
            }

            match input[pc] {
                Instruction::Addx(val) => {
                    if instr_in_progress {
                        instr_in_progress = false;
                        x += val;
                        pc += 1;
                    } else {
                        instr_in_progress = true;
                    }
                }
                Instruction::Noop => {
                    pc += 1;
                }
            }

            if pc >= input.len() {
                break;
            }
        }

        println!("Signal strength: {signal_strength}");

        Ok(signal_strength)
    }

    fn part2(input: &mut Vec<Instruction>, _part_1_solution: i32) -> Result<String> {
        let mut pc = 0;
        let mut clock_cycle = 0;
        let mut x = 1;
        let mut instr_in_progress = false;
        let mut screen: Vec<Vec<char>> = Vec::new();
        let mut row: Vec<char> = Vec::new();

        // Each loop should be one clock cycle
        loop {
            clock_cycle += 1;

            let idx = (clock_cycle - 1) % 40;
            if idx >= x - 1 && idx <= x + 1 {
                row.push('#');
            } else {
                row.push('.');
            }

            if clock_cycle % 40 == 0 {
                screen.push(row);
                row = Vec::new();
            }

            match input[pc] {
                Instruction::Addx(val) => {
                    if instr_in_progress {
                        instr_in_progress = false;
                        x += val;
                        pc += 1;
                    } else {
                        instr_in_progress = true;
                    }
                }
                Instruction::Noop => {
                    pc += 1;
                }
            }

            if pc >= input.len() {
                break;
            }
        }

        let result = screen.iter().map(|row| row.iter().join("")).join("\n");

        println!("{result}");

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::Day10Solution;
    use crate::utils::solver_types::SolutionLinear;
    use rstest::rstest;

    #[rstest]
    #[case(
        "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop",
        13140,
        "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
    )]
    #[case(
        "noop
addx 3
addx -5",
        0,
        ""
    )]
    fn validate_linear(#[case] input: &str, #[case] expected_1: i32, #[case] expected_2: String) {
        let mut input = Day10Solution::load(input).unwrap();
        let p1 = Day10Solution::part1(&mut input).unwrap();
        let p2 = Day10Solution::part2(&mut input, p1).unwrap();

        assert_eq!(expected_1, p1);
        assert_eq!(expected_2, p2);
    }
}
