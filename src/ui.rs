use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    symbols::DOT,
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Padding, Paragraph, Tabs},
    Frame,
};

use crate::components::fs;
use crate::{
    app::App,
    components::{footer, header},
};

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples
    // frame.render_widget(
    //     Paragraph::new(format!(
    //         "This is a tui template.\n\
    //             Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
    //             Press left and right to increment and decrement the counter respectively.\n\
    //             Counter: {}",
    //         app.counter
    //     ))
    //     .block(
    //         Block::default()
    //             .title("Template")
    //             .title_alignment(Alignment::Left)
    //             .borders(Borders::ALL)
    //             // .padding(Padding::new(1, 1, 1, 1))
    //             .border_type(BorderType::Rounded),
    //     )
    //     .style(Style::default().fg(Color::Cyan).bg(Color::Black))
    //     .alignment(Alignment::Center),
    //     frame.size(),
    // );

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(3),      // header
                Constraint::Percentage(90), // body
                Constraint::Length(3),      // footer
            ]
            .as_ref(),
        )
        .split(frame.size());

    // Render header.
    header::draw_header(app, frame, chunks[0]);

    // Render body.
    match app.routes.current {
        0 => {
            fs::draw_project_select_page(app, frame, chunks[1]);
        }
        1 => {
            // draw_project_select_page(app, frame, chunks[1]);
        }
        2 => {
            // draw_project_select_page(app, frame, chunks[1]);
        }
        3 => {
            // draw_project_select_page(app, frame, chunks[1]);
        }
        4 => {
            // draw_project_select_page(app, frame, chunks[1]);
        }
        _ => unreachable!(),
    }

    // Render footer.
    footer::draw_footer(app, frame, chunks[2]);
}
