#![warn(clippy::all)]

use crate::processing::{
    calculate_durations, extract_log_lines, parse_log_lines, round_timestamp_tasks,
    AsMyStringTrait, Task, TaskState, TimeframeTrait,
};
use chrono::{Duration, NaiveDateTime};
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use super::Timeframe;

#[derive(Debug, Clone)]
pub struct AppState {
    // input:
    pub markdown_input: String,
    pub projects_input: String,

    // load button:
    pub markdown_content_backbuffer: Arc<Mutex<String>>,
    pub overwrite_input: Arc<AtomicBool>,

    // processing:
    pub log_lines: Vec<String>,
    pub timestamp_tasks: Vec<(NaiveDateTime, String)>,

    // debug/intermediates/tests:
    pub rounded_timestamp_tasks: Vec<(NaiveDateTime, String)>,
    pub durations: Vec<Duration>,
    pub rounded_durations: Vec<Duration>,

    // other:
    pub show_debug: bool,
    pub rounded_plots: bool,
    pub log_scale: bool,
    pub synchronize_markdown: bool,

    // processing results:
    pub tasks: Vec<Task>,

    // pub parsed_project_names: Vec<String>,
    pub project_names: Vec<String>,
    pub task_states: Vec<TaskState>,

    pub summary: String,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            markdown_input: include_str!("example.md").to_string(),
            projects_input: "ProjectA\nProjectB\nTeam Meetings\nUncategorized\n".to_string(), // parse from input
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
            rounded_plots: true,
            log_scale: true,
            synchronize_markdown: true,
            tasks: vec![],
            project_names: vec![],
            task_states: vec![],
            summary: "".to_string(),
        }
    }
}

pub trait Update {
    fn update(&mut self);
    fn create_summary(&mut self);
}

impl Update for AppState {
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

        // debug/intermediates/tests:
        self.rounded_timestamp_tasks = round_timestamp_tasks(&self.timestamp_tasks);
        self.durations = calculate_durations(&self.timestamp_tasks);
        self.rounded_durations = calculate_durations(&self.rounded_timestamp_tasks);

        self.project_names = self
            .projects_input
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| l.to_string())
            .collect();

        // fn default_name() ->Option<&String>{Some("[project]".to_string())};
        // let default_name = "[project]".to_string();

        // let project_name= self.project_names.get(0);

        self.tasks = self
            .timestamp_tasks
            .iter()
            .zip(self.timestamp_tasks.iter().skip(1))
            .enumerate()
            .map(|(index, (t0, t1))| {
                let begin = t0.0;
                let end = t1.0;
                Task {
                    timeframe: Timeframe::new(begin, end),
                    project: match self.project_names.get(index) {
                        None => String::from("[project]"),
                        Some(x) => x.clone(),
                    },
                    description: t0.1.clone(),
                }
            })
            .collect();

        if self.task_states.len() < self.tasks.len() {
            self.task_states
                .resize(self.tasks.len(), TaskState::default());
        }

        // update final summary report:
        self.create_summary();
        // self.summary = "Hello World!".to_string();
    }

    fn create_summary(&mut self) {
        let mut projects: HashMap<String, Vec<Task>> = HashMap::new();

        for task in self.tasks.as_mut_slice() {
            // projects.insert(task.project.clone(), task.);

            match projects.get_mut(&task.project) {
                None => {
                    projects.insert(task.project.clone(), vec![task.clone()]);
                }
                Some(v) => {
                    v.push(task.clone());
                }
            };
        }

        let mut summary = "".to_string();

        let mut sorted_keys: Vec<String> = projects
            .keys()
            .collect::<Vec<_>>()
            .iter()
            .map(|&project_name| project_name.clone())
            .collect();
        // sorted_keys

        sorted_keys.sort_by(|a, b| {
            let sum_a: i64 = projects
                .get(a)
                .unwrap()
                .iter()
                .map(|t| t.timeframe.round().duration().num_seconds())
                .sum();
            let sum_b: i64 = projects
                .get(b)
                .unwrap()
                .iter()
                .map(|t| t.timeframe.round().duration().num_seconds())
                .sum();

            if sum_a != sum_b {
                sum_a.cmp(&sum_b)
            } else {
                a.to_lowercase().cmp(&b.to_lowercase())
            }
        });

        for project_name in sorted_keys {
            let task_vec = projects.get_mut(&project_name).unwrap();
            summary += format!("- {}\n", project_name).as_str();

            for task in task_vec {
                summary += format!(
                    "    - {}: ({})\n",
                    task.timeframe.round().duration().to_my_string(),
                    task.description
                )
                .as_str();
            }
            summary += "\n";
        }

        self.summary = summary;

        /*
        - ProjectA: 10.00h
            - 2.00h Did some work on ProjectA
            - 5.00h Some other work on this
            - 3.00h Something more
            - 0.00h extra comment
        - ProjectB: 3.00h
            - ...
         */

        // alternatively or additionally output precise timestamps => detailed summary
        /*
         */
    }
}
