// Day 1: Chronal Calibration

use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

fn run(path: PathBuf) -> Result<i32, Box<dyn Error>> {
    let reader = match File::open(path) {
        Ok(f) => BufReader::new(f),
        Err(e) => return Err(e.into()),
    };

    let mut freq: i32 = 0;
    for line in reader.lines() {
        if let Ok(line) = line {
            let delta: i32 = match line.parse() {
                Ok(i) => i,
                Err(e) => return Err(e.into()),
            };
            freq += delta;
        }
    }

    Ok(freq)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc01() {
        let input = PathBuf::from("tests/inputs/aoc01.txt");
        assert_eq!(run(input).unwrap(), 474);
    }
}
