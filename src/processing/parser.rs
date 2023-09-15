#![warn(clippy::all)]
#![allow(unused)]

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

pub fn parse_markdown(text: &str) -> String {
    let text = text.to_string();

    // remove all "\r"
    let text = text.replace('\r', "");

    let lines = text.split('\n').collect::<Vec<_>>();

    let log_line = lines.iter().position(|line| line.starts_with("## Log"));
    let notes_line = lines.iter().position(|line| line.starts_with("## Notes"));

    // take all lines between "## Log" and "## Notes"
    let lines = match (log_line, notes_line) {
        (Some(log_line), Some(notes_line)) => {
            if log_line > notes_line {
                return "Could not parse file".to_string();
            }
            &lines[log_line + 1..notes_line]
        }
        // (None, Some(_)) => &lines[0..0],
        // (Some(_), None) => &lines[0..0],
        (Some(log_line), None) => &lines[log_line + 1..],
        (None, Some(notes_line)) => &lines[..notes_line],
        (None, None) => &lines[..],
    };

    // if lines.is_empty() {
    //     return "Could not parse file".to_string();
    // }

    // remove empty lines
    let lines = lines
        .iter()
        .filter(|line| !line.is_empty())
        .cloned()
        .collect::<Vec<_>>();

    if lines.is_empty() {
        return "No time entries found".to_string();
    }

    lines.join("\n")
}

pub fn parse_log(text: &str) -> (String, String) {
    let lines = text.split('\n').collect::<Vec<_>>();

    let re = Regex::new(r"- [0-9]{4}-[0-9]{2}-[0-9]{2} [0-9]{2}:[0-9]{2}:[0-9]{2} .*").unwrap();

    // let entries: Vec<&str> = re.find_iter(text).map(|m| m.as_str()).collect();

    // do this for all lines:
    let entries = lines
        .iter()
        .filter(|line| re.is_match(line))
        .filter(|line| line.len() > 22)
        .collect::<Vec<_>>();

    let dates = entries.iter().map(|date| &date[2..21]).collect::<Vec<_>>();

    // get the 22th to the second to last character:

    use egui::TextBuffer;
    let tasks = entries
        .iter()
        // .map(|date| &date[22..date.chars().count()-1 * date.byte_index_from_char_index(char_index)])
        .map(|date| {
            &date[date.byte_index_from_char_index(22)
                ..date.byte_index_from_char_index(date.chars().count())]
        })
        .collect::<Vec<_>>();

    // also handle multi-byte characters like öäü:
    // let tasks = entries
    // .iter()
    // .map(|date| &date[22..date.chars().count()-1])

    (dates.join("\n"), tasks.join("\n"))
}

pub fn parse_date(text: &str) -> String {
    // convert "2023-09-01 12:11:22" to datetime:
    // let datetime = chrono::NaiveDateTime::parse_from_str("2023-09-01 12:11:22", "%Y-%m-%d %H:%M:%S").unwrap();

    let lines = text.split('\n').collect::<Vec<_>>();

    // convert each line to a datetime:
    let dates = lines
        .iter()
        .map(|line| {
            let result = chrono::NaiveDateTime::parse_from_str(line, "%Y-%m-%d %H:%M:%S"); //.unwrap()
            result.unwrap_or_default()
        })
        .collect::<Vec<_>>();

    // convert each datetime to a timestamp:
    let timestamps = dates
        .iter()
        .map(|date| date.timestamp())
        .collect::<Vec<_>>();

    // sort the timestamps:
    let mut timestamps = timestamps;
    let timestamps_unsorted = timestamps.clone();
    timestamps.sort();

    if timestamps != timestamps_unsorted {
        return "Timestamps are not sorted".to_string();
    }

    if timestamps.len() < 2 {
        return "0,0".to_string();
    }

    // calculate the difference between each timestamp:
    let differences = timestamps
        .iter()
        .zip(timestamps.iter().skip(1))
        .map(|(a, b)| b - a)
        .collect::<Vec<_>>();

    // differences[0]

    // calculate the sum of all differences:
    let sum = differences.iter().sum::<i64>();

    let mut durations = differences
        .iter()
        .map(|duration| {
            let duration = chrono::Duration::seconds(*duration);
            let time = duration.num_hours() as f32 + (duration.num_minutes() % 60) as f32 / 60.0;
            format!("{:.2}", time)
        })
        .collect::<Vec<_>>();

    // convert the sum to a duration:
    let duration = chrono::Duration::seconds(sum);
    // let duration = chrono::Duration::seconds(sum);

    // duration.to_string()

    // convert the duration to a string:
    let time = duration.num_hours() as f32 + (duration.num_minutes() % 60) as f32 / 60.0;
    // let duration = format!("{},{}", duration.num_hours(), (duration.num_minutes() % 60) as f32 / 60.0 * 10.0);
    let duration = format!("{:.2}", time);

    durations.append(&mut vec![duration]);

    // durations.append(other);
    durations.join("\n")

    // durations
}
