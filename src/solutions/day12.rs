use std::{collections::VecDeque, path};

use crate::utils::solver_types::{solve_linear, SolutionLinear};
use anyhow::{anyhow, Result};
use itertools::Itertools;

pub struct Day12Solution {}

pub fn day12(input: &str) -> Result<f32> {
    solve_linear::<Day12Solution, _, _, _>(input)
}

#[derive(Copy, Clone, Debug)]
struct Point {
    x: usize,
    y: usize,
}

fn format_grid(input: Vec<Vec<i32>>) -> String {
    input
        .iter()
        .map(|row| {
            row.iter()
                .map(|item| {
                    if item < &0 {
                        format!("{:->3}", "")
                    } else {
                        format!("{:03}", item)
                    }
                })
                .join("|")
        })
        .join("\n")
}

fn get_distance(height_grid: Vec<Vec<i32>>, start: Point, end: Point) -> i32 {
    let height = height_grid.len();
    let width = height_grid[0].len();
    let mut path_map = vec![vec![-1; width]; height];
    let mut next_points: VecDeque<Point> = VecDeque::new();
    next_points.push_back(start);
    path_map[start.y][start.x] = 0;

    // let rel_points: Vec<(i32, i32)> = vec![(1, 1), (1, 0), (1, -1), (0, -1), (-1, -1), (-1, 0), (-1, 1), (0, 1)];
    let rel_points: Vec<(i32, i32)> = vec![(1, 0), (0, -1), (-1, 0), (0, 1)];

    loop {
        let point = next_points.pop_front().unwrap();
        // println!("Evaluating {:?}", point);
        let x_point: i32 = point.x.try_into().unwrap();
        let y_point: i32 = point.y.try_into().unwrap();

        let h1 = height_grid[point.y][point.x];

        for (x_rel, y_rel) in &rel_points {
            let new_x = x_point + x_rel;
            let new_y = y_point + y_rel;

            if new_x < 0 || new_x >= width.try_into().unwrap() {
                continue;
            }
            if new_y < 0 || new_y >= height.try_into().unwrap() {
                continue;
            }

            let test_point = Point {
                x: new_x.try_into().unwrap(),
                y: new_y.try_into().unwrap(),
            };
            // println!("Rel: {:?}", test_point);

            if path_map[test_point.y][test_point.x] != -1 {
                // println!("Skipping as already done");
                continue;
            }

            let h2 = height_grid[test_point.y][test_point.x];
            if h2 > h1 && h2 - h1 > 1 {
                // println!("{} {} Skipping as too different vertically", h1, h2);
                continue;
            }

            path_map[test_point.y][test_point.x] = path_map[point.y][point.x] + 1;
            next_points.push_back(test_point);
        }

        if next_points.len() == 0 {
            break;
        }
    }

    // println!("{}", format_grid(path_map.clone()));

    let result = path_map[end.y][end.x];
    // println!("{}", result);
    result
}

impl SolutionLinear<(Vec<Vec<i32>>, Point, Point), i32, i32> for Day12Solution {
    fn load(input: &str) -> Result<(Vec<Vec<i32>>, Point, Point)> {
        let char_grid = input.lines().map(|x| x.chars().collect_vec()).collect_vec();
        let mut height_grid: Vec<Vec<i32>> = Vec::new();
        let mut start = Point { x: 0, y: 0 };
        let mut end = Point { x: 0, y: 0 };

        for i in 0..char_grid.len() {
            let mut row: Vec<i32> = Vec::new();

            for j in 0..char_grid[i].len() {
                let c = char_grid[i][j] as i32;
                let a = 'a' as i32;
                let z = 'z' as i32;

                if c >= a && c <= z {
                    row.push(c - a);
                } else if c == 'S' as i32 {
                    row.push(a - a);
                    start = Point { x: j, y: i };
                } else if c == 'E' as i32 {
                    row.push(z - a);
                    end = Point { x: j, y: i };
                }
            }

            height_grid.push(row);
        }

        Ok((height_grid, start, end))
    }

    fn part1(input: &mut (Vec<Vec<i32>>, Point, Point)) -> Result<i32> {
        let (height_grid, start, end) = input.clone();
        let result = get_distance(height_grid, start, end);
        println!("{}", result);
        Ok(result)
    }

    fn part2(input: &mut (Vec<Vec<i32>>, Point, Point), _part_1_solution: i32) -> Result<i32> {
        let (height_grid, _, end) = input.clone();

        let height = height_grid.len();
        let width = height_grid[0].len();
        let mut distances: Vec<i32> = Vec::new();

        for i in 0..height {
            for j in 0..width {
                if height_grid[i][j] == 0 {
                    let start = Point { x: j, y: i };
                    let distance = get_distance(height_grid.clone(), start, end);
                    if distance != -1 {
                        distances.push(distance);
                    }
                }
            }
        }

        distances.sort();

        // println!("{:?}", distances);

        Ok(distances[0])
    }
}

#[cfg(test)]
mod tests {
    use super::Day12Solution;
    use crate::utils::solver_types::SolutionLinear;
    use rstest::rstest;

    #[rstest]
    #[case(
        "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi",
        31,
        29
    )]
    fn validate_linear(#[case] input: &str, #[case] expected_1: i32, #[case] expected_2: i32) {
        let mut input = Day12Solution::load(input).unwrap();
        let p1 = Day12Solution::part1(&mut input).unwrap();
        let p2 = Day12Solution::part2(&mut input, p1).unwrap();

        assert_eq!(expected_1, p1);
        assert_eq!(expected_2, p2);
    }
}
