use std::{fs::{DirEntry, self}, env::current_dir, error::Error, path::Path};

use tui::{backend::Backend, Frame, layout::{Layout, Constraint, Rect, Alignment, Direction}, widgets::{Borders, Paragraph, Block, ListState, ListItem, BorderType, Wrap, List}, style::{Color, Style}, text::Text};

use crate::app::App;

pub fn draw_project_select_page<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>, area: Rect) {
    // const FOOTER_TEXT: &str = "Press `Esc`, `Ctrl-C` or `q` to stop running.";
    // let footer = Paragraph::new(FOOTER_TEXT)
    //     .block(Block::default().borders(Borders::ALL).title("Body"))
    //     .style(Style::default().fg(Color::Magenta))
    //     .alignment(Alignment::Center);
    // frame.render_widget(footer, area);

    draw_cube_path_tree(app, frame, area);
}

/// select project path
#[derive(Debug)]
pub struct FolderList {
    pub current: String,
    pub dirs: Vec<DirEntry>,
    pub files: Vec<DirEntry>,
    pub index: ListState,
}

impl FolderList {
    pub fn default() -> Result<Self, Box<dyn Error>> {
        let path = current_dir()?;
        let path_str = path.to_str().expect("Convert error");
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        let mut exp = Self {
            current: path_str.to_string(),
            files: vec![],
            dirs: vec![],
            index: list_state,
        };
        let (dirs, files) = exp.visit_dir(path_str)?;
        exp.files = files;
        exp.dirs = dirs;
        Ok(exp)
    }

    pub fn refresh(&mut self) {
        let str = String::from(self.current.as_str());
        match self.visit_dir(str.as_str()) {
            Ok(entries) => {
                self.dirs = entries.0;
                self.files = entries.1;
            }
            Err(_) => {}
        }
    }

    fn visit_dir(&mut self, path: &str) -> Result<(Vec<DirEntry>, Vec<DirEntry>), Box<dyn Error>> {
        let path = Path::new(path);
        let mut dir_entries = vec![];
        let mut file_entries = vec![];
        match path.is_dir() {
            true => {
                for entry in fs::read_dir(path)? {
                    match entry {
                        Ok(entry) => {
                            let path = entry.path();
                            if path.is_dir() {
                                dir_entries.push(entry);
                                break;
                            } else {
                                file_entries.push(entry);
                            }
                        }
                        Err(_) => {
                            continue;
                        }
                    }
                }
            }
            false => {
                return Err("Path not valid".into());
            }
        }
        Ok((dir_entries, file_entries))
    }
}

fn draw_dir_item(entry: &DirEntry, vec: &mut Vec<ListItem>) {
    let file_name = String::from(entry.file_name().to_str().unwrap()) + "/";
    vec.push(ListItem::new(file_name));
}

fn draw_file_item(entry: &DirEntry, vec: &mut Vec<ListItem>) {
    let file_name = String::from(entry.file_name().to_str().unwrap());
    vec.push(ListItem::new(file_name));
}

pub fn draw_cube_path_tree<B: Backend>(app: &mut App, frame: &mut Frame<B>, area: Rect) {
    let current_folder_list = &mut app.fl;
    let fs_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)])
        .split(area);

    // let folder = Paragraph::new(Text::from(current_folder_list.current.as_str()))
    //     .wrap(Wrap { trim: true })
    //     .alignment(Alignment::Center)
    //     .block(
    //         Block::default()
    //             .title("Choosed Project path")
    //             .title_alignment(Alignment::Left)
    //             .border_type(BorderType::Rounded)
    //             .borders(Borders::ALL),
    //     );
    // frame.render_widget(folder, fs_chunks[0]);

    let mut items = vec![ListItem::new("..")]; // to parent
    for entry in &current_folder_list.dirs {
        draw_dir_item(entry, &mut items);
    }
    for entry in &current_folder_list.files {
        draw_file_item(entry, &mut items);
    }
    let mut blk = Block::default()
        .title("File Explorer")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    // match app.active_modules == ActiveModules::Fs {
    //     true => {
    //         blk = blk.border_style(Style::default().fg(Color::Cyan));
    //     }
    //     false => (),
    // }
    let file_list = List::new(items)
        .block(blk)
        .highlight_style(Style::default().bg(Color::Cyan))
        .highlight_symbol("> ");
    frame.render_stateful_widget(file_list, fs_chunks[0], &mut current_folder_list.index);
}



// select project kind
