mod revealing_button;
mod uuid_entry;
mod variant_toggler;

pub use self::{
    revealing_button::RevealingButton,
    uuid_entry::UuidEntry,
    variant_toggler::{ToggleVariant, VariantToggler},
};

use gtk::prelude::*;

/// Inserts a separator as a header between rows in a list box.
fn standard_header(current: &gtk::ListBoxRow, before: Option<&gtk::ListBoxRow>) {
    if before.is_some() {
        current.set_header(Some(&gtk::Separator::new(gtk::Orientation::Horizontal)));
    }
}
