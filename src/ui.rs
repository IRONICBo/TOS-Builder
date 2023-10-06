use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    symbols::DOT,
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Padding, Paragraph, Tabs},
    Frame,
};

use crate::{
    app::{App, ActiveModules},
    components::{footer, header, popup},
};
use crate::{
    components::fs,
    pages::{at_config, make_config, project_select, tos_config, tos_download},
};

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples
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
            project_select::draw_page(app, frame, chunks[1]);
            match app.active_modules {
                ActiveModules::ProjectSelect(crate::app::ProjectSelect::Fs) | ActiveModules::ProjectSelect(crate::app::ProjectSelect::Kind) => {
                }
                _ => {
                    // set default active module
                    app.active_modules = ActiveModules::ProjectSelect(crate::app::ProjectSelect::Fs);
                }
            }
        }
        1 => {
            tos_download::draw_page(app, frame, chunks[1]);
            match app.active_modules {
                ActiveModules::TOSDownload(crate::app::TOSDownload::Fs) | ActiveModules::TOSDownload(crate::app::TOSDownload::Version) => {
                }
                _ => {
                    // set default active module
                    app.active_modules = ActiveModules::TOSDownload(crate::app::TOSDownload::Fs);
                }
            }
        }
        2 => {
            tos_config::draw_page(app, frame, chunks[1]);
            match app.active_modules {
                ActiveModules::TOSConfig(crate::app::TOSConfig::Config) => {
                }
                _ => {
                    // set default active module
                    app.active_modules = ActiveModules::TOSConfig(crate::app::TOSConfig::Config);
                }
            }
        }
        3 => {
            at_config::draw_page(app, frame, chunks[1]);
            match app.active_modules {
                ActiveModules::AtConfig(crate::app::AtConfig::Config) => {
                }
                _ => {
                    // set default active module
                    app.active_modules = ActiveModules::AtConfig(crate::app::AtConfig::Config);
                }
            }
        }
        4 => {
            make_config::draw_page(app, frame, chunks[1]);
            match app.active_modules {
                ActiveModules::MakeConfig(crate::app::MakeConfig::Config) => {
                }
                _ => {
                    // set default active module
                    app.active_modules = ActiveModules::MakeConfig(crate::app::MakeConfig::Config);
                }
            }
        }
        _ => unreachable!(),
    }

    // Render footer.
    footer::draw_footer(app, frame, chunks[2]);

    // Show popup
    if app.input_popup {
        popup::draw_input_popup(app, frame);
    }
    if app.download_popup {
        popup::draw_download_popup(app, frame);
    }
}
