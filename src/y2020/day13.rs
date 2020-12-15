use crate::solver::Solver;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub struct Timetable {
    earliest_departure: usize,
    departures: Vec<usize>,
    departures_with_index: HashMap<usize, usize>,
}

pub struct Problem;

impl Solver for Problem {
    type Input = Timetable;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input(&self, file: File) -> Timetable {
        let buf_reader = BufReader::new(file);
        let mut lines = buf_reader.lines();
        let earliest_departure = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
        let mut departures = Vec::new();
        let mut departures_with_index = HashMap::new();
        let departure_line = lines.next().unwrap().unwrap();
        let split = departure_line.split(',').collect::<Vec<&str>>();

        for (i, d) in split.iter().enumerate() {
            if *d == "x" {
                continue;
            }

            let departure = d.to_string().parse::<usize>().unwrap();
            departures.push(departure);
            departures_with_index.insert(departure, i);
        }

        Timetable {
            earliest_departure: earliest_departure,
            departures: departures,
            departures_with_index: departures_with_index,
        }
    }

    fn solve_part_one(&self, input: &Timetable) -> usize {
        find_departure(input)
    }

    fn solve_part_two(&self, input: &Timetable) -> usize {
        win_contest(input)
    }
}

fn find_departure(timetable: &Timetable) -> usize {
    let mut iterations = 0;
    loop {
        let bus = timetable
            .departures
            .iter()
            .find(|d| iterations * **d >= timetable.earliest_departure);

        match bus {
            Some(b) => {
                let wait = (iterations * *b) - timetable.earliest_departure;
                return *b * wait;
            }
            None => (),
        };

        iterations = iterations + 1;
    }
}

fn win_contest(timetable: &Timetable) -> usize {
    let mut t = 0;
    let mut add = 1;
    let mut iterated = Vec::new();
    for departure in &timetable.departures {
        iterated.push(*departure);
        loop {
            if iterated
                .iter()
                .all(|i| (t + timetable.departures_with_index[i]) % i == 0)
            {
                break;
            }
            t = t + add;
        }
        add = iterated.iter().product();
    }

    t
}

#[cfg(test)]
mod tests {
    use crate::y2020::day13::*;

    #[test]
    fn test_find_departure() {
        let timetable = Timetable {
            earliest_departure: 939,
            departures: vec![7, 13, 59, 31, 19],
            departures_with_index: HashMap::new(),
        };

        assert_eq!(find_departure(&timetable), 295);
    }

    #[test]
    fn test_win_contest() {
        let mut ex1_index = HashMap::new();
        ex1_index.insert(7, 0);
        ex1_index.insert(13, 1);
        ex1_index.insert(59, 4);
        ex1_index.insert(31, 6);
        ex1_index.insert(19, 7);
        let timetable_ex1 = Timetable {
            earliest_departure: 0,
            departures: vec![7, 13, 59, 31, 19],
            departures_with_index: ex1_index,
        };

        let mut ex2_index = HashMap::new();
        ex2_index.insert(17, 0);
        ex2_index.insert(13, 2);
        ex2_index.insert(19, 3);
        let timetable_ex2 = Timetable {
            earliest_departure: 0,
            departures: vec![17, 13, 19],
            departures_with_index: ex2_index,
        };

        let mut ex3_index = HashMap::new();
        ex3_index.insert(67, 0);
        ex3_index.insert(7, 1);
        ex3_index.insert(59, 2);
        ex3_index.insert(61, 3);
        let timetable_ex3 = Timetable {
            earliest_departure: 0,
            departures: vec![67, 7, 59, 61],
            departures_with_index: ex3_index,
        };

        let mut ex4_index = HashMap::new();
        ex4_index.insert(67, 0);
        ex4_index.insert(7, 2);
        ex4_index.insert(59, 3);
        ex4_index.insert(61, 4);
        let timetable_ex4 = Timetable {
            earliest_departure: 0,
            departures: vec![67, 7, 59, 61],
            departures_with_index: ex4_index,
        };

        let mut ex5_index = HashMap::new();
        ex5_index.insert(67, 0);
        ex5_index.insert(7, 1);
        ex5_index.insert(59, 3);
        ex5_index.insert(61, 4);
        let timetable_ex5 = Timetable {
            earliest_departure: 0,
            departures: vec![67, 7, 59, 61],
            departures_with_index: ex5_index,
        };

        let mut ex6_index = HashMap::new();
        ex6_index.insert(1789, 0);
        ex6_index.insert(37, 1);
        ex6_index.insert(47, 2);
        ex6_index.insert(1889, 3);
        let timetable_ex6 = Timetable {
            earliest_departure: 0,
            departures: vec![1789, 37, 47, 1889],
            departures_with_index: ex6_index,
        };

        assert_eq!(win_contest(&timetable_ex1), 1068781);
        assert_eq!(win_contest(&timetable_ex2), 3417);
        assert_eq!(win_contest(&timetable_ex3), 754018);
        assert_eq!(win_contest(&timetable_ex4), 779210);
        assert_eq!(win_contest(&timetable_ex5), 1261476);
        assert_eq!(win_contest(&timetable_ex6), 1202161486);
    }
}
