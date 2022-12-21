use std::{
    cell,
    cmp::Ordering,
    collections::HashSet,
    fmt::{write, Display},
};

use crate::utils::solver_types::{solve_linear, SolutionLinear};
use anyhow::{anyhow, Result};
use itertools::Itertools;
use regex::Regex;

pub struct Day15Solution {}

pub fn day15(input: &str) -> Result<f32> {
    solve_linear::<Day15Solution, _, _, _>(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn distance_to(&self, other: &Point) -> isize {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write(f, format_args!("({},{})", self.x, self.y))
    }
}

impl SolutionLinear<(Vec<(Point, Point)>, isize), i32, i32> for Day15Solution {
    fn load(input: &str) -> Result<(Vec<(Point, Point)>, isize)> {
        let mut rows = input.lines().collect_vec();
        let initial_row_re = Regex::new(r"target at y=(-?\d+)").unwrap();
        let initial_row = rows.remove(0);
        let target_row = initial_row_re.captures(initial_row).unwrap()[1]
            .parse::<isize>()
            .unwrap();

        let re = Regex::new(
            r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)",
        )
        .unwrap();

        let mut output: Vec<(Point, Point)> = Vec::new();
        for line in rows.iter() {
            let result = re.captures(line).unwrap();

            let sensor = Point {
                x: result[1].parse::<isize>().unwrap(),
                y: result[2].parse::<isize>().unwrap(),
            };
            let beacon = Point {
                x: result[3].parse::<isize>().unwrap(),
                y: result[4].parse::<isize>().unwrap(),
            };

            output.push((sensor, beacon));
        }

        Ok((output, target_row))
    }

    fn part1(input: &mut (Vec<(Point, Point)>, isize)) -> Result<i32> {
        let (data, target_row) = input.clone();

        let mut distances = Vec::new();
        let mut beacons = HashSet::new();

        let mut cells_on_target_row_in_use = HashSet::new();

        for (sensor, beacon) in data.clone() {
            let distance = sensor.distance_to(&beacon);
            distances.push((sensor, distance));
            beacons.insert(beacon);

            if sensor.y == target_row {
                cells_on_target_row_in_use.insert(sensor.x);
            }
            if beacon.y == target_row {
                cells_on_target_row_in_use.insert(beacon.x);
            }
        }

        // println!("{:?}", cells_on_target_row_in_use);

        let mut covered_ranges = Vec::new();

        for (sensor, distance) in distances.clone() {
            let y_dist = (sensor.y - target_row).abs();
            let x_dist = distance - y_dist;

            if x_dist < 0 {
                continue;
            }

            let left_x = sensor.x - x_dist;
            let right_x = sensor.x + x_dist;
            covered_ranges.push((left_x, right_x));
            // println!(
            //     "{} {} {} {} {} {}",
            //     sensor, distance, y_dist, x_dist, left_x, right_x
            // );
        }

        if covered_ranges.len() == 0 {
            return Ok(0);
        }

        covered_ranges.sort_by(|(a1, a2), (b1, b2)| match a1.cmp(b1) {
            Ordering::Equal => return a2.cmp(b2),
            a => {
                return a;
            }
        });

        // println!("{:?}", covered_ranges);

        let mut resolved_ranges = Vec::new();

        let mut curr_range = covered_ranges.remove(0);
        // println!("Curr range: {:?}", curr_range);
        let mut pushed_modified_range;

        loop {
            let next_range = covered_ranges.remove(0);
            // println!("Examining: {:?}", next_range);

            if curr_range.1 + 1 >= next_range.0 {
                pushed_modified_range = false;
                if curr_range.1 < next_range.1 {
                    // Skip reassignment if can overlapping
                    curr_range = (curr_range.0, next_range.1);
                }
                // println!("Curr range: {:?}", curr_range);
            } else {
                resolved_ranges.push(curr_range);
                pushed_modified_range = true;
            }

            if covered_ranges.len() == 0 {
                break;
            }
        }

        if !pushed_modified_range {
            resolved_ranges.push(curr_range)
        }

        // println!("{:?}", resolved_ranges);

        let mut count = 0;

        for (a, b) in resolved_ranges {
            count += (b - a) + 1;
            for &item in cells_on_target_row_in_use.iter() {
                if a <= item && item <= b {
                    count -= 1;
                }
            }
        }

        // println!("{}", count);
        Ok(count.try_into().unwrap())
    }

    fn part2(input: &mut (Vec<(Point, Point)>, isize), _part_1_solution: i32) -> Result<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::Day15Solution;
    use crate::utils::solver_types::SolutionLinear;
    use rstest::rstest;

    #[rstest]
    #[case(
        "target at y=10
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3",
        26,
        0
    )]
    fn validate_linear(#[case] input: &str, #[case] expected_1: i32, #[case] expected_2: i32) {
        let mut input = Day15Solution::load(input).unwrap();
        let p1 = Day15Solution::part1(&mut input).unwrap();
        let p2 = Day15Solution::part2(&mut input, p1).unwrap();

        assert_eq!(expected_1, p1);
        assert_eq!(expected_2, p2);
    }
}
