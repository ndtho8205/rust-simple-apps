// Day 6: Chronal Coordinates

use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::PathBuf;

use crate::error::TResult;

#[allow(dead_code)]
fn solve_part1(path: PathBuf) -> TResult<i32> {
    unimplemented!()
}

#[allow(dead_code)]
fn solve_part2(path: PathBuf) -> TResult<i32> {
    unimplemented!()
}

//
// Data Structures
//
#[derive(Eq, PartialEq, Hash)]
struct Point(i32, i32);

type Grid = HashMap<Point, Vec<Point>>;

//
// Utility functions
//
fn manhattan_distance(a: &Point, b: &Point) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

// OK, then, how can we construct Grid from list of Points obtained from the input?
fn construct_grid(coords: &Vec<Point>) -> Grid {
    let max_x = coords.iter().map(|point| point.0).max().unwrap();
    let max_y = coords.iter().map(|point| point.1).max().unwrap();

    let origin = Point(0, 0);
    let limit = Point(max_x, max_y);
    for x in origin.0..limit.0 {
        for y in origin.1..limit.1 {
            let point = Point(x, y);
            let distances: Vec<_> = coords
                .iter()
                .map(|coord| manhattan_distance(&point, &coord))
                .collect();
            if distances.
        }
    }

    Grid::new()
}

//
// Tests
//

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manhattan_distance() {
        assert_eq!(manhattan_distance(&Point(1, 2), &Point(1, 1)), 1);
        assert_eq!(manhattan_distance(&Point(5, 5), &Point(3, 3)), 4);
    }

    #[test]
    fn test_construct_grid_one_point() {
        let point_a = Point(1, 1);
        let point_a_area = vec![
            Point(0, 0),
            Point(0, 1),
            Point(0, 2),
            Point(1, 0),
            Point(1, 2),
            Point(2, 0),
            Point(2, 1),
            Point(2, 2),
        ];
        let points = vec![Point(1, 1)];

        let mut grid = Grid::new();
        grid.insert(point_a, point_a_area);
    }

    #[test]
    #[ignore]
    fn aoc06_part1_fixture() {
        let input = PathBuf::from("tests/fixtures/aoc06_1.txt");
        assert_eq!(solve_part1(input).unwrap(), 17);
    }

    #[test]
    #[ignore]
    fn aoc06_part1() {
        let input = PathBuf::from("tests/inputs/aoc06.txt");
        assert_eq!(solve_part1(input).unwrap(), 0);
    }

    #[test]
    #[ignore]
    fn aoc06_part2_fixture() {
        let input = PathBuf::from("tests/fixtures/aoc06_1.txt");
        assert_eq!(solve_part2(input).unwrap(), 0);
    }

    #[test]
    #[ignore]
    fn aoc06_part2() {
        let input = PathBuf::from("tests/inputs/aoc06.txt");
        assert_eq!(solve_part2(input).unwrap(), 0);
    }
}
