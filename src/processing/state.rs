#![warn(clippy::all)]

use chrono::NaiveDateTime;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct State {
    pub markdown_input: String,
    pub markdown_content_backbuffer: Arc<Mutex<String>>,
    pub overwrite_input: Arc<AtomicBool>,

    pub log_lines: Vec<String>,
    pub timestamp_tasks: Vec<(NaiveDateTime, String)>,
    pub rounded_timestamp_tasks: Vec<(NaiveDateTime, String)>,
    pub durations: Vec<i64>,
    pub rounded_durations: Vec<i64>,
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
        }
    }
}
