use std::collections::HashMap;

use crate::utils::solver_types::{solve_linear, SolutionLinear};
use anyhow::{anyhow, Result};
use itertools::Itertools;
use petgraph::{
    dot::Dot,
    graph::{Graph, NodeIndex},
    Undirected,
};
use regex::Regex;

pub struct Day16Solution {}

pub fn day16(input: &str) -> Result<f32> {
    solve_linear::<Day16Solution, _, _, _>(input)
}

impl SolutionLinear<(HashMap<String, i32>, HashMap<String, Vec<String>>), i32, i32>
    for Day16Solution
{
    fn load(input: &str) -> Result<(HashMap<String, i32>, HashMap<String, Vec<String>>)> {
        let mut flow_rates: HashMap<String, i32> = HashMap::new();
        let mut connections: HashMap<String, Vec<String>> = HashMap::new();
        let mut nodes: HashMap<String, NodeIndex> = HashMap::new();
        let mut graph = Graph::<&str, u32, Undirected>::new_undirected();
        let line_re = Regex::new(r"Valve ([A-Z]+) has flow rate=(\d+); tunnel(?:s|) lead(?:s|) to valve(?:s|) ([A-Z]+(?:, [A-Z]+)*)").unwrap();

        for line in input.lines() {
            let m = line_re.captures(line).unwrap();
            let valve_label = m[1].to_string();
            let valve_rate = m[2].parse::<i32>().unwrap();
            let connected_valves = m[3].split(", ").map(|x| x.to_string()).collect_vec();

            flow_rates.insert(valve_label.clone(), valve_rate);
            connections.insert(valve_label, connected_valves);
        }

        for item in flow_rates.keys() {
            nodes.insert(item.to_string(), graph.add_node(item));
        }

        for (valve, connected_valves) in &connections {
            for connected_valve in connected_valves {
                match graph.find_edge(nodes[valve], nodes[connected_valve]) {
                    Some(_) => {
                        continue;
                    }
                    None => {
                        graph.add_edge(nodes[valve], nodes[connected_valve], 1);
                    }
                }
            }
        }

        println!("{:?}\n{:?}", flow_rates, connections);
        println!("{:?}", Dot::with_config(&graph, &[]));

        Ok((flow_rates, connections))
    }

    fn part1(input: &mut (HashMap<String, i32>, HashMap<String, Vec<String>>)) -> Result<i32> {
        todo!()
    }

    fn part2(
        input: &mut (HashMap<String, i32>, HashMap<String, Vec<String>>),
        _part_1_solution: i32,
    ) -> Result<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::Day16Solution;
    use crate::utils::solver_types::SolutionLinear;
    use rstest::rstest;

    #[rstest]
    #[case(
        "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II",
        1651,
        0
    )]
    fn validate_linear(#[case] input: &str, #[case] expected_1: i32, #[case] expected_2: i32) {
        let mut input = Day16Solution::load(input).unwrap();
        let p1 = Day16Solution::part1(&mut input).unwrap();
        let p2 = Day16Solution::part2(&mut input, p1).unwrap();

        assert_eq!(expected_1, p1);
        assert_eq!(expected_2, p2);
    }
}
