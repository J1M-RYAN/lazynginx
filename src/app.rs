use std::error;

use tui::widgets::{List, ListState, TableState};

use crate::{
    status::get_nginx_status,
    systemctl::SystemctlCommand,
    tabs::{get_current_screen, Screen},
    version::{get_nginx_version, NginxVersion},
};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App<'a> {
    /// Is the application running?
    pub running: bool,

    pub vertical_position: u8,
    pub horizontal_position: usize,

    pub list_state: ListState,
    pub log_list_state: ListState,
    pub status: String,
    pub nginx_version: NginxVersion,
    pub titles: Vec<&'a str>,
    pub tab_index: usize,
}

impl<'a> Default for App<'a> {
    fn default() -> Self {
        let mut list_state = ListState::default();
        let mut log_list_state = ListState::default();
        list_state.select(Some(0));
        log_list_state.select(Some(0));
        Self {
            running: true,
            vertical_position: 0,
            horizontal_position: 0,
            list_state,
            status: get_nginx_status(),
            nginx_version: get_nginx_version().unwrap(),
            tab_index: 0,
            titles: vec!["Status", "Config", "Logs", "Templates"],
            log_list_state,
        }
    }
}

impl<'a> App<'a> {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn increment_vertical(&mut self) {
        if let Some(res) = self.vertical_position.checked_add(1) {
            self.vertical_position = res;
        }
    }

    pub fn decrement_vertical(&mut self) {
        if let Some(res) = self.vertical_position.checked_sub(1) {
            self.vertical_position = res;
        }
    }
    pub fn increment_horizontal(&mut self) {
        if let Some(res) = self.horizontal_position.checked_add(1) {
            self.horizontal_position = res % self.titles.len();
        }
        self.tab_index = self.horizontal_position;
    }

    pub fn decrement_horizontal(&mut self) {
        if let Some(res) = self.horizontal_position.checked_sub(1) {
            self.horizontal_position = res % self.titles.len();
        }
        self.tab_index = self.horizontal_position;
    }

    pub fn increment_selection(&mut self) {
        match get_current_screen(self) {
            Screen::Config => {}
            Screen::Status => {
                let i = match self.list_state.selected() {
                    Some(i) => {
                        if i >= 6 {
                            0
                        } else {
                            i + 1
                        }
                    }
                    None => 0,
                };
                self.list_state.select(Some(i));
            }
            Screen::Logs => {
                let i = match self.log_list_state.selected() {
                    Some(i) => {
                        //todo be nice if i didn't hardcode this 1, also code is replicated from above
                        if i >= 1 {
                            0
                        } else {
                            i + 1
                        }
                    }
                    None => 0,
                };
                self.log_list_state.select(Some(i));
            }
            Screen::Template => {}
            Screen::Unknown => {}
        }
    }

    pub fn decrement_selection(&mut self) {
        match get_current_screen(self) {
            Screen::Config => {}
            Screen::Status => {
                let i = match self.list_state.selected() {
                    Some(i) => {
                        if i == 0 {
                            6
                        } else {
                            i - 1
                        }
                    }
                    None => 0,
                };
                self.list_state.select(Some(i));
            }
            Screen::Logs => {
                let i = match self.log_list_state.selected() {
                    Some(i) => {
                        if i == 0 {
                            1
                        } else {
                            i - 1
                        }
                    }
                    None => 0,
                };
                self.log_list_state.select(Some(i));
            }
            Screen::Template => {}
            Screen::Unknown => {}
        }
    }

    pub fn selected_command(&self) -> SystemctlCommand {
        match self.list_state.selected() {
            Some(0) => SystemctlCommand::Start,
            Some(1) => SystemctlCommand::Stop,
            Some(2) => SystemctlCommand::Restart,
            Some(3) => SystemctlCommand::Reload,
            Some(4) => SystemctlCommand::Status,
            Some(5) => SystemctlCommand::Enable,
            Some(6) => SystemctlCommand::Disable,
            _ => panic!("Unexpected command selection"),
        }
    }
}
