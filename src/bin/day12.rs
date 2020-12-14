use anyhow::Result as AnyResult;
use aoc_2020::lines;

#[derive(Debug, Clone, Copy)]
enum Action {
    North(usize),
    South(usize),
    East(usize),
    West(usize),
    Left(usize),
    Right(usize),
    Forward(usize),
}

fn load_actions() -> AnyResult<Vec<Action>> {
    fn parse(line: String) -> Option<Action> {
        let mut chars = line.chars();

        let action_code = chars.next()?;
        let action_value = chars.collect::<String>().parse::<usize>().ok()?;

        let action = match action_code {
            'N' => Action::North(action_value),
            'S' => Action::South(action_value),
            'E' => Action::East(action_value),
            'W' => Action::West(action_value),
            'L' => Action::Left(action_value),
            'R' => Action::Right(action_value),
            'F' => Action::Forward(action_value),
            _ => return None,
        };

        Some(action)
    }

    Ok(lines("files/day12/input.txt")?.filter_map(parse).collect())
}

#[derive(Debug, Copy, Clone)]
struct Point(i32, i32);

impl Point {
    /// Rotate the point using a value in degrees (clockwise) using the
    /// rotation matrix for a 2d point.
    fn rotate(self, deg: f64) -> Self {
        // Convert degrees to radians
        let rad = deg.to_radians();
        let cos = rad.cos();
        let sin = rad.sin();

        let x = self.0 as f64 * cos + self.1 as f64 * sin;
        let y = self.0 as f64 * -sin + self.1 as f64 * cos;

        Self(x.round() as i32, y.round() as i32)
    }
}

#[derive(Debug, Clone)]
struct Ferry {
    pos: Point,
}

impl Ferry {
    fn new() -> Self {
        Self { pos: Point(0, 0) }
    }

    fn navigate_part_one(&mut self, actions: &[Action]) {
        // Let's have a unit vector indicating where are we facing
        // (1, 0) is over the +x axis (east)
        let mut facing = Point(1, 0);

        for action in actions {
            match *action {
                Action::East(val) => self.pos.0 += val as i32,
                Action::West(val) => self.pos.0 -= val as i32,
                Action::North(val) => self.pos.1 += val as i32,
                Action::South(val) => self.pos.1 -= val as i32,

                Action::Forward(val) => {
                    self.pos.0 += facing.0 * val as i32;
                    self.pos.1 += facing.1 * val as i32;
                }

                Action::Left(deg) => {
                    facing = facing.rotate(-(deg as f64));
                }

                Action::Right(deg) => {
                    facing = facing.rotate(deg as f64);
                }
            }
        }
    }

    fn navigate_part_two(&mut self, actions: &[Action]) {
        // Waypoint starting point
        let mut waypoint = Point(10, 1);

        for action in actions {
            match *action {
                Action::East(val) => waypoint.0 += val as i32,
                Action::West(val) => waypoint.0 -= val as i32,
                Action::North(val) => waypoint.1 += val as i32,
                Action::South(val) => waypoint.1 -= val as i32,

                Action::Left(deg) => waypoint = waypoint.rotate(-(deg as f64)),
                Action::Right(deg) => waypoint = waypoint.rotate(deg as f64),

                Action::Forward(val) => {
                    self.pos.0 += val as i32 * waypoint.0;
                    self.pos.1 += val as i32 * waypoint.1;
                }
            }
        }
    }

    fn manhattan_distance(&self) -> usize {
        self.pos.0.abs() as usize + self.pos.1.abs() as usize
    }
}

fn main() -> AnyResult<()> {
    let actions = load_actions()?;

    let mut ferry = Ferry::new();
    ferry.navigate_part_one(&actions);
    println!("Day 12, Part 1: {}", ferry.manhattan_distance());

    let mut ferry = Ferry::new();
    ferry.navigate_part_two(&actions);
    println!("Day 12, Part 2: {}", ferry.manhattan_distance());

    Ok(())
}
