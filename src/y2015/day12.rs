use crate::solver::Solver;
use std::io::prelude::*;
use std::io::BufReader;
use std::{fs::File, str::Bytes};

pub struct Problem;

impl Solver for Problem {
    type Input = String;
    type Output1 = isize;
    type Output2 = isize;

    fn parse_input(&self, file: File) -> String {
        let buf_reader = BufReader::new(file);
        buf_reader.lines().nth(0).unwrap().unwrap()
    }

    fn solve_part_one(&self, input: &mut String) -> isize {
        add_numbers(&input)
    }

    fn solve_part_two(&self, input: &mut String) -> isize {
        add_numbers_check_red(&input)
    }
}

fn add_numbers(json: &String) -> isize {
    let mut sum = 0;

    let mut bytes = json.bytes();
    loop {
        match bytes.next() {
            Some(b) => {
                if b == b'-' {
                    sum = sum - get_number(&mut bytes, &mut String::new()).0;
                }

                if b >= b'1' && b <= b'9' {
                    sum = sum + get_number(&mut bytes, &mut String::from(b as char)).0;
                }
            }
            None => break,
        }
    }

    sum
}

fn add_numbers_check_red(json: &String) -> isize {
    let mut sum = 0;

    let mut bytes = json.bytes();
    loop {
        match bytes.next() {
            Some(b) => {
                // new object
                if b == b'{' {
                    sum = sum + get_object_value(&mut bytes);
                }

                if b == b'-' {
                    sum = sum - get_number(&mut bytes, &mut String::new()).0;
                }

                if b >= b'1' && b <= b'9' {
                    sum = sum + get_number(&mut bytes, &mut String::from(b as char)).0;
                }
            }
            None => break,
        }
    }

    sum
}

fn get_object_value(bytes: &mut Bytes) -> isize {
    let mut sum = 0;
    let mut red = false;
    let mut object_string = String::new();
    loop {
        let b = bytes.next().unwrap();
        object_string.push(b as char);
        // check red
        if object_string.contains(":\"red\"") {
            red = true;
        }

        // new object
        if b == b'{' {
            sum = sum + get_object_value(&mut *bytes);
        }
        // end object
        else if b == b'}' {
            break;
        } else if b == b'-' {
            let number = get_number(&mut *bytes, &mut String::new());
            sum = sum - number.0;
            if number.1 {
                break;
            }
        } else if b >= b'1' && b <= b'9' {
            let number = get_number(&mut *bytes, &mut String::from(b as char));
            sum = sum + number.0;
            if number.1 {
                break;
            }
        }
    }
    if red {
        0
    } else {
        sum
    }
}

fn get_number(bytes: &mut Bytes, number_string: &mut String) -> (isize, bool) {
    let mut end_of_object = false;
    loop {
        let next = bytes.next().unwrap();
        if next < b'0' || next > b'9' {
            if next == b'}' {
                end_of_object = true;
            }
            break;
        }
        number_string.push(next as char);
    }
    (number_string.parse::<isize>().unwrap(), end_of_object)
}

#[cfg(test)]
mod tests {
    use crate::y2015::day12::*;

    #[test]
    fn test_add_numbers() {
        assert_eq!(add_numbers(&"[1,2,3]".to_string()), 6);
        assert_eq!(add_numbers(&"{\"a\":2,\"b\":4}".to_string()), 6);
        assert_eq!(add_numbers(&"[[[3]]]".to_string()), 3);
        assert_eq!(add_numbers(&"{\"a\":{\"b\":4},\"c\":-1}".to_string()), 3);
        assert_eq!(add_numbers(&"{\"a\":[-1,1]}".to_string()), 0);
        assert_eq!(add_numbers(&"[-1,{\"a\":1}]".to_string()), 0);
        assert_eq!(add_numbers(&"[]".to_string()), 0);
        assert_eq!(add_numbers(&"{}".to_string()), 0);
    }

    #[test]
    fn test_add_numbers_check_red() {
        assert_eq!(add_numbers_check_red(&"[1,2,3]".to_string()), 6);
        assert_eq!(
            add_numbers_check_red(&"[1,{\"c\":\"red\",\"b\":2},3]".to_string()),
            4
        );
        assert_eq!(
            add_numbers_check_red(&"{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}".to_string()),
            0
        );
        assert_eq!(add_numbers_check_red(&"[1,\"red\",5]".to_string()), 6);
        assert_eq!(
            add_numbers_check_red(
                &"[1,{{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5},\"b\":2},3]".to_string()
            ),
            6
        )
    }
}
