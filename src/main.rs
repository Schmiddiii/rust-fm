extern crate boxion;
extern crate termion;

mod filemanager;
mod fmloop;
mod fzf;
mod state;
mod theme;
mod utils;

use crate::filemanager::{EntryType, FileManager};
use crate::fmloop::start_loop;
use crate::state::FmState;
use crate::theme::{std_theme, std_theme_no_highlight};

use boxion::border;
use boxion::layout;
use boxion::rect;

use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;

use std::io::{stdout, Write};

const PATH_RECT_NAME: &str = "path_layout";
const PREVIEW_RECT_NAME: &str = "preview_layout";
const MAIN_RECT_NAME: &str = "main_layout";

/// Programm start
fn main() {
    let screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());

    let path_layout = layout::Layout::new_str(PATH_RECT_NAME);
    let preview_layout = layout::Layout::new_border_str(PREVIEW_RECT_NAME, border::LINED);
    let main_layout = layout::Layout::new_border_str(MAIN_RECT_NAME, border::LINED);

    let layout = layout::Layout::merge_value(
        path_layout,
        layout::Layout::merge(
            main_layout,
            preview_layout,
            layout::SplitDirection::HORIZONTAL,
            0.5,
        ),
        layout::SplitDirection::VERTICAL,
        1,
    );

    let hm = rect::Rect::from_layout_whole(&layout);

    let path_rect = &hm[PATH_RECT_NAME];
    let preview_rect = &hm[PREVIEW_RECT_NAME];
    let main_rect = &hm[MAIN_RECT_NAME];
    let main_rect_list = main_rect.clone().into_list_colored::<EntryType>(&std_theme);
    let preview_rect_list = preview_rect
        .clone()
        .into_list_colored::<EntryType>(&std_theme_no_highlight);

    let mut state = FmState {
        path_rect: path_rect,
        main_rect_list: *main_rect_list,
        preview_rect_list: *preview_rect_list,
        stdout: screen,
        fzf: None,
        fm: FileManager::new("."),
    };

    write!(state.stdout, "{}", termion::cursor::Hide).unwrap();
    start_loop(&mut state);
    write!(state.stdout, "{}", termion::cursor::Show).unwrap();
}
