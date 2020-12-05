use anyhow::Result as AnyResult;
use aoc_2020::lines;

struct Map {
    map: Vec<Vec<char>>,
}

impl Map {
    fn build_map() -> AnyResult<Self> {
        let map = lines("files/day3/input.txt")?
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        Ok(Self { map })
    }

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

fn quest1(map: &Map) -> usize {
    map.traverse(3, 1)
}

fn quest2(map: &Map) -> usize {
    let params = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    params
        .iter()
        .map(|(right, down)| map.traverse(*right, *down))
        .product()
}

fn main() -> AnyResult<()> {
    let map = Map::build_map()?;

    let quest1_result = quest1(&map);
    println!("Day 3, Quest 1: {}", quest1_result);

    let quest2_result = quest2(&map);
    println!("Day 3, Quest 2: {}", quest2_result);

    Ok(())
}
