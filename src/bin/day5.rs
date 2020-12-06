use anyhow::Result as AnyResult;
use aoc_2020::lines;

/// Get the seat id for a given line.
fn get_seat_id(line: &str) -> usize {
    let (row_dirs, col_dirs) = (&line[..7], &line[7..]);

    let (mut min_row, mut max_row) = (0, 127);
    for row_dir in row_dirs.chars() {
        match row_dir {
            'F' => max_row = (max_row + min_row) / 2,
            'B' => min_row = (max_row + min_row) / 2 + 1,
            _ => unreachable!(),
        }
    }

    let (mut min_col, mut max_col) = (0, 7);
    for col_dir in col_dirs.chars() {
        match col_dir {
            'L' => max_col = (max_col + min_col) / 2,
            'R' => min_col = (max_col + min_col) / 2 + 1,
            _ => unreachable!(),
        }
    }

    min_row * 8 + min_col
}

/// Load contents of file and map each line to its seat id.
fn build_seat_ids() -> AnyResult<Vec<usize>> {
    Ok(lines("files/day5/input.txt")?
        .map(|line| get_seat_id(&line))
        .collect())
}

/// Receives a sorted array of seat ids, returns the max seat id, i.e. the last.
fn part1(seat_ids: &[usize]) -> Option<usize> {
    seat_ids.last().copied()
}

/// Received a sorted array of seat ids, traverses the array, at each element
/// we expect last + 1, if it's missing we test if the two contigous elements
/// are equal to last and last + 2.
fn part2(seat_ids: &[usize]) -> Option<usize> {
    let mut expecting = seat_ids[0] + 1;
    for (idx, seat_id) in seat_ids.iter().enumerate().skip(1) {
        if *seat_id != expecting
            && (seat_ids[idx - 1] == expecting - 1 && seat_ids[idx] == expecting + 1)
        {
            return Some(expecting);
        }

        expecting = seat_id + 1;
    }

    None
}

fn main() -> AnyResult<()> {
    let mut seat_ids = build_seat_ids()?;
    seat_ids.sort_unstable();
    println!("Day 5, Part 1: {:?}", part1(&seat_ids));
    println!("Day 5, Part 2: {:?}", part2(&seat_ids));

    Ok(())
}
