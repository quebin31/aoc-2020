use std::collections::HashSet;

use anyhow::Result as AnyResult;
use aoc_2020::lines;

/// Groups of 'yes' answers.
type GroupAnswers = HashSet<char>;

/// Group together all answers from each group to which anyone
/// in it answered 'yes'.
fn anyone_yes_answers() -> AnyResult<Vec<GroupAnswers>> {
    let mut groups = Vec::new();
    let mut group_yes_answers = HashSet::new();

    for line in lines("files/day6/input.txt")? {
        if line.trim().is_empty() {
            // Finished collecting answers from one group
            groups.push(group_yes_answers);
            group_yes_answers = HashSet::new();
        } else {
            group_yes_answers.extend(line.chars());
        }
    }

    if !group_yes_answers.is_empty() {
        groups.push(group_yes_answers);
    }

    Ok(groups)
}

/// Group together all answers from each group to which everyone
/// in it answered 'yes'.
fn everyone_yes_answers() -> AnyResult<Vec<GroupAnswers>> {
    let mut groups = Vec::new();
    let mut group_yes_answers = HashSet::new();

    // Used to initialized the group yes answers set.
    let mut first_person = true;

    for line in lines("files/day6/input.txt")? {
        if line.trim().is_empty() {
            // Finished collecting answers from one group
            groups.push(group_yes_answers);
            group_yes_answers = HashSet::new();

            first_person = true;
        } else {
            let person_yes_answers: HashSet<_> = line.chars().collect();

            group_yes_answers = if first_person {
                person_yes_answers
            } else {
                group_yes_answers
                    .intersection(&person_yes_answers)
                    .copied()
                    .collect()
            };

            first_person = false;
        }
    }

    if !group_yes_answers.is_empty() {
        groups.push(group_yes_answers);
    }

    Ok(groups)
}

/// Count 'yes' answers from an array of group answers.
fn count_yes_answers(groups: &[GroupAnswers]) -> usize {
    groups.iter().map(|group| group.len()).sum()
}

fn main() -> AnyResult<()> {
    println!(
        "Day 6, Part 1: {}",
        count_yes_answers(&anyone_yes_answers()?)
    );
    println!(
        "Day 6, Part 2: {}",
        count_yes_answers(&everyone_yes_answers()?)
    );

    Ok(())
}
