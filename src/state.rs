use std::fs;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

// use crate::theme::std_theme;
use crate::fzf::Fzf;
use crate::theme::Types;

use boxion::rect::Rect;
use boxion::rects::listrectcolored::ListRectColored;

pub struct FmState<'a, W: Write> {
    pub path: PathBuf,
    pub stdout: W,
    pub path_rect: &'a Rect,
    pub preview_rect_list: ListRectColored<'a, Types>,
    pub main_rect_list: ListRectColored<'a, Types>,
    pub fzf: Option<Fzf<char, (Types, String)>>,
}

impl<'a, W: Write> FmState<'a, W> {
    pub fn show(&mut self) {
        self.main_rect_list.show(&mut self.stdout);
        self.preview();
        self.preview_rect_list.show(&mut self.stdout);
        self.stdout.flush().unwrap();
    }

    pub fn reload(&mut self) {
        let mut contents = get_contents_name(self.path.as_path()).unwrap();
        if self.fzf.is_some() {
            contents = self.fzf.as_ref().unwrap().get_remaining();
        }
        self.set_fzf(contents.clone());
        self.main_rect_list.clear(&mut self.stdout);
        self.main_rect_list.set_elements(contents);
        self.path_rect.clear(&mut self.stdout);
        self.path_rect
            .write_trimmed(
                &mut self.stdout,
                self.path.canonicalize().unwrap().to_str().unwrap_or(""),
                0,
                0,
            )
            .unwrap();
    }

    pub fn handle_key(&mut self, key: termion::event::Key) {
        match key {
            termion::event::Key::Char('J') => self.main_rect_list.next(),
            termion::event::Key::Char('K') => self.main_rect_list.prev(),
            termion::event::Key::Char('H') => {
                self.fzf = None;
                self.go_to_parent();
            }
            termion::event::Key::Char('L') => {
                self.fzf = None;
                self.open();
            }
            termion::event::Key::Esc => {
                self.set_fzf(get_contents_name(self.path.as_path()).unwrap());
                self.reload();
            }
            termion::event::Key::Char(a) => {
                if self.fzf.is_some() && !a.is_uppercase() {
                    self.fzf.as_mut().unwrap().next(a);
                }
                self.reload();
            }
            _ => {}
        }
    }

    fn go_to_parent(&mut self) {
        self.path.pop();
        self.reload();
    }

    fn open(&mut self) {
        let selection = self.main_rect_list.get_selected();
        if selection.is_none() {
            return;
        }
        self.path.push(&selection.unwrap());
        if self.path.is_dir() {
            self.reload();
        } else {
            self.path.pop();
        }
    }

    fn preview(&mut self) {
        self.preview_rect_list.clear(&mut self.stdout);
        let selection = self.main_rect_list.get_selected();
        if selection.is_none() {
            self.preview_rect_list.set_elements(vec![]);
            return;
        }
        let mut selected = self.path.clone();
        selected.push(&selection.unwrap());
        if selected.is_dir() {
            self.preview_rect_list
                .set_elements(get_contents_name(&selected).unwrap());
        } else {
            self.preview_rect_list
                .set_elements(vec![(Types::File, selected.to_str().unwrap().to_string())])
        }
    }

    fn set_fzf(&mut self, contents: Vec<(Types, String)>) {
        self.fzf = Some(Fzf::new(
            contents
                .clone()
                .into_iter()
                .map(|(v, k)| (k.to_lowercase().chars().collect(), (v, k)))
                .collect(),
        ));
    }

}

fn get_contents_name(path: &Path) -> std::io::Result<Vec<(Types, String)>> {
    if !path.is_dir() {
        return Ok(Vec::new());
    }

    let contents = fs::read_dir(path)?;
    let mut result = Vec::new();

    for entry in contents {
        let entry_path = entry?.path();
        let path_str = entry_path.to_str().unwrap_or("/");
        let path = Path::new(path_str);
        if path.is_dir() {
            result.push((
                Types::Directory,
                String::from(path.file_name().unwrap().to_str().unwrap()),
            ));
        } else if path.is_file() {
            result.push((
                Types::File,
                String::from(path.file_name().unwrap().to_str().unwrap()),
            ));
        }
    }

    // result.sort_unstable();
    return Ok(result);
}
