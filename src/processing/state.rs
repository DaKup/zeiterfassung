#![warn(clippy::all)]

use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct State {
    pub text: String,
    pub markdown_content: Arc<Mutex<String>>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            text: include_str!("example.md").to_string(),
            markdown_content: Arc::new(Mutex::new(include_str!("example.md").to_string())),
        }
    }
}
