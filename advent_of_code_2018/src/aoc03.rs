// Day 2: Inventory Management System

use std::collections::HashMap;
use std::fs::read_to_string;
use std::iter::{IntoIterator, Iterator};
use std::path::PathBuf;

use lazy_static::lazy_static;
use regex::Regex;

use crate::error::{TError, TResult};

type Point = (i32, i32);

#[allow(dead_code)]
fn prepare(path: PathBuf) -> TResult<(Vec<Claim>, HashMap<Point, u32>)> {
    let content = read_to_string(path)?;
    let claims = content
        .lines()
        .map(|line| Claim::parse(line))
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let mut points: HashMap<Point, u32> = HashMap::new();

    claims.iter().for_each(|claim| {
        claim.into_iter().for_each(|point| {
            *points.entry(point).or_insert(0) += 1;
        })
    });

    Ok((claims, points))
}

#[allow(dead_code)]
fn solve_part1(path: PathBuf) -> TResult<u32> {
    let (_, points) = prepare(path)?;

    Ok(points.values().filter(|&&count| count > 1).count() as u32)
}

#[allow(dead_code)]
fn solve_part2(path: PathBuf) -> TResult<i32> {
    let (claims, points) = prepare(path)?;

    for claim in claims {
        if claim.into_iter().all(|point| points[&point] == 1) {
            return Ok(claim.id);
        }
    }

    Err("not found".into())
}

#[derive(Debug)]
struct Claim {
    id: i32,
    margin_left: i32,
    margin_top: i32,
    width: i32,
    height: i32,
}

impl Claim {
    fn parse(s: &str) -> Result<Self, TError> {
        lazy_static! {
        static ref PARSER: Regex = Regex::new(
            r"#(?P<id>\d+) @ (?P<margin_left>\d+),(?P<margin_top>\d+): (?P<width>\d+)x(?P<height>\d+)",
        )
            .unwrap();
        }

        let caps = match PARSER.captures(s) {
            Some(c) => c,
            None => return Err("cannot parse".into()),
        };

        Ok(Self {
            id: caps["id"].parse()?,
            margin_left: caps["margin_left"].parse()?,
            margin_top: caps["margin_top"].parse()?,
            width: caps["width"].parse()?,
            height: caps["height"].parse()?,
        })
    }
}

#[derive(Debug)]
struct ClaimIter<'a> {
    claim: &'a Claim,
    p: Point,
}

impl<'a> Iterator for ClaimIter<'a> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        self.p.0 += 1;

        if self.p.0 >= self.claim.margin_left + self.claim.width {
            self.p.0 = self.claim.margin_left;
            self.p.1 += 1;
        }

        if self.p.1 >= self.claim.margin_top + self.claim.height {
            return None;
        }

        Some(self.p)
    }
}

impl<'a> IntoIterator for &'a Claim {
    type Item = Point;
    type IntoIter = ClaimIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ClaimIter {
            claim: &self,
            p: (self.margin_left - 1, self.margin_top),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn claim_parse_sucessfully() {
        let s = "#123 @ 1,123: 12x123";
        let claim = Claim::parse(s).unwrap();

        assert_eq!(claim.id, 123);
        assert_eq!(claim.margin_left, 1);
        assert_eq!(claim.margin_top, 123);
        assert_eq!(claim.width, 12);
        assert_eq!(claim.height, 123);
    }

    #[test]
    fn claim_iter_sucessfully() {
        let (margin_left, margin_top, width, height) = (3, 2, 5, 4);
        let claim = Claim {
            id: 1,
            margin_left,
            margin_top,
            width,
            height,
        };

        let mut iter = claim.into_iter();
        for y in margin_top..margin_top + height {
            for x in margin_left..margin_left + width {
                let point = iter.next();
                assert_eq!(point, Some((x, y)));
            }
        }

        assert_eq!(iter.next(), None);
    }

    #[test]
    fn aoc03_part1_fixture() {
        let input = PathBuf::from("tests/fixtures/aoc03_1.txt");
        assert_eq!(solve_part1(input).unwrap(), 4);
    }

    #[test]
    fn aoc03_part1() {
        let input = PathBuf::from("tests/inputs/aoc03.txt");
        assert_eq!(solve_part1(input).unwrap(), 124850);
    }

    #[test]
    fn aoc03_part2_fixture() {
        let input = PathBuf::from("tests/fixtures/aoc03_1.txt");
        assert_eq!(solve_part2(input).unwrap(), 3);
    }

    #[test]
    fn aoc03_part2() {
        let input = PathBuf::from("tests/inputs/aoc03.txt");
        assert_eq!(solve_part2(input).unwrap(), 1097);
    }
}
