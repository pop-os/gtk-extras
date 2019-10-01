use glib::GString;
use gtk::prelude::*;

/// Additional methods for interacting with GTK entries
pub trait EntriesExt {
    /// Convenience method for `entry.get_text_length() == 0`.
    fn is_empty(&self) -> bool;

    /// Get the text of an entry, or `None` if it is empty.
    ///
    /// Equivalent to `entry.get_text().filter(|string| !string.is_empty())`
    fn get_text_nonempty(&self) -> Option<GString>;
}

impl<T: IsA<gtk::Entry>> EntriesExt for T {
    fn is_empty(&self) -> bool { self.get_text_length() == 0 }

    fn get_text_nonempty(&self) -> Option<GString> {
        self.get_text().filter(|string| !string.is_empty())
    }
}
