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
    Part1,
    Part2,
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
                    NewStateOp::Part1 => self.new_state_p1(i, j),
                    NewStateOp::Part2 => self.new_state_p2(i, j),
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

    fn new_state_p1(&self, r: usize, c: usize) -> State {
        if let State::Floor = self.map[r][c] {
            return State::Floor;
        }

        let mut occupied = 0;
        'row_loop: for i in r.saturating_sub(1)..=(r + 1) {
            for j in c.saturating_sub(1)..=(c + 1) {
                if i == r && j == c {
                    continue;
                }

                let maybe_seat = self.map.get(i).map(|r| r.get(j)).flatten();
                if let Some(State::Occupied) = maybe_seat {
                    occupied += 1;
                }

                if occupied >= 4 {
                    break 'row_loop;
                }
            }
        }

        match self.map[r][c] {
            State::Empty if occupied == 0 => State::Occupied,
            State::Occupied if occupied >= 4 => State::Empty,
            state => state,
        }
    }

    fn new_state_p2(&self, r: usize, c: usize) -> State {
        if let State::Floor = self.map[r][c] {
            return State::Floor;
        }

        let mut occupied = 0;
        for level in 1.. {
            for i in (r.saturating_sub(level)..=(r + level)).step_by(level) {
                for j in (c.saturating_sub(level)..=(c + level)).step_by(level) {
                    if i == r && j == c {
                        continue;
                    }
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
    while layout.step(NewStateOp::Part1) {}
    println!("Day 11, Part 1: {}", layout.occupied());

    let mut layout = Layout::load()?;
    while layout.step(NewStateOp::Part2) {}
    println!("Day 11, Part 2: {}", layout.occupied());

    Ok(())
}
