use crate::solver::Solver;
use regex::Regex;
use std::io::prelude::*;
use std::io::BufReader;
use std::{collections::HashSet, fs::File};

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<String>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input(&self, file: File) -> Vec<String> {
        let buf_reader = BufReader::new(file);
        buf_reader.lines().map(|l| l.unwrap()).collect()
    }

    fn solve_part_one(&self, input: &Vec<String>) -> usize {
        find_black_tiles(input).len()
    }

    fn solve_part_two(&self, input: &Vec<String>) -> usize {
        simulate_days(input, 100).len()
    }
}

fn find_tile_from_instructions(instruction: &String) -> (isize, isize) {
    let re = Regex::new(r"nw|ne|se|sw|w|e").unwrap();
    let mut tile_coordinate = (0, 0);
    for result in re.find_iter(instruction) {
        let dir = result.as_str();
        tile_coordinate = match dir {
            "nw" => (tile_coordinate.0 + 1, tile_coordinate.1 - 1),
            "ne" => (tile_coordinate.0 + 1, tile_coordinate.1 + 1),
            "se" => (tile_coordinate.0 - 1, tile_coordinate.1 + 1),
            "sw" => (tile_coordinate.0 - 1, tile_coordinate.1 - 1),
            "w" => (tile_coordinate.0, tile_coordinate.1 - 2),
            "e" => (tile_coordinate.0, tile_coordinate.1 + 2),
            _ => tile_coordinate,
        };
    }

    tile_coordinate
}

fn find_black_tiles(instructions: &Vec<String>) -> HashSet<(isize, isize)> {
    let mut black_tiles = HashSet::new();

    for instruction in instructions {
        let tile = find_tile_from_instructions(instruction);
        if black_tiles.contains(&tile) {
            black_tiles.remove(&tile);
        } else {
            black_tiles.insert(tile);
        }
    }

    black_tiles
}

fn simulate_days(instructions: &Vec<String>, days: usize) -> HashSet<(isize, isize)> {
    let mut tiles = find_black_tiles(instructions);
    for _i in 0..days {
        tiles = change_tiles(&tiles);
    }

    tiles
}

fn change_tiles(black_tiles: &HashSet<(isize, isize)>) -> HashSet<(isize, isize)> {
    let mut new_black_tiles = HashSet::new();
    for black_tile in black_tiles {
        let adjacent_tiles = get_adjacent(black_tile);

        let number_adjacent_black_tiles = adjacent_tiles
            .iter()
            .filter(|a| black_tiles.contains(a))
            .count();
        if !(number_adjacent_black_tiles == 0 || number_adjacent_black_tiles > 2) {
            new_black_tiles.insert(*black_tile);
        }

        for adjacent_white_tile in adjacent_tiles.iter().filter(|a| !black_tiles.contains(a)) {
            let adjacent_to_white_tile = get_adjacent(adjacent_white_tile);
            if adjacent_to_white_tile
                .iter()
                .filter(|a| black_tiles.contains(a))
                .count()
                == 2
            {
                new_black_tiles.insert(*adjacent_white_tile);
            }
        }
    }
    new_black_tiles
}

fn get_adjacent(coordinate: &(isize, isize)) -> Vec<(isize, isize)> {
    vec![
        (coordinate.0 + 1, coordinate.1 - 1),
        (coordinate.0 + 1, coordinate.1 + 1),
        (coordinate.0, coordinate.1 + 2),
        (coordinate.0 - 1, coordinate.1 + 1),
        (coordinate.0 - 1, coordinate.1 - 1),
        (coordinate.0, coordinate.1 - 2),
    ]
}

#[cfg(test)]
mod tests {
    use crate::y2020::day24::*;

    #[test]
    fn test_find_black_tiles() {
        let ex = vec![
            String::from("sesenwnenenewseeswwswswwnenewsewsw"),
            String::from("neeenesenwnwwswnenewnwwsewnenwseswesw"),
            String::from("seswneswswsenwwnwse"),
            String::from("nwnwneseeswswnenewneswwnewseswneseene"),
            String::from("swweswneswnenwsewnwneneseenw"),
            String::from("eesenwseswswnenwswnwnwsewwnwsene"),
            String::from("sewnenenenesenwsewnenwwwse"),
            String::from("wenwwweseeeweswwwnwwe"),
            String::from("wsweesenenewnwwnwsenewsenwwsesesenwne"),
            String::from("neeswseenwwswnwswswnw"),
            String::from("nenwswwsewswnenenewsenwsenwnesesenew"),
            String::from("enewnwewneswsewnwswenweswnenwsenwsw"),
            String::from("sweneswneswneneenwnewenewwneswswnese"),
            String::from("swwesenesewenwneswnwwneseswwne"),
            String::from("enesenwswwswneneswsenwnewswseenwsese"),
            String::from("wnwnesenesenenwwnenwsewesewsesesew"),
            String::from("nenewswnwewswnenesenwnesewesw"),
            String::from("eneswnwswnwsenenwnwnwwseeswneewsenese"),
            String::from("neswnwewnwnwseenwseesewsenwsweewe"),
            String::from("wseweeenwnesenwwwswnew"),
        ];

        assert_eq!(find_black_tiles(&ex).len(), 10);
        assert_eq!(simulate_days(&ex, 100).len(), 2208);
    }
}
