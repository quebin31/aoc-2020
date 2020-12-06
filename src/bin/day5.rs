use anyhow::Result as AnyResult;
use aoc_2020::lines;

/// Returns a partitioner function.
fn partitioner(l: char, r: char) -> impl Fn(&str, usize, usize) -> usize {
    move |arr: &str, mut lo: usize, mut hi: usize| -> usize {
        for d in arr.chars() {
            match d {
                d if d == l => hi = (hi + lo) / 2,
                d if d == r => lo = (hi + lo) / 2 + 1,
                _ => panic!("Unexpected char in string"),
            }
        }

        lo
    }
}

/// Get the seat id for a given line.
fn get_seat_id(line: &str) -> usize {
    let (rows, cols) = (&line[..7], &line[7..]);
    let row = partitioner('F', 'B')(rows, 0, 127);
    let col = partitioner('L', 'R')(cols, 0, 7);

    row * 8 + col
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
