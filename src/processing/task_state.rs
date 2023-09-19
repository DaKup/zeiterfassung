#![warn(clippy::all)]
#![allow(unused)]

use crate::processing::ProjectType;

#[derive(Debug, Clone)]
pub struct TaskState {
    pub enabled: bool,
    pub project_type: ProjectType,
}

impl TaskState {
    pub fn default() -> TaskState {
        TaskState {
            enabled: true,
            project_type: ProjectType::Unknown,
        }
    }
}
