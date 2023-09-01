#![warn(clippy::all)]

pub fn parse1(text: &str) -> String {
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
        return "Could not parse file".to_string();
    }
    // remove empty lines
    let lines = lines
        .iter()
        .filter(|line| !line.is_empty())
        .cloned()
        .collect::<Vec<_>>();

    // join lines
    let text = lines.join("\n");

    text
}
