use anyhow::Result as AnyResult;
use aoc_2020::lines;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum State {
    Floor,
    Empty,
    Occupied,
}

#[derive(Debug, Copy, Clone)]
enum NewStateOp {
    PartOne,
    PartTwo,
}

#[derive(Debug, Clone)]
struct Layout {
    map: Vec<Vec<State>>,
}

impl Layout {
    fn load() -> AnyResult<Self> {
        fn parse(line: String) -> Vec<State> {
            line.chars()
                .map(|c| match c {
                    '.' => State::Floor,
                    'L' => State::Empty,
                    '#' => State::Occupied,
                    c => panic!("Bad char: {}", c),
                })
                .collect()
        }

        let map = lines("files/day11/input.txt")?.map(parse).collect();
        Ok(Self { map })
    }

    fn occupied(&self) -> usize {
        self.map.iter().fold(0, |acc, row| {
            acc + row.iter().fold(0, |acc, state| {
                if let State::Occupied = state {
                    acc + 1
                } else {
                    acc
                }
            })
        })
    }

    #[allow(clippy::needless_range_loop)]
    fn step(&mut self, op: NewStateOp) -> bool {
        let mut temp_map = self.map.clone();
        let rows = temp_map.len();
        let cols = temp_map.get(0).map_or(0, |r| r.len());

        for i in 0..rows {
            for j in 0..cols {
                temp_map[i][j] = match op {
                    NewStateOp::PartOne => self.new_state_part_one(i, j),
                    NewStateOp::PartTwo => self.new_state_part_two(i, j),
                };
            }
        }

        if self.map != temp_map {
            self.map = temp_map;
            true
        } else {
            false
        }
    }

    fn get(&self, i: i32, j: i32) -> Option<&State> {
        if i < 0 || j < 0 {
            None
        } else {
            self.map
                .get(i as usize)
                .map(|row| row.get(j as usize))
                .flatten()
        }
    }

    fn new_state_part_one(&self, r: usize, c: usize) -> State {
        if let State::Floor = self.map[r][c] {
            return State::Floor;
        }

        let mut occupied = 0;
        'main: for i in -1..=1i32 {
            for j in -1..=1i32 {
                if i == 0 && j == 0 {
                    continue;
                }

                let maybe_seat = self.get(r as i32 + i, c as i32 + j);
                if let Some(State::Occupied) = maybe_seat {
                    occupied += 1;
                }

                if occupied >= 4 {
                    break 'main;
                }
            }
        }

        match self.map[r][c] {
            State::Empty if occupied == 0 => State::Occupied,
            State::Occupied if occupied >= 4 => State::Empty,
            state => state,
        }
    }

    fn new_state_part_two(&self, r: usize, c: usize) -> State {
        if let State::Floor = self.map[r][c] {
            return State::Floor;
        }

        let mut occupied = 0;
        'main: for i in -1..=1i32 {
            for j in -1..=1i32 {
                if i == 0 && j == 0 {
                    continue;
                }

                let mut ii = r as i32 + i;
                let mut jj = c as i32 + j;
                while let Some(state) = self.get(ii, jj) {
                    match state {
                        State::Floor => {
                            ii += i;
                            jj += j;
                        }

                        State::Empty => {
                            break;
                        }

                        State::Occupied => {
                            occupied += 1;
                            break;
                        }
                    }
                }

                if occupied >= 5 {
                    break 'main;
                }
            }
        }

        match self.map[r][c] {
            State::Empty if occupied == 0 => State::Occupied,
            State::Occupied if occupied >= 5 => State::Empty,
            state => state,
        }
    }
}

fn main() -> AnyResult<()> {
    let mut layout = Layout::load()?;
    while layout.step(NewStateOp::PartOne) {}
    println!("Day 11, Part 1: {}", layout.occupied());

    let mut layout = Layout::load()?;
    while layout.step(NewStateOp::PartTwo) {}
    println!("Day 11, Part 2: {}", layout.occupied());

    Ok(())
}
