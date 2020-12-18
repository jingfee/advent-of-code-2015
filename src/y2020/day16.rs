use regex::Regex;

use crate::solver::Solver;
use itertools::Itertools;
use std::io::prelude::*;
use std::io::BufReader;
use std::{collections::HashMap, fs::File};

pub struct Field {
    name: String,
    ranges: Vec<(usize, usize)>,
}

pub struct Notes {
    fields: Vec<Field>,
    your_ticket: Vec<usize>,
    nearby_tickets: Vec<Vec<usize>>,
}

pub struct Problem;

impl Solver for Problem {
    type Input = Notes;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input(&self, file: File) -> Notes {
        let mut buf_reader = BufReader::new(file);
        let mut lines = String::new();
        buf_reader.read_to_string(&mut lines).unwrap();
        parse_lines(lines)
    }

    fn solve_part_one(&self, input: &Notes) -> usize {
        find_error_rate(input)
    }

    fn solve_part_two(&self, input: &Notes) -> usize {
        let mut value = 1;
        let ticket = map_your_ticket(input);
        for (k, v) in ticket {
            if k.starts_with("departure") {
                value = value * v;
            }
        }
        value
    }
}

fn parse_lines(lines: String) -> Notes {
    let re_fields = Regex::new(r"(?P<fields>([a-z\s]+: [0-9]+-[0-9]+ or [0-9]+-[0-9]+)+)").unwrap();
    let fields = re_fields
        .captures(&lines)
        .unwrap()
        .name("fields")
        .unwrap()
        .as_str()
        .split("\r\n")
        .filter(|l| *l != "")
        .map(|c| {
            let split = c.split(':').collect::<Vec<&str>>();
            let name = split[0].to_string();
            let ranges = split[1]
                .split(" ")
                .filter(|r| *r != "or" && *r != "")
                .map(|r| {
                    let range_split = r.split('-').collect::<Vec<&str>>();
                    return (
                        range_split[0].split('\n').collect::<Vec<&str>>()[0]
                            .parse::<usize>()
                            .unwrap(),
                        range_split[1].split('\n').collect::<Vec<&str>>()[0]
                            .parse::<usize>()
                            .unwrap(),
                    );
                })
                .collect();
            Field {
                name: name,
                ranges: ranges,
            }
        })
        .collect::<Vec<Field>>();

    let re_your_ticket = Regex::new(r"your ticket:\r\n(?P<your_ticket>([0-9]+,?)+)").unwrap();
    let your_ticket = re_your_ticket
        .captures(&lines)
        .unwrap()
        .name("your_ticket")
        .unwrap()
        .as_str()
        .split(",")
        .map(|t| t.parse::<usize>().unwrap())
        .collect();

    let re_nearby_tickets =
        Regex::new(r"nearby tickets:\r\n(?P<nearby_tickets>([0-9]+,?(\r\n)?)+)").unwrap();
    let nearby_tickets = re_nearby_tickets
        .captures(&lines)
        .unwrap()
        .name("nearby_tickets")
        .unwrap()
        .as_str()
        .split("\r\n")
        .map(|l| l.split(',').map(|t| t.parse::<usize>().unwrap()).collect())
        .collect();

    Notes {
        fields: fields,
        your_ticket: your_ticket,
        nearby_tickets: nearby_tickets,
    }
}

fn find_error_rate(notes: &Notes) -> usize {
    let mut error_rate = 0;

    for nearby_ticket in &notes.nearby_tickets {
        for num in nearby_ticket {
            if !notes
                .fields
                .iter()
                .any(|f| f.ranges.iter().any(|r| num >= &r.0 && num <= &r.1))
            {
                error_rate = error_rate + num;
            }
        }
    }

    error_rate
}

fn map_your_ticket(notes: &Notes) -> HashMap<String, usize> {
    let mut your_ticket = HashMap::new();
    let mut possible_fields = HashMap::new();

    let valid_nearby_tickets = notes
        .nearby_tickets
        .iter()
        .filter(|n| {
            n.iter().all(|t| {
                notes
                    .fields
                    .iter()
                    .any(|f| f.ranges.iter().any(|r| t >= &r.0 && t <= &r.1))
            })
        })
        .map(|x| x.to_vec())
        .collect::<Vec<Vec<usize>>>();

    for field in &notes.fields {
        possible_fields.insert(field.name.to_string(), Vec::new());
        for i in 0..notes.fields.len() {
            if valid_nearby_tickets
                .iter()
                .all(|n| field.ranges.iter().any(|r| n[i] >= r.0 && n[i] <= r.1))
            {
                possible_fields.get_mut(&field.name).unwrap().push(i);
            }
        }
    }

    let mut found_fields = Vec::new();
    for field in possible_fields
        .iter()
        .sorted_by(|a, b| Ord::cmp(&a.1.len(), &b.1.len()))
    {
        let found_field_index = field
            .1
            .iter()
            .filter(|f| !found_fields.contains(f))
            .nth(0)
            .unwrap();

        found_fields.push(found_field_index);

        your_ticket.insert(field.0.to_string(), notes.your_ticket[*found_field_index]);
    }

    your_ticket
}

#[cfg(test)]
mod tests {
    use crate::y2020::day16::*;

    #[test]
    fn test_find_error_rate() {
        let ex = parse_lines(
            "class: 1-3 or 5-7\r\n".to_string()
                + "row: 6-11 or 33-44\r\n"
                + "seat: 13-40 or 45-50\r\n"
                + "\r\n"
                + "your ticket:\r\n"
                + "7,1,14\r\n"
                + "\r\n"
                + "nearby tickets:\r\n"
                + "7,3,47\r\n"
                + "40,4,50\r\n"
                + "55,2,20\r\n"
                + "38,6,12",
        );

        assert_eq!(find_error_rate(&ex), 71);
    }

    #[test]
    fn test_map_your_ticket() {
        let ex = parse_lines(
            "class: 0-1 or 4-19\r\n".to_string()
                + "row: 0-5 or 8-19\r\n"
                + "seat: 0-13 or 16-19\r\n"
                + "\r\n"
                + "your ticket:\r\n"
                + "11,12,13\r\n"
                + "\r\n"
                + "nearby tickets:\r\n"
                + "3,9,18\r\n"
                + "15,1,5\r\n"
                + "5,14,9",
        );

        let your_ticket = map_your_ticket(&ex);

        assert_eq!(your_ticket["class"], 12);
        assert_eq!(your_ticket["row"], 11);
        assert_eq!(your_ticket["seat"], 13);
    }
}
