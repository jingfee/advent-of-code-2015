use crate::solver::Solver;
use md5::{Digest, Md5};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub struct Problem;

impl Solver for Problem {
    type Input = String;
    type Output1 = u64;
    type Output2 = u64;

    fn parse_input(&self, file: File) -> String {
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents).unwrap();
        contents
    }

    fn solve_part_one(&self, input: &mut String) -> u64 {
        get_lowest_number(input, "00000")
    }

    fn solve_part_two(&self, input: &mut String) -> u64 {
        get_lowest_number(input, "000000")
    }
}

fn get_lowest_number(secret: &str, start_pattern: &str) -> u64 {
    let mut number = 1;
    let mut hasher = Md5::new();

    loop {
        hasher.update(format!("{}{}", secret, number));
        let hex = format!("{:x}", hasher.finalize_reset());

        if hex.starts_with(start_pattern) {
            break;
        }

        number = number + 1;
    }

    number
}

#[cfg(test)]
mod tests {
    use crate::y2015::day04::*;

    #[test]
    fn test_lowest_number() {
        let ex1 = "abcdef";
        let ex2 = "pqrstuv";

        assert_eq!(get_lowest_number(&ex1, "00000"), 609043);
        assert_eq!(get_lowest_number(&ex2, "00000"), 1048970);
    }
}
