use crate::solver::Solver;
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

    fn solve_part_one(&self, input: &mut Vec<String>) -> usize {
        input.iter().map(|s| seat_id(s)).max().unwrap()
    }

    fn solve_part_two(&self, input: &mut Vec<String>) -> usize {
        let mut seat_ids: Vec<usize> = input.iter().map(|s| seat_id(s)).collect();
        seat_ids.sort();

        let first = *seat_ids.first().unwrap();
        let last = *seat_ids.last().unwrap();

        let mut my_seat = None;
        for seat in first..=last {
            if !seat_ids.contains(&seat) {
                my_seat = Some(seat);
                break;
            }
        }
        my_seat.unwrap()
    }
}

fn seat_id(seat_binary: &String) -> usize {
    let mut row_lower_bound = 0;
    let mut row_upper_bound = 127;
    let mut col_lower_bound = 0;
    let mut col_upper_bound = 7;
    for char in seat_binary.chars() {
        match char {
            'F' => {
                row_upper_bound = row_upper_bound - ((row_upper_bound - row_lower_bound + 1) / 2)
            }
            'B' => {
                row_lower_bound = row_lower_bound + ((row_upper_bound - row_lower_bound + 1) / 2)
            }
            'L' => {
                col_upper_bound = col_upper_bound - ((col_upper_bound - col_lower_bound + 1) / 2)
            }
            'R' => {
                col_lower_bound = col_lower_bound + ((col_upper_bound - col_lower_bound + 1) / 2)
            }
            _ => panic!("Unexpected char in row"),
        }
    }

    assert_eq!(row_lower_bound, row_upper_bound);
    assert_eq!(col_lower_bound, col_upper_bound);

    row_lower_bound * 8 + col_lower_bound
}

#[cfg(test)]
mod tests {
    use crate::y2020::day05::*;

    #[test]
    fn test_seat_id() {
        assert_eq!(seat_id(&"FBFBBFFRLR".to_string()), 357);
        assert_eq!(seat_id(&"BFFFBBFRRR".to_string()), 567);
        assert_eq!(seat_id(&"FFFBBBFRRR".to_string()), 119);
        assert_eq!(seat_id(&"BBFFBBFRLL".to_string()), 820);
    }
}
