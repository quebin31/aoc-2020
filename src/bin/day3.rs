use anyhow::Result as AnyResult;
use aoc_2020::lines;

/// Hold the map as a matrix.
struct Map {
    map: Vec<Vec<char>>,
}

impl Map {
    /// Load and build the map
    fn build_map() -> AnyResult<Self> {
        let map = lines("files/day3/input.txt")?
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        Ok(Self { map })
    }

    /// Traverse the map by turning right `right` steps and down `down` steps
    /// repeteadly until we reach the bottom part of the map. Returns the number
    /// of trees we found.
    fn traverse(&self, right: usize, down: usize) -> usize {
        let max_y = self.map.len();
        let max_x = self.map[0].len();
        let (mut x, mut y) = (0, 0);

        let mut tree_count = 0;
        while (y + down) < max_y {
            x = (x + right) % max_x;
            y += down;

            if self.map[y][x] == '#' {
                tree_count += 1;
            }
        }

        tree_count
    }
}

/// Initial traverse.
fn part1(map: &Map) -> usize {
    map.traverse(3, 1)
}

/// Traverse with different configurations and return the product.
fn part2(map: &Map) -> usize {
    let params = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    params
        .iter()
        .map(|(right, down)| map.traverse(*right, *down))
        .product()
}

fn main() -> AnyResult<()> {
    let map = Map::build_map()?;

    let part1_result = part1(&map);
    println!("Day 3, Part 1: {}", part1_result);

    let part2_result = part2(&map);
    println!("Day 3, Part 2: {}", part2_result);

    Ok(())
}
