use serde::{Deserialize, Serialize};
use tui::{
    style::{Color, Style},
    widgets::{Paragraph, Borders, Block},
};

use crate::app::App;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[derive(Serialize, Deserialize)]
pub enum InputMode {
    #[default]
    Normal,
    Editing,
}

#[derive(Debug, Default)]
#[derive(Serialize, Deserialize)]
pub struct Input {
    /// Current value of the input box
    pub input: String,
    /// Position of cursor in the editor area.
    pub cursor_position: usize,
    /// Current input mode
    pub input_mode: InputMode,
}

impl Input {
    pub fn default() -> Self {
        Self {
            input: String::new(),
            cursor_position: 0,
            input_mode: InputMode::Normal,
        }
    }

    pub fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.cursor_position.saturating_sub(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_left);
    }

    pub fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.cursor_position.saturating_add(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_right);
    }

    pub fn enter_char(&mut self, new_char: char) {
        self.input.insert(self.cursor_position, new_char);

        self.move_cursor_right();
    }

    pub fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.cursor_position != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.cursor_position;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.input.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    pub fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.len())
    }

    pub fn reset_cursor(&mut self) {
        self.cursor_position = 0;
    }
}

pub fn get_input_block<'a>(app: &'a App, title: &'a str, current: &'a str) -> Paragraph<'a> {
    Paragraph::new(current.clone())
        .style(match app.input.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(Block::default().borders(Borders::ALL).title(title.clone()))
}