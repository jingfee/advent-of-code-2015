use crate::solver::Solver;
use itertools::Itertools;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub struct Tile {
    id: usize,
    data: Vec<Vec<bool>>,
    rotation: u8, //default = 0, 90dcw = 1, 180dcw = 2; 270dcw = 3
    flipped: bool, //default = false, flipped = true
                  // rotate first then flip
}

impl Clone for Tile {
    fn clone(&self) -> Tile {
        Tile {
            id: self.id,
            data: self.data.clone(),
            rotation: self.rotation,
            flipped: self.flipped,
        }
    }
}

impl Tile {
    fn get_borders(&self) -> Vec<Vec<bool>> {
        let mut borders = Vec::new();

        let up = self.data[0].clone();
        let mut up_flipped = up.clone();
        up_flipped.reverse();
        borders.push(up);
        borders.push(up_flipped);

        let right = self.data.iter().map(|d| d[9]).collect::<Vec<bool>>();
        let mut right_flipped = right.clone();
        right_flipped.reverse();
        borders.push(right);
        borders.push(right_flipped);

        let down = self.data[9].clone();
        let mut down_flipped = down.clone();
        down_flipped.reverse();
        borders.push(down);
        borders.push(down_flipped);

        let left = self.data.iter().map(|d| d[0]).collect::<Vec<bool>>();
        let mut left_flipped = left.clone();
        left_flipped.reverse();
        borders.push(left);
        borders.push(left_flipped);

        borders
    }

    fn get_border(&self, side_number: u8) -> Vec<bool> {
        let mut norm_side_number = (side_number as i8 - self.rotation as i8).rem_euclid(4);
        norm_side_number = if self.flipped && (side_number == 1 || side_number == 3) {
            (norm_side_number + 2) % 4
        } else {
            norm_side_number
        };

        let mut border = match norm_side_number {
            0 => self.data[0].clone(),
            1 => self.data.iter().map(|d| d[9]).collect::<Vec<bool>>(),
            2 => {
                let mut border = self.data[9].clone();
                border.reverse();
                border
            }
            3 => {
                let mut border = self.data.iter().map(|d| d[0]).collect::<Vec<bool>>();
                border.reverse();
                border
            }
            _ => panic!("Invalid side number"),
        };

        if self.flipped {
            border.reverse();
        }

        border
    }

    fn get_image_without_borders(&self) -> Vec<Vec<bool>> {
        let mut image = Vec::new();
        for i in 0..8 {
            image.push(Vec::new());
            for _j in 0..8 {
                image[i].push(false);
            }
        }

        match self.rotation {
            0 => {
                for i in 1..=8 {
                    for j in 1..=8 {
                        if self.flipped {
                            image[i - 1][j - 1] = self.data[i][9 - j];
                        } else {
                            image[i - 1][j - 1] = self.data[i][j];
                        }
                    }
                }
            }
            1 => {
                for i in 1..=8 {
                    for j in 1..=8 {
                        if self.flipped {
                            image[i - 1][j - 1] = self.data[j][i];
                        } else {
                            image[i - 1][j - 1] = self.data[9 - j][i];
                        }
                    }
                }
            }
            2 => {
                for i in 1..=8 {
                    for j in 1..=8 {
                        if self.flipped {
                            image[i - 1][j - 1] = self.data[9 - i][j];
                        } else {
                            image[i - 1][j - 1] = self.data[9 - i][9 - j];
                        }
                    }
                }
            }
            3 => {
                for i in 1..=8 {
                    for j in 1..=8 {
                        if self.flipped {
                            image[i - 1][j - 1] = self.data[9 - j][9 - i];
                        } else {
                            image[i - 1][j - 1] = self.data[j][9 - i];
                        }
                    }
                }
            }
            _ => (),
        }

        image
    }
}

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<Tile>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input(&self, file: File) -> Vec<Tile> {
        let mut buf_reader = BufReader::new(file);
        let mut lines = String::new();
        buf_reader.read_to_string(&mut lines).unwrap();
        parse_tiles(lines)
    }

    fn solve_part_one(&self, input: &Vec<Tile>) -> usize {
        find_corners(input).iter().map(|t| t.id).product()
    }

    fn solve_part_two(&self, input: &Vec<Tile>) -> usize {
        let image = get_image(&input);
        let monsters = find_monsters(&image);

        let water_count = image
            .iter()
            .map(|r| r.iter().filter(|c| **c).count())
            .sum::<usize>();
        let monster_count = monsters.len();

        water_count - monster_count
    }
}

