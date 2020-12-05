use std::collections::{HashMap, HashSet};

use anyhow::Result as AnyResult;
use aoc_2020::lines;
use common_macros::hash_set;

struct Document {
    data: HashMap<String, String>,
}

impl Document {
    fn new() -> Self {
        Self {
            data: Default::default(),
        }
    }

    fn is_valid(&self) -> bool {
        lazy_static::lazy_static! {
            static ref VALID_KEYS: HashSet<&'static str> =
                hash_set!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"];
        }

        let mut validated = 0;
        for key in self.data.keys() {
            if !VALID_KEYS.contains(key.as_str()) {
                return false;
            } else {
                validated += match key.as_str() {
                    "cid" => 0,
                    _ => 1,
                };
            }
        }

        VALID_KEYS.len() - 1 == validated
    }

    fn insert(&mut self, key: &str, val: &str, validate: bool) -> bool {
        if !validate {
            self.data.insert(key.to_owned(), val.to_owned());
            true
        } else {
            let valid = match key {
                "byr" => Self::is_valid_year(val, 1920, 2002),
                "iyr" => Self::is_valid_year(val, 2010, 2020),
                "eyr" => Self::is_valid_year(val, 2020, 2030),
                "hgt" => Self::is_valid_height(val),
                "hcl" => Self::is_valid_hex_color(val),
                "ecl" => Self::is_valid_eye_color(val),
                "pid" => Self::is_valid_pid(val),
                _ => true,
            };

            if valid {
                self.data.insert(key.to_owned(), val.to_owned());
            }

            valid
        }
    }

    fn is_valid_year(val: &str, min: u32, max: u32) -> bool {
        if let Ok(num) = val.parse::<u32>() {
            min <= num && num <= max
        } else {
            false
        }
    }

    fn is_valid_height(val: &str) -> bool {
        let unit = &val[(val.len() - 2)..];

        if unit == "cm" || unit == "in" {
            if let Ok(num) = val[..(val.len() - 2)].parse::<u32>() {
                if unit == "cm" {
                    150 <= num && num <= 193
                } else {
                    59 <= num && num <= 76
                }
            } else {
                false
            }
        } else {
            false
        }
    }

    fn is_valid_hex_color(val: &str) -> bool {
        let mut chars = val.chars();
        let valid = chars.next().map(|c| c == '#').unwrap_or(false);
        valid && chars.all(|c| c.is_ascii_hexdigit())
    }

    fn is_valid_eye_color(val: &str) -> bool {
        matches!(val, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth")
    }

    fn is_valid_pid(val: &str) -> bool {
        val.len() == 9 && val.chars().all(|c| c.is_ascii_digit())
    }
}

fn quest1() -> AnyResult<usize> {
    let mut count = 0;

    let mut doc = Document::new();
    for line in lines("files/day4/input.txt")? {
        if line.trim().is_empty() {
            // Finished processing current "passport"
            count += if doc.is_valid() { 1 } else { 0 };
            doc = Document::new();
        }

        for kv in line.split(' ') {
            if kv.trim().is_empty() {
                continue;
            }

            let kv = kv.split(':').collect::<Vec<_>>();
            doc.insert(&kv[0].to_owned(), &kv[1].to_owned(), false);
        }
    }

    Ok(count)
}

fn quest2() -> AnyResult<usize> {
    let mut count = 0;

    let mut ignore_current = false;
    let mut doc = Document::new();
    for line in lines("files/day4/input.txt")? {
        if line.trim().is_empty() {
            // Finished processing current "passport"
            count += if ignore_current || !doc.is_valid() {
                0
            } else {
                1
            };

            doc = Document::new();
            ignore_current = false;
        }

        if ignore_current {
            continue;
        }

        for kv in line.split(' ') {
            if kv.trim().is_empty() {
                continue;
            }

            let kv = kv.split(':').collect::<Vec<_>>();
            ignore_current = !doc.insert(&kv[0].to_owned(), &kv[1].to_owned(), true);
        }
    }

    Ok(count)
}

fn main() -> AnyResult<()> {
    let quest1_result = quest1()?;
    println!("Day 4, Quest 1: {}", quest1_result);

    let quest2_result = quest2()?;
    println!("Day 4, Quest 2: {}", quest2_result);

    Ok(())
}
