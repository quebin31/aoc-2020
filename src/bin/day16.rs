use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use anyhow::{anyhow, Error, Result as AnyResult};
use aoc_2020::lines;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Clone)]
struct Range(usize, usize);

impl Range {
    fn contains(&self, val: usize) -> bool {
        self.0 <= val && val <= self.1
    }
}

#[derive(Debug, Clone)]
struct ClassRange {
    label: String,
    first: Range,
    second: Range,
}

impl FromStr for ClassRange {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref REGEX: Regex =
                Regex::new("^([[:alpha:] ]*): (\\d+)-(\\d+) or (\\d+)-(\\d+)$").unwrap();
        }

        let captures = REGEX.captures(s).ok_or_else(|| anyhow!("No matches"))?;

        let label = captures[1].to_string();
        let first = Range(captures[2].parse()?, captures[3].parse()?);
        let second = Range(captures[4].parse()?, captures[5].parse()?);

        Ok(Self {
            label,
            first,
            second,
        })
    }
}

impl ClassRange {
    fn contains(&self, val: usize) -> bool {
        self.first.contains(val) || self.second.contains(val)
    }
}

#[derive(Debug, Clone)]
struct Ticket(Vec<usize>);

impl Ticket {
    fn find_invalid_value(&self, class_ranges: &HashMap<String, ClassRange>) -> Option<usize> {
        for val in &self.0 {
            let valid_for_any = class_ranges.values().any(|range| range.contains(*val));

            if !valid_for_any {
                return Some(*val);
            }
        }

        None
    }
}

#[derive(Debug, Clone)]
struct Notes {
    class_ranges: HashMap<String, ClassRange>,
    my_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

impl Notes {
    fn load() -> AnyResult<Self> {
        let mut lines = lines("files/day16/input.txt")?;

        let mut class_ranges = HashMap::new();
        while let Some(line) = lines.next() {
            if line.trim().is_empty() {
                break;
            }

            let class_range = ClassRange::from_str(&line)?;
            class_ranges.insert(class_range.label.clone(), class_range);
        }

        lines.next(); // skip label
        let my_ticket = lines
            .next()
            .ok_or_else(|| anyhow!("Empty line"))?
            .split(',')
            .map(|n| n.parse())
            .collect::<Result<Vec<_>, _>>()?;

        lines.nth(1); // skip empty line and label
        let mut nearby_tickets = Vec::new();
        for line in lines {
            if line.trim().is_empty() {
                break;
            }

            let ticket = line
                .split(',')
                .map(|n| n.parse())
                .collect::<Result<Vec<_>, _>>()?;

            nearby_tickets.push(Ticket(ticket));
        }

        Ok(Self {
            class_ranges,
            my_ticket: Ticket(my_ticket),
            nearby_tickets,
        })
    }

    fn scanning_error_rate(&self) -> usize {
        self.nearby_tickets
            .iter()
            .filter_map(|ticket| ticket.find_invalid_value(&self.class_ranges))
            .sum()
    }

    fn discard_invalid_tickets(&mut self) {
        self.nearby_tickets = self
            .nearby_tickets
            .clone()
            .into_iter()
            .filter(|ticket| ticket.find_invalid_value(&self.class_ranges).is_none())
            .collect();
    }

    fn find_out_labels_order(&self) -> Vec<&str> {
        let ticket_len = self.my_ticket.0.len();
        let labels: HashSet<_> = self.class_ranges.keys().map(|k| k.as_str()).collect();

        let mut valid_per_column = Vec::new();
        for position in 0..ticket_len {
            let mut invalid_labels = HashSet::new();

            for ticket in &self.nearby_tickets {
                for (label, range) in &self.class_ranges {
                    if !range.contains(ticket.0[position]) {
                        invalid_labels.insert(label.as_str());
                    }
                }
            }

            let valid_labels: HashSet<_> = labels.difference(&invalid_labels).cloned().collect();
            valid_per_column.push(valid_labels);
        }

        let mut sorted: Vec<_> = valid_per_column
            .iter()
            .map(|valid| valid.len())
            .enumerate()
            .collect();

        sorted.sort_unstable_by(|(_, a), (_, b)| a.cmp(b));
        for (start, (idx_a, _)) in sorted.iter().enumerate() {
            assert_eq!(valid_per_column[*idx_a].len(), 1);
            let label = valid_per_column[*idx_a].iter().next().unwrap().to_owned();

            for (idx_b, _) in &sorted[start + 1..] {
                valid_per_column[*idx_b].remove(label);
            }
        }

        valid_per_column
            .into_iter()
            .map(|mut s| s.drain().next().unwrap())
            .collect()
    }
}

fn main() -> AnyResult<()> {
    let mut notes = Notes::load()?;

    println!("Day 16, Part 1: {}", notes.scanning_error_rate());

    notes.discard_invalid_tickets();
    let order = notes.find_out_labels_order();
    let product: usize = order
        .iter()
        .enumerate()
        .filter_map(|(idx, label)| {
            if label.contains("departure") {
                Some(notes.my_ticket.0[idx])
            } else {
                None
            }
        })
        .product();

    println!("Day 16, Part 2: {:?}", product);
    Ok(())
}
