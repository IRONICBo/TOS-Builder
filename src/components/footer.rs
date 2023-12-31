use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::app::App;

pub fn draw_footer<B: Backend>(_app: &mut App, frame: &mut Frame<'_, B>, area: Rect) {
    let key_color = Color::Yellow;
    let spans = Line::from(vec![
        Span::styled("<TAB>", Style::default().fg(key_color).add_modifier(Modifier::BOLD)),
        Span::raw(" Next tab "),
        Span::styled("<Shift + TAB>", Style::default().fg(key_color).add_modifier(Modifier::BOLD)),
        Span::raw(" Prev tab "),
        Span::styled("<d|D>", Style::default().fg(key_color).add_modifier(Modifier::BOLD)),
        Span::raw(" Next module "),
        Span::styled("<a|A>", Style::default().fg(key_color).add_modifier(Modifier::BOLD)),
        Span::raw(" Prev module "),
        Span::styled("<SPACE>", Style::default().fg(key_color).add_modifier(Modifier::BOLD)),
        Span::raw(" Select "),
        Span::styled("<ENTER>", Style::default().fg(key_color).add_modifier(Modifier::BOLD)),
        Span::raw(" Enter dir "),
        Span::styled("<g|G>", Style::default().fg(key_color).add_modifier(Modifier::BOLD)),
        Span::raw(" Generate "),
        Span::styled("<q|Q>", Style::default().fg(key_color).add_modifier(Modifier::BOLD)),
        Span::raw(" Quit "),
    ]);
    let paragraph = Paragraph::new(spans);

    frame.render_widget(paragraph, area);
}
