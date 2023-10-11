


use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, Row, Table, Cell},
    Frame,
};

use crate::{app::{App, ActiveModules}};

pub fn draw_page<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>, area: Rect) {
    // split window
    let chunks = Layout::default().direction(Direction::Vertical).constraints([Constraint::Percentage(100)]).split(area);

    // Display values
    draw_config_table(app, frame, chunks[0]);
}

fn draw_config_table<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>, area: Rect) {
    // Unwarp json and get items
    let binding = app.at_config_table.at_config.to_vec();
    
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

    let mut blk = Block::default().title("AT Driver Config").borders(Borders::ALL).border_type(BorderType::Rounded).title_alignment(Alignment::Center);
    
    match app.active_modules == ActiveModules::AtConfig(crate::app::AtConfig::Config) {
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

    frame.render_stateful_widget(project_config_table, area, &mut app.at_config_table.index);
}
