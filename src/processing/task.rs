#![warn(clippy::all)]
#![allow(unused)]

use crate::processing::zeit::Timeframe;

#[derive(Debug, Clone, Default)]
pub struct Task {
    // timeframe (from, to), alias => (from, duration)
    // timeframe variants: exact, rounded, selected?
    // project: break, [project], note, ignore
    // description: id?
    pub timeframe: Timeframe,
    pub project: String,
    pub description: String,
}

impl Task {
    pub fn new(timeframe: Timeframe, project: String, description: String) -> Self {
        Self {
            timeframe,
            project,
            description,
        }
    }
}
