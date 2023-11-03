#![warn(clippy::all)]

use crate::processing::ProjectType;

#[derive(Debug, Clone)]
pub struct TaskState {
    pub enabled: bool,
    pub project_type: ProjectType,
}

impl Default for TaskState {
    fn default() -> Self {
        TaskState {
            enabled: true,
            project_type: ProjectType::Unknown,
        }
    }
}
