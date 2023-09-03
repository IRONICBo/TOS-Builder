use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    symbols::DOT,
    text::{Line, Span},
    widgets::{Block, Borders, Padding, Paragraph, Tabs},
    Frame,
};

use crate::app::App;

pub fn draw_header<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>, area: Rect) {
    let titles = app
        .routes
        .name
        .iter()
        .cloned()
        .map(|t| Line::from(vec![Span::styled(t.clone(), Style::default().fg(Color::Blue))]))
        .collect();
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("Tencentos Tiny Build System."))
        .select(app.routes.current)
        .highlight_style(Style::default().add_modifier(Modifier::BOLD).fg(Color::LightBlue))
        .divider(DOT);
    frame.render_widget(tabs, area);
}
