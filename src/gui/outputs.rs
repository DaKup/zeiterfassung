#![warn(clippy::all)]
#![allow(unused)]

use crate::processing::{State, Update};

#[derive(Debug, Clone, Default)]
pub struct Outputs {
    pub parser: ParserOutputs,
    pub processing: ProcessingOutputs,
    pub results: ResultOutputs,
}

#[derive(Debug, Clone, Default)]
pub struct ParserOutputs {
    // markdown_input: String,
    pub lines_of_interest: String,
    pub parsed_timestamps: String,
    pub parsed_descriptions: String,
}

#[derive(Debug, Clone, Default)]
pub struct ProcessingOutputs {
    pub durations_tasks: String,
    pub rounded_timestamp_descriptions: String,
    pub rounded_durations_tasks: String,

    pub total_durations: String,
    pub total_rounded_durations: String,
}

#[derive(Debug, Clone, Default)]
pub struct ResultOutputs {
    pub durations_tasks: String,
    pub rounded_timestamp_descriptions: String,
    pub rounded_durations_tasks: String,

    pub total_durations: String,
    pub total_rounded_durations: String,
}

pub trait UpdateOutputsTrait {
    fn update(&mut self, state: &State);
}

impl UpdateOutputsTrait for ParserOutputs {
    fn update(&mut self, state: &State) {
        self.lines_of_interest = state.log_lines.join("\n");
    }
}

impl UpdateOutputsTrait for ProcessingOutputs {
    fn update(&mut self, state: &State) {
        // todo!()
    }
}

impl UpdateOutputsTrait for ResultOutputs {
    fn update(&mut self, state: &State) {
        // todo!()
    }
}

impl UpdateOutputsTrait for Outputs {
    fn update(&mut self, state: &State) {
        self.parser.update(state);
        self.processing.update(state);
        self.results.update(state);

        let mut rounded_timestamps = state
            .rounded_timestamp_tasks
            .iter()
            .map(|x| {
                let (a, _) = x;
                a.to_string()
            })
            .collect::<Vec<_>>()
            .join("\n");

        let mut timestamps = state
            .timestamp_tasks
            .iter()
            .map(|x| {
                let (a, _) = x;
                a.to_string()
            })
            .collect::<Vec<_>>()
            .join("\n");

        let mut tasks = state
            .timestamp_tasks
            .iter()
            .map(|x| {
                let (_, b) = x;
                b.clone()
            })
            .collect::<Vec<_>>()
            .join("\n");

        let mut durations = state
            .durations
            .iter()
            .map(|duration| {
                let time =
                    duration.num_hours() as f32 + (duration.num_minutes() % 60) as f32 / 60.0;
                format!("{:.2}h", time)
            })
            .collect::<Vec<_>>()
            .join("\n");

        let mut rounded_durations = state
            .rounded_durations
            .iter()
            .map(|duration| {
                let time =
                    duration.num_hours() as f32 + (duration.num_minutes() % 60) as f32 / 60.0;
                format!("{:.2}h", time)
            })
            .collect::<Vec<_>>()
            .join("\n");

        let total_duration: f32 = state
            .durations
            .iter()
            .map(|duration| {
                duration.num_hours() as f32 + (duration.num_minutes() % 60) as f32 / 60.0
            })
            .sum();
        let total_duration = format!("{:.2}h", total_duration);

        let total_rounded_duration: f32 = state
            .rounded_durations
            .iter()
            .map(|duration| {
                duration.num_hours() as f32 + (duration.num_minutes() % 60) as f32 / 60.0
            })
            .sum();
        let total_rounded_duration = format!("{:.2}h", total_rounded_duration);
    }
}
