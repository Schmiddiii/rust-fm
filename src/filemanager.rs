use std::env::var;
use std::io;
use std::path::{Path, PathBuf};

use std::process::{Command, Stdio};
use termion::screen::*;

use mime;

use crate::utils::get_mime_type;

/// Represents the management for the files
#[derive(Clone)]
pub struct FileManager {
    pub path: PathBuf,
    pub contents: Vec<(EntryType, String, bool)>
}

/// Represents the type of a element in a directory
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum EntryType {
    Directory,
    File,
}

impl FileManager {
    /// Creates a new FileManager
    pub fn new(path: &str) -> FileManager {
        let mut fm = FileManager {
            path: PathBuf::from(path),
            contents: vec![]
        };
        fm.canonicalize();
        fm.set_contents();
        fm
    }

    /// Get the contents of the folder given by the path
    pub fn get_contents_from_path(path: &PathBuf) -> io::Result<Vec<(EntryType, String)>> {
        let contents = path.read_dir()?;
        let mut result = Vec::new();

        for entry in contents {
            let entry_path = entry?.path();
            let path_str = entry_path.to_str().unwrap_or("/");
            let path = Path::new(path_str);
            if path.is_dir() {
                result.push((
                    EntryType::Directory,
                    String::from(path.file_name().unwrap().to_str().unwrap()),
                ));
            } else if path.is_file() {
                result.push((
                    EntryType::File,
                    String::from(path.file_name().unwrap().to_str().unwrap()),
                ));
            }
        }

        result.sort();

        return Ok(result);
    }

    /// Opens a children by cd into directories, using $EDITOR or by xdg-open
    pub fn open_child(&mut self, stdout: &mut dyn io::Write, dir: &str) {
        let self_clone = self.clone();
        let child_type = self_clone.get_type_of_child(dir);
        match child_type {
            Err(_) => {}
            Ok(EntryType::Directory) => self.change_dir(dir).unwrap(),
            Ok(EntryType::File) => {
                let path_to_child = self.get_path_to_child(dir);
                if path_to_child.is_err() {
                    return;
                }
                match get_mime_type((*path_to_child.clone().unwrap()).to_str().unwrap())
                    .unwrap()
                    .type_()
                {
                    mime::TEXT => {
                        let editor = var("EDITOR").unwrap();
                        write!(stdout, "{}", ToMainScreen).unwrap();
                        Command::new(editor)
                            .arg(&*(path_to_child.unwrap()))
                            .status()
                            .expect("Command failed to launch");
                        write!(stdout, "{}", ToAlternateScreen).unwrap();
                        write!(stdout, "{}", termion::cursor::Hide).unwrap();
                    }
                    _ => {
                        Command::new("xdg-open").arg(&*path_to_child.unwrap()).stdout(Stdio::null()).spawn().expect("Command failed to launch");
                    }
                }
            }
        }
    }

    /// Changes the directory of the file manager
    pub fn change_dir(&mut self, str: &str) -> Result<(), &str> {
        self.path.push(str);
        if !self.path.exists() {
            self.path.pop();
            return Err("Path not existent");
        }

        if !self.path.is_dir() {
            self.path.pop();
            return Err("Path is not a folder");
        }

        self.canonicalize();
        self.set_contents();

        return Ok(());
    }

    pub fn toggle_highlight_of(&mut self, str: String) {
        for tup in self.contents.iter_mut() {
            if tup.1 == str {
                tup.2 = !tup.2;
            }
        }
    }

    /// Get contents of the file manager
    pub fn get_contents(&self) -> Vec<(EntryType, String, bool)> {
        return self.contents.clone();
    }

    /// Sets the internal contents of the file manager
    fn set_contents(&mut self) {
        self.contents = FileManager::get_contents_from_path(&self.path).unwrap_or(vec![]).into_iter().map(|(t, s)| (t, s, false)).collect();
    }

    /// Get contents of the file manager at the given child
    pub fn get_contents_of_child(&self, dir: &str) -> io::Result<Vec<(EntryType, String)>> {
        let type_of_child = self.get_type_of_child(dir);

        if type_of_child.is_err() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                type_of_child.err().unwrap(),
            ));
        }

        if type_of_child.unwrap() == EntryType::File {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Entry is file"));
        }

        let path = self.get_path_to_child(dir).unwrap();
        return FileManager::get_contents_from_path(&path.to_path_buf());
    }

    /// Returns the current directory of the file manager as a String
    pub fn get_path_string(&self) -> String {
        return (*self.path.to_string_lossy()).to_string();
    }

    /// Returns the type of the given child
    pub fn get_type_of_child(&self, dir: &str) -> Result<EntryType, &str> {
        let mut path_clone = self.path.clone();
        path_clone.push(dir);
        if !path_clone.exists() {
            return Err("Path does not exist");
        }

        if path_clone.is_dir() {
            return Ok(EntryType::Directory);
        }
        if path_clone.is_file() {
            return Ok(EntryType::File);
        }
        return Err("Path type not known");
    }

    /// Returns the path to the given child
    pub fn get_path_to_child(&self, dir: &str) -> Result<Box<Path>, &str> {
        let mut path_clone = self.path.clone();
        path_clone.push(dir);

        if !path_clone.exists() {
            return Err("Path does not exist");
        }

        return Ok(path_clone.into_boxed_path());
    }

    /// Canonicalizes the path of the file manager
    fn canonicalize(&mut self) {
        match self.path.canonicalize() {
            Ok(a) => self.path = a,
            _ => {}
        };
    }
}
