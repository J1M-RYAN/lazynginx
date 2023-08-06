use std::fmt::format;

use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::Line,
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Tabs},
    Frame,
};

use crate::{
    app::App,
    logs::{access_log, error_log, log_locations_component},
    version::get_nginx_version,
};

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    let nginx_version = get_nginx_version();

    frame.render_widget(
        Paragraph::new("")
            .block(
                Block::default()
                    .title(format!("lazynginx, nginx v{}", nginx_version.unwrap()))
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::Green).bg(Color::Black))
            .alignment(Alignment::Center),
        frame.size(),
    );

    let chunks = Layout::default()
        .margin(1)
        .constraints(
            [
                Constraint::Length(3),       // for the tabs
                Constraint::Percentage(100), // for the canvas
            ]
            .as_ref(),
        )
        .split(frame.size());

    let titles = app.titles.iter().map(|t| Line::from(*t)).collect();
    let tabs = Tabs::new(titles)
        .style(Style::default().bg(Color::Gray))
        .add_modifier(Modifier::BOLD)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .select(app.horizontal_position)
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::LightBlue),
        );

    frame.render_widget(tabs, chunks[0]);

    match app.horizontal_position {
        0 => {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Ratio(1, 5), Constraint::Ratio(4, 5)])
                .split(chunks[1]);

            let status_block = Block::default()
                .title("Status")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded);
            frame.render_widget(
                Paragraph::new(app.status.clone()).block(status_block),
                chunks[0],
            );

            // Create a list of systemctl commands
            let commands: Vec<_> = vec![
                "Start", "Stop", "Restart", "Reload", "Status", "Enable", "Disable",
            ]
            .into_iter()
            .map(ListItem::new)
            .collect();

            let commands_list = List::new(commands)
                .highlight_style(Style::default().fg(Color::Yellow))
                .highlight_symbol(">>")
                .block(
                    Block::default()
                        .title("Systemctl Commands")
                        .title_alignment(Alignment::Center)
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded),
                );

            frame.render_stateful_widget(commands_list, chunks[1], &mut app.list_state);
        }
        1 => {}
        2 => {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Min(27), Constraint::Ratio(4, 5)])
                .split(chunks[1]);

            frame.render_stateful_widget(
                log_locations_component().block(
                    Block::default()
                        .title("Pick Log File")
                        .title_alignment(Alignment::Center)
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded),
                ),
                chunks[0],
                &mut app.log_list_state,
            );

            if let Some(log_selection) = app.log_list_state.selected() {
                match log_selection {
                    0 => frame.render_widget(access_log(), chunks[1]), // Render access_log when selection is 0
                    1 => frame.render_widget(error_log(), chunks[1]), // Render error_log when selection is 1
                    _ => {} // Optional: Handle other cases if needed
                }
            }
        }
        _ => {}
    }
}
