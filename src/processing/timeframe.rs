#![warn(clippy::all)]
#![allow(unused)]

use crate::processing::round_single_timestamp;
use chrono::{Duration, NaiveDateTime};

#[derive(Debug, Clone, Default)]
pub struct Timeframe {
    begin: NaiveDateTime,
    end: NaiveDateTime,
}

pub trait TimeframeTrait {
    fn begin(&self) -> NaiveDateTime;

    fn end(&self) -> NaiveDateTime;

    fn duration(&self) -> Duration;

    fn round(&self) -> Timeframe;
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
