// Day 4: Repose Record

use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::PathBuf;
use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

use crate::error::{TError, TResult};

fn prepare(path: PathBuf) -> TResult<GuardSchedule> {
    let contents = read_to_string(path)?;
    let mut events = contents
        .lines()
        .map(|line| line.parse::<Event>())
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    events.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

    let mut idx = 0;
    let mut guard_schedule = GuardSchedule::new();
    let mut current_guard_id: GuardId = 0;

    while idx < events.len() {
        match events[idx].kind {
            EventKind::GuardShift(guard_id) => {
                current_guard_id = guard_id;
                idx += 1;
            }
            EventKind::Asleep => {
                let start_sleep_time: usize = events[idx].timestamp.time[3..5].parse()?;
                let end_sleep_time: usize = events[idx + 1].timestamp.time[3..5].parse()?;

                (start_sleep_time..end_sleep_time).for_each(|t| {
                    let sleep_freqs = guard_schedule.entry(current_guard_id).or_insert([0; 60]);
                    sleep_freqs[t] += 1;
                });

                idx += 2;
            }
            _ => (),
        };
    }

    Ok(guard_schedule)
}

#[allow(dead_code)]
fn solve_part1(path: PathBuf) -> TResult<u32> {
    let guard_schedule = prepare(path)?;

    let (chosen_guard_id, sleep_freqs) = guard_schedule
        .iter()
        .max_by_key(|&(_, freqs)| freqs.iter().sum::<u32>())
        .unwrap();

    let (chosen_minute, _) = sleep_freqs
        .iter()
        .enumerate()
        .max_by_key(|&(_, freq)| freq)
        .unwrap();

    Ok(chosen_guard_id * chosen_minute as u32)
}

#[allow(dead_code)]
fn solve_part2(path: PathBuf) -> TResult<u32> {
    let guard_schedule = prepare(path)?;

    let (chosen_guard_id, sleep_freqs) = guard_schedule
        .iter()
        .max_by_key(|&(_, sleep_freqs)| sleep_freqs.iter().max())
        .unwrap();

    let (chosen_minute, _) = sleep_freqs
        .iter()
        .enumerate()
        .max_by_key(|&(_, freq)| freq)
        .unwrap();

    Ok(chosen_guard_id * chosen_minute as u32)
}

//
// Data Structures
//

type GuardId = u32;
type SleepFrequency = [u32; 60];
type GuardSchedule = HashMap<GuardId, SleepFrequency>;

#[derive(Debug, PartialEq, Ord, PartialOrd, Eq)]
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

#[derive(Debug)]
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
    fn aoc04_part1_fixture() {
        let input = PathBuf::from("tests/fixtures/aoc04_1.txt");
        assert_eq!(solve_part1(input).unwrap(), 240);
    }

    #[test]
    fn aoc04_part1() {
        let input = PathBuf::from("tests/inputs/aoc04.txt");
        assert_eq!(solve_part1(input).unwrap(), 12169);
    }

    #[test]
    fn aoc04_part2_fixture() {
        let input = PathBuf::from("tests/fixtures/aoc04_1.txt");
        assert_eq!(solve_part2(input).unwrap(), 4455);
    }

    #[test]
    fn aoc04_part2() {
        let input = PathBuf::from("tests/inputs/aoc04.txt");
        assert_eq!(solve_part2(input).unwrap(), 16164);
    }
}
