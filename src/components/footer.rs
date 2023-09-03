use tui::{Frame, backend::Backend, layout::Rect, style::{Color, Style, Modifier}, text::{Line, Span}, widgets::Paragraph};

use crate::app::App;

pub fn draw_footer<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>, area: Rect) {
    let key_color = Color::Yellow;
    let spans = Line::from(vec![
        Span::styled("<TAB>", Style::default().fg(key_color).add_modifier(Modifier::BOLD)),
        Span::raw(" Next tab "),
        Span::styled("<Shift + TAB>", Style::default().fg(key_color).add_modifier(Modifier::BOLD)),
        Span::raw(" Previous tab "),
        Span::styled("<SPACE>", Style::default().fg(key_color).add_modifier(Modifier::BOLD)),
        Span::raw(" Select "),
        Span::styled("<ENTER>", Style::default().fg(key_color).add_modifier(Modifier::BOLD)),
        Span::raw(" Enter dir "),
        Span::styled("<g|G>", Style::default().fg(key_color).add_modifier(Modifier::BOLD)),
        Span::raw(" Generate project "),
        Span::styled("<q|Q|Esc>", Style::default().fg(key_color).add_modifier(Modifier::BOLD)),
        Span::raw(" Quit "),
    ]);
    let paragraph = Paragraph::new(spans);

    frame.render_widget(paragraph, area);

}