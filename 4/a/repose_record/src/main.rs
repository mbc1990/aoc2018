extern crate chrono;
extern crate regex;

use chrono::{DateTime, NaiveDateTime, NaiveTime, Timelike, Duration};

use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::collections::HashMap;
use std::borrow::Borrow;


#[derive(Debug)]
struct GuardEvent {
    timestamp: NaiveDateTime,
    event_text: String,
}

fn parse_ge(line: String) -> Result<GuardEvent, &'static str> {
    // Get timestamp
    let re: Regex = Regex::new(r"\[.*\]").unwrap();
    let matches: HashSet<&str> = re
        .find_iter(line.as_ref())
        .map(|mat| mat.as_str())
        .collect();
    let event_time: &str = matches.into_iter().next().unwrap();
    let input = event_time.replace("[", "").replace("]", "");
    let ts: NaiveDateTime = NaiveDateTime::parse_from_str(&input, "%Y-%m-%d %H:%M").unwrap();

    // Get the actual event
    let split = line.split(" ");
    let mut by_spaces = split.collect::<Vec<&str>>();

    // Pop the first two elements, which are the time stamps
    by_spaces.remove(0);
    by_spaces.remove(0);

    // Put the rest of the string back together
    let event_text = by_spaces.join(" ");

    let ge = GuardEvent {
        timestamp: ts,
        event_text: event_text
    };
    return Ok(ge);
}

fn read_input(fname: String) -> Result<Vec<GuardEvent>, &'static str> {
    let mut ret = Vec::new();
    let input = File::open(fname).unwrap();
    let buffered = BufReader::new(input);
    for line in buffered.lines() {
        let my_line = line.unwrap();
        let ge = parse_ge(my_line).unwrap();
        ret.push(ge);
    }
    return Ok(ret);
}

fn main() {
    let input = read_input("input.txt".parse().unwrap());
    match input {
        Ok(mut guard_events) => {

            // Sort by event date
            guard_events.sort_by_key(|k| k.timestamp);

            let mut guard_sleep_schedule = HashMap::new();

            let mut current_guard_id: i32 = -1;
            let mut current_sleep_start = NaiveDateTime::from_timestamp(0, 42000000);
            let mut guard_to_total_asleep = HashMap::new();
            for evt in guard_events {
                let minute = evt.timestamp.minute();

                // If event is beginning shift, update guard id
                if evt.event_text.contains("begins shift") {
                    let re: Regex = Regex::new(r"\d+").unwrap();
                    let matches: HashSet<&str> = re
                        .find_iter(evt.event_text.as_ref())
                        .map(|mat| mat.as_str())
                        .collect();
                    let guard_id: &str = &matches.into_iter().next().unwrap();
                    current_guard_id = guard_id.parse().unwrap();
                } else if evt.event_text == "falls asleep" {
                    println!("Sleep starts for guard {:?}", current_guard_id);
                    // Set current sleep time
                    current_sleep_start = evt.timestamp;

                } else if evt.event_text == "wakes up" {
                    let mut sleep_counter = current_sleep_start.clone();
                    if !guard_sleep_schedule.contains_key(&current_guard_id) {
                        let mut guard_sleeps = HashMap::new();
                        guard_sleep_schedule.insert(current_guard_id, guard_sleeps);
                    }
                    while sleep_counter < evt.timestamp {

                        let minute_asleep = sleep_counter.time();
                        let mut guard_sleeps = guard_sleep_schedule.get_mut(current_guard_id.borrow()).unwrap();
                        // Increment the particular minute that the guard is sleeping
                        *guard_sleeps.entry(minute_asleep).or_insert(0) += 1;
                        *guard_to_total_asleep.entry(current_guard_id).or_insert(0) += 1;

                        println!("Guard {:?} is asleep at timestamp {:?}", current_guard_id, minute_asleep);

                        sleep_counter = sleep_counter.checked_add_signed(Duration::minutes(1)).unwrap();
                    }
                    println!("Sleep ends for guard {:?}", current_guard_id);
                } else {
                    panic!("Unexpected event: {:?}", evt);
                }
            }

            let mut most_asleep_id = -1;
            let mut most_asleep_mins = -1;
            for (k, v) in guard_to_total_asleep.iter() {
                if v > &most_asleep_mins {
                    most_asleep_mins = v.clone();
                    most_asleep_id = k.clone();
                }
            }

            println!("Guard sleep schedule: {:?}", guard_sleep_schedule);
            println!("Guard most asleep is {:?} with {:?} total minutes", most_asleep_id, most_asleep_mins);
            let schedule = guard_sleep_schedule.get(&most_asleep_id).unwrap();
            let mut best_min = 0;
            let mut best_min_count = -1;

            for (k, v) in schedule.iter() {
                if v > &best_min_count {
                    best_min_count = v.clone();
                    best_min = k.minute();
                }
            }
            println!("Best minute is {:?} with {:?} total sleeps found", best_min, best_min_count);

        }
        Err(_) => {}
    }
}
