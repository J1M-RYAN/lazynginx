use std::fmt::format;

use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::Line,
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Tabs},
    Frame,
};

use crate::{app::App, logs::log_locations_component, version::get_nginx_version};

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    let nginx_version = get_nginx_version();

    frame.render_widget(
        Paragraph::new("welcome to lazynginx")
            .block(
                Block::default()
                    .title(format!("nginx v{}", nginx_version.unwrap()))
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
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(u16::try_from(app.titles.len()).unwrap() * 2 + 1),
                Constraint::Percentage(30),
            ]
            .as_ref(),
        )
        .split(frame.size());

    let titles = app.titles.iter().map(|t| Line::from(*t)).collect();
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("Tabs"))
        .select(app.horizontal_position)
        .style(Style::default().fg(Color::Cyan))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::LightBlue),
        );
    frame.render_widget(tabs, chunks[0]);

    match app.horizontal_position {
        0 => {
            let status_block = Block::default().title("Status").borders(Borders::ALL);
            frame.render_widget(
                Paragraph::new(app.status.clone()).block(status_block),
                chunks[1],
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

            frame.render_stateful_widget(commands_list, chunks[2], &mut app.list_state);
        }
        1 => {}
        2 => frame.render_widget(log_locations_component(), chunks[1]),
        _ => {}
    }
}
