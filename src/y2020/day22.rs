use crate::solver::Solver;
use std::{collections::VecDeque, fs::File};
use std::{
    collections::{hash_map, HashSet},
    hash::Hasher,
    io::prelude::*,
};
use std::{hash::Hash, io::BufReader};

pub struct Problem;

impl Solver for Problem {
    type Input = (VecDeque<u8>, VecDeque<u8>);
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input(&self, file: File) -> (VecDeque<u8>, VecDeque<u8>) {
        let mut buf_reader = BufReader::new(file);
        let mut text = String::new();
        buf_reader.read_to_string(&mut text).unwrap();

        let player_split = text.split("\n\nPlayer 2:\n").collect::<Vec<&str>>();

        let player_1_deck = player_split[0]
            .split("\n")
            .skip(1)
            .map(|c| c.parse::<u8>().unwrap())
            .collect::<VecDeque<u8>>();
        let player_2_deck = player_split[1]
            .split("\n")
            .map(|c| c.parse::<u8>().unwrap())
            .collect::<VecDeque<u8>>();
        (player_1_deck, player_2_deck)
    }

    fn solve_part_one(&self, input: &(VecDeque<u8>, VecDeque<u8>)) -> usize {
        play_game(input)
    }

    fn solve_part_two(&self, input: &(VecDeque<u8>, VecDeque<u8>)) -> usize {
        let mut player_1_deck = input.0.clone();
        let mut player_2_deck = input.1.clone();
        let result = play_game_recurse(&mut player_1_deck, &mut player_2_deck);
        let score;
        if result == 1 {
            score = calculate_score(&player_1_deck);
        } else {
            score = calculate_score(&player_2_deck);
        }
        score
    }
}

fn play_game(start_deck: &(VecDeque<u8>, VecDeque<u8>)) -> usize {
    let mut player_1_deck = VecDeque::from(start_deck.0.clone());
    let mut player_2_deck = VecDeque::from(start_deck.1.clone());

    loop {
        let play_1 = player_1_deck.pop_front().unwrap();
        let play_2 = player_2_deck.pop_front().unwrap();

        if play_1 > play_2 {
            player_1_deck.push_back(play_1);
            player_1_deck.push_back(play_2);
        } else if play_2 > play_1 {
            player_2_deck.push_back(play_2);
            player_2_deck.push_back(play_1);
        }

        if player_1_deck.len() == 0 {
            return calculate_score(&player_2_deck);
        }
        if player_2_deck.len() == 0 {
            return calculate_score(&player_1_deck);
        }
    }
}

fn play_game_recurse(player_1_deck: &mut VecDeque<u8>, player_2_deck: &mut VecDeque<u8>) -> usize {
    let mut rounds = HashSet::new();

    loop {
        if !rounds.insert(hash(&player_1_deck, &player_2_deck)) {
            return 1;
        }

        let play_1 = player_1_deck.pop_front().unwrap();
        let play_2 = player_2_deck.pop_front().unwrap();

        let mut result = if play_1 > play_2 { 1 } else { 2 };

        if play_1 <= player_1_deck.len() as u8 && play_2 <= player_2_deck.len() as u8 {
            result = play_game_recurse(
                &mut player_1_deck
                    .iter()
                    .take(play_1 as usize)
                    .copied()
                    .collect(),
                &mut player_2_deck
                    .iter()
                    .take(play_2 as usize)
                    .copied()
                    .collect(),
            );
        }

        if result == 1 {
            player_1_deck.push_back(play_1);
            player_1_deck.push_back(play_2);
        } else {
            player_2_deck.push_back(play_2);
            player_2_deck.push_back(play_1);
        }

        if player_1_deck.len() == 0 || player_2_deck.len() == 0 {
            return result;
        }
    }
}

fn calculate_score(deck: &VecDeque<u8>) -> usize {
    let mut score = 0;
    for i in 0..deck.len() {
        score = score + (deck.len() - i) * (deck[i] as usize);
    }

    score
}

fn hash(d1: &VecDeque<u8>, d2: &VecDeque<u8>) -> u64 {
    let mut hasher = hash_map::DefaultHasher::new();
    d1.hash(&mut hasher);
    d2.hash(&mut hasher);
    hasher.finish()
}

#[cfg(test)]
mod tests {
    use crate::y2020::day22::*;

    #[test]
    fn test_play_game() {
        let ex = (
            VecDeque::from(vec![9, 2, 6, 3, 1]),
            VecDeque::from(vec![5, 8, 4, 7, 10]),
        );

        assert_eq!(play_game(&ex), 306);
    }

    #[test]
    fn test_play_game_recurse() {
        let ex1 = (
            VecDeque::from(vec![9, 2, 6, 3, 1]),
            VecDeque::from(vec![5, 8, 4, 7, 10]),
        );
        let ex2 = (
            VecDeque::from(vec![43, 19]),
            VecDeque::from(vec![2, 29, 14]),
        );
        let mut ex1_1 = ex1.0.clone();
        let mut ex1_2 = ex1.1.clone();
        let result = play_game_recurse(&mut ex1_1, &mut ex1_2);
        let _result_no_loop = play_game_recurse(&mut ex2.0.clone(), &mut ex2.1.clone());
        let score;
        if result == 1 {
            score = calculate_score(&ex1_1);
        } else {
            score = calculate_score(&ex1_2);
        }

        assert_eq!(score, 291);
    }
}
