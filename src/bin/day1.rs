use anyhow::Result as AnyResult;
use aoc_2020::lines;

/// One possible naive solution:
/// Needs two traverse the array at least (n * (n - 1))/ 2 times, i.e. O(n^2)
/// This solution sorts the array, i.e. O(n log(n)). Then traverses the array
/// one time, for each element calculates how much it needs to reach 2020, and
/// performs a binary search, i.e. O(n log(n)), if the element is in the array
/// we found both numbers. Total O(n log(n)).
fn quest1() -> AnyResult<Option<i32>> {
    let mut numbers = lines("files/day1/quest1.txt")?
        .filter_map(|number| number.parse().ok())
        .collect::<Vec<i32>>();

    numbers.sort_unstable();

    for number in &numbers {
        if *number > 2020 {
            break;
        }

        let comp = 2020 - number;
        if numbers.binary_search(&comp).is_ok() {
            return Ok(Some(number * comp));
        }
    }

    Ok(None)
}

/// Same logic as before but instead now we traverse n(n - 1) / 2 times the array
/// (two for's), sum both numbers and search the complement to 2020, this takes
/// O(n^2 * log(n)) which is already better than O(n^3) for naive solution (three for's).
fn quest2() -> AnyResult<Option<i32>> {
    let mut numbers = lines("files/day1/quest1.txt")?
        .filter_map(|number| number.parse().ok())
        .collect::<Vec<i32>>();

    numbers.sort_unstable();

    for (idx, number_a) in numbers.iter().enumerate() {
        for number_b in numbers.iter().skip(idx + 1) {
            let sum = number_a + number_b;
            if sum > 2020 {
                break;
            }

            let comp = 2020 - sum;
            if numbers.binary_search(&comp).is_ok() {
                return Ok(Some(number_a * number_b * comp));
            }
        }
    }

    Ok(None)
}

fn main() -> AnyResult<()> {
    let quest1_result = quest1()?;
    println!("Day 1, Quest 1: {:?}", quest1_result);

    let quest2_result = quest2()?;
    println!("Dat 2, Quest 2: {:?}", quest2_result);

    Ok(())
}
