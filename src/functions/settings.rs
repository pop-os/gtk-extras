//! Convenience types for interacting with common GSettings parameters.

use gio::{Settings, SettingsSchemaSource};
use glib::GString;
use gdk::prelude::*;

/// Checks if a schema exists before attempting to create a `Settings` for it
///
/// # Notes
///
/// This is equivalent to
///
/// ```
/// use gtk_extras::settings;
///
/// let schema = "org.freedesktop.Tracker";
/// let buf = &mut String::with_capacity(64);
/// if let Some(settings) = settings::new_checked_with_buffer(buf, schema) {
///     println!("settings for {} was found", schema);
/// }
/// ```
pub fn new_checked(schema: &str) -> Option<Settings> {
    if schema_exists(schema) {
        Some(Settings::new(schema))
    } else {
        None
    }
}

/// Verifies that a schema exists
///
/// The default behavior of `GSettings` is to abort a program which tries to access a
/// schema which does not exist. However, this is less than ideal in the real world,
/// where the existing of a schema is entirely optional, so this function provides a
/// means to validate if a schema exists in advance.
///
/// # Notes
///
/// This is equivalent to
///
/// ```
/// use gtk_extras::settings;
///
/// let schema = "org.gnome.nautilus";
/// let buf = &mut String::with_capacity(64);
/// if settings::schema_exists_with_buffer(buf, schema) {
///     println!("settings for {} exists", schema);
/// }
/// ```
pub fn schema_exists(schema: &str) -> bool {
    match SettingsSchemaSource::default() {
        Some(source) => source.lookup(schema, true).is_some(),
        None => false,
    }
}

/// Convenience type for `org.gnome.gedit.preferences.editor`
pub struct GeditPreferencesEditor(pub Settings);

impl GeditPreferencesEditor {
    pub fn new() -> Self { Self(Settings::new("org.gnome.gedit.preferences.editor")) }

    pub fn new_checked() -> Option<Self> {
        new_checked("org.gnome.gedit.preferences.editor").map(Self)
    }

    /// Get the active scheme
    pub fn scheme(&self) -> GString { self.0.string("scheme") }

    /// Set the active scheme
    pub fn set_scheme(&self, scheme: &str) {
        let _ = self.0.set_string("scheme", scheme);
        Settings::sync();
    }
}

/// Convenience type for `org.gnome.desktop.interface`
pub struct GnomeDesktopInterface(pub Settings);

impl GnomeDesktopInterface {
    pub fn new() -> Self { Self(Settings::new("org.gnome.desktop.interface")) }

    pub fn new_checked() -> Option<Self> {
        new_checked("org.gnome.desktop.interface").map(Self)
    }

    /// Get the active color scheme
    pub fn color_scheme(&self) -> GString { self.0.string("color-scheme") }

    /// Set the active color scheme
    pub fn set_color_scheme(&self, theme: &str) {
        let _ = self.0.set_string("color-scheme", theme);
        Settings::sync();
    }

    /// Get the active GTK theme
    pub fn gtk_theme(&self) -> GString { self.0.string("gtk-theme") }

    /// Set the active GTK theme
    pub fn set_gtk_theme(&self, theme: &str) {
        let _ = self.0.set_string("gtk-theme", theme);
        Settings::sync();
    }
}

/// Convenience type for `org.gnome.meld`
pub struct MeldPreferencesEditor(pub Settings);

impl MeldPreferencesEditor {
    pub fn new() -> Self { Self(Settings::new("org.gnome.meld")) }

    pub fn new_checked() -> Option<Self> {
        new_checked("org.gnome.meld").map(Self)
    }

    /// Get the active scheme
    pub fn style_scheme(&self) -> GString { self.0.string("style-scheme") }

    /// Set the active scheme
    pub fn set_style_scheme(&self, scheme: &str) {
        let _ = self.0.set_string("style-scheme", scheme);
        Settings::sync();
    }
}
