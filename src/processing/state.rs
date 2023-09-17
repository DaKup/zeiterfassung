#![warn(clippy::all)]

use crate::processing::{
    calculate_durations, extract_log_lines, parse_log_lines, round_timestamp_tasks, Task,
};
use chrono::{Duration, NaiveDateTime};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct State {
    pub markdown_input: String,
    pub markdown_content_backbuffer: Arc<Mutex<String>>,
    pub overwrite_input: Arc<AtomicBool>,

    pub log_lines: Vec<String>,
    pub timestamp_tasks: Vec<(NaiveDateTime, String)>,
    pub rounded_timestamp_tasks: Vec<(NaiveDateTime, String)>,
    pub durations: Vec<Duration>,
    pub rounded_durations: Vec<Duration>,

    pub show_debug: bool,
    pub synchronize_markdown: bool,
    pub tasks: Vec<Task>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            markdown_input: include_str!("example.md").to_string(),
            markdown_content_backbuffer: Arc::new(Mutex::new(
                include_str!("example.md").to_string(),
            )),
            overwrite_input: Arc::new(AtomicBool::new(false)),
            log_lines: vec![],
            timestamp_tasks: vec![],
            rounded_timestamp_tasks: vec![],
            durations: vec![],
            rounded_durations: vec![],
            show_debug: false,
            synchronize_markdown: true,
            tasks: vec![],
        }
    }
}

pub trait Update {
    fn update(&mut self);
}

impl Update for State {
    fn update(&mut self) {
        // new files were opened:
        if self.overwrite_input.load(Ordering::Relaxed) {
            self.overwrite_input.store(false, Ordering::Relaxed);
            self.markdown_input = self.markdown_content_backbuffer.lock().unwrap().to_string();
        }

        // parse:
        // input = self.state.markdown_input;
        self.log_lines = extract_log_lines(&self.markdown_input);
        self.timestamp_tasks = parse_log_lines(&self.log_lines);
        self.rounded_timestamp_tasks = round_timestamp_tasks(&self.timestamp_tasks);
        self.durations = calculate_durations(&self.timestamp_tasks);
        self.rounded_durations = calculate_durations(&self.rounded_timestamp_tasks);

        // for (i, e) in self.timestamp_tasks.iter().rev().skip(1).rev().enumerate() {
        //
        // }

        self.tasks = vec![
            Task::default(),
            Task::default(),
            Task::default(),
            Task::default(),
        ];
    }
}
