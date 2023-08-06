use std::path::PathBuf;

use tui::widgets::{List, ListItem};

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
