use std::{
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

        for (sensor, beacon) in data.clone() {
            let distance = sensor.distance_to(&beacon);
            distances.push((sensor, distance));
            beacons.insert(beacon);
        }

        let mut min_x = distances[0].0.x - distances[0].1;
        let mut max_x = min_x;

        for (sensor, distance) in distances.clone() {
            let min_x_tmp = sensor.x - distance;
            let max_x_tmp = sensor.x + distance;

            if min_x > min_x_tmp {
                min_x = min_x_tmp;
            }
            if max_x < max_x_tmp {
                max_x = max_x_tmp;
            }
        }

        // println!("{} {}", min_x, max_x);

        let mut count = 0;

        for i in min_x..max_x + 1 {
            let p = Point {
                x: i,
                y: target_row,
            };

            for (sensor, distance) in &distances {
                if sensor.distance_to(&p) <= *distance && !beacons.contains(&p) {
                    count += 1;
                    break;
                }
            }
        }

        println!("{}", count);
        Ok(count)
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
