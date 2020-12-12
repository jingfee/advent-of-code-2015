use crate::solver::Solver;
use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub struct Guest {
    name: String,
    neighbours: HashMap<String, isize>,
}

impl Clone for Guest {
    fn clone(&self) -> Guest {
        Guest {
            name: self.name.to_string(),
            neighbours: self.neighbours.clone(),
        }
    }
}

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<Guest>;
    type Output1 = isize;
    type Output2 = isize;

    fn parse_input(&self, file: File) -> Vec<Guest> {
        let mut guests: Vec<Guest> = Vec::new();
        let buf_reader = BufReader::new(file);
        buf_reader.lines().for_each(|l| {
            let line = l.unwrap();
            let line_split: Vec<&str> = line.split(' ').collect();
            let guest = line_split[0];

            if !guests.iter().any(|g| g.name == guest) {
                guests.push(Guest {
                    name: guest.to_string(),
                    neighbours: HashMap::new(),
                });
            }

            let gain = if line_split[2] == "gain" { true } else { false };
            let happiness = line_split[3].parse::<isize>().unwrap();
            let neighbour = line_split[10].trim_end_matches(".");

            let neighbours = &mut guests
                .iter_mut()
                .find(|g| g.name == guest)
                .unwrap()
                .neighbours;

            neighbours.insert(
                neighbour.to_string(),
                if gain { happiness } else { -1 * happiness },
            );
        });

        guests
    }

    fn solve_part_one(&self, input: &Vec<Guest>) -> isize {
        find_best_pairings(input)
    }

    fn solve_part_two(&self, input: &Vec<Guest>) -> isize {
        add_self_to_list(&mut input.to_vec());
        find_best_pairings(input)
    }
}

fn find_best_pairings(guests: &Vec<Guest>) -> isize {
    let permutations = guests.iter().permutations(guests.len());
    let mut best_happiness = isize::MIN;
    for permutation in permutations {
        let mut happiness = 0;
        for (index, guest) in permutation.iter().enumerate() {
            let left_index = if index == 0 {
                permutation.len() - 1
            } else {
                index - 1
            };
            let right_index = if index == permutation.len() - 1 {
                0
            } else {
                index + 1
            };

            happiness = happiness
                + guest.neighbours[&permutation[left_index].name]
                + guest.neighbours[&permutation[right_index].name];
        }

        if happiness > best_happiness {
            best_happiness = happiness;
        }
    }

    best_happiness
}

fn add_self_to_list(input: &mut Vec<Guest>) {
    let mut self_neighbours: HashMap<String, isize> = HashMap::new();
    for guest in input.iter_mut() {
        guest.neighbours.insert("Self".to_string(), 0);
        self_neighbours.insert(guest.name.to_string(), 0);
    }
    let self_guest = Guest {
        name: "Self".to_string(),
        neighbours: self_neighbours,
    };

    input.push(self_guest);
}

#[cfg(test)]
mod tests {
    use crate::y2015::day13::*;

    #[test]
    fn test_find_best_pairings() {
        let mut guests = vec![
            Guest {
                name: "Alice".to_string(),
                neighbours: HashMap::new(),
            },
            Guest {
                name: "Bob".to_string(),
                neighbours: HashMap::new(),
            },
            Guest {
                name: "Carol".to_string(),
                neighbours: HashMap::new(),
            },
            Guest {
                name: "David".to_string(),
                neighbours: HashMap::new(),
            },
        ];

        guests[0].neighbours.insert("Bob".to_string(), 54);
        guests[0].neighbours.insert("Carol".to_string(), -79);
        guests[0].neighbours.insert("David".to_string(), -2);
        guests[1].neighbours.insert("Alice".to_string(), 83);
        guests[1].neighbours.insert("Carol".to_string(), -7);
        guests[1].neighbours.insert("David".to_string(), -63);
        guests[2].neighbours.insert("Alice".to_string(), -62);
        guests[2].neighbours.insert("Bob".to_string(), 60);
        guests[2].neighbours.insert("David".to_string(), 55);
        guests[3].neighbours.insert("Alice".to_string(), 46);
        guests[3].neighbours.insert("Bob".to_string(), -7);
        guests[3].neighbours.insert("Carol".to_string(), 41);

        assert_eq!(find_best_pairings(&guests), 330);
    }
}
