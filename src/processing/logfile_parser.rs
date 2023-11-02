#![warn(clippy::all)]

use chrono::{self, NaiveDateTime};
use regex::Regex;

pub fn extract_log_lines(markdown: &str) -> Vec<String> {
    let lines = markdown
        .lines()
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>();

    let begin = lines.iter().position(|&l| l.starts_with("## Log"));
    let end = lines.iter().rev().position(|&l| l.starts_with("## Notes"));

    match (begin, end) {
        (Some(begin), Some(end)) => {
            let end = lines.len() - end - 1;
            if end > begin {
                markdown
                    .lines()
                    .filter(|l| !l.is_empty())
                    .map(String::from)
                    .collect::<Vec<_>>()[begin + 1..end]
                    .iter()
                    .map(String::from)
                    .collect()
            } else {
                vec![]
            }
        }
        (None, None) => markdown.lines().map(String::from).collect::<Vec<String>>(),
        (_, _) => {
            vec![]
        }
    }
}

pub fn parse_log_lines(markdown_lines: &[String]) -> Vec<(NaiveDateTime, String)> {
    let re = Regex::new(r"- [0-9]{4}-[0-9]{2}-[0-9]{2} [0-9]{2}:[0-9]{2}:[0-9]{2} .*").unwrap();
    let log_lines = markdown_lines
        .iter()
        .filter(|line| line.len() > 22 && re.is_match(line))
        .map(String::as_str)
        .collect::<Vec<_>>();

    let timestamps_tasks = log_lines
        .iter()
        .filter_map(|line| {
            let timestamp_begin = egui::TextBuffer::byte_index_from_char_index(line, 2);
            let timestamp_end = egui::TextBuffer::byte_index_from_char_index(line, 21);
            let time_string = &line[timestamp_begin..timestamp_end];

            let task_begin = egui::TextBuffer::byte_index_from_char_index(line, 22);
            let task_end = egui::TextBuffer::byte_index_from_char_index(line, line.chars().count());
            let task_string = &line[task_begin..task_end];
            let datetime = chrono::NaiveDateTime::parse_from_str(time_string, "%Y-%m-%d %H:%M:%S");

            match datetime {
                Ok(datetime) => Some((datetime, task_string.to_string())),
                Err(_) => None,
            }
        })
        .collect::<Vec<_>>();

    let mut sorted = timestamps_tasks.clone();
    sorted.sort_by(|a, b| a.0.cmp(&b.0));

    if sorted != timestamps_tasks {
        return vec![];
    }

    timestamps_tasks
}
