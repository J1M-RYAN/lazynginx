use crate::app::App;

#[derive(PartialEq, Debug)]
pub enum Screen {
    Status,
    Config,
    Logs,
    Template,
    Unknown,
}

pub fn get_current_screen(app: &App) -> Screen {
    match app.tab_index {
        0 => Screen::Status,
        1 => Screen::Config,
        2 => Screen::Logs,
        3 => Screen::Template,
        _ => Screen::Unknown,
    }
}
