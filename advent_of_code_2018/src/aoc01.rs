// Day 1: Chronal Calibration

use std::collections::HashSet;
use std::fs::{read_to_string, File};
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use crate::error::TResult;

#[allow(dead_code)]
fn solve_part1(path: PathBuf) -> TResult<i32> {
    let reader = match File::open(path) {
        Ok(f) => BufReader::new(f),
        Err(e) => return Err(e.into()),
    };

    let mut freq: i32 = 0;
    for line in reader.lines() {
        if let Ok(line) = line {
            let delta: i32 = line.parse()?;
            freq += delta;
        }
    }

    Ok(freq)
}

#[allow(dead_code)]
fn solve_part2(path: PathBuf) -> TResult<i32> {
    let content = read_to_string(path)?;

    let mut freq: i32 = 0;
    let mut seen = HashSet::new();

    seen.insert(freq);
    loop {
        for line in content.lines() {
            let delta: i32 = line.parse()?;
            freq += delta;
            if seen.contains(&freq) {
                return Ok(freq);
            }
            seen.insert(freq);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc01_part1() {
        let input = PathBuf::from("tests/inputs/aoc01.txt");
        assert_eq!(solve_part1(input).unwrap(), 474);
    }

    #[test]
    fn aoc01_part2() {
        let input = PathBuf::from("tests/inputs/aoc01.txt");
        assert_eq!(solve_part2(input).unwrap(), 137041);
    }
}
