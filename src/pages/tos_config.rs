use std::{
    env::current_dir,
    error::Error,
    fs::{self, DirEntry},
    path::Path,
};

use serde_json::Value;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Text,
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph, Row, Table, Wrap, Cell},
    Frame,
};

use crate::{app::{App, ActiveModules}, components::fs::draw_cube_path_tree, components::kinds::draw_cube_kind_tree, config::common::StringValue};

pub fn draw_page<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>, area: Rect) {
    // split window
    let chunks = Layout::default().direction(Direction::Vertical).constraints([Constraint::Percentage(100)]).split(area);

    // Display values
    draw_config_table(app, frame, chunks[0]);
}

fn draw_config_table<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>, area: Rect) {
    // Unwarp json and get items
    let binding = app.tos_header_table.tos_header_config.to_vec();
    
    let rows = binding.iter().map(|item| {
        let height = item
            .iter()
            .map(|content| content.chars().filter(|c| *c == '\n').count())
            .max()
            .unwrap_or(0)
            + 1;
        let cells = item.iter().map(|c| Cell::from(c.as_str()));
        Row::new(cells).height(height as u16).bottom_margin(1)
    });

    let mut blk = Block::default().title("TOS Project Header Config").borders(Borders::ALL).border_type(BorderType::Rounded).title_alignment(Alignment::Center);
    
    match app.active_modules == ActiveModules::TOSConfig(crate::app::TOSConfig::Config) {
        true => {
            blk = blk.border_style(Style::default().fg(Color::LightBlue).add_modifier(Modifier::BOLD));
        }
        false => {
            blk = blk.border_style(Style::default().fg(Color::Black));
        }
    }

    let project_config_table = Table::new(rows)
    .header(Row::new(vec!["Config", "Value", "Description"]).style(Style::default().add_modifier(Modifier::BOLD)).bottom_margin(1))
    .block(blk)
    .column_spacing(2)
    .highlight_style(Style::default().bg(Color::LightYellow))
    .highlight_symbol("> ")
    .widths(&[Constraint::Min(45), Constraint::Percentage(15), Constraint::Percentage(85)]);

    frame.render_stateful_widget(project_config_table, area, &mut app.tos_header_table.index);
}
