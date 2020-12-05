use anyhow::Result as AnyResult;
use aoc_2020::lines;

struct Components(Vec<usize>, char, String);

/// Separate a line with the following format:
/// <number>-<number> <char>: <strig>
/// Into their basic components: ([<number>], <char>, <string>)
fn split_terms(line: &str) -> Components {
    let splitted = line.split(' ').collect::<Vec<_>>();

    let numbers = splitted[0]
        .split('-')
        .filter_map(|n| n.parse().ok())
        .collect::<Vec<_>>();

    let chr = splitted[1].chars().next().unwrap();
    let string = splitted[2].to_owned();

    Components(numbers, chr, string)
}

/// Load file contents and build vec of components.
fn load_and_split() -> AnyResult<Vec<Components>> {
    Ok(lines("files/day2/input.txt")?
        .map(|line| split_terms(&line))
        .collect())
}

/// Check whether a password is valid given the first interpretation.
/// Count how many times `letter` appears in the password, then check if
/// it's betweem the range.
fn is_valid_pass_part1(min_times: usize, max_times: usize, letter: char, password: &str) -> bool {
    let letter_count = password.chars().filter(|c| *c == letter).count();
    (min_times <= letter_count) && (letter_count <= max_times)
}

/// Count all the passwords that are valid given the first interpretation.
fn part1(entries: &[Components]) -> usize {
    entries
        .iter()
        .filter(|Components(times, letter, password)| {
            is_valid_pass_part1(times[0], times[1], *letter, &password)
        })
        .count()
}

/// Check whether a password is valid given the second interpretation.
/// There should be exactly one and only one char that matches `letter`
/// in one of the given positions.
fn is_valid_pass_part2(positions: &[usize], letter: char, password: &str) -> bool {
    let chars = password.chars().collect::<Vec<_>>();

    let mut found_one = false;
    for position in positions {
        if let Some(chr) = chars.get(*position - 1) {
            if found_one && *chr == letter {
                return false;
            } else if *chr == letter {
                found_one = true;
            }
        } else {
            return false;
        }
    }

    found_one
}

/// Count all the passwords that are valid given the first interpretation.
fn part2(entries: &[Components]) -> usize {
    entries
        .iter()
        .filter(|Components(positions, letter, password)| {
            is_valid_pass_part2(positions, *letter, &password)
        })
        .count()
}

fn main() -> AnyResult<()> {
    let entries = load_and_split()?;

    let part1_result = part1(&entries);
    println!("Day 2, Part 1: {}", part1_result);

    let part2_result = part2(&entries);
    println!("Day 2, Part 2: {}", part2_result);

    Ok(())
}
