use crate::{
    app::{App, AppResult},
    status::get_nginx_status,
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc | KeyCode::Char('q') => {
            app.quit();
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        KeyCode::Right => {
            app.increment_horizontal();
        }
        KeyCode::Left => {
            app.decrement_horizontal();
        }
        KeyCode::Up => {
            app.decrement_selection();
        }
        KeyCode::Down => {
            app.increment_selection();
        }
        KeyCode::Enter => {
            let command = app.selected_command();
            command.execute();
            app.status = get_nginx_status()
        }
        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}
