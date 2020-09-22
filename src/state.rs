use std::fs::File;
use std::io::{BufReader, Read, Write};

use crate::filemanager::{EntryType, FileManager};
use crate::fzf::Fzf;

use boxion::rect::Rect;
use boxion::rects::listrectcolored::ListRectColored;

/// Represents a state of the program
pub struct FmState<'a, W: Write> {
    pub stdout: W,
    pub path_rect: &'a Rect,
    pub preview_rect_list: ListRectColored<'a, EntryType>,
    pub main_rect_list: ListRectColored<'a, (EntryType, bool)>,
    pub fzf: Option<Fzf<char, (EntryType, String, bool)>>,
    pub fm: FileManager,
}

impl<'a, W: Write> FmState<'a, W> {
    /// Shows the current state
    pub fn show(&mut self) {
        self.main_rect_list.show(&mut self.stdout);
        self.preview();
        self.preview_rect_list.show(&mut self.stdout);
        self.stdout.flush().unwrap();
    }

    /// Resets the elements of main_rect_list and path_rect
    pub fn reload(&mut self) {
        let mut contents = self.fm.get_contents();
        if self.fzf.is_some() {
            contents = self.fzf.as_ref().unwrap().get_remaining();
        }
        self.set_fzf(contents.clone());
        self.main_rect_list.clear(&mut self.stdout);
        self.main_rect_list.set_elements(contents.into_iter().map(|(t,s,b)| ((t,b),s)).collect());
        self.path_rect.clear(&mut self.stdout);
        self.path_rect
            .write_trimmed(&mut self.stdout, &self.fm.get_path_string(), 0, 0)
            .unwrap();
    }

    /// Handles a key press
    pub fn handle_key(&mut self, key: termion::event::Key) {
        match key {
            termion::event::Key::Char('J') => self.main_rect_list.next(),
            termion::event::Key::Char('K') => self.main_rect_list.prev(),
            termion::event::Key::Char('H') => {
                self.fzf = None;
                self.go_to_parent();
            }
            termion::event::Key::Char('L') | termion::event::Key::Char('\n') => {
                self.fzf = None;
                self.open();
            }
            termion::event::Key::Esc => {
                self.set_fzf(self.fm.get_contents());
                self.reload();
            }
            termion::event::Key::Char(' ') => {
                self.highlight_selected();

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

    /// Change directory to the parent directory
    fn go_to_parent(&mut self) {
        self.fm.change_dir("..").unwrap_or(());
        self.reload();
    }

    /// Changes directory to the highlighted directory
    fn open(&mut self) {
        let selection = self.main_rect_list.get_selected();
        if selection.is_none() {
            return;
        }

        self.fm.open_child(&mut self.stdout, &selection.unwrap());
        self.reload();
    }

    /// Previews the currently selected file/folder in the preview rect
    fn preview(&mut self) {
        self.preview_rect_list.clear(&mut self.stdout);
        let selection = self.main_rect_list.get_selected();
        if selection.is_none() {
            self.preview_rect_list.set_elements(vec![]);
            return;
        }

        let type_of_child = self
            .fm
            .get_type_of_child(&selection.clone().unwrap())
            .unwrap_or(EntryType::Directory);
        if type_of_child == EntryType::Directory {
            let contents = self
                .fm
                .get_contents_of_child(&selection.unwrap())
                .unwrap_or(vec![]);
            self.preview_rect_list.set_elements(contents);
        } else {
            let path = self.fm.get_path_to_child(&selection.unwrap());
            if path.is_err() {
                return;
            }

            let file = File::open(path.unwrap());
            if file.is_err() {
                return;
            }
            let mut buf_reader = BufReader::new(file.unwrap());
            let mut contents = String::new();

            buf_reader.read_to_string(&mut contents).unwrap_or(0);

            let elements: Vec<(EntryType, String)> = contents
                .split('\n')
                .map(|s| (EntryType::File, s.to_string()))
                .collect();
            self.preview_rect_list.set_elements(elements);
        }
    }

    /// Highlights the currently selected item
    fn highlight_selected(&mut self) {
        let selected = self.main_rect_list.get_selected();
        if selected.is_none() {
            return;
        }

        // Highlight in file manager
        self.fm.toggle_highlight_of(selected.clone().unwrap());

        // Highlight in fzf
        if self.fzf.is_some() {
            let old_fzf_value = self.fzf.as_ref().unwrap().get_value_of(selected.clone().unwrap().to_lowercase().chars().collect());
            if old_fzf_value.is_some() {
                let new_fzf_value = (old_fzf_value.clone().unwrap().0, old_fzf_value.clone().unwrap().1, !old_fzf_value.unwrap().2);
                self.fzf.as_mut().unwrap().change_value_of(selected.clone().unwrap().to_lowercase().chars().collect(), new_fzf_value);
            }
        }

        // Highlight in list view
        let index = self.main_rect_list.get_index();
        let old_extras = self.main_rect_list.get_selected_extra();
        if old_extras.is_some() {
            let new_extras = (old_extras.clone().unwrap().0, !old_extras.clone().unwrap().1);
            self.main_rect_list.set_element(index, (new_extras, selected.clone().unwrap()));


        }

        self.show();
    }

    /// Sets up the fuzzy find to the contents of the directory
    fn set_fzf(&mut self, contents: Vec<(EntryType, String, bool)>) {
        self.fzf = Some(Fzf::new(
            contents
                .clone()
                .into_iter()
                .map(|(v, k, b)| (k.to_lowercase().chars().collect(), (v, k, b)))
                .collect(),
        ));
    }
}
