#![warn(clippy::all)]
#![allow(unused)]

use crate::processing::round_single_timestamp;
use chrono::{Duration, NaiveDateTime};

#[derive(Debug, Clone, Default)]
pub struct Timeframe {
    begin: NaiveDateTime,
    end: NaiveDateTime,
}

impl Timeframe {
    pub fn new(begin: NaiveDateTime, end: NaiveDateTime) -> Self {
        Self { begin, end }
    }
}

pub trait TimeframeTrait {
    fn begin(&self) -> NaiveDateTime;

    fn end(&self) -> NaiveDateTime;

    fn duration(&self) -> Duration;

    fn round(&self) -> Timeframe;
}

pub trait AsMyStringTrait {
    fn to_my_string(&self) -> String;
}

impl AsMyStringTrait for Duration {
    fn to_my_string(&self) -> String {
        format!(
            "{:.2}h",
            self.num_hours() as f32 + (self.num_minutes() % 60) as f32 / 60.0
        )
    }
}

impl TimeframeTrait for Timeframe {
    fn begin(&self) -> NaiveDateTime {
        self.begin
    }

    fn end(&self) -> NaiveDateTime {
        self.end
    }

    fn duration(&self) -> Duration {
        Duration::seconds(self.end.timestamp() - self.begin.timestamp())
    }

    fn round(&self) -> Timeframe {
        Timeframe {
            begin: round_single_timestamp(&self.begin),
            end: round_single_timestamp(&self.end),
        }
    }
}
