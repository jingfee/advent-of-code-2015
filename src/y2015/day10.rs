use crate::solver::Solver;
use std::io::prelude::*;
use std::io::BufReader;
use std::{fs::File, str::Chars};

pub struct Problem;

impl Solver for Problem {
    type Input = String;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input(&self, file: File) -> String {
        let buf_reader = BufReader::new(file);
        buf_reader.lines().nth(0).unwrap().unwrap()
    }

    fn solve_part_one(&self, input: &mut String) -> usize {
        let mut output = input.to_string();
        for _ in 0..40 {
            output = look_and_say(&output);
        }

        output.len()
    }

    fn solve_part_two(&self, input: &mut String) -> usize {
        let mut output = input.to_string();
        for _ in 0..50 {
            output = look_and_say(&output);
        }

        output.len()
    }
}

fn look_and_say(input: &str) -> String {
    let mut new_string = String::new();

    let mut chars = input.chars();
    let mut c = chars.next().unwrap();
    loop {
        let next = look_and_say_char(c, &mut new_string, &mut chars);
        match next {
            Some(next_c) => c = next_c,
            None => break,
        }
    }
    new_string
}

fn look_and_say_char(c: char, new_string: &mut String, chars: &mut Chars) -> Option<char> {
    let mut count = 1;
    loop {
        let look_ahead_next = chars.next();
        match look_ahead_next {
            Some(look_ahead) => {
                if look_ahead == c {
                    count = count + 1;
                } else {
                    new_string.push_str(&format!("{}{}", count, c));
                    return look_ahead_next;
                }
            }
            None => {
                new_string.push_str(&format!("{}{}", count, c));
                return None;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::y2015::day10::*;

    #[test]
    fn test_look_and_say() {
        assert_eq!(look_and_say("1"), "11");
        assert_eq!(look_and_say("11"), "21");
        assert_eq!(look_and_say("21"), "1211");
        assert_eq!(look_and_say("1211"), "111221");
        assert_eq!(look_and_say("111221"), "312211");
    }
}
