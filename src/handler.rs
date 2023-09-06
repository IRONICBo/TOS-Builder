use crate::{app::{App, AppResult}, components::input::InputMode};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    // Set input priority to the popup if it is active.
    match app.input.input_mode {
        InputMode::Editing => {
            crate::handlers::input_handler::handle_key_events(key_event.code, app)?;
        }
        InputMode::Normal => {
            // common handlers
            match key_event.code {
                // TODO:test pop up
                KeyCode::Char('p') => {
                    app.input_popup = !app.input_popup;
                    if app.input.input_mode == InputMode::Editing {
                        app.input.input_mode = InputMode::Normal;
                    } else {
                        app.input.input_mode = InputMode::Editing;
                    }
                }
                // Exit application on `ESC` or `q`
                KeyCode::Char('q') | KeyCode::Char('Q') => {
                    app.quit();
                }
                // Exit application on `Ctrl-C`
                KeyCode::Char('c') | KeyCode::Char('C') => {
                    if key_event.modifiers == KeyModifiers::CONTROL {
                        app.quit();
                    }
                }
                // Counter handlers
                KeyCode::Tab => {
                    app.routes.next();
                }
                KeyCode::BackTab => {
                    app.routes.previous();
                }
                // Other handlers you could add here.
                pages_event => {
                    match app.routes.current {
                        0 => {
                            crate::handlers::project_select_handler::handle_key_events(pages_event, app)?;
                        }
                        1 => {
                            crate::handlers::tos_download_hander::handle_key_events(pages_event, app)?;
                        }
                        2 => {
                            crate::handlers::tos_config_handler::handle_key_events(pages_event, app)?;
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    Ok(())
}
