use left_pad::leftpad;
use std::collections::{HashMap, HashSet};
use transpose::transpose;

use crate::utils::solver_types::{solve_linear, SolutionLinear};
use anyhow::{anyhow, Result};
use itertools::Itertools;

pub struct Day14Solution {}

pub fn day14(input: &str) -> Result<f32> {
    solve_linear::<Day14Solution, _, _, _>(input)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

fn format_infinite_grid(
    filled_points_map: &HashMap<Point, char>,
    unfilled_cell_mark: char,
) -> Result<String> {
    let mut output = String::new();

    let filled_points = filled_points_map.keys().map(|x| x.clone()).collect_vec();

    if filled_points.len() < 1 {
        return Err(anyhow!("Got 0 values"));
    }

    let a_point = filled_points[0].clone();
    let mut min_x = a_point.x;
    let mut max_x = a_point.x;
    let mut min_y = a_point.y;
    let mut max_y = a_point.y;

    for point in filled_points {
        if point.x > max_x {
            max_x = point.x
        }
        if point.x < min_x {
            min_x = point.x
        }
        if point.y > max_y {
            max_y = point.y
        }
        if point.y < min_y {
            min_y = point.y
        }
    }

    let str_min_x_len = min_x.to_string().len();
    let str_max_x_len = max_x.to_string().len();
    let str_min_y_len = min_y.to_string().len();
    let str_max_y_len = max_y.to_string().len();

    let str_x_len = if str_max_x_len > str_min_x_len {
        str_max_x_len
    } else {
        str_min_x_len
    }
    .try_into()
    .unwrap();
    let str_y_len = if str_max_y_len > str_min_y_len {
        str_max_y_len
    } else {
        str_min_y_len
    };

    let mut grid = vec![vec![unfilled_cell_mark; max_x - min_x + 1]; max_y - min_y + 1];

    for (&point, &value) in filled_points_map {
        grid[point.y - min_y][point.x - min_x] = value;
    }

    let mut header = String::new();
    header += &leftpad("", (str_y_len + 1) * (str_x_len)).to_string();
    for i in min_x..max_x + 1 {
        header += &leftpad(i.to_string(), str_x_len).to_string();
    }

    let header_input = header.chars().collect_vec();
    let mut header_output = vec![' '; header_input.len()];
    let header_width = max_x - min_x + 2 + str_y_len;
    let header_height = str_x_len;
    transpose(
        &header_input,
        &mut header_output,
        header_height,
        header_width,
    );
    let header_str = header_output
        .chunks(header_width)
        .map(|x| x.iter().collect::<String>())
        .join("\n");
    output += &header_str;
    output += "\n";

    let mut idx_y = min_y;
    let out_str = grid
        .iter()
        .map(|x| {
            let prefix = leftpad(idx_y.to_string(), str_y_len).to_string();
            idx_y += 1;
            prefix + " " + &x.iter().collect::<String>()
        })
        .join("\n");
    output += &out_str;

    Ok(output)
}

impl SolutionLinear<HashSet<Point>, i32, i32> for Day14Solution {
    fn load(input: &str) -> Result<HashSet<Point>> {
        let mut output: HashSet<Point> = HashSet::new();
        for line in input.lines() {
            let mut coords = line
                .split(" -> ")
                .map(|x| {
                    let parsed = x
                        .split(",")
                        .map(|y| y.parse::<usize>().unwrap())
                        .collect_vec();
                    Point {
                        x: parsed[0],
                        y: parsed[1],
                    }
                })
                .collect_vec();

            let mut p1 = coords.remove(0);
            let mut p2: Point;

            output.insert(p1.clone());
            loop {
                if coords.len() <= 0 {
                    break;
                }

                p2 = coords.remove(0);
                output.insert(p2.clone());
                // println!("P1: {:?} P2: {:?}", p1, p2);

                let x_1;
                let x_2;
                let y_1;
                let y_2;

                if p1.x > p2.x {
                    x_1 = p2.x;
                    x_2 = p1.x;
                } else {
                    x_1 = p1.x;
                    x_2 = p2.x;
                }

                if p1.y > p2.y {
                    y_1 = p2.y;
                    y_2 = p1.y;
                } else {
                    y_1 = p1.y;
                    y_2 = p2.y;
                }

                // println!("X1 {} X2 {} Y1 {} Y2 {}", x_1, x_2, y_1, y_2);

                for i in x_1..x_2 + 1 {
                    for j in y_1..y_2 + 1 {
                        // println!("I: {} J: {}", i, j);
                        let p3 = Point { x: i, y: j };
                        output.insert(p3);
                    }
                }

                p1 = p2;
            }
        }

        // println!("{:?}", output);

        /*
          4     5  5
          9     0  0
          4     0  3
        0 ......+...
        1 ..........
        2 ..........
        3 ..........
        4 ....#...##
        5 ....#...#.
        6 ..###...#.
        7 ........#.
        8 ........#.
        9 #########.
        */

        Ok(output)
    }

    fn part1(input: &mut HashSet<Point>) -> Result<i32> {
        let start_point = Point { x: 500, y: 0 };

        let mut filled_points_map = HashMap::new();

        let mut max_y = 0;
        for point in input.iter() {
            if point.y > max_y {
                max_y = point.y;
            }
            filled_points_map.insert(point.clone(), '#');
        }

        // println!("{}", format_infinite_grid(&filled_points_map, '.').unwrap());

        let mut landed_count = 0;

        // Each unit of sand
        loop {
            let mut sand = start_point.clone();
            let landed;
            // Each time step
            loop {
                sand.y += 1;
                if sand.y > max_y {
                    landed = false;
                    break;
                }
                if !filled_points_map.contains_key(&sand) {
                    continue;
                }
                sand.x -= 1;
                if !filled_points_map.contains_key(&sand) {
                    continue;
                }
                sand.x += 2;
                if !filled_points_map.contains_key(&sand) {
                    continue;
                }
                sand.x -= 1;
                sand.y -= 1;
                filled_points_map.insert(sand.clone(), 'o');
                landed = true;
                break;
            }
            if landed {
                landed_count += 1;
            } else {
                break;
            }
        }

        filled_points_map.insert(start_point, '+');

        println!("{}", format_infinite_grid(&filled_points_map, '.').unwrap());

        println!("{}", landed_count);

        Ok(landed_count)
    }

    fn part2(input: &mut HashSet<Point>, _part_1_solution: i32) -> Result<i32> {
        let start_point = Point { x: 500, y: 0 };

        let mut filled_points_map = HashMap::new();

        let mut max_y = 0;
        for point in input.iter() {
            if point.y > max_y {
                max_y = point.y;
            }
            filled_points_map.insert(point.clone(), '#');
        }

        let mut landed_count = 0;

        // Each unit of sand
        loop {
            let mut sand = start_point.clone();
            // Each time step
            loop {
                sand.y += 1;
                if !filled_points_map.contains_key(&sand) && sand.y != max_y + 2 {
                    continue;
                }
                sand.x -= 1;
                if !filled_points_map.contains_key(&sand) && sand.y != max_y + 2 {
                    continue;
                }
                sand.x += 2;
                if !filled_points_map.contains_key(&sand) && sand.y != max_y + 2 {
                    continue;
                }
                sand.x -= 1;
                sand.y -= 1;
                filled_points_map.insert(sand.clone(), 'o');
                break;
            }

            landed_count += 1;

            if filled_points_map.contains_key(&start_point) {
                break;
            }
        }

        filled_points_map.insert(start_point, '+');

        println!("{}", format_infinite_grid(&filled_points_map, '.').unwrap());

        println!("{}", landed_count);

        Ok(landed_count)
    }
}

#[cfg(test)]
mod tests {
    use super::Day14Solution;
    use crate::utils::solver_types::SolutionLinear;
    use rstest::rstest;

    #[rstest]
    #[case(
        "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9",
        24,
        93
    )]
    fn validate_linear(#[case] input: &str, #[case] expected_1: i32, #[case] expected_2: i32) {
        let mut input = Day14Solution::load(input).unwrap();
        let p1 = Day14Solution::part1(&mut input).unwrap();
        let p2 = Day14Solution::part2(&mut input, p1).unwrap();

        assert_eq!(expected_1, p1);
        assert_eq!(expected_2, p2);
    }
}
