
use crate::utils::solver_types::{solve_linear, SolutionLinear};
use anyhow::{anyhow, Result};
use itertools::Itertools;
use std::collections::HashSet;

pub struct Day9Solution {}

pub fn day09(input: &str) -> Result<f32> {
    solve_linear::<Day9Solution, _, _, _>(input)
}

#[derive(Debug, Clone)]
pub enum Direction {
    Up(usize),
    Down(usize),
    Left(usize),
    Right(usize)
}

impl Direction {
    fn make(input: &str, num: usize) -> Direction {
        match input {
            "U" => Direction::Up(num),
            "D" => Direction::Down(num),
            "L" => Direction::Left(num),
            "R" => Direction::Right(num),
            _ => panic!()
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point {
    x: i32,
    y: i32
}

fn simulate(input: Vec<Direction>, rope_length: usize) -> i32 {
    let mut rope = vec![Point { x: 0, y: 0}; rope_length];

    let mut set: HashSet<Point> = HashSet::new();

    // Everything except the first step in a direction will be simple
    for mov in input {
        match mov {
            Direction::Up(num) => {
                let count = num;

                for _ in 0..count {
                    rope[0].y -= 1;

                    for i in 1..rope.len() {
                        if (rope[i].y - rope[i-1].y) > 1 {
                            rope[i].y -= 1;
    
                            if rope[i-1].x != rope[i].x {
                                rope[i].x = rope[i-1].x;
                            }
                        }
                    }
                    set.insert(rope[rope.len() - 1].clone());
                }
            },
            Direction::Down(num) => {
                let count = num;

                for _ in 0..count {
                    rope[0].y += 1;

                    for i in 1..rope.len() {
                        if (rope[i-1].y - rope[i].y) > 1 {
                            rope[i].y += 1;

                            if rope[i-1].x != rope[i].x {
                                rope[i].x = rope[i-1].x;
                            }
                        }
                    }

                    // println!("{rope[i-1]:?} {rope[i]:?}");
                    set.insert(rope[rope.len() - 1].clone());
                }
            },
            Direction::Left(num) => {
                let count = num;

                for _ in 0..count {
                    rope[0].x -= 1;

                    for i in 1..rope.len() {
                        if (rope[i].x - rope[i-1].x) > 1 {
                            rope[i].x -= 1;

                            if rope[i-1].y != rope[i].y {
                                rope[i].y = rope[i-1].y
                            }
                        } 
                    }

                    // println!("{rope[i-1]:?} {rope[i]:?}");
                    set.insert(rope[rope.len() - 1].clone());
                }
            },
            Direction::Right(num) => {
                let count = num;

                for _ in 0..count {
                    rope[0].x += 1;

                    for i in 1..rope.len() {
                        if (rope[i-1].x - rope[i].x) > 1 {
                            rope[i].x += 1;

                            if rope[i-1].y != rope[i].y {
                                rope[i].y = rope[i-1].y
                            }
                        } 
                    }

                    set.insert(rope[rope.len() - 1].clone());
                }
            }
        }
    }

    let result: i32 = set.len().try_into().unwrap();

    result
}

impl SolutionLinear<Vec<Direction>, i32, i32> for Day9Solution {
    fn load(input: &str) -> Result<Vec<Direction>> {
        Ok(
            input.lines().map(|x| {
                let parts = x.split(" ").collect_vec();
                let num = parts[1].parse::<usize>();
                Direction::make(parts[0], num.unwrap())
            }).collect_vec()
        )
    }

    fn part1(input: &mut Vec<Direction>) -> Result<i32> {
        let result = simulate(input.to_vec(), 2);
        println!("P1 result {result}");
        Ok(result)
    }

    fn part2(input: &mut Vec<Direction>, _part_1_solution: i32) -> Result<i32> {
        let result = simulate(input.to_vec(), 10);
        println!("P2 result {result}");
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::Day9Solution;
    use crate::utils::solver_types::SolutionLinear;
    use rstest::rstest;

    #[rstest]
    #[case("R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2", 13, 36)]
    fn validate_linear(#[case] input: &str, #[case] expected_1: i32, #[case] expected_2: i32) {
        let mut input = Day9Solution::load(input).unwrap();
        let p1 = Day9Solution::part1(&mut input).unwrap();
        let p2 = Day9Solution::part2(&mut input, p1).unwrap();

        assert_eq!(expected_1, p1);
        assert_eq!(expected_2, p2);
    }
}
