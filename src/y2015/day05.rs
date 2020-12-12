use crate::solver::Solver;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<String>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input(&self, file: File) -> Vec<String> {
        let buf_reader = BufReader::new(file);
        buf_reader.lines().map(|l| l.unwrap()).collect()
    }

    fn solve_part_one(&self, input: &Vec<String>) -> usize {
        input.iter().map(|w| is_word_nice(w)).filter(|w| *w).count()
    }

    fn solve_part_two(&self, input: &Vec<String>) -> usize {
        input
            .iter()
            .map(|w| new_is_word_nice(w))
            .filter(|w| *w)
            .count()
    }
}

fn is_word_nice(word: &str) -> bool {
    let vowels = "aeiou";
    let forbidden_sequence = ["ab", "cd", "pq", "xy"];
    let mut num_vowels = 0;
    let mut has_twice_in_row = false;

    for s in forbidden_sequence.iter() {
        if word.contains(s) {
            return false;
        }
    }

    for (i, c) in word.to_string().chars().enumerate() {
        if vowels.contains(c) {
            num_vowels = num_vowels + 1;
        }
        if i > 0 && word.to_string().chars().nth(i - 1).unwrap() == c {
            has_twice_in_row = true;
        }
    }

    num_vowels >= 3 && has_twice_in_row
}

fn new_is_word_nice(word: &str) -> bool {
    let mut pairs = HashSet::new();
    let mut has_pairs_twice = false;
    let mut has_sandwich_letter = false;
    let mut prev_chars = ['0', '0'];

    for (i, c) in word.to_string().chars().enumerate() {
        if i > 0 {
            if i > 1 {
                if c == prev_chars[1] {
                    has_sandwich_letter = true;
                }

                if c == prev_chars[0] && c == prev_chars[1] {
                    continue;
                }
            }

            let pair = format!("{}{}", prev_chars[0], c);
            if pairs.contains(&pair) {
                has_pairs_twice = true;
            }

            pairs.insert(pair);
        }

        prev_chars[1] = prev_chars[0];
        prev_chars[0] = c;
    }

    has_pairs_twice && has_sandwich_letter
}

#[cfg(test)]
mod tests {
    use crate::y2015::day05::*;

    #[test]
    fn test_word_is_nice() {
        let ex1 = "ugknbfddgicrmopn";
        let ex2 = "aaa";
        let ex3 = "jchzalrnumimnmhp";
        let ex4 = "haegwjzuvuyypxyu";
        let ex5 = "dvszwmarrgswjxmb";

        assert_eq!(is_word_nice(ex1), true);
        assert_eq!(is_word_nice(ex2), true);
        assert_eq!(is_word_nice(ex3), false);
        assert_eq!(is_word_nice(ex4), false);
        assert_eq!(is_word_nice(ex5), false);
    }

    #[test]
    fn test_new_word_is_nice() {
        let ex1 = "qjhvhtzxzqqjkmpb";
        let ex2 = "xxyxx";
        let ex3 = "uurcxstgmygtbstg";
        let ex4 = "ieodomkazucvgmuy";

        assert_eq!(new_is_word_nice(ex1), true);
        assert_eq!(new_is_word_nice(ex2), true);
        assert_eq!(new_is_word_nice(ex3), false);
        assert_eq!(new_is_word_nice(ex4), false);
    }
}
