use crate::filemanager::EntryType;

/// Standard theme used in the main list
pub fn std_theme(
    t: EntryType,
    b: bool,
) -> (
    &'static dyn termion::color::Color,
    &'static dyn termion::color::Color,
) {
    match (t, b) {
        (EntryType::Directory, false) => (&termion::color::Blue, &termion::color::Reset),
        (EntryType::Directory, true) => (&termion::color::Reset, &termion::color::Blue),
        (EntryType::File, false) => (&termion::color::Reset, &termion::color::Reset),
        (EntryType::File, true) => (&termion::color::Reset, &termion::color::White),
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
