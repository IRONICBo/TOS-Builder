use crossterm::event::KeyCode;

use crate::app::{App, AppResult};

pub fn handle_key_events(_key_event: KeyCode, _app: &mut App) -> AppResult<()> {
    Ok(())
}
