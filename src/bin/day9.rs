use std::cmp::Ordering;
use std::collections::HashSet;

use anyhow::Result as AnyResult;
use aoc_2020::lines;
use itertools::Itertools;
use itertools::MinMaxResult;

/// Load lines and parse numbers.
fn load() -> AnyResult<Vec<usize>> {
    Ok(lines("files/day9/input.txt")?
        .filter_map(|n| n.parse::<usize>().ok())
        .collect())
}

/// Check wheter a number is valid given the preamble.
fn is_valid(preamble: &[usize], number: usize) -> bool {
    let preamble: HashSet<_> = preamble.iter().collect();

    for a in &preamble {
        if **a > number {
            continue;
        }

        let c = number - *a;
        if preamble.contains(&c) {
            return true;
        }
    }

    false
}

/// Find the invalid number in the data, if there is one.
fn find_invalid(data: &[usize], preamble_size: usize) -> Option<usize> {
    for window in data.windows(preamble_size + 1) {
        let preamble = &window[..preamble_size];
        let num_to_test = window[preamble_size];
        if !is_valid(preamble, num_to_test) {
            return Some(num_to_test);
        }
    }

    None
}

/// Find the weakness in the data given the invalid number, if there is one.
fn find_weakness(data: &[usize], invalid_number: usize) -> Option<usize> {
    if data.len() < 2 {
        return None;
    }

    let mut lo = 0;
    let mut hi = 0;
    let mut sum = data[lo];

    loop {
        match sum.cmp(&invalid_number) {
            Ordering::Equal => {
                let res = data[lo..hi + 1].iter().minmax();
                match res {
                    // At least two elements in between lo..hi + 1.
                    MinMaxResult::MinMax(min, max) => {
                        return Some(min + max);
                    }

                    // There's only one or zero element(s) in
                    // between lo..hi + 1.
                    _ => return None,
                }
            }

            Ordering::Greater => {
                sum -= data[lo];
                lo += 1;
            }

            Ordering::Less => {
                hi += 1;
                sum += data[hi];
            }
        }
    }
}

fn main() -> AnyResult<()> {
    let data = load()?;
    let invalid = find_invalid(&data, 25);
    println!("Day 9, Part 1: {:?}", invalid);

    if let Some(invalid) = invalid {
        let weakness = find_weakness(&data, invalid);
        println!("Day 9, Part 2: {:?}", weakness);
    }

    Ok(())
}
