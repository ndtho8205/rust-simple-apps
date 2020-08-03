// Day 4: Repose Record

use std::collections::HashMap;
use std::fs::{read_to_string, File};
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

use crate::error::{TError, TResult};

#[allow(dead_code)]
fn solve_part1(path: PathBuf) -> TResult<u16> {
    unimplemented!();
}

#[allow(dead_code)]
fn solve_part2(path: PathBuf) -> TResult<u16> {
    unimplemented!();
}

//
// Data Structures
//

type GuardId = u32;
type SleepFrequency = [u32; 60];
type GuardSchedule = HashMap<GuardId, SleepFrequency>;

#[derive(Debug, PartialEq)]
struct Timestamp {
    date: String,
    time: String,
}

#[derive(Debug, PartialEq)]
enum EventKind {
    GuardShift(GuardId),
    Asleep,
    WakeUp,
}

struct Event {
    timestamp: Timestamp,
    kind: EventKind,
}

impl FromStr for Event {
    type Err = TError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref PARSER: Regex =
                Regex::new(r"\[(?P<date>(\d{4})-(\d{2})-(\d{2})) (?P<time>\d{2}:\d{2})] (Guard #(?P<id>\d+)|(?P<sleep>\w+))")
                    .unwrap();
        }

        let caps = match PARSER.captures(s) {
            Some(c) => c,
            None => return Err("cannot parse".into()),
        };

        Ok(Event {
            timestamp: Timestamp {
                date: caps["date"].parse()?,
                time: caps["time"].parse()?,
            },
            kind: if caps.name("id").is_some() {
                EventKind::GuardShift(caps["id"].parse()?)
            } else {
                println!("{:?}", &caps["sleep"]);
                match &caps["sleep"] {
                    "falls" => EventKind::Asleep,
                    "wakes" => EventKind::WakeUp,
                    _ => return Err("cannot parse event kind".into()),
                }
            },
        })
    }
}

//
// Tests
//

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_event_guardshift_sucessful() {
        let s = "[1518-11-01 00:00] Guard #10 begins shift";
        let got = s.parse::<Event>().unwrap();

        assert_eq!(
            got.timestamp,
            Timestamp {
                date: "1518-11-01".to_owned(),
                time: "00:00".to_owned()
            }
        );

        assert_eq!(got.kind, EventKind::GuardShift(10));
    }

    #[test]
    fn parse_event_asleep_sucessful() {
        let s = "[1518-11-01 00:00] falls asleep";
        let got = s.parse::<Event>().unwrap();

        assert_eq!(got.kind, EventKind::Asleep);
    }

    #[test]
    fn parse_event_wakeup_sucessful() {
        let s = "[1518-11-01 00:00] wakes up";
        let got = s.parse::<Event>().unwrap();

        assert_eq!(got.kind, EventKind::WakeUp);
    }

    #[test]
    #[ignore]
    fn aoc04_part1_fixture() {
        let input = PathBuf::from("tests/fixtures/aoc04_1.txt");
        assert_eq!(solve_part1(input).unwrap(), 0);
    }

    #[test]
    #[ignore]
    fn aoc04_part1() {
        let input = PathBuf::from("tests/inputs/aoc04.txt");
        assert_eq!(solve_part1(input).unwrap(), 0);
    }

    #[test]
    #[ignore]
    fn aoc04_part2_fixture() {
        let input = PathBuf::from("tests/fixtures/aoc04_1.txt");
        assert_eq!(solve_part2(input).unwrap(), 0);
    }

    #[test]
    #[ignore]
    fn aoc04_part2() {
        let input = PathBuf::from("tests/inputs/aoc04.txt");
        assert_eq!(solve_part2(input).unwrap(), 0);
    }
}
