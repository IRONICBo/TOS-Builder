

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    widgets::{Block, Borders, Row, Table},
    Frame,
};

use crate::{app::App};

pub fn draw_page<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>, area: Rect) {
    // split window
    let chunks = Layout::default().direction(Direction::Vertical).constraints([Constraint::Percentage(100)]).split(area);

    // Display values
    draw_config_table(app, frame, chunks[0]);
}

fn draw_config_table<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>, area: Rect) {
    let project_config_table = Table::new([
        Row::new(["UV Project Path", app.cube_mx_project_config.path.as_str()]),
        Row::new(["Armcc Project Path", app.cube_mx_project_config.kind.as_str()]),
    ])
    .header(Row::new(vec!["Config", "Value"]).style(Style::default().add_modifier(Modifier::BOLD)).bottom_margin(1))
    .block(Block::default().title("CubeMX Project Config").borders(Borders::ALL))
    .column_spacing(2)
    .widths(&[Constraint::Min(20), Constraint::Percentage(100)]);
    frame.render_widget(project_config_table, area);
}
