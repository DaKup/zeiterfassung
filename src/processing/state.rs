#![warn(clippy::all)]

use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct State {
    pub markdown_input: String,
    pub markdown_content_backbuffer: Arc<Mutex<String>>,
    pub overwrite_input: Arc<AtomicBool>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            markdown_input: include_str!("example.md").to_string(),
            markdown_content_backbuffer: Arc::new(Mutex::new(
                include_str!("example.md").to_string(),
            )),
            overwrite_input: Arc::new(AtomicBool::new(false)),
        }
    }
}
