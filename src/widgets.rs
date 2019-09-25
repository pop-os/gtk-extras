use gtk::prelude::*;

/// Fetches all immediate widgets which are entries in the given container.
pub fn iter_from<T: IsA<gtk::Widget>, C: IsA<gtk::Container>>(
    container: &C,
) -> impl DoubleEndedIterator<Item = T> {
    container.get_children().into_iter().filter_map(|w| w.downcast::<T>().ok())
}
