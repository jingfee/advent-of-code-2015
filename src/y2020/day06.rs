use crate::solver::Solver;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<Vec<String>>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input(&self, file: File) -> Vec<Vec<String>> {
        let buf_reader = BufReader::new(file);
        let lines: Vec<String> = buf_reader.lines().map(|l| l.unwrap()).collect();
        get_groups(lines)
    }

    fn solve_part_one(&self, input: &mut Vec<Vec<String>>) -> usize {
        input.iter().map(|g| count_answers(g)).sum()
    }

    fn solve_part_two(&self, input: &mut Vec<Vec<String>>) -> usize {
        input.iter().map(|g| count_answers_everyone(g)).sum()
    }
}

fn get_groups(lines: Vec<String>) -> Vec<Vec<String>> {
    let mut groups: Vec<Vec<String>> = Vec::new();
    let mut index = 0;
    loop {
        if index == lines.len() {
            break;
        }

        let mut group = Vec::new();
        loop {
            if index == lines.len() {
                break;
            }

            let line = &lines[index];
            index = index + 1;
            if line == "" {
                break;
            }

            group.push(line.to_string());
        }
        groups.push(group);
    }

    groups
}

fn count_answers(group: &Vec<String>) -> usize {
    let mut answers = HashSet::new();

    for person in group {
        for answer in person.chars() {
            answers.insert(answer);
        }
    }

    answers.len()
}

fn count_answers_everyone(group: &Vec<String>) -> usize {
    let mut answers = HashMap::new();

    for person in group {
        for answer in person.chars() {
            if answers.contains_key(&answer) {
                answers.insert(answer, answers[&answer] + 1);
            } else {
                answers.insert(answer, 1);
            }
        }
    }

    let mut answers_everyone = 0;
    for (_k, v) in answers.drain() {
        if v == group.len() {
            answers_everyone = answers_everyone + 1;
        }
    }

    answers_everyone
}

#[cfg(test)]
mod tests {
    use crate::y2020::day06::*;

    #[test]
    fn test_count_answers() {
        let groups = get_groups(vec![
            "abc".to_string(),
            "".to_string(),
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "".to_string(),
            "ab".to_string(),
            "ac".to_string(),
            "".to_string(),
            "a".to_string(),
            "a".to_string(),
            "a".to_string(),
            "a".to_string(),
            "".to_string(),
            "b".to_string(),
        ]);

        assert_eq!(count_answers(&groups[0]), 3);
        assert_eq!(count_answers(&groups[1]), 3);
        assert_eq!(count_answers(&groups[2]), 3);
        assert_eq!(count_answers(&groups[3]), 1);
        assert_eq!(count_answers(&groups[4]), 1);
    }

    #[test]
    fn test_count_answers_everyone() {
        let groups = get_groups(vec![
            "abc".to_string(),
            "".to_string(),
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "".to_string(),
            "ab".to_string(),
            "ac".to_string(),
            "".to_string(),
            "a".to_string(),
            "a".to_string(),
            "a".to_string(),
            "a".to_string(),
            "".to_string(),
            "b".to_string(),
        ]);

        assert_eq!(count_answers_everyone(&groups[0]), 3);
        assert_eq!(count_answers_everyone(&groups[1]), 0);
        assert_eq!(count_answers_everyone(&groups[2]), 1);
        assert_eq!(count_answers_everyone(&groups[3]), 1);
        assert_eq!(count_answers_everyone(&groups[4]), 1);
    }
}
