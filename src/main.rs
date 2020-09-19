
extern crate boxion;
extern crate termion;

mod state;
mod fmloop;
mod theme;
mod fzf;

use crate::state::FmState;
use crate::fmloop::start_loop;
use crate::theme::{std_theme, std_theme_no_highlight};
use crate::theme::Types;

use boxion::layout;
use boxion::border;
use boxion::rect;

use termion::screen::AlternateScreen;
use termion::raw::IntoRawMode;

use std::io::{stdout, Write};
use std::path::Path;

const PATH_RECT_NAME: &str = "path_layout";
const PREVIEW_RECT_NAME: &str = "preview_layout";
const MAIN_RECT_NAME: &str = "main_layout";

fn main() {
    let screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());

    let path_layout = layout::Layout::new_str(PATH_RECT_NAME);
    let preview_layout= layout::Layout::new_border_str(PREVIEW_RECT_NAME, border::LINED);
    let main_layout = layout::Layout::new_border_str(MAIN_RECT_NAME, border::LINED);

    let layout = layout::Layout::merge_value(path_layout, layout::Layout::merge(main_layout, preview_layout, layout::SplitDirection::HORIZONTAL, 0.5), layout::SplitDirection::VERTICAL, 1);

    let hm = rect::Rect::from_layout_whole(&layout);

    let path_rect = &hm[PATH_RECT_NAME];
    let preview_rect= &hm[PREVIEW_RECT_NAME];
    let main_rect = &hm[MAIN_RECT_NAME];
    let main_rect_list = main_rect.clone().into_list_colored::<Types>(&std_theme);
    let preview_rect_list = preview_rect.clone().into_list_colored(&std_theme_no_highlight);

    let cwd_path_buf = Path::new(".").canonicalize().unwrap();


    let mut state = FmState {
        path: cwd_path_buf,
        path_rect: path_rect,
        main_rect_list: *main_rect_list,
        preview_rect_list: *preview_rect_list,
        stdout: screen,
        fzf: None
    };

    write!(state.stdout, "{}", termion::cursor::Hide).unwrap();
    start_loop(&mut state);
    write!(state.stdout, "{}", termion::cursor::Show).unwrap();
}

