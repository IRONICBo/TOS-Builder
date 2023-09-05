use std::{
    env::current_dir,
    error::Error,
    fs::{self, DirEntry},
    path::Path,
};

use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Text,
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph, Row, Table, Wrap},
    Frame,
};

use crate::{app::App, components::fs::draw_cube_path_tree, components::kinds::draw_cube_kind_tree};

pub fn draw_page<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>, area: Rect) {
    // split window
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(6), Constraint::Percentage(100)])
        .split(area);

    // Display values
    draw_config_table(app, frame, chunks[0]);

    let select_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
        .split(chunks[1]);
    draw_cube_path_tree(app, frame, select_chunks[0]);
    draw_cube_kind_tree(app, frame, select_chunks[1]);
}

fn draw_config_table<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>, area: Rect) {
    let project_config_table = Table::new([
        Row::new(["CubeMX Project Path", app.cube_mx_project_config.path.as_str(), "Choose a CubeMX project directory"]),
        Row::new(["Project Kind", app.cube_mx_project_config.kind.as_str(), "Choose your CubeMX project kind"]),
    ])
    .header(Row::new(vec!["Config", "Value", "Description"]).style(Style::default().add_modifier(Modifier::BOLD)).bottom_margin(1))
    .block(Block::default().title("CubeMX Project Config").borders(Borders::ALL))
    .column_spacing(3)
    .widths(&[Constraint::Min(20), Constraint::Percentage(50), Constraint::Percentage(50)]);
    frame.render_widget(project_config_table, area);
}

// fn draw_cube_kind_tree<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>, area: Rect) {
//     let cube_kind_tree = List::new(
//         vec![CubeMXProjectType::GCC, CubeMXProjectType::IAR, CubeMXProjectType::MDK]
//             .iter()
//             .map(|t| ListItem::new(t.as_str()))
//             .collect::<Vec<ListItem>>(),
//     )
//     .block(
//         Block::default()
//             .title("CubeMX Project Kind")
//             .border_type(BorderType::Rounded)
//             .title_alignment(Alignment::Center)
//             .borders(Borders::ALL),
//     )
//     .highlight_style(Style::default().add_modifier(Modifier::BOLD).fg(Color::LightBlue))
//     .highlight_symbol("> ");
//     frame.render_widget(cube_kind_tree, area);
// }