fn find_corners(tiles: &Vec<Tile>) -> Vec<&Tile> {
    tiles
        .iter()
        .filter(|t| {
            let mut matches = 0;
            let borders = t.get_borders();
            for tile in tiles {
                if t.id == tile.id {
                    continue;
                }

                if borders
                    .iter()
                    .any(|b| tile.get_borders().iter().any(|bn| b == bn))
                {
                    matches = matches + 1;
                }
            }
            matches == 2
        })
        .collect::<Vec<&Tile>>()
}

fn find_first_corner(tiles: &Vec<Tile>) -> Tile {
    let mut corner = find_corners(tiles)[0].clone();
    for rotation in 0..4 {
        for flip in 0..2 {
            corner.rotation = rotation;
            corner.flipped = if flip == 0 { false } else { true };

            let right_border = corner.get_border(1);
            let down_border = corner.get_border(2);

            let matches = tiles.iter().filter(|t| {
                t.id != corner.id
                    && t.get_borders()
                        .iter()
                        .any(|b| b == &right_border || b == &down_border)
            });

            if matches.count() == 2 {
                return corner;
            }
        }
    }

    panic!("No corner found");
}

fn get_image(tiles: &Vec<Tile>) -> Vec<Vec<bool>> {
    let mut current_tile = find_first_corner(tiles);
    let mut first_last_row = current_tile.clone();
    let mut image = Vec::new();
    let square = (tiles.len() as f32).sqrt() as usize;
    let image_length = square * 8;
    for i in 0..image_length {
        image.push(Vec::new());
        for _j in 0..image_length {
            image[i].push(false);
        }
    }

    add_to_image(&mut image, &current_tile, 0, 0);

    for i in 0..square {
        for j in 0..square {
            if i == 0 && j == 0 {
                continue;
            }
            if j == 0 {
                let down_border = first_last_row.get_border(2);
                current_tile = find_matching_tile(tiles, first_last_row.id, &down_border, 0);
                add_to_image(&mut image, &current_tile, j, i);
                first_last_row = current_tile.clone();
            } else {
                let right_border = &current_tile.get_border(1);
                current_tile = find_matching_tile(tiles, current_tile.id, &right_border, 3);
                add_to_image(&mut image, &current_tile, j, i);
            }
        }
    }

    image
}

fn find_matching_tile(
    tiles: &Vec<Tile>,
    current_tile_id: usize,
    border: &Vec<bool>,
    side_to_match: u8,
) -> Tile {
    for tile in tiles {
        if tile.id == current_tile_id {
            continue;
        }
        let mut tile_clone = tile.clone();
        for rotation in 0..4 {
            for flip in 0..2 {
                tile_clone.rotation = rotation;
                tile_clone.flipped = if flip == 0 { false } else { true };

                let mut tile_border = tile_clone.get_border(side_to_match);
                tile_border.reverse();

                if border == &tile_border {
                    return tile_clone;
                }
            }
        }
    }

    panic!("No tile found for {}", current_tile_id);
}

fn add_to_image(image: &mut Vec<Vec<bool>>, tile: &Tile, tile_x: usize, tile_y: usize) {
    let tile_image = tile.get_image_without_borders();
    for i in tile_y * 8..tile_y * 8 + 8 {
        for j in tile_x * 8..tile_x * 8 + 8 {
            image[i][j] = tile_image[i - tile_y * 8][j - tile_x * 8];
        }
    }
}

fn parse_tiles(lines: String) -> Vec<Tile> {
    lines
        .split("\n\n")
        .map(|l| {
            let tile_lines = l.split("\n").collect::<Vec<&str>>();
            let id = tile_lines[0][5..9].parse::<usize>().unwrap();

            let data = tile_lines[1..]
                .iter()
                .map(|r| {
                    r.chars()
                        .map(|c| if c == '.' { false } else { true })
                        .collect::<Vec<bool>>()
                })
                .collect::<Vec<Vec<bool>>>();

            Tile {
                id: id,
                data: data,
                rotation: 0,
                flipped: false,
            }
        })
        .collect::<Vec<Tile>>()
}

