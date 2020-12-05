use anyhow::Result as AnyResult;
use aoc_2020::lines;

/// Load the file with the numbers and sort the array.
fn load_and_sort() -> AnyResult<Vec<i32>> {
    let mut numbers = lines("files/day1/input.txt")?
        .filter_map(|number| number.parse().ok())
        .collect::<Vec<i32>>();
    numbers.sort_unstable();
    Ok(numbers)
}

/// Naive solution:
/// Needs two traverse the array at least (n * (n - 1))/ 2 times, i.e. O(n^2).
///
/// Optimized solution:
/// Receives a sorted array, i.e. O(n log(n)). Then traverses the array
/// one time, for each element calculates how much it needs to reach 2020, and
/// performs a binary search, i.e. O(n log(n)), if the element is in the array
/// we found both numbers. Total O(n log(n)).
fn part1(numbers: &[i32]) -> Option<i32> {
    numbers.iter().find_map(|number| {
        if *number > 2020 {
            return None;
        }

        let comp = 2020 - number;
        numbers.binary_search(&comp).ok().map(|_| number * comp)
    })
}

/// Same logic as before but instead now we traverse n(n - 1) / 2 times the array
/// (two for's), sum both numbers and search the complement to 2020, this takes
/// O(n^2 * log(n)) which is already better than O(n^3) for a naive solution (three for's).
fn part2(numbers: &[i32]) -> Option<i32> {
    numbers.iter().enumerate().find_map(|(idx, number_a)| {
        numbers.iter().skip(idx + 1).find_map(|number_b| {
            let sum = number_a + number_b;
            if sum > 2020 {
                return None;
            }

            let comp = 2020 - sum;
            numbers
                .binary_search(&comp)
                .ok()
                .map(|_| number_a * number_b * comp)
        })
    })
}

fn main() -> AnyResult<()> {
    let numbers = load_and_sort()?;

    let part1_result = part1(&numbers);
    println!("Day 1, Part 1: {:?}", part1_result);

    let part2_result = part2(&numbers);
    println!("Dat 2, Part 2: {:?}", part2_result);

    Ok(())
}
