// Day 2: Inventory Management System

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use crate::error::TResult;

#[allow(dead_code)]
fn solve_part1(path: PathBuf) -> TResult<u16> {
    let reader = match File::open(path) {
        Ok(f) => BufReader::new(f),
        Err(e) => return Err(e.into()),
    };

    let (mut twice_counter, mut triple_counter) = (0_u16, 0_u16);
    let mut freq: HashMap<u8, u16> = HashMap::new();

    for line in reader.lines() {
        freq.clear();
        if let Ok(line) = line {
            for b in line.into_bytes() {
                let counter = freq.entry(b).or_insert(0);
                *counter += 1;
            }
        }

        if freq.values().any(|&f| f == 2) {
            twice_counter += 1;
        }

        if freq.values().any(|&f| f == 3) {
            triple_counter += 1;
        }
    }

    Ok(twice_counter * triple_counter)
}

#[allow(dead_code)]
fn solve_part2(path: PathBuf) -> TResult<String> {
    let reader = match File::open(path) {
        Ok(f) => BufReader::new(f),
        Err(e) => return Err(e.into()),
    };

    Ok("".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc02_part1_fixture_1() {
        let input = PathBuf::from("tests/fixtures/aoc02_1.txt");
        assert_eq!(solve_part1(input).unwrap(), 12);
    }

    #[test]
    fn aoc02_part1() {
        let input = PathBuf::from("tests/inputs/aoc02.txt");
        assert_eq!(solve_part1(input).unwrap(), 8118);
    }

    #[test]
    fn aoc02_part2_fixture_1() {
        let input = PathBuf::from("tests/fixtures/aoc02_2.txt");
        assert_eq!(solve_part2(input).unwrap(), "fgij".to_string());
    }

    //    #[test]
    //    fn aoc02_part2() {
    //        let input = PathBuf::from("tests/inputs/aoc02.txt");
    //        assert_eq!(solve_part2(input).unwrap(), 137041);
    //    }
}
