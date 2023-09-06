use tui::{
    backend::Backend,
    layout::{Rect, Layout, Direction, Constraint},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Borders, Block, Clear},
    Frame,
};

use crate::app::App;

use super::input::{self, InputMode};

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

pub fn draw_input_popup<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    let size = frame.size();
    let block = input::get_input_block(app, "Test Input", app.input.input.as_str());
    let area = centered_rect(60, 10, size);

    match app.input.input_mode {
        InputMode::Normal =>
            {}

        InputMode::Editing => {
            // Make the cursor visible and ask ratatui to put it at the specified coordinates after
            // rendering
            frame.set_cursor(
                // Draw the cursor at the current position in the input field.
                // This position is can be controlled via the left and right arrow key
                area.x + app.input.cursor_position as u16 + 1,
                // Move one line down, from the border to the input line
                area.y + 1,
            )
        }
    }

    frame.render_widget(Clear, area);
    frame.render_widget(block, area);
}