use std::collections::HashMap;

use anyhow::{anyhow, Result as AnyResult};
use aoc_2020::lines;

fn load() -> AnyResult<Vec<usize>> {
    Ok(lines("files/day10/input.txt")?
        .filter_map(|n| n.parse::<usize>().ok())
        .collect())
}

/// Expects that `adapters` is sorted.
fn diffs(adapters: &[usize]) -> HashMap<usize, usize> {
    let mut diffs = HashMap::new();

    for window in adapters.windows(2) {
        let diff = window[1] - window[0];
        *diffs.entry(diff).or_insert(0) += 1;
    }

    diffs
}

fn possibilities(adapters: &[usize]) -> usize {
    let diffs = adapters.windows(2).map(|w| w[1] - w[0]);

    let mut acc = 1;
    let mut len = 0;

    for diff in diffs {
        if diff != 1 {
            if len > 1 {
                acc *= (len * (len - 1)) / 2 + 1
            }

            len = 0;
        } else {
            len += 1;
        }
    }

    acc
}

fn main() -> AnyResult<()> {
    let mut adapters = load()?;

    // Charging outlet.
    adapters.push(0);
    // Sort now.
    adapters.sort_unstable();
    // Device built-in adapter.
    adapters.push(adapters.last().ok_or_else(|| anyhow!("Empty array"))? + 3);

    let diffs = diffs(&adapters);
    println!("Diffs: {:?}", diffs);
    println!("Day 10, Part 1: {}", diffs[&1] * diffs[&3]);
    println!("Day 10, Part 2: {}", possibilities(&adapters));

    Ok(())
}
