use gtk::prelude::*;
use std::{cell::RefCell, rc::Rc};
use uuid::Uuid;

/// Variant of an Entry for handling UUID inputs
///
/// When inputs are given to this entry, the input will be cleared if it does
/// not contain a valid UUID value after the allotted timeout value has passed
/// since the last input into the entry.
///
/// # Use Case
///
/// System76 uses this widget for an internal project which involves scanning
/// bar codes into entries, which the scanner translates into a string
/// representation of a UUID.
///
/// To reduce the chance of human error, entries will clear their fields when
/// they contain invalid inputs. However, because scanners input one character
/// at a time into the entry, a timeout is necessary to wait for the scanner to
/// complete its input.
///
/// Furthermore, once a UUID has been submitted, the scanner sends the return
/// key, which activates the entry, submits the UUID to be handled in another
/// process, and clears the entry so that the user can scan the next bar code.
///
/// # Examples
///
/// ```rust
/// use gtk_extras::UuidEntry;
/// use uuid::Uuid;
///
/// enum UiEvent {
///     Received(Uuid)
/// }
///
/// gtk::init();
///
/// let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
///
/// let entry = UuidEntry::new(1000);
/// let sender = sender.clone();
/// entry.connect_activate(move |entry| {
///     if let Some(uuid) = entry.get_uuid() {
///         let _ = sender.send(UiEvent::Received(uuid));
///     }
/// });
///
/// receiver.attach(None, move |event| {
///     match event {
///         UiEvent::Received(uuid) => {
///             println!("received {}", uuid);
///         }
///     }
///
///     glib::Continue(true)
/// });
/// ```
#[derive(Shrinkwrap)]
pub struct UuidEntry(gtk::Entry);

impl UuidEntry {
    pub fn new(timeout: u32) -> Self {
        let entry = gtk::Entry::new();
        let source = Rc::new(RefCell::new(None));

        entry.connect_changed(move |entry| {
            // Ignore the change if the change was to set the entry to an empty string.
            if entry.get_text().map_or(true, |text| text.as_str().is_empty()) {
                return;
            }

            let entry = entry.clone();

            let mut source = source.borrow_mut();
            if let Some(source) = source.take() {
                glib::source_remove(source);
            }

            *source = Some(gtk::timeout_add(timeout, move || {
                if let Some(text) = entry.get_text() {
                    if text.parse::<Uuid>().is_err() {
                        entry.set_text("");
                    }
                }

                gtk::Continue(false)
            }));
        });

        Self(entry)
    }

    pub fn connect_activate<F: Fn(&Self) + 'static>(&self, f: F) -> glib::SignalHandlerId {
        self.0.connect_activate(move |e| f(&Self(e.clone())))
    }

    /// Fetches the UUID, and clears the contents of the entry.
    pub fn get_uuid(&self) -> Option<Uuid> {
        self.get_text().and_then(|text| {
            self.set_text("");
            text.parse::<Uuid>().ok()
        })
    }
}
