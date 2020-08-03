// Day 5: Alchemical Reduction

use std::fs::read_to_string;
use std::path::PathBuf;

use crate::error::TResult;

#[allow(dead_code)]
fn solve_part1(path: PathBuf) -> TResult<u32> {
    let contents = read_to_string(path)?;

    let polymer = contents.trim();
    let reacted_polymer = react(polymer);

    Ok(reacted_polymer.len() as u32)
}

#[allow(dead_code)]
fn solve_part2(path: PathBuf) -> TResult<u32> {
    let contents = read_to_string(path)?;

    let polymer = contents.trim();
    let mut shortest = polymer.len();
    for c in b'A'..b'Z' {
        let upper_unit = c as char;
        let lower_unit = upper_unit.to_ascii_lowercase();

        let cleaned_polymer = polymer.replace(lower_unit, "").replace(upper_unit, "");
        let reacted_polymer = react(&cleaned_polymer);
        if reacted_polymer.len() < shortest {
            shortest = reacted_polymer.len();
        }
    }

    Ok(shortest as u32)
}

//
// Utility functions
//

fn react(polymer: &str) -> String {
    let mut polymer: Vec<char> = polymer.chars().collect();
    let mut reacted_polymer: Vec<char> = vec![];

    loop {
        let mut idx = 1;

        while idx < polymer.len() {
            let first_unit = polymer[idx - 1];
            let second_unit = polymer[idx];

            if !can_react(first_unit, second_unit) {
                reacted_polymer.push(first_unit);
            } else {
                idx += 1;
            }

            idx += 1;

            if idx == polymer.len() {
                reacted_polymer.push(polymer[idx - 1]);
            }
        }

        if polymer == reacted_polymer {
            break;
        }

        polymer = reacted_polymer.clone();
        reacted_polymer.clear();
    }

    reacted_polymer.iter().collect::<String>()
}

fn can_react(first_unit: char, second_unit: char) -> bool {
    if first_unit == second_unit {
        return false;
    }

    first_unit.to_ascii_lowercase() == second_unit.to_ascii_lowercase()
}

//
// Tests
//

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_able_to_react() {
        assert!(can_react('a', 'A'), "aA");
        assert!(can_react('A', 'a'), "Aa");
        assert!(can_react('c', 'C'), "cC");
        assert!(can_react('z', 'Z'), "zZ");
    }

    #[test]
    fn cannot_react() {
        assert!(!can_react('a', 'a'), "aa");
        assert!(!can_react('A', 'A'), "AA");
        assert!(!can_react('z', 'a'), "za");
        assert!(!can_react('Z', 'A'), "ZA");
        assert!(!can_react('z', 'A'), "zA");
        assert!(!can_react('w', 'z'), "wz");
    }

    #[test]
    fn aoc05_part1_fixture() {
        let input = PathBuf::from("tests/fixtures/aoc05_1.txt");
        assert_eq!(solve_part1(input).unwrap(), 10);
    }

    #[test]
    fn aoc05_part1() {
        let input = PathBuf::from("tests/inputs/aoc05.txt");
        assert_eq!(solve_part1(input).unwrap(), 10598);
    }

    #[test]
    fn aoc05_part2_fixture() {
        let input = PathBuf::from("tests/fixtures/aoc05_1.txt");
        assert_eq!(solve_part2(input).unwrap(), 4);
    }

    #[test]
    fn aoc05_part2() {
        let input = PathBuf::from("tests/inputs/aoc05.txt");
        assert_eq!(solve_part2(input).unwrap(), 5312);
    }
}