fn find_monsters(image: &Vec<Vec<bool>>) -> Vec<(usize, usize)> {
    let monster_pattern: Vec<(isize, isize)> = vec![
        (0, 1),
        (1, 1),
        (0, 2),
        (-1, 3),
        (-1, 6),
        (0, 7),
        (0, 8),
        (-1, 9),
        (-1, 12),
        (0, 13),
        (0, 14),
        (-1, 15),
        (-1, 18),
        (0, 19),
    ];

    let mut monsters = Vec::new();

    for i in 1..image.len() - 1 {
        for j in 0..image[i].len() - 20 {
            if image[i][j] && !monsters.contains(&(i as usize, j as usize)) {
                let mut possible_monsters = vec![(i, j)];

                let mut found_monster = true;
                for p in &monster_pattern {
                    if image[(i as isize + p.0) as usize][(j as isize + p.1) as usize]
                        && !monsters
                            .contains(&((i as isize + p.0) as usize, (j as isize + p.1) as usize))
                    {
                        possible_monsters
                            .push(((i as isize + p.0) as usize, (j as isize + p.1) as usize));
                    } else {
                        found_monster = false;
                        break;
                    }
                }

                if found_monster {
                    for possible_monster in possible_monsters {
                        monsters.push(possible_monster);
                    }
                }
            }
        }
    }

    monsters
}

#[cfg(test)]
mod tests {
    use crate::y2020::day20::*;

    #[test]
    fn test_find_corners() {
        let ex = parse_tiles(
            "Tile 2311:\n".to_string()
                + "..##.#..#.\n"
                + "##..#.....\n"
                + "#...##..#.\n"
                + "####.#...#\n"
                + "##.##.###.\n"
                + "##...#.###\n"
                + ".#.#.#..##\n"
                + "..#....#..\n"
                + "###...#.#.\n"
                + "..###..###\n"
                + "\n"
                + "Tile 1951:\n"
                + "#.##...##.\n"
                + "#.####...#\n"
                + ".....#..##\n"
                + "#...######\n"
                + ".##.#....#\n"
                + ".###.#####\n"
                + "###.##.##.\n"
                + ".###....#.\n"
                + "..#.#..#.#\n"
                + "#...##.#..\n"
                + "\n"
                + "Tile 1171:\n"
                + "####...##.\n"
                + "#..##.#..#\n"
                + "##.#..#.#.\n"
                + ".###.####.\n"
                + "..###.####\n"
                + ".##....##.\n"
                + ".#...####.\n"
                + "#.##.####.\n"
                + "####..#...\n"
                + ".....##...\n"
                + "\n"
                + "Tile 1427:\n"
                + "###.##.#..\n"
                + ".#..#.##..\n"
                + ".#.##.#..#\n"
                + "#.#.#.##.#\n"
                + "....#...##\n"
                + "...##..##.\n"
                + "...#.#####\n"
                + ".#.####.#.\n"
                + "..#..###.#\n"
                + "..##.#..#.\n"
                + "\n"
                + "Tile 1489:\n"
                + "##.#.#....\n"
                + "..##...#..\n"
                + ".##..##...\n"
                + "..#...#...\n"
                + "#####...#.\n"
                + "#..#.#.#.#\n"
                + "...#.#.#..\n"
                + "##.#...##.\n"
                + "..##.##.##\n"
                + "###.##.#..\n"
                + "\n"
                + "Tile 2473:\n"
                + "#....####.\n"
                + "#..#.##...\n"
                + "#.##..#...\n"
                + "######.#.#\n"
                + ".#...#.#.#\n"
                + ".#########\n"
                + ".###.#..#.\n"
                + "########.#\n"
                + "##...##.#.\n"
                + "..###.#.#.\n"
                + "\n"
                + "Tile 2971:\n"
                + "..#.#....#\n"
                + "#...###...\n"
                + "#.#.###...\n"
                + "##.##..#..\n"
                + ".#####..##\n"
                + ".#..####.#\n"
                + "#..#.#..#.\n"
                + "..####.###\n"
                + "..#.#.###.\n"
                + "...#.#.#.#\n"
                + "\n"
                + "Tile 2729:\n"
                + "...#.#.#.#\n"
                + "####.#....\n"
                + "..#.#.....\n"
                + "....#..#.#\n"
                + ".##..##.#.\n"
                + ".#.####...\n"
                + "####.#.#..\n"
                + "##.####...\n"
                + "##..#.##..\n"
                + "#.##...##.\n"
                + "\n"
                + "Tile 3079:\n"
                + "#.#.#####.\n"
                + ".#..######\n"
                + "..#.......\n"
                + "######....\n"
                + "####.#..#.\n"
                + ".#...#.##.\n"
                + "#.#####.##\n"
                + "..#.###...\n"
                + "..#.......\n"
                + "..#.###...",
        );

        assert_eq!(
            find_corners(&ex).iter().map(|t| t.id).product::<usize>(),
            20899048083289
        );
    }
}
