use std::collections::VecDeque;

use crate::utils::solver_types::{solve_linear, SolutionLinear};
use anyhow::{anyhow, Result};
use itertools::Itertools;

pub struct Day13Solution {}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
enum Token {
    LBRA,
    RBRA,
    Num(i32),
    COMMA,
    NumTemp,
}

#[derive(Debug, Clone)]
enum Item {
    Num(i32),
    List(VecDeque<Item>),
}

pub fn day13(input: &str) -> Result<f32> {
    solve_linear::<Day13Solution, _, i32, i32>(input)
}

/*
item := "[" list "]"
      | num
*/
fn parse_item(mut input: VecDeque<Token>) -> (Item, VecDeque<Token>) {
    let item = input.pop_front().unwrap();
    let result: Item;
    match item {
        Token::LBRA => {
            (result, input) = parse_list(input);
            let rbra_test = input.pop_front().unwrap();
            if rbra_test != Token::RBRA {
                panic!();
            }
        }
        Token::Num(_) => {
            input.push_front(item);
            (result, input) = parse_num(input);
        }
        _ => {
            panic!("Got {:?}, expected LBRA or NUM", item);
        }
    }
    (result, input)
}

/*
list := item list_prime
      | e
*/

fn parse_list(mut input: VecDeque<Token>) -> (Item, VecDeque<Token>) {
    let tester = input.pop_front().unwrap();

    match tester {
        Token::LBRA | Token::Num(_) => {
            input.push_front(tester);
        }
        _ => {
            input.push_front(tester);
            return (Item::List(VecDeque::new()), input);
        }
    }

    let item;
    (item, input) = parse_item(input);
    let remaining_list;
    (remaining_list, input) = parse_list_prime(input);

    match remaining_list {
        None => {
            let mut vecde = VecDeque::new();
            vecde.push_back(item);
            return (Item::List(vecde), input);
        }
        Some(mut list) => {
            list.push_front(item);
            return (Item::List(list), input);
        }
    }
}

/*
list_prime := "," item list_prime
             | e
 */
fn parse_list_prime(mut input: VecDeque<Token>) -> (Option<VecDeque<Item>>, VecDeque<Token>) {
    let comma_test = input.pop_front().unwrap();
    if comma_test != Token::COMMA {
        input.push_front(comma_test);
        return (None, input);
    }

    let item;
    (item, input) = parse_item(input);

    let remaining_list;
    (remaining_list, input) = parse_list_prime(input);

    match remaining_list {
        None => {
            let mut vecde = VecDeque::new();
            vecde.push_back(item);
            return (Some(vecde), input);
        }
        Some(mut list) => {
            list.push_front(item);
            return (Some(list), input);
        }
    }
}

// num := [0-9]+
fn parse_num(mut input: VecDeque<Token>) -> (Item, VecDeque<Token>) {
    let item = input.pop_front().unwrap();
    match item {
        Token::Num(num) => return (Item::Num(num), input),
        _ => {
            panic!("Got {:?}, expecting num", item)
        }
    }
}

fn lex(input: &str) -> Vec<Token> {
    let mut output: Vec<Token> = Vec::new();
    let mut num = 0;

    for item in input.chars() {
        if (item == '[' || item == ']' || item == ',')
            && output.len() >= 1
            && output[output.len() - 1] == Token::NumTemp
        {
            output.pop();
            output.push(Token::Num(num));
            num = 0;
        }
        if item == '[' {
            output.push(Token::LBRA);
        } else if item == ']' {
            output.push(Token::RBRA);
        } else if item == ',' {
            output.push(Token::COMMA);
        } else if (item as i32) >= '0' as i32 && (item as i32) <= '9' as i32 {
            if output[output.len() - 1] != Token::NumTemp {
                output.push(Token::NumTemp);
            }
            num *= 10;
            num += (item as i32) - ('0' as i32);
        } else {
            panic!("Unknown character {}", item);
        }
    }

    output
}

fn process(input: &str) -> Item {
    let lex_output = lex(input);

    let vedeq = VecDeque::from(lex_output);

    let (parse_result, _) = parse_item(vedeq);

    parse_result
}

fn compare_pair(pair: (Item, Item)) -> Option<bool> {
    match pair {
        (Item::List(list_a), Item::List(list_b)) => {
            return compare_lists(list_a, list_b);
        }
        (Item::List(list_a), Item::Num(num_b)) => {
            let mut list_b: VecDeque<Item> = VecDeque::new();
            list_b.push_back(Item::Num(num_b));
            return compare_lists(list_a, list_b);
        }
        (Item::Num(num_a), Item::List(list_b)) => {
            let mut list_a: VecDeque<Item> = VecDeque::new();
            list_a.push_back(Item::Num(num_a));
            return compare_lists(list_a, list_b);
        }
        (Item::Num(num_a), Item::Num(num_b)) => {
            if num_a < num_b {
                return Some(true);
            } else if num_a > num_b {
                return Some(false);
            } else {
                return None;
            }
        }
    }
}

fn compare_lists(list_a: VecDeque<Item>, list_b: VecDeque<Item>) -> Option<bool> {
    let mut idx = 0;
    loop {
        if idx >= list_a.len() && idx >= list_b.len() {
            return None;
        }
        if idx >= list_a.len() {
            return Some(true);
        } else if idx >= list_b.len() {
            return Some(false);
        }

        match compare_pair((list_a[idx].clone(), list_b[idx].clone())) {
            Some(result) => return Some(result),
            None => {}
        }

        idx += 1;
    }
}

impl SolutionLinear<Vec<(Item, Item)>, i32, i32> for Day13Solution {
    fn load(input: &str) -> Result<Vec<(Item, Item)>> {
        let raw_groups = input.lines().group_by(|x| x.to_string() == "");
        let groups = raw_groups.into_iter().filter_map(|(key, group)| {
            if key == true {
                None
            } else {
                Some(group.collect_vec())
            }
        });

        let mut output: Vec<(Item, Item)> = Vec::new();

        for item in groups {
            assert_eq!(item.len(), 2);

            let a = process(item[0]);
            let b = process(item[1]);
            output.push((a, b));
            println!("{:?}", output[output.len() - 1]);
        }

        Ok(output)
    }

    fn part1(input: &mut Vec<(Item, Item)>) -> Result<i32> {
        let pairs = input.clone();
        let mut sum: i32 = 0;

        for i in 0..pairs.len() {
            let pair = pairs[i].clone();
            if compare_pair(pair).unwrap() {
                let val: i32 = i.try_into().unwrap();
                sum += val + 1;
            }
        }

        println!("{}", sum);

        Ok(sum)
    }

    fn part2(input: &mut Vec<(Item, Item)>, part_1_solution: i32) -> Result<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::Day13Solution;
    use crate::utils::solver_types::SolutionLinear;
    use rstest::rstest;

    #[rstest]
    #[case(
        "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]",
        13,
        0
    )]
    fn validate_linear(#[case] input: &str, #[case] expected_1: i32, #[case] expected_2: i32) {
        let mut input = Day13Solution::load(input).unwrap();
        let p1 = Day13Solution::part1(&mut input).unwrap();
        let p2 = Day13Solution::part2(&mut input, p1).unwrap();

        assert_eq!(expected_1, p1);
        assert_eq!(expected_2, p2);
    }
}
