use std::collections::HashMap;

use anyhow::Result as AnyResult;
use aoc_2020::lines;
use boolinator::Boolinator;

/// Single bag, holds a list of bags ids and the required number of them.
#[derive(Debug, Clone)]
struct Bag {
    contains: Vec<(String, usize)>,
}

/// Bags graph represented as an adjacency list.
#[derive(Debug, Clone)]
struct BagsGraph {
    list: HashMap<String, Bag>,
}

/// Helper to hold operation which finds bags that directly or indirectly
/// can hold an specific bag by its id.
#[derive(Debug, Clone)]
struct Holds<'a> {
    graph: &'a BagsGraph,
    results: HashMap<&'a str, Option<bool>>,
}

impl<'a> Holds<'a> {
    fn new(graph: &'a BagsGraph) -> Self {
        Self {
            graph,
            results: HashMap::new(),
        }
    }

    /// Check whether we already visited one bag (node).
    fn visited(&self, target: &str) -> bool {
        self.results.contains_key(target)
    }

    /// Check by resursive processing if `curr` can hold `target`, if we
    /// already known some bag can hold it we return early.
    fn holds(&mut self, curr: &'a str, target: &str) -> bool {
        if self.results.get(curr).copied().flatten().unwrap_or(false) {
            return true;
        }

        self.results.insert(curr, None);
        for (inner, _) in &self.graph.list[curr].contains {
            if inner == target || self.holds(inner, target) {
                self.results.insert(curr, Some(true));
                return true;
            }
        }

        self.results.insert(curr, Some(false));
        false
    }
}

impl<'a> IntoIterator for Holds<'a> {
    type Item = (&'a str, Option<bool>);
    type IntoIter = std::collections::hash_map::IntoIter<&'a str, Option<bool>>;

    fn into_iter(self) -> Self::IntoIter {
        self.results.into_iter()
    }
}

impl BagsGraph {
    /// Load and parse the input file to build the adjacency list.
    fn load() -> AnyResult<Self> {
        fn parse(line: String) -> Option<(String, Bag)> {
            let mut splitted = line.split("contain");

            let this_bag = splitted.next()?;
            let this_bag = this_bag.trim().split(' ').take(2).collect::<String>();

            let contains = splitted.next()?;
            let contains = contains.trim().split(',');

            let contains = contains
                .filter_map(|item| -> Option<(String, usize)> {
                    let mut splitted = item.trim().split(' ');

                    let qty = match splitted.next()?.parse::<usize>() {
                        Ok(num) => num,
                        Err(_) => return None,
                    };

                    let color = splitted.take(2).collect::<String>();

                    Some((color, qty))
                })
                .collect();

            Some((this_bag, Bag { contains }))
        }

        let list = lines("files/day7/input.txt")?.filter_map(parse).collect();
        Ok(Self { list })
    }

    /// Build a `Holds` structure and start computing `Holds::holds` for every node that
    /// wasn't yet visited, return an iterator which already filtered bags that can't
    /// hold `target`.
    fn holds<'a>(&'a self, target: &str) -> impl Iterator<Item = &'a str> {
        let mut holds = Holds::new(self);
        for bag in self.list.keys() {
            // Process non-visited nodes
            if !holds.visited(bag.as_str()) {
                holds.holds(bag, target);
            }
        }

        holds
            .into_iter()
            .filter_map(|(bag, hold)| hold.unwrap_or(false).as_some(bag))
    }

    /// Count how many individual bags are required inside `target`.
    fn inside_count(&self, target: &str) -> usize {
        let mut total = 0;
        for (inner, qty) in &self.list[target].contains {
            let inner_qty = self.inside_count(inner);
            total += if inner_qty != 0 {
                qty + qty * inner_qty
            } else {
                *qty
            };
        }

        total
    }
}

fn main() -> AnyResult<()> {
    let bags = BagsGraph::load()?;
    println!("Day 7, Part 1: {}", bags.holds("shinygold").count());
    println!("Day 7, Part 2: {}", bags.inside_count("shinygold"));

    Ok(())
}
