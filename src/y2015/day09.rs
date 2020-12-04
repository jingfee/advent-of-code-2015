use crate::solver::Solver;
use itertools::Itertools;
use std::io::prelude::*;
use std::io::BufReader;
use std::{collections::HashMap, collections::HashSet, fs::File};

pub struct Problem;

impl Solver for Problem {
    type Input = (HashSet<String>, HashMap<String, usize>);
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input(&self, file: File) -> (HashSet<String>, HashMap<String, usize>) {
        let mut locations = HashSet::new();
        let mut routes = HashMap::new();
        let buf_reader = BufReader::new(file);
        buf_reader.lines().for_each(|l| {
            let line = l.unwrap();
            let line_split: Vec<&str> = line.split(' ').collect();
            let from_location = line_split[0];
            let to_location = line_split[2];
            let distance = line_split[4].parse::<usize>().unwrap();

            locations.insert(from_location.to_string());
            locations.insert(to_location.to_string());
            routes.insert(
                format!("{}{}", from_location.to_string(), to_location.to_string()),
                distance,
            );
            routes.insert(
                format!("{}{}", to_location.to_string(), from_location.to_string()),
                distance,
            );
        });

        (locations, routes)
    }

    fn solve_part_one(&self, input: &mut (HashSet<String>, HashMap<String, usize>)) -> usize {
        fastest_routes(&input)
    }

    fn solve_part_two(&self, input: &mut (HashSet<String>, HashMap<String, usize>)) -> usize {
        longest_routes(&input)
    }
}

fn fastest_routes(input: &(HashSet<String>, HashMap<String, usize>)) -> usize {
    let mut fastest_route = usize::MAX;
    let combinations = input.0.iter().permutations(input.0.len());

    for combination in combinations {
        let mut current_route = 0;
        for (i, _location) in combination.iter().enumerate() {
            if i == combination.len() - 1 {
                break;
            }

            current_route =
                current_route + input.1[&format!("{}{}", combination[i], combination[i + 1])];
        }

        if current_route < fastest_route {
            fastest_route = current_route;
        }
    }

    fastest_route
}

fn longest_routes(input: &(HashSet<String>, HashMap<String, usize>)) -> usize {
    let mut longest_route = 0;
    let combinations = input.0.iter().permutations(input.0.len());

    for combination in combinations {
        let mut current_route = 0;
        for (i, _location) in combination.iter().enumerate() {
            if i == combination.len() - 1 {
                break;
            }

            current_route =
                current_route + input.1[&format!("{}{}", combination[i], combination[i + 1])];
        }

        if current_route > longest_route {
            longest_route = current_route;
        }
    }

    longest_route
}

#[cfg(test)]
mod tests {
    use crate::y2015::day09::*;

    #[test]
    fn test_fastest_routes() {
        let mut locations = HashSet::new();
        let mut routes: HashMap<String, usize> = HashMap::new();

        locations.insert("London".to_string());
        locations.insert("Dublin".to_string());
        locations.insert("Belfast".to_string());

        routes.insert("LondonDublin".to_string(), 464);
        routes.insert("DublinLondon".to_string(), 464);
        routes.insert("LondonBelfast".to_string(), 518);
        routes.insert("BelfastLondon".to_string(), 518);
        routes.insert("DublinBelfast".to_string(), 141);
        routes.insert("BelfastDublin".to_string(), 141);

        assert_eq!(fastest_routes(&(locations, routes)), 605);
    }

    #[test]
    fn test_longest_routes() {
        let mut locations = HashSet::new();
        let mut routes: HashMap<String, usize> = HashMap::new();

        locations.insert("London".to_string());
        locations.insert("Dublin".to_string());
        locations.insert("Belfast".to_string());

        routes.insert("LondonDublin".to_string(), 464);
        routes.insert("DublinLondon".to_string(), 464);
        routes.insert("LondonBelfast".to_string(), 518);
        routes.insert("BelfastLondon".to_string(), 518);
        routes.insert("DublinBelfast".to_string(), 141);
        routes.insert("BelfastDublin".to_string(), 141);

        assert_eq!(longest_routes(&(locations, routes)), 982);
    }
}
