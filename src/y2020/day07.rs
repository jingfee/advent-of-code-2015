use crate::solver::Solver;
use std::io::prelude::*;
use std::io::BufReader;
use std::{collections::HashSet, fs::File};

pub struct BagRule {
    holder: String,
    content: Option<String>,
    amount: u8,
}

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<BagRule>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input(&self, file: File) -> Vec<BagRule> {
        let buf_reader = BufReader::new(file);
        let lines: Vec<String> = buf_reader.lines().map(|l| l.unwrap()).collect();
        get_bag_rules(lines)
    }

    fn solve_part_one(&self, input: &mut Vec<BagRule>) -> usize {
        let mut holders = HashSet::new();
        bag_holders("shiny gold".to_string(), &mut holders, &input);
        holders.len()
    }

    fn solve_part_two(&self, input: &mut Vec<BagRule>) -> usize {
        bag_content("shiny gold".to_string(), &input)
    }
}

fn get_bag_rules(lines: Vec<String>) -> Vec<BagRule> {
    let mut bag_rules: Vec<BagRule> = Vec::new();
    for line in lines {
        let line_split = line.split(' ').collect::<Vec<&str>>();
        let holder = format!("{} {}", line_split[0], line_split[1]);

        if line_split.len() < 8 {
            bag_rules.push(BagRule {
                amount: 0,
                content: None,
                holder: holder.to_string(),
            });
            continue;
        }

        let number_of_bags = (line_split.len() - 8) / 4 + 1;
        for i in 0..number_of_bags {
            let index = 4 * (i + 1);
            let amount = line_split[index].parse::<u8>().unwrap();
            let content = format!("{} {}", line_split[index + 1], line_split[index + 2]);

            bag_rules.push(BagRule {
                amount: amount,
                content: Some(content),
                holder: holder.to_string(),
            });
        }
    }

    bag_rules
}

fn bag_holders(target: String, holders: &mut HashSet<String>, bag_rules: &Vec<BagRule>) {
    bag_rules
        .iter()
        .filter(|rule| match &rule.content {
            Some(c) => c == &target,
            None => false,
        })
        .for_each(|rule| {
            holders.insert(rule.holder.to_string());
            bag_holders(rule.holder.to_string(), holders, bag_rules);
        });
}

fn bag_content(target: String, bag_rules: &Vec<BagRule>) -> usize {
    let rules = bag_rules.iter().filter(|rule| rule.holder == target);

    let mut bags = 0;

    for rule in rules {
        bags = bags + rule.amount as usize;
        match &rule.content {
            Some(content) => {
                bags = bags + (rule.amount as usize) * bag_content(content.to_string(), bag_rules);
            }
            None => (),
        }
    }

    bags
}

#[cfg(test)]
mod tests {
    use crate::y2020::day07::*;

    #[test]
    fn test_bag_holders() {
        let rules = get_bag_rules(vec![
            "light red bags contain 1 bright white bag, 2 muted yellow bags.".to_string(),
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.".to_string(),
            "bright white bags contain 1 shiny gold bag.".to_string(),
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.".to_string(),
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.".to_string(),
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.".to_string(),
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.".to_string(),
            "faded blue bags contain no other bags.".to_string(),
            "dotted black bags contain no other bags.".to_string(),
        ]);

        let mut holders = HashSet::new();
        bag_holders("shiny gold".to_string(), &mut holders, &rules);

        assert_eq!(holders.len(), 4);
    }

    #[test]
    fn test_bag_content() {
        let ex1 = get_bag_rules(vec![
            "light red bags contain 1 bright white bag, 2 muted yellow bags.".to_string(),
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.".to_string(),
            "bright white bags contain 1 shiny gold bag.".to_string(),
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.".to_string(),
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.".to_string(),
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.".to_string(),
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.".to_string(),
            "faded blue bags contain no other bags.".to_string(),
            "dotted black bags contain no other bags.".to_string(),
        ]);

        let ex2 = get_bag_rules(vec![
            "shiny gold bags contain 2 dark red bags.".to_string(),
            "dark red bags contain 2 dark orange bags.".to_string(),
            "dark orange bags contain 2 dark yellow bags.".to_string(),
            "dark yellow bags contain 2 dark green bags.".to_string(),
            "dark green bags contain 2 dark blue bags.".to_string(),
            "dark blue bags contain 2 dark violet bags.".to_string(),
            "dark violet bags contain no other bags.".to_string(),
        ]);

        assert_eq!(bag_content("shiny gold".to_string(), &ex1), 32);
        assert_eq!(bag_content("shiny gold".to_string(), &ex2), 126);
    }
}
