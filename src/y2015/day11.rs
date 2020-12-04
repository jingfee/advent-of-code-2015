use crate::solver::Solver;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub struct Problem;

impl Solver for Problem {
    type Input = String;
    type Output1 = String;
    type Output2 = String;

    fn parse_input(&self, file: File) -> String {
        let buf_reader = BufReader::new(file);
        buf_reader.lines().nth(0).unwrap().unwrap()
    }

    fn solve_part_one(&self, input: &mut String) -> String {
        get_next_password(input)
    }

    fn solve_part_two(&self, input: &mut String) -> String {
        get_next_password(&get_next_password(input)[..])
    }
}

fn get_next_password(password: &str) -> String {
    let mut bytes = password.to_string().into_bytes();
    let mut index = bytes.len() - 1;

    loop {
        let c = bytes[index];

        if c == 104 || c == 107 || c == 110 {
            bytes[index] = bytes[index] + 2;
        } else {
            bytes[index] = bytes[index] + 1;
        }

        if bytes[index] > 122 {
            bytes[index] = 97;
            index = index - 1;
            continue;
        }

        let password: String = bytes.iter().map(|b| *b as char).into_iter().collect();

        if is_password_valid(&password) {
            return password;
        } else {
            index = bytes.len() - 1;
            continue;
        }
    }
}

fn is_password_valid(password: &str) -> bool {
    let mut increasing_straight = false;
    let mut forbidden_chars = false;
    let mut pairs = 0;

    let bytes = password.to_string().into_bytes();

    for (i, c) in bytes.iter().enumerate() {
        if c == &105 || c == &108 || c == &111 {
            forbidden_chars = true;
        }

        if i >= password.len() - 1 {
            continue;
        }

        let c1 = bytes[i + 1];

        if pairs < 2 && c == &c1 && (i == 0 || c != &bytes[i - 1]) {
            pairs = pairs + 1;
        }

        if i >= password.len() - 2 {
            continue;
        }

        if !increasing_straight && c + 1 == c1 && c + 2 == bytes[i + 2] {
            increasing_straight = true;
        }
    }

    increasing_straight && !forbidden_chars && pairs == 2
}

#[cfg(test)]
mod tests {
    use crate::y2015::day11::*;

    #[test]
    fn test_is_password_valid() {
        assert_eq!(is_password_valid("hijklmmn"), false);
        assert_eq!(is_password_valid("abbceffg"), false);
        assert_eq!(is_password_valid("abbcegjk"), false);
        assert_eq!(is_password_valid("abcdffaa"), true);
        assert_eq!(is_password_valid("ghjaabcc"), true);
    }

    #[test]
    fn test_get_next_password() {
        assert_eq!(get_next_password("abcdefgh"), "abcdffaa");
        assert_eq!(get_next_password("ghijklmn"), "ghjaabcc");
    }
}
