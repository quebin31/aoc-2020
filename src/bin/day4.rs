use std::collections::{HashMap, HashSet};

use anyhow::Result as AnyResult;
use aoc_2020::lines;
use common_macros::hash_set;

/// Holds a map to store key value pairs.
struct Document {
    data: HashMap<String, String>,
}

impl Document {
    fn new() -> Self {
        Self {
            data: Default::default(),
        }
    }

    /// Check if the whole document is valid checking whether it contains all
    /// the required keys and no more.
    fn is_valid(&self) -> bool {
        lazy_static::lazy_static! {
            static ref REQUIRED_KEYS: HashSet<&'static str> =
                hash_set!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"];
        }

        let mut found = 0;
        let valid = self
            .data
            .keys()
            .filter(|key| key.as_str() != "cid")
            .all(|key| {
                let present = REQUIRED_KEYS.contains(key.as_str());
                found += if present { 1 } else { 0 };
                present
            });

        valid && REQUIRED_KEYS.len() - 1 == found
    }

    /// Insert a new key value pair, if `validate` is true then `val` will be
    /// validates according to the rules imposed by each key, if `val` was valid
    /// returns true, otherwise returns false. If `validate` is false this function
    /// always returns true.
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

    /// Check whether a year is between the min and max values.
    fn is_valid_year(val: &str, min: u32, max: u32) -> bool {
        if let Ok(num) = val.parse::<u32>() {
            min <= num && num <= max
        } else {
            false
        }
    }

    /// Check whether the height, given in cm or inches, is valid.
    fn is_valid_height(val: &str) -> bool {
        let unit = &val[(val.len() - 2)..];
        let height = || val[..(val.len() - 2)].parse::<u32>();

        match (unit, height()) {
            ("cm", Ok(height)) => 150 <= height && height <= 193,
            ("in", Ok(height)) => 59 <= height && height <= 76,
            _ => false,
        }
    }

    /// Check whether a string is a valid hexadecimal color, i.e.:
    /// - Lenght is 7
    /// - Starts with '#'
    /// - And the six remaining chars are hexadecimal digits
    fn is_valid_hex_color(val: &str) -> bool {
        let chars: Vec<_> = val.chars().collect();
        let valid_prefix = || chars.get(0).map(|c| *c == '#').unwrap_or(false);
        let valid_suffix = || chars.iter().skip(1).all(|c| c.is_ascii_hexdigit());

        chars.len() == 7 && valid_prefix() && valid_suffix()
    }

    /// Check an string against a fixed set of values.
    fn is_valid_eye_color(val: &str) -> bool {
        matches!(val, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth")
    }

    /// Check whether a string is a valid passport id, i.e.:
    /// - Lenght is 9
    /// - All characters are digits.
    fn is_valid_pid(val: &str) -> bool {
        val.len() == 9 && val.chars().all(|c| c.is_ascii_digit())
    }
}

/// Count how many documents are valid, part 1 without data validation.
fn part1() -> AnyResult<usize> {
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

/// Count how many documents are valid, part 2 with data validation.
fn part2() -> AnyResult<usize> {
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
            // If doc.insert(..) returns false, we should ignore this document since
            // its data is invalid.
            ignore_current = !doc.insert(&kv[0].to_owned(), &kv[1].to_owned(), true);
        }
    }

    Ok(count)
}

fn main() -> AnyResult<()> {
    println!("Day 4, Part 1: {}", part1()?);
    println!("Day 4, Part 2: {}", part2()?);

    Ok(())
}
