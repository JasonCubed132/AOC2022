use crate::utils::solver_types::{solve_linear, SolutionLinear};
use anyhow::{anyhow, Result};
use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;

pub struct Day7Solution {}

pub fn day07(input: &str) -> Result<f32> {
    solve_linear::<Day7Solution, _, _, _>(input)
}

#[derive(Clone, Debug)]
struct Item {
    size: i32,
    items: HashSet<String>,
    is_file: bool,
}

fn backfill_sizes(structure: &mut HashMap<String, Item>, curr_path: Option<String>) {
    let mut path: Vec<String> = Vec::new();
    match curr_path {
        Some(str) => path.push(str),
        None => {}
    }
    let mut size = 0;

    let str_path = path.join("/");
    let curr_item = structure.get(str_path.as_str()).cloned();
    let items = curr_item.expect(str_path.as_str()).items;

    for item in items {
        path.push(item.to_string());

        let test_path = path.join("/");
        let test_item = structure.get(test_path.as_str()).cloned();
        if !test_item.expect(test_path.as_str()).is_file {
            backfill_sizes(structure, Some(test_path.clone()));
        }

        size += structure.get(test_path.as_str()).unwrap().size;
        path.pop();
    }

    // println!("Size {size} at path {str_path}");
    structure.get_mut(str_path.as_str()).unwrap().size = size;
}

fn count_sizes_that_are_at_most(
    structure: &mut HashMap<String, Item>,
    upper_limit: i32,
    curr_path: Option<String>,
) -> i32 {
    let mut path: Vec<String> = Vec::new();
    match curr_path {
        Some(str) => path.push(str),
        None => {}
    }
    let mut size = 0;

    let str_path = path.join("/");
    let curr_item = structure.get(str_path.as_str()).cloned();
    let items = curr_item.clone().expect(str_path.as_str()).items;

    let curr_size = curr_item.expect(str_path.as_str()).size;
    // println!("Testing item {str_path} with size {curr_size}");
    if curr_size <= upper_limit {
        size += curr_size;
    }

    for item in items {
        path.push(item.to_string());

        let test_path = path.join("/");
        let test_item = structure.get(test_path.as_str()).cloned();
        if !test_item.clone().expect(test_path.as_str()).is_file {
            size += count_sizes_that_are_at_most(structure, upper_limit, Some(test_path.clone()));
        }
        path.pop();
    }
    size
}

impl SolutionLinear<HashMap<String, Item>, i32, i32> for Day7Solution {
    fn load(input: &str) -> Result<HashMap<String, Item>> {
        let mut lines = input.lines().collect_vec();
        if lines.remove(0) != "$ cd /" {
            return Err(anyhow!("Expected cd into root at line 0"));
        }

        let mut path: Vec<String> = Vec::new();
        let mut str_path = path.join("/").to_string();

        let head = Item {
            size: 0,
            items: HashSet::new(),
            is_file: false,
        };

        let mut structure: HashMap<String, Item> = HashMap::new();
        structure.insert(str_path.clone(), head);

        let mut expecting_files = false;

        for line in lines {
            let parts = line.split(" ").collect_vec();
            str_path = path.join("/").to_string();
            match parts[0] {
                "$" => {
                    expecting_files = false;
                    match parts[1] {
                        "cd" => {
                            match parts[2] {
                                "/" => {
                                    // println!("Cd root");
                                    path.clear();
                                }
                                ".." => {
                                    // println!("Cd up");
                                    path.pop();
                                }
                                folder_name => {
                                    let curr_folder = structure.get_mut(str_path.as_str());
                                    let not_found;
                                    match curr_folder {
                                        Some(item) => {
                                            not_found = item.items.insert(folder_name.to_string());
                                        }
                                        None => {
                                            panic!()
                                        }
                                    }
                                    path.push(folder_name.to_string());

                                    if not_found {
                                        let folder = Item {
                                            size: 0,
                                            items: HashSet::new(),
                                            is_file: false,
                                        };
                                        structure.insert(path.join("/").to_string(), folder);
                                    }

                                    // println!("cd into {folder_name} from {str_path}");
                                }
                            }
                        }
                        "ls" => {
                            expecting_files = true;
                        }
                        _ => return Err(anyhow!("Unknown symbol")),
                    }
                }
                "dir" => {
                    if expecting_files {
                        let name = parts[1].to_string();

                        let curr_folder = structure.get_mut(str_path.as_str());
                        let not_found;
                        match curr_folder {
                            Some(item) => {
                                not_found = item.items.insert(name.clone());
                            }
                            None => {
                                panic!()
                            }
                        }

                        if not_found {
                            path.push(name.to_string());
                            let folder = Item {
                                size: 0,
                                items: HashSet::new(),
                                is_file: false,
                            };

                            structure.insert(path.join("/"), folder);
                            path.pop();
                        }

                        // println!("Create dir {name} at {str_path}");
                    } else {
                        return Err(anyhow!("Unknown symbol"));
                    }
                }
                size => {
                    if expecting_files {
                        let num_size = size.parse::<i32>().unwrap();
                        let name = parts[1].to_string();

                        let curr_folder = structure.get_mut(str_path.as_str());
                        let not_found;
                        match curr_folder {
                            Some(item) => {
                                not_found = item.items.insert(name.clone());
                            }
                            None => {
                                panic!()
                            }
                        }

                        if not_found {
                            path.push(name.to_string());
                            let file = Item {
                                size: num_size,
                                items: HashSet::new(),
                                is_file: true,
                            };

                            structure.insert(path.join("/"), file);
                            path.pop();
                        }

                        // println!("Create file {name} size {num_size} at {str_path}");
                    } else {
                        return Err(anyhow!("Unknown symbol"));
                    }
                }
            }
        }

        backfill_sizes(&mut structure, None);

        Ok(structure)
    }

    fn part1(input: &mut HashMap<String, Item>) -> Result<i32> {
        // println!("{input:?}");
        let result = count_sizes_that_are_at_most(input, 100000, None);
        // println!("P1 result {result}");
        Ok(result)
    }

    fn part2(input: &mut HashMap<String, Item>, _part_1_solution: i32) -> Result<i32> {
        let root_size = input.get("").cloned().unwrap().size;
        let total_size = 70000000;
        let target_size = 30000000;
        let need_size = target_size - (total_size - root_size);

        let mut sizes = input
            .values()
            .filter(|item| !item.is_file)
            .map(|item| item.size)
            .collect_vec();
        sizes.sort();
        // println!("{sizes:?}");

        let new_sizes = sizes
            .iter()
            .filter(|x| **x >= need_size)
            .map(|x| *x)
            .collect_vec();
        let result = new_sizes[0];
        // println!("P2 result {result}");
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::Day7Solution;
    use crate::utils::solver_types::SolutionLinear;
    use rstest::rstest;

    #[rstest]
    #[case(
        "$ cd /
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
7214296 k",
        95437,
        24933642
    )]
    fn validate_linear(#[case] input: &str, #[case] expected_1: i32, #[case] expected_2: i32) {
        let mut input = Day7Solution::load(input).unwrap();
        let p1 = Day7Solution::part1(&mut input).unwrap();
        let p2 = Day7Solution::part2(&mut input, p1).unwrap();

        assert_eq!(expected_1, p1);
        assert_eq!(expected_2, p2);
    }
}
