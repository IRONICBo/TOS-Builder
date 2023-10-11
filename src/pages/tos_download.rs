

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    widgets::{Block, Borders, Row, Table},
    Frame,
};

use crate::{app::App, components::{fs::{draw_tos_path_tree}, kinds::draw_tos_version_tree}};

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
    draw_tos_path_tree(app, frame, select_chunks[0]);
    draw_tos_version_tree(app, frame, select_chunks[1]);
}

fn draw_config_table<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>, area: Rect) {
    let project_config_table = Table::new([
        Row::new(["TOS Project Path", app.tos_project_config.path.as_str(), "Choose a TOS project directory"]),
        Row::new(["TOS Version", app.tos_project_config.version.as_str(), "Choose your TOS project version"]),
    ])
    .header(Row::new(vec!["Config", "Value", "Description"]).style(Style::default().add_modifier(Modifier::BOLD)).bottom_margin(1))
    .block(Block::default().title("TOS Project Config").borders(Borders::ALL))
    .column_spacing(3)
    .widths(&[Constraint::Min(20), Constraint::Percentage(50), Constraint::Percentage(50)]);
    frame.render_widget(project_config_table, area);
}
