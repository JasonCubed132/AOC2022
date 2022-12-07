
use crate::utils::solver_types::{solve_linear, SolutionLinear};
use anyhow::{anyhow, Result};
use itertools::Itertools;

pub struct Day7Solution {}

pub fn day07(input: &str) -> Result<f32> {
    solve_linear::<Day7Solution, _, _, _>(input)
}

#[derive(Clone, Copy, Debug)]
enum Node {
    File { str: String, size: i32 },
    Dir { str: String, items: Vec<Node> }
}

impl SolutionLinear<Node, i32, i32> for Day7Solution {
    fn load(input: &str) -> Result<Node> {
        let mut lines = input.lines().collect_vec(); 
        if lines.remove(0) != "$ cd /" {
            return Err(anyhow!("Expected cd into root at line 0"));
        }

        let mut head = Node::Dir { str:"".to_string(), items: [].to_vec() };
        let mut current_path: Vec<&Node> = Vec::new();
        current_path.push(&head);

        let mut expecting_files = false;

        for line in lines {
            let parts = line.split(" ").collect_vec();
            match parts[0] {
                "$" => {
                    expecting_files = false;
                    match parts[1] {
                        "cd" => {
                            match parts[2] {
                                "/" => {
                                    while current_path.len() > 1 {
                                        current_path.pop();
                                    }
                                }
                                ".." => {
                                    current_path.pop();
                                }
                                folder_name => {
                                    let node = current_path[current_path.len() - 1];
                                    match node {
                                        Node::File { .. } => { return Err(anyhow!("Tried to cd from file!")) }
                                        Node::Dir { str: parent_str,  items } => {
                                            let mut folder_found = false;

                                            for item in items {
                                                match item {
                                                    Node::File { .. } => { return Err(anyhow!("Tried to cd to a file!")) }
                                                    Node::Dir { str: to_str, items: _ } => {
                                                        if folder_name == to_str {
                                                            current_path.push(item);
                                                            folder_found = true;
                                                        }
                                                    }
                                                }
                                            }

                                            if !folder_found {
                                                let folder_node = Node::Dir { str: folder_name.to_string(), items: Vec::new() };
                                                let new_items = items.iter().copied().collect_vec();
                                                new_items.push(folder_node);
                                                let new_parent = Node::Dir { str: parent_str.to_string(), items: new_items };
                                                current_path.pop();
                                                current_path.push(&new_parent);
                                            }
                                        }
                                    }
                                    
                                }
                            }
                        },
                        "ls" => {
                            expecting_files = true;
                        }
                        _ => { return Err(anyhow!("Unknown symbol")) }
                    }
                }
                "dir" => {
                    if expecting_files {
                        let name = parts[1].to_string();
                        let new_node = Node::Dir { str: name, items: Vec::new() };
                        let curr_node = current_path[current_path.size() - 1];
                        match curr_node {
                            Node::File { .. } => { return Err(anyhow!("At a file for some reason")) },
                            Node::Dir { str: name, items: items } => {
                                let new_items = items.iter().copied().collect_vec();
                                new_items.push(new_node);
                                let new_parent = Node::Dir { str: name, items: new_items };
                                current_path.pop();
                                current_path.push(&new_parent);
                            }
                        }
                    } else {
                        return Err(anyhow!("Unknown symbol"))
                    }
                }
                size => {
                    if expecting_files {
                        let num_size = size.parse::<i32>().unwrap();
                        let name = parts[1].to_string();
                        let new_node = Node::File{ str: name, size: num_size };
                        let curr_node = current_path[current_path.size() - 1];
                        match curr_node {
                            Node::File { .. } => { return Err(anyhow!("At a file for some reason")) },
                            Node::Dir { str: name, items: items } => {
                                let new_items = items.iter().copied().collect_vec();
                                new_items.push(new_node);
                                let new_parent = Node::Dir { str: name, items: new_items };
                                current_path.pop();
                                current_path.push(&new_parent);
                            }
                        }
                    } else {
                        return Err(anyhow!("Unknown symbol"))
                    }
                }
            }
        }
        Ok(head)
    }

    fn part1(input: &mut Node) -> Result<i32> {
        println!("{input}");
        todo!()
    }

    fn part2(input: &mut Node, part_1_solution: i32) -> Result<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::Day7Solution;
    use crate::utils::solver_types::SolutionLinear;
    use rstest::rstest;

    #[rstest]
    #[case("$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k", 95437, 0)]
    fn validate_linear(#[case] input: &str, #[case] expected_1: i32, #[case] expected_2: i32) {
        let mut input = Day7Solution::load(input).unwrap();
        let p1 = Day7Solution::part1(&mut input).unwrap();
        let p2 = Day7Solution::part2(&mut input, p1).unwrap();

        assert_eq!(expected_1, p1);
        assert_eq!(expected_2, p2);
    }
}
