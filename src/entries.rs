use gtk::prelude::*;
use itertools::Itertools;
use std::rc::Rc;

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
