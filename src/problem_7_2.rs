use std::collections::HashMap;

use crate::util;

type Bag = usize;

#[derive(Debug)]
struct BagRule {
    bag: Bag,
    amount: usize,
}

#[derive(Debug)]
struct BagWithRules {
    bag: Bag,
    rules: Vec<BagRule>,
}

pub fn problem_7_2() -> String {
    let mut bag_id = 0usize;
    let mut str_to_bag_id = HashMap::<String, usize>::new();

    let mut get_bag_id = |bag_str: &String| match str_to_bag_id.get(bag_str) {
        Some(id) => *id,
        None => {
            let res = bag_id;
            bag_id += 1;
            str_to_bag_id.insert(bag_str.clone(), res);
            res
        }
    };

    let bags = util::read("input/problem_7_input.txt")
        .lines()
        .take_while(|i| !i.is_empty())
        .map(|i| {
            let mut bag_and_rules = i.split(" bags contain ");
            let bag_str = bag_and_rules.nth(0).expect("Expected bag!").to_string();
            let bag = get_bag_id(&bag_str);
            let rules = bag_and_rules
                .nth(0)
                .expect("Expected rules!")
                .split(", ")
                .filter_map(|i| {
                    let nums = i.chars().take_while(|j| j.is_numeric()).collect::<String>();

                    match nums.is_empty() {
                        true => None,
                        false => {
                            let amount = nums.parse().expect("Expected num!");
                            let bag_str = &i
                                .split(" bag")
                                .nth(0)
                                .expect("Expected bag for rule!")
                                .chars()
                                .skip(nums.len() + 1)
                                .collect::<String>();
                            let bag = get_bag_id(bag_str);
                            Some(BagRule { bag, amount })
                        }
                    }
                })
                .collect();
            (bag, BagWithRules { bag, rules })
        })
        .collect::<HashMap<_, _>>();

    let bag_mtx: Vec<Vec<_>> = (0..bag_id)
        .map(|i| {
            let bag = &bags.get(&i).expect(format!("Expected bag {}!", i).as_str());
            (0..bag_id)
                .map(|j| {
                    bag.rules
                        .iter()
                        .find(|k| k.bag == j)
                        .map(|k| k.amount)
                        .unwrap_or(0)
                })
                .collect()
        })
        .collect();

    let gold_id = str_to_bag_id["shiny gold"];

    let mut stack = vec![(&bag_mtx[gold_id], 1)];
    let mut hold_amount = 0;
    while let Some((bags_held, depth)) = stack.pop() {
        for (bag_id, holds_amount) in bags_held.iter().enumerate() {
            if *holds_amount != 0 {
                let new_depth = holds_amount * depth;
                hold_amount += new_depth;
                stack.push((&bag_mtx[bag_id], new_depth))
            }
        }
    }
    hold_amount.to_string()
}
