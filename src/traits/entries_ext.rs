use glib::GString;
use gtk::prelude::*;

/// Additional methods for interacting with GTK entries
pub trait EntriesExt {
    /// Convenience method for `entry.text_length() == 0`.
    fn is_empty(&self) -> bool;

    /// Get the text of an entry, or `None` if it is empty.
    ///
    /// Equivalent to `entry.text().filter(|string| !string.is_empty())`
    fn text_nonempty(&self) -> Option<GString>;
}

impl<T: IsA<gtk::Entry>> EntriesExt for T {
    fn is_empty(&self) -> bool { self.text_length() == 0 }

    fn text_nonempty(&self) -> Option<GString> {
        let text = self.text();
        if text.is_empty() { None } else { Some(text) }
    }
}
