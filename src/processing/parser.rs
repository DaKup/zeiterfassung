#![warn(clippy::all)]

use regex::Regex;

pub fn parse1(text: &str) -> (String, String) {
    let text = text.to_string();

    // split into lines
    let lines = text.split('\n').collect::<Vec<_>>();

    // find line which start with "## Log":
    let log_line = lines.iter().position(|line| line.starts_with("## Log"));

    // find line which start with "## Notes":
    let notes_line = lines.iter().position(|line| line.starts_with("## Notes"));

    // take all lines between "## Log" and "## Notes"
    let lines = match (log_line, notes_line) {
        (Some(log_line), Some(notes_line)) => &lines[log_line + 1..notes_line],
        (None, _) => &lines[0..0],
        (_, None) => &lines[0..0],
        // (Some(log_line), None) => &lines[log_line + 1..],
        // (None, Some(notes_line)) => &lines[..notes_line],
        // (None, None) => &lines[..],
    };

    if lines.is_empty() {
        return ("".to_string(), "Could not parse file".to_string());
    }

    // remove empty lines
    let lines = lines
        .iter()
        .filter(|line| !line.is_empty())
        .cloned()
        .collect::<Vec<_>>();

    if lines.is_empty() {
        return ("".to_string(), "No time entries found".to_string());
    }

    // join lines
    let text = lines.join("\n");

    let re = Regex::new(r"- [0-9]{4}-[0-9]{2}-[0-9]{2} [0-9]{2}:[0-9]{2}:[0-9]{2} .*\n").unwrap();

    // let hay = "What do 1865-04-14, 1881-07-02, 1901-09-06 and 1963-11-22 have in common?";
    let hay = &text;
    // 'm' is a 'Match', and 'as_str()' returns the matching part of the haystack.

    let entries: Vec<&str> = re.find_iter(hay).map(|m| m.as_str()).collect();
    // assert_eq!(dates, vec![
    //     "1865-04-14",
    //     "1881-07-02",
    //     "1901-09-06",
    //     "1963-11-22",
    // ]);

    // e.g dates[0] = "- 2023-09-01 08:23:42 Working on TaskA"
    // take only the date part
    let dates = entries.iter().map(|date| &date[2..21]).collect::<Vec<_>>();

    let tasks = entries
        .iter()
        .map(|date| &date[22..date.len() - 1])
        .collect::<Vec<_>>();

    // // take the part after the date
    // let lines = lines
    //     .iter()
    //     .map(|line| &line[20..])
    //     .collect::<Vec<_>>();

    (dates.join("\n"), tasks.join("\n"))
}
