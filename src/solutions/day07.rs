
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
    name: String,
    size: i32,
    items: HashSet<String>
}

fn backfill_sizes(mut structure: HashMap<String, Item>, curr_path: String) {
    let mut path: Vec<String> = Vec::new();
    path.push(curr_path);
    let mut size = 0;

    let mut node: &mut Item  = structure.get_mut(path.join("/").as_str()).unwrap();

    for item in node.items {
        path.push(item.to_string());
        backfill_sizes(structure, path.join("/"));

        size += structure.get(path.join("/").as_str()).unwrap().size;
        path.pop();
    }

    node.size = size;
}

impl SolutionLinear<HashMap<String, Item>, i32, i32> for Day7Solution {
    fn load(input: &str) -> Result<HashMap<String, Item>> {
        let mut lines = input.lines().collect_vec(); 
        if lines.remove(0) != "$ cd /" {
            return Err(anyhow!("Expected cd into root at line 0"));
        }

        let mut path: Vec<String> = Vec::new();
        let mut str_path = path.join("/").to_string();

        let head = Item { name: str_path.clone(), size: 0, items: HashSet::new() };

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
                                    println!("Cd root");
                                    path.clear();
                                }
                                ".." => {
                                    println!("Cd up");
                                    path.pop();
                                }
                                folder_name => {
                                    let curr_folder = structure.get_mut(str_path.as_str());
                                    let not_found;
                                    match curr_folder {
                                        Some(item) => {
                                            not_found = item.items.insert(folder_name.to_string());
                                        }
                                        None => { panic!() }
                                    }
                                    path.push(folder_name.to_string());

                                    if not_found {
                                        let folder = Item { name: folder_name.to_string(), size: 0, items: HashSet::new() };
                                        structure.insert(path.join("/").to_string(), folder);
                                    }

                                    println!("cd into {folder_name} from {str_path}");
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

                        let curr_folder = structure.get_mut(str_path.as_str());
                        let not_found;
                        match curr_folder {
                            Some(item) => {
                                not_found = item.items.insert(name.clone());
                            }
                            None => { panic!() }
                        }

                        if not_found {
                            path.push(name.to_string());
                            let folder = Item { name: name.clone(), size: 0, items: HashSet::new() };

                            structure.insert(path.join("/"), folder);
                            path.pop();
                        }

                        println!("Create dir {name} at {str_path}");
                    } else {
                        return Err(anyhow!("Unknown symbol"))
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
                            None => { panic!() }
                        }

                        if not_found {
                            path.push(name.to_string());
                            let file = Item { name: name.clone(), size: num_size, items: HashSet::new() };

                            structure.insert(path.join("/"), file);
                            path.pop();
                        }

                        println!("Create file {name} size {num_size} at {str_path}");
                    } else {
                        return Err(anyhow!("Unknown symbol"))
                    }
                }
            }
        }

        backfill_sizes(structure, "".to_string());

        Ok(structure)
    }

    fn part1(input: &mut HashMap<String, Item>) -> Result<i32> {
        println!("{input:?}");
        todo!()
    }

    fn part2(input: &mut HashMap<String, Item>, part_1_solution: i32) -> Result<i32> {
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
