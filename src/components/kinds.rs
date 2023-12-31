use std::{
    error::Error,
};

use serde::{Deserialize, Serialize};
use tui::{
    backend::Backend,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState},
    Frame,
};

use crate::app::{ActiveModules, App};

/// select item value
#[derive(Debug, Default)]
#[derive(Serialize, Deserialize)]
pub struct KindList {
    pub current: String,
    pub value: Vec<String>,
    #[serde(skip)]
    pub index: ListState,
}

impl KindList {
    pub fn default(kind_list: Vec<String>) -> Result<Self, Box<dyn Error>> {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        let exp = Self {
            current: kind_list[0].to_string(),
            value: kind_list,
            index: list_state,
        };
        Ok(exp)
    }
}

fn draw_kind_item(kind: &String, vec: &mut Vec<ListItem>) {
    vec.push(ListItem::new(kind.clone()));
}

pub fn draw_cube_kind_tree<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>, area: Rect) {
    let current_kind_list = &mut app.kl;
    let mut items: Vec<ListItem<'_>> = vec![];
    for kind in current_kind_list.value.iter() {
        draw_kind_item(kind, &mut items);
    }

    let mut blk = Block::default()
        .title("CubeMX Project Kind")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    match app.active_modules == ActiveModules::ProjectSelect(crate::app::ProjectSelect::Kind) {
        true => {
            blk = blk.border_style(Style::default().fg(Color::LightBlue).add_modifier(Modifier::BOLD));
        }
        false => {
            blk = blk.border_style(Style::default().fg(Color::Black));
        }
    }

    let kind_list = List::new(items).block(blk).highlight_style(Style::default().bg(Color::LightYellow)).highlight_symbol("> ");
    frame.render_stateful_widget(kind_list, area, &mut current_kind_list.index);
}

pub fn draw_tos_version_tree<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>, area: Rect) {
    let current_kind_list = &mut app.tl;
    let mut items: Vec<ListItem<'_>> = vec![];
    for kind in current_kind_list.value.iter() {
        draw_kind_item(kind, &mut items);
    }

    let mut blk = Block::default()
        .title("Download TOS Project")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    match app.active_modules == ActiveModules::TOSDownload(crate::app::TOSDownload::Version) {
        true => {
            blk = blk.border_style(Style::default().fg(Color::LightBlue).add_modifier(Modifier::BOLD));
        }
        false => {
            blk = blk.border_style(Style::default().fg(Color::Black));
        }
    }

    let kind_list = List::new(items).block(blk).highlight_style(Style::default().bg(Color::LightYellow)).highlight_symbol("> ");
    frame.render_stateful_widget(kind_list, area, &mut current_kind_list.index);
}

pub fn draw_cube_arch_tree<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>, area: Rect) {
    let current_kind_list = &mut app.arch;
    let mut items: Vec<ListItem<'_>> = vec![];
    for kind in current_kind_list.value.iter() {
        draw_kind_item(kind, &mut items);
    }

    let mut blk = Block::default()
        .title("CubeMX Arch Kind")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    match app.active_modules == ActiveModules::ProjectSelect(crate::app::ProjectSelect::Arch) {
        true => {
            blk = blk.border_style(Style::default().fg(Color::LightBlue).add_modifier(Modifier::BOLD));
        }
        false => {
            blk = blk.border_style(Style::default().fg(Color::Black));
        }
    }

    let kind_list = List::new(items).block(blk).highlight_style(Style::default().bg(Color::LightYellow)).highlight_symbol("> ");
    frame.render_stateful_widget(kind_list, area, &mut current_kind_list.index);
}
