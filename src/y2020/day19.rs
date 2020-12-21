use crate::solver::Solver;
use itertools::Itertools;
use pcre2::bytes::Regex;
use std::io::prelude::*;
use std::io::BufReader;
use std::{collections::HashMap, fs::File};

pub struct Problem;

impl Solver for Problem {
    type Input = (HashMap<usize, String>, Vec<String>);
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input(&self, file: File) -> (HashMap<usize, String>, Vec<String>) {
        let buf_reader = BufReader::new(file);
        let lines = buf_reader.lines().map(|l| l.unwrap()).collect();
        parse_lines(lines)
    }

    fn solve_part_one(&self, input: &(HashMap<usize, String>, Vec<String>)) -> usize {
        let regex = generate_regex(0, &input.0, false);
        validate_messages(regex, &input.1)
    }

    fn solve_part_two(&self, input: &(HashMap<usize, String>, Vec<String>)) -> usize {
        let regex = generate_regex(0, &input.0, true);
        validate_messages(regex, &input.1)
    }
}

fn generate_regex(
    rule_number: usize,
    rules: &HashMap<usize, String>,
    with_replace: bool,
) -> String {
    let rule = &rules[&rule_number];
    if rule == "\"a\"" {
        return "a".to_string();
    }

    if rule == "\"b\"" {
        return "b".to_string();
    }

    rule.split(" | ")
        .map(|p| {
            let mut pipe_regex = String::from("(?:");
            let rec_rules = p
                .split(" ")
                .map(|r| {
                    let mut follow_regex = String::from("(?:");
                    let rule_number = r.parse::<usize>().unwrap();
                    follow_regex.push_str(&generate_regex(rule_number, rules, with_replace));
                    follow_regex.push_str(")");
                    follow_regex
                })
                .collect_vec();

            if with_replace && rule_number == 11 {
                pipe_regex.push_str(&format!("({}(?1)?{})", rec_rules[0], rec_rules[1]));
            } else {
                for rec_rule in rec_rules {
                    pipe_regex.push_str(&rec_rule);
                }
            }

            pipe_regex.push_str(")");

            if with_replace && rule_number == 8 {
                pipe_regex.push_str("+");
            }

            pipe_regex
        })
        .join("|")
}

fn validate_messages(regex: String, messages: &Vec<String>) -> usize {
    let re = Regex::new(&format!(r"^{}$", regex)).unwrap();
    let mut valid_messages = 0;
    for message in messages {
        valid_messages = if re.is_match(message.as_bytes()).unwrap() {
            valid_messages + 1
        } else {
            valid_messages
        };
    }
    valid_messages
}

fn parse_lines(lines: Vec<String>) -> (HashMap<usize, String>, Vec<String>) {
    let mut rules = HashMap::new();
    let mut messages = Vec::new();

    let mut i = 0;
    loop {
        let line = &lines[i];
        if line == "" {
            i = i + 1;
            break;
        }

        let split = line.split(": ").collect::<Vec<&str>>();
        rules.insert(split[0].parse::<usize>().unwrap(), split[1].to_string());

        i = i + 1;
    }

    loop {
        if i == lines.len() {
            break;
        }

        messages.push(lines[i].to_string());

        i = i + 1;
    }

    (rules, messages)
}

#[cfg(test)]
mod tests {
    use crate::y2020::day19::*;

    #[test]
    fn test_validate_messages() {
        let ex1 = parse_lines(vec![
            "0: 4 1 5".to_string(),
            "1: 2 3 | 3 2".to_string(),
            "2: 4 4 | 5 5".to_string(),
            "3: 4 5 | 5 4".to_string(),
            "4: \"a\"".to_string(),
            "5: \"b\"".to_string(),
            "".to_string(),
            "ababbb".to_string(),
            "bababa".to_string(),
            "abbbab".to_string(),
            "aaabbb".to_string(),
            "aaaabbb".to_string(),
        ]);

        let ex2 = parse_lines(vec![
            "42: 9 14 | 10 1".to_string(),
            "9: 14 27 | 1 26".to_string(),
            "10: 23 14 | 28 1".to_string(),
            "1: \"a\"".to_string(),
            "11: 42 31".to_string(),
            "5: 1 14 | 15 1".to_string(),
            "19: 14 1 | 14 14".to_string(),
            "12: 24 14 | 19 1".to_string(),
            "16: 15 1 | 14 14".to_string(),
            "31: 14 17 | 1 13".to_string(),
            "6: 14 14 | 1 14".to_string(),
            "2: 1 24 | 14 4".to_string(),
            "0: 8 11".to_string(),
            "13: 14 3 | 1 12".to_string(),
            "15: 1 | 14".to_string(),
            "17: 14 2 | 1 7".to_string(),
            "23: 25 1 | 22 14".to_string(),
            "28: 16 1".to_string(),
            "4: 1 1".to_string(),
            "20: 14 14 | 1 15".to_string(),
            "3: 5 14 | 16 1".to_string(),
            "27: 1 6 | 14 18".to_string(),
            "14: \"b\"".to_string(),
            "21: 14 1 | 1 14".to_string(),
            "25: 1 1 | 1 14".to_string(),
            "22: 14 14".to_string(),
            "8: 42".to_string(),
            "26: 14 22 | 1 20".to_string(),
            "18: 15 15".to_string(),
            "7: 14 5 | 1 21".to_string(),
            "24: 14 1".to_string(),
            "".to_string(),
            "abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa".to_string(),
            "bbabbbbaabaabba".to_string(),
            "babbbbaabbbbbabbbbbbaabaaabaaa".to_string(),
            "aaabbbbbbaaaabaababaabababbabaaabbababababaaa".to_string(),
            "bbbbbbbaaaabbbbaaabbabaaa".to_string(),
            "bbbababbbbaaaaaaaabbababaaababaabab".to_string(),
            "ababaaaaaabaaab".to_string(),
            "ababaaaaabbbaba".to_string(),
            "baabbaaaabbaaaababbaababb".to_string(),
            "abbbbabbbbaaaababbbbbbaaaababb".to_string(),
            "aaaaabbaabaaaaababaa".to_string(),
            "aaaabbaaaabbaaa".to_string(),
            "aaaabbaabbaaaaaaabbbabbbaaabbaabaaa".to_string(),
            "babaaabbbaaabaababbaabababaaab".to_string(),
            "aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba".to_string(),
        ]);

        let reg1 = generate_regex(0, &ex1.0, false);
        //let reg2_no_replacement = generate_regex(0, &ex2.0, false);
        let reg2_with_replacement = generate_regex(0, &ex2.0, true);

        assert_eq!(validate_messages(reg1, &ex1.1), 2);
        //assert_eq!(validate_messages(reg2_no_replacement, &ex2.1), 3);
        assert_eq!(validate_messages(reg2_with_replacement, &ex2.1), 12);
    }
}
