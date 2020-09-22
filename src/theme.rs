use crate::filemanager::EntryType;

/// Standard theme used in the main list
pub fn std_theme(
    t: (EntryType, bool),
    b: bool,
) -> (
    &'static dyn termion::color::Color,
    &'static dyn termion::color::Color,
) {
    match (t, b) {
        ((_, true), false) => (&termion::color::Yellow, &termion::color::Reset),
        ((_, true), true) => (&termion::color::Black, &termion::color::Yellow),
        ((EntryType::Directory, false), false) => (&termion::color::Blue, &termion::color::Reset),
        ((EntryType::Directory, false), true) => (&termion::color::Reset, &termion::color::Blue),
        ((EntryType::File, false), false) => (&termion::color::Reset, &termion::color::Reset),
        ((EntryType::File, false), true) => (&termion::color::Reset, &termion::color::White),
    }
}

/// Standard theme without highlights used in the preview
pub fn std_theme_no_highlight(
    t: EntryType,
    b: bool,
) -> (
    &'static dyn termion::color::Color,
    &'static dyn termion::color::Color,
) {
    match (t, b) {
        (EntryType::Directory, _) => (&termion::color::Blue, &termion::color::Reset),
        (EntryType::File, _) => (&termion::color::Reset, &termion::color::Reset),
    }
}
