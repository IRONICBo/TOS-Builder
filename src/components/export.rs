


use serde::{Deserialize, Serialize};
use tui::{
    layout::{Alignment},
    style::{Color, Style},
    widgets::{Block, Borders, Gauge},
};

use crate::app::App;

#[derive(Debug, Default)]
#[derive(Serialize, Deserialize)]
pub struct Export {
    /// Current export value
    pub current: u64,
    /// Total export value
    pub total: u64,
    /// Current start time
    pub start_time: u64,
    /// Current end time
    pub end_time: u64,
    /// Current message
    pub message: String,
}

impl Export {
    pub fn default() -> Self {
        Self {
            current: 0,
            total: 0,
            start_time: 0,
            end_time: 0,
            message: String::new(),
        }
    }

    pub fn set_current(&mut self, current: u64) {
        self.current = current;
    }

    pub fn set_total(&mut self, total: u64) {
        self.total = total;
    }

    pub fn set_start_time(&mut self, time: u64) {
        self.start_time = time;
    }

    pub fn set_end_time(&mut self, time: u64) {
        self.end_time = time;
    }

    pub fn set_message(&mut self, message: String) {
        self.message = message;
    }
}

pub fn get_export_block<'a>(app: &'a App) -> Gauge<'a> {
    let label = format!("{}/{}", app.export.current, app.export.total);
    let title = format!(
        "Export TOS:`{}` (cost {}s)",
        app.export.message,
        app.export.end_time - app.export.start_time
    );

    let gauge = Gauge::default()
        .block(Block::default().title(title.clone()).title_alignment(Alignment::Center).borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Blue).bg(Color::White))
        .percent(((app.export.current as f64 / app.export.total as f64) * 100.0) as u16)
        .label(label);

    return gauge;
}
