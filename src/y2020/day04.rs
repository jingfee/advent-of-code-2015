use crate::solver::Solver;
use regex::Regex;
use std::io::prelude::*;
use std::io::BufReader;
use std::{collections::HashMap, fs::File};

pub struct Passport {
    fields: HashMap<String, String>,
}

impl Passport {
    fn new() -> Passport {
        Passport {
            fields: HashMap::new(),
        }
    }

    fn is_valid(&self) -> bool {
        self.fields.contains_key(&"byr".to_string())
            && self.fields.contains_key(&"iyr".to_string())
            && self.fields.contains_key(&"eyr".to_string())
            && self.fields.contains_key(&"hgt".to_string())
            && self.fields.contains_key(&"hcl".to_string())
            && self.fields.contains_key(&"ecl".to_string())
            && self.fields.contains_key(&"pid".to_string())
    }

    fn is_valid_strict(&self) -> bool {
        self.is_valid()
            && self.valid_byr()
            && self.valid_iyr()
            && self.valid_eyr()
            && self.valid_hgt()
            && self.valid_hcl()
            && self.valid_ecl()
            && self.valid_pid()
    }

    fn valid_byr(&self) -> bool {
        let re = Regex::new(r"^(19[2-9][0-9]|200[0-2])$").unwrap();
        re.is_match(&self.fields["byr"])
    }

    fn valid_iyr(&self) -> bool {
        let re = Regex::new(r"^(20(1[0-9]|20))$").unwrap();
        re.is_match(&self.fields["iyr"])
    }

    fn valid_eyr(&self) -> bool {
        let re = Regex::new(r"^(20(2[0-9]|30))$").unwrap();
        re.is_match(&self.fields["eyr"])
    }

    fn valid_hgt(&self) -> bool {
        let re = Regex::new(r"^((1([5-8][0-9]|9[0-3])cm)|((59|6[0-9]|7[0-6])in))$").unwrap();
        re.is_match(&self.fields["hgt"])
    }

    fn valid_hcl(&self) -> bool {
        let re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
        re.is_match(&self.fields["hcl"])
    }

    fn valid_ecl(&self) -> bool {
        let re = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
        re.is_match(&self.fields["ecl"])
    }

    fn valid_pid(&self) -> bool {
        let re = Regex::new(r"^[0-9]{9}$").unwrap();
        re.is_match(&self.fields["pid"])
    }
}

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<Passport>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input(&self, file: File) -> Vec<Passport> {
        let buf_reader = BufReader::new(file);

        let lines: Vec<String> = buf_reader.lines().map(|l| l.unwrap()).collect();
        get_passports(lines)
    }

    fn solve_part_one(&self, input: &mut Vec<Passport>) -> usize {
        input.iter().map(|p| p.is_valid()).filter(|p| *p).count()
    }

    fn solve_part_two(&self, input: &mut Vec<Passport>) -> usize {
        input
            .iter()
            .map(|p| p.is_valid_strict())
            .filter(|p| *p)
            .count()
    }
}

fn get_passports(lines: Vec<String>) -> Vec<Passport> {
    let mut passports: Vec<Passport> = Vec::new();
    let mut index = 0;
    loop {
        if index == lines.len() {
            break;
        }

        let mut passport = Passport::new();
        loop {
            if index == lines.len() {
                break;
            }

            let line = &lines[index];
            index = index + 1;
            if line == "" {
                break;
            }

            let line_split: Vec<&str> = line.split(' ').collect();
            for split in line_split {
                let key_value_split: Vec<&str> = split.split(':').collect();
                passport.fields.insert(
                    key_value_split[0].to_string(),
                    key_value_split[1].to_string(),
                );
            }
        }
        passports.push(passport);
    }

    passports
}

#[cfg(test)]
mod tests {
    use crate::y2020::day04::*;

    #[test]
    fn test_valid_passport() {
        let passports = get_passports(vec![
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd".to_string(),
            "byr:1937 iyr:2017 cid:147 hgt:183cm".to_string(),
            "".to_string(),
            "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884".to_string(),
            "hcl:#cfa07d byr:1929".to_string(),
            "".to_string(),
            "hcl:#ae17e1 iyr:2013".to_string(),
            "eyr:2024".to_string(),
            "ecl:brn pid:760753108 byr:1931".to_string(),
            "hgt:179cm".to_string(),
            "".to_string(),
            "hcl:#cfa07d eyr:2025 pid:166559648".to_string(),
            "iyr:2011 ecl:brn hgt:59in".to_string(),
        ]);

        assert_eq!(passports[0].is_valid(), true);
        assert_eq!(passports[1].is_valid(), false);
        assert_eq!(passports[2].is_valid(), true);
        assert_eq!(passports[3].is_valid(), false);
    }

    #[test]
    fn test_valid_passport_strict() {
        let invalid_passports = get_passports(vec![
            "eyr:1972 cid:100".to_string(),
            "hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926".to_string(),
            "".to_string(),
            "iyr:2019".to_string(),
            "hcl:#602927 eyr:1967 hgt:170cm".to_string(),
            "ecl:grn pid:012533040 byr:1946".to_string(),
            "".to_string(),
            "hcl:dab227 iyr:2012".to_string(),
            "ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277".to_string(),
            "".to_string(),
            "hgt:59cm ecl:zzz".to_string(),
            "eyr:2038 hcl:74454a iyr:2023".to_string(),
            "pid:3556412378 byr:2007".to_string(),
        ]);

        let valid_passports = get_passports(vec![
            "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980".to_string(),
            "hcl:#623a2f".to_string(),
            "".to_string(),
            "eyr:2029 ecl:blu cid:129 byr:1989".to_string(),
            "iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm".to_string(),
            "".to_string(),
            "hcl:#888785".to_string(),
            "hgt:164cm byr:2001 iyr:2015 cid:88".to_string(),
            "pid:545766238 ecl:hzl".to_string(),
            "eyr:2022".to_string(),
            "".to_string(),
            "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719".to_string(),
        ]);

        for invalid_passport in invalid_passports {
            assert_eq!(invalid_passport.is_valid_strict(), false);
        }

        for valid_passport in valid_passports {
            assert_eq!(valid_passport.is_valid_strict(), true);
        }
    }
}
