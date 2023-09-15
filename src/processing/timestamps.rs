#![warn(clippy::all)]
#![allow(unused)]

use chrono::{Duration, NaiveDateTime, SubsecRound, Timelike};
use std::ops::Add;

pub fn round_single_timestamp(timestamp: &NaiveDateTime) -> NaiveDateTime {
    let rounded_minutes = (timestamp.minute() as f32 / 60.0 * 4.0).round() / 4.0 * 60.0;
    let difference = rounded_minutes as i64 - timestamp.minute() as i64;
    timestamp
        .add(Duration::minutes(difference))
        .trunc_subsecs(0)
        .with_second(0)
        .unwrap()
}

pub fn round_timestamp_tasks(
    timestamp_tasks: &[(NaiveDateTime, String)],
) -> Vec<(NaiveDateTime, String)> {
    let result = timestamp_tasks
        .iter()
        .map(|x| {
            let (a, b) = x;
            (round_single_timestamp(a), b.clone())
        })
        .collect();

    result
}
