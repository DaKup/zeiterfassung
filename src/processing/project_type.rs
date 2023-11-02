#![warn(clippy::all)]

#[derive(Debug, Clone, PartialEq)]
pub enum ProjectType {
    Unknown,
    Break,
    Id(u32),
}
