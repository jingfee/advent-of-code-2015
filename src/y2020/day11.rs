use crate::solver::Solver;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<Vec<char>>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input(&self, file: File) -> Vec<Vec<char>> {
        let buf_reader = BufReader::new(file);
        buf_reader
            .lines()
            .map(|l| l.unwrap().chars().collect())
            .collect()
    }

    fn solve_part_one(&self, input: &Vec<Vec<char>>) -> usize {
        stabilize_seating(&mut input.to_vec(), true, 4)
    }

    fn solve_part_two(&self, input: &Vec<Vec<char>>) -> usize {
        stabilize_seating(&mut input.to_vec(), false, 5)
    }
}

fn stabilize_seating(
    seating: &mut Vec<Vec<char>>,
    immediate_adjacent: bool,
    adjacent_people_to_leave: usize,
) -> usize {
    loop {
        let mut state_changed = false;
        let mut new_seating = Vec::new();
        for (i, row) in seating.iter().enumerate() {
            new_seating.push(Vec::new());
            for (j, place) in row.iter().enumerate() {
                let adjacent_seats = if immediate_adjacent {
                    get_immediate_adjacent_seats(i, j, seating.len(), seating[i].len())
                } else {
                    get_closest_adjacent_seats(i, j, seating)
                };

                if *place == 'L' && should_occupy_seat(seating, &adjacent_seats) {
                    new_seating[i].push('#');
                    state_changed = true;
                } else if *place == '#'
                    && should_leave_seat(seating, adjacent_people_to_leave, &adjacent_seats)
                {
                    new_seating[i].push('L');
                    state_changed = true;
                } else {
                    new_seating[i].push(seating[i][j]);
                }
            }
        }
        *seating = new_seating;

        if !state_changed {
            break;
        }
    }

    seating
        .iter()
        .map(|row| row.iter().filter(|s| **s == '#').count())
        .sum()
}

fn should_occupy_seat(seating: &Vec<Vec<char>>, adjacent_seats: &Vec<(usize, usize)>) -> bool {
    for adjacent_seat in adjacent_seats {
        if seating[adjacent_seat.0][adjacent_seat.1] == '#' {
            return false;
        }
    }

    true
}

fn should_leave_seat(
    seating: &Vec<Vec<char>>,
    adjacent_people_to_leave: usize,
    adjacent_seats: &Vec<(usize, usize)>,
) -> bool {
    let mut adjacent_occupied = 0;
    for adjacent_seat in adjacent_seats {
        if seating[adjacent_seat.0][adjacent_seat.1] == '#' {
            adjacent_occupied = adjacent_occupied + 1;
        }
    }

    adjacent_occupied >= adjacent_people_to_leave
}

fn get_immediate_adjacent_seats(
    i: usize,
    j: usize,
    max_i: usize,
    max_j: usize,
) -> Vec<(usize, usize)> {
    let mut adjacent_seats = Vec::new();
    for i_diff in -1..=1 {
        for j_diff in -1..=1 {
            if i_diff == 0 && j_diff == 0 {
                continue;
            }

            let i_check = i as isize + i_diff;
            let j_check = j as isize + j_diff;

            if i_check < 0 || j_check < 0 || i_check as usize >= max_i || j_check as usize >= max_j
            {
                continue;
            }

            adjacent_seats.push((i_check as usize, j_check as usize));
        }
    }

    adjacent_seats
}

fn get_closest_adjacent_seats(i: usize, j: usize, seating: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut adjacent_seats = Vec::new();

    for i_diff in -1..=1 {
        for j_diff in -1..=1 {
            if i_diff == 0 && j_diff == 0 {
                continue;
            }
            let mut mult = 1;
            loop {
                let i_check = i as isize + i_diff * mult;
                let j_check = j as isize + j_diff * mult;

                if i_check < 0
                    || j_check < 0
                    || i_check as usize >= seating.len()
                    || j_check as usize >= seating[i_check as usize].len()
                {
                    break;
                }

                if seating[i_check as usize][j_check as usize] != '.' {
                    adjacent_seats.push((i_check as usize, j_check as usize));
                    break;
                }

                mult = mult + 1;
            }
        }
    }

    adjacent_seats
}

#[cfg(test)]
mod tests {
    use crate::y2020::day11::*;

    #[test]
    fn test_stabilize_seating_part_1() {
        let mut ex = vec![
            "L.LL.LL.LL".chars().collect(),
            "LLLLLLL.LL".chars().collect(),
            "L.L.L..L..".chars().collect(),
            "LLLL.LL.LL".chars().collect(),
            "L.LL.LL.LL".chars().collect(),
            "L.LLLLL.LL".chars().collect(),
            "..L.L.....".chars().collect(),
            "LLLLLLLLLL".chars().collect(),
            "L.LLLLLL.L".chars().collect(),
            "L.LLLLL.LL".chars().collect(),
        ];

        assert_eq!(stabilize_seating(&mut ex, true, 4), 37);
    }

    #[test]
    fn test_stabilize_seating_part_2() {
        let mut ex = vec![
            "L.LL.LL.LL".chars().collect(),
            "LLLLLLL.LL".chars().collect(),
            "L.L.L..L..".chars().collect(),
            "LLLL.LL.LL".chars().collect(),
            "L.LL.LL.LL".chars().collect(),
            "L.LLLLL.LL".chars().collect(),
            "..L.L.....".chars().collect(),
            "LLLLLLLLLL".chars().collect(),
            "L.LLLLLL.L".chars().collect(),
            "L.LLLLL.LL".chars().collect(),
        ];

        assert_eq!(stabilize_seating(&mut ex, false, 5), 26);
    }
}
