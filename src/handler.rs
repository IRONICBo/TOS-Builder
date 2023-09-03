use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => {
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
                    // crate::pages::project_select::handle_key_events(pages_event, app)?;
                }
                _ => {}
            }
        }
    }
    Ok(())
}
