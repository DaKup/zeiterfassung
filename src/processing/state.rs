#![warn(clippy::all)]

#[derive(Debug, Clone, PartialEq)]
pub struct State {
    pub text: String,
}

impl Default for State {
    fn default() -> Self {
        Self {
            text: include_str!("example.md").to_string(),
        }
    }
}
