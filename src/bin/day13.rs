use anyhow::{anyhow, Result as AnyResult};
use aoc_2020::lines;

#[derive(Debug, Clone)]
struct Notes {
    earliest_ts: usize,
    bus_ids: Vec<usize>,
}

impl Notes {
    fn load() -> AnyResult<Self> {
        let mut lines = lines("files/day13/input.txt")?;

        let earliest_ts = lines.next().ok_or_else(|| anyhow!("Empty file"))?;
        let earliest_ts = earliest_ts.parse()?;

        let bus_ids = lines
            .next()
            .ok_or_else(|| anyhow!("Empty file"))?
            .split(',')
            .filter_map(|s| s.parse().ok())
            .collect();

        Ok(Self {
            earliest_ts,
            bus_ids,
        })
    }

    fn best_bus(&self) -> Option<(usize, usize)> {
        let mut best_bus = None;
        let mut best_wait_time = None;

        for bus_id in &self.bus_ids {
            let factor = (self.earliest_ts / bus_id) + 1;
            let wait_time = (factor * bus_id) - self.earliest_ts;

            let best_bus = best_bus.get_or_insert(bus_id);
            let best_wait_time = best_wait_time.get_or_insert(wait_time);

            if *best_wait_time > wait_time {
                *best_bus = bus_id;
                *best_wait_time = wait_time;
            }
        }

        match (best_bus, best_wait_time) {
            (Some(best_bus), Some(best_wait_time)) => Some((*best_bus, best_wait_time)),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
struct Departures {
    bus_ids: Vec<(usize, usize)>,
}

impl Departures {
    fn load() -> AnyResult<Self> {
        let bus_ids = lines("files/day13/input.txt")?
            .nth(1)
            .ok_or_else(|| anyhow!("Empty file"))?
            .split(',')
            .enumerate()
            .filter_map(|(i, s)| s.parse().ok().map(|n| (i, n)))
            .collect();

        Ok(Self { bus_ids })
    }

    fn solve(&self) -> Option<usize> {
        let (remainders, modulis): (Vec<i64>, Vec<i64>) = self
            .bus_ids
            .iter()
            .copied()
            .map(|(offset, id)| ((id as i64 - offset as i64) % id as i64, id as i64))
            .unzip();

        ring_algorithm::chinese_remainder_theorem(&remainders, &modulis).map(|v| v as usize)
    }
}

fn main() -> AnyResult<()> {
    let notes = Notes::load()?;
    let bus_times_wait = notes.best_bus().map(|(bus, wait)| bus * wait);
    println!("Day 13, Part 1: {:?}", bus_times_wait);

    let departures = Departures::load()?;
    println!("{:?}", departures);
    println!("Day 13, Part 2: {:?}", departures.solve());

    Ok(())
}
