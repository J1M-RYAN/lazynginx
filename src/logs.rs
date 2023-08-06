use std::{fs::read_to_string, path::PathBuf};

use tui::{
    style::{Modifier, Style},
    text::{Line, Span, Spans, Text},
    widgets::{List, ListItem, Paragraph, Wrap},
};

pub struct LogLocation {
    access_log: PathBuf,
    error_log: PathBuf,
}
pub fn get_log_locations() -> LogLocation {
    LogLocation {
        access_log: PathBuf::from("/var/log/nginx/access.log"),
        error_log: PathBuf::from("/var/log/nginx/error.log"),
    }
}

pub fn log_locations_component() -> List<'static> {
    let log_locations: LogLocation = get_log_locations();
    let log_location_vec: Vec<_> = vec![log_locations.access_log, log_locations.error_log]
        .into_iter()
        .map(|path| path.to_string_lossy().into_owned())
        .map(ListItem::new)
        .collect();

    List::new(log_location_vec)
}
pub fn access_log() -> Paragraph<'static> {
    let log_location = get_log_locations().access_log;
    let log_contents = read_log(&log_location);

    // Construct a single string from the split lines
    let log_str = log_contents.content.to_string();

    Paragraph::new(Text::from(log_str)).wrap(Wrap { trim: false })
}

fn read_log(path: &PathBuf) -> Span<'static> {
    match read_to_string(path) {
        Ok(contents) => Span::raw(contents),
        Err(_) => Span::raw("Failed to read log file."),
    }
}
