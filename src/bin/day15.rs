use std::collections::HashMap;

use anyhow::{anyhow, Result as AnyResult};

#[derive(Debug, Clone)]
struct NumberInfo {
    last_idx: usize,
    before_last_idx: Option<usize>,
}

#[derive(Debug, Clone)]
struct Game {
    memory: HashMap<usize, NumberInfo>,
    turn: usize,
    last_num: usize,
}

impl Game {
    fn new(starting_numbers: &[usize]) -> Option<Self> {
        let mut memory = HashMap::new();
        for (idx, num) in starting_numbers.iter().enumerate() {
            memory.insert(
                *num,
                NumberInfo {
                    last_idx: idx,
                    before_last_idx: None,
                },
            );
        }

        let turn = starting_numbers.len();
        let last_num = *starting_numbers.last()?;

        Some(Self {
            memory,
            turn,
            last_num,
        })
    }

    fn simulate(&mut self, until: usize) {
        while self.turn <= until {
            if let Some(info) = self.memory.get(&self.last_num) {
                if let Some(before_last_idx) = info.before_last_idx {
                    self.last_num = info.last_idx - before_last_idx;
                } else {
                    self.last_num = 0;
                };

                if let Some(info) = self.memory.get_mut(&self.last_num) {
                    info.before_last_idx = Some(info.last_idx);
                    info.last_idx = self.turn;
                } else {
                    self.memory.insert(
                        self.last_num,
                        NumberInfo {
                            last_idx: self.turn,
                            before_last_idx: None,
                        },
                    );
                }
            }

            self.turn += 1;
        }
    }
}

fn main() -> AnyResult<()> {
    let mut game = Game::new(&[1, 0, 18, 10, 19, 6]).ok_or_else(|| anyhow!("Invalid array"))?;
    // Turn 2020 starting from 1
    game.simulate(2020 - 1);
    println!("Day 15, Part 1: {}", game.last_num);

    let mut game = Game::new(&[1, 0, 18, 10, 19, 6]).ok_or_else(|| anyhow!("Invalid array"))?;
    // Turn 30000000 starting from 1
    // cargo run --bin day15 --release, go brrrr
    game.simulate(30000000 - 1);
    println!("Day 15, Part 2: {}", game.last_num);

    Ok(())
}
