#![warn(clippy::all)]
#![allow(unused)]

#[derive(Debug, Clone, PartialEq)]
pub enum ProjectType {
    Unknown,
    Break,
    Id(u32),
}
