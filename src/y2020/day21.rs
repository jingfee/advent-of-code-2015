use crate::solver::Solver;
use itertools::Itertools;
use std::io::prelude::*;
use std::io::BufReader;
use std::{collections::HashMap, fs::File};

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<(Vec<String>, Vec<String>)>;
    type Output1 = usize;
    type Output2 = String;

    fn parse_input(&self, file: File) -> Vec<(Vec<String>, Vec<String>)> {
        let buf_reader = BufReader::new(file);
        buf_reader
            .lines()
            .map(|l| {
                let line = l.unwrap();
                let ingredient_allergen_split = line.split(" (contains ").collect::<Vec<&str>>();
                let ingredients = ingredient_allergen_split[0]
                    .split(" ")
                    .map(|i| i.to_string())
                    .collect::<Vec<String>>();
                let allergens = ingredient_allergen_split[1]
                    .split(", ")
                    .map(|a| {
                        let allergen = a.to_string();
                        if allergen.ends_with(")") {
                            return allergen[0..allergen.len() - 1].to_string();
                        } else {
                            return allergen;
                        }
                    })
                    .collect::<Vec<String>>();

                (ingredients, allergens)
            })
            .collect()
    }

    fn solve_part_one(&self, input: &Vec<(Vec<String>, Vec<String>)>) -> usize {
        find_ingredients_without_allergens(input)
    }

    fn solve_part_two(&self, input: &Vec<(Vec<String>, Vec<String>)>) -> String {
        map_allergens(input).join(",")
    }
}

fn find_ingredients_without_allergens(food_list: &Vec<(Vec<String>, Vec<String>)>) -> usize {
    let mapped_allergens = map_allergens(food_list);
    let ingredients_without_allergens = food_list
        .iter()
        .flat_map(|l| l.0.clone())
        .unique()
        .filter(|i| !mapped_allergens.iter().any(|ai| ai.contains(i)))
        .collect::<Vec<String>>();

    food_list
        .iter()
        .map(|f| {
            f.0.iter()
                .filter(|fi| ingredients_without_allergens.contains(fi))
                .count()
        })
        .sum()
}

fn map_allergens(food_list: &Vec<(Vec<String>, Vec<String>)>) -> Vec<String> {
    let ingredients = food_list
        .iter()
        .flat_map(|l| l.0.clone())
        .unique()
        .collect::<Vec<String>>();
    let allergens = food_list
        .iter()
        .flat_map(|l| l.1.clone())
        .unique()
        .collect::<Vec<String>>();

    let mut allergen_map = HashMap::new();

    for allergen in &allergens {
        allergen_map.insert(allergen.to_string(), ingredients.clone());
    }

    for food in food_list {
        for allergen in &food.1 {
            let possible_ingredients = allergen_map[allergen]
                .iter()
                .filter(|i| food.0.contains(i))
                .map(|i| i.to_string())
                .collect::<Vec<String>>();

            if possible_ingredients.len() == 1 {
                let allergens_to_check = &allergens
                    .clone()
                    .iter()
                    .filter(|a| *a != allergen)
                    .map(|a| a.to_string())
                    .collect::<Vec<String>>();
                remove_ingredient_from_allergen(
                    &mut allergen_map,
                    allergens_to_check,
                    &possible_ingredients[0],
                );
            }

            allergen_map.insert(allergen.to_string(), possible_ingredients);
        }
    }

    let mut allergen_ingredients = Vec::new();
    for allergen in allergen_map.keys().sorted() {
        allergen_ingredients.push(allergen_map[allergen].first().unwrap().to_string());
    }

    allergen_ingredients
}

fn remove_ingredient_from_allergen(
    allergen_map: &mut HashMap<String, Vec<String>>,
    allergens: &Vec<String>,
    ingredient_to_remove: &String,
) {
    for other_allergen in allergens {
        let possible_ingredients = allergen_map[other_allergen]
            .iter()
            .filter(|i| *i != ingredient_to_remove)
            .map(|i| i.to_string())
            .collect::<Vec<String>>();

        if possible_ingredients.len() == 1 {
            let allergens_to_check = &allergens
                .clone()
                .iter()
                .filter(|a| *a != other_allergen)
                .map(|a| a.to_string())
                .collect::<Vec<String>>();
            remove_ingredient_from_allergen(
                allergen_map,
                allergens_to_check,
                &possible_ingredients[0],
            );
        }

        allergen_map.insert(other_allergen.to_string(), possible_ingredients);
    }
}

#[cfg(test)]
mod tests {
    use crate::y2020::day21::*;

    #[test]
    fn test_map_allergens() {
        let ex = vec![
            (
                vec![
                    "mxmxvkd".to_string(),
                    "kfchds".to_string(),
                    "sqjhc".to_string(),
                    "nhms".to_string(),
                ],
                vec!["dairy".to_string(), "fish".to_string()],
            ),
            (
                vec![
                    "trh".to_string(),
                    "fvjkl".to_string(),
                    "sbzzf".to_string(),
                    "mxmxvkd".to_string(),
                ],
                vec!["dairy".to_string()],
            ),
            (
                vec!["sqjhc".to_string(), "fvjkl".to_string()],
                vec!["soy".to_string()],
            ),
            (
                vec![
                    "sqjhc".to_string(),
                    "mxmxvkd".to_string(),
                    "sbzzf".to_string(),
                ],
                vec!["fish".to_string()],
            ),
        ];

        assert_eq!(find_ingredients_without_allergens(&ex), 5);
        assert_eq!(map_allergens(&ex).join(","), "mxmxvkd,sqjhc,fvjkl");
    }
}
