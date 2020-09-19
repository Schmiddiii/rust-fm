

#[derive(Clone)]
pub enum Types {
    Directory,
    File
}

pub fn std_theme(t: Types, b: bool) -> (&'static dyn termion::color::Color, &'static dyn termion::color::Color) {
    match (t,b) {
        (Types::Directory, false) => (&termion::color::Blue, &termion::color::Reset),
        (Types::Directory, true) => (&termion::color::Reset, &termion::color::Blue),
        (Types::File, false) => (&termion::color::Reset, &termion::color::Reset),
        (Types::File, true) => (&termion::color::Reset, &termion::color::White),
    }
}

pub fn std_theme_no_highlight(t: Types, b: bool) -> (&'static dyn termion::color::Color, &'static dyn termion::color::Color) {
    match (t,b) {
        (Types::Directory, _) => (&termion::color::Blue, &termion::color::Reset),
        (Types::File, _) => (&termion::color::Reset, &termion::color::Reset),
    }
}
