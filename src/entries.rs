use glib::GString;
use gtk::prelude::*;
use itertools::Itertools;
use std::rc::Rc;

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

/// Links multiple entries by triggering a focus grab on an activation.
///
/// The last entry will activate the `last` closure.
pub fn link<'a, C: Fn(&gtk::Entry) -> bool + 'static, F: Fn(&gtk::Entry) + 'static>(
    entries: impl Iterator<Item = gtk::Entry>,
    condition: C,
    last: F,
) {
    let condition = Rc::new(condition);
    let mut last_entry = None::<gtk::Entry>;
    for (current, next) in entries.tuple_windows() {
        let next_ = next.clone();
        let condition = condition.clone();
        current.connect_activate(move |entry| {
            if condition(entry) {
                next_.grab_focus()
            }
        });
        last_entry = Some(next);
    }

    if let Some(entry) = last_entry {
        entry.connect_activate(move |entry| last(entry));
    }
}
