use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use anyhow::Result as AnyResult;

/// Open a file and return an iterator that yields lines.
pub fn lines(path: impl AsRef<Path>) -> AnyResult<impl Iterator<Item = String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(reader.lines().filter_map(|line| line.ok()))
}
