//! Convenience types for interacting with common GSettings parameters.

use gio::{Settings, SettingsExt};
use glib::GString;
use std::path::Path;

const SCHEMA_PATH: &str = "/usr/share/glib-2.0/schemas/";

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
    new_checked_with_buffer(&mut String::with_capacity(64), schema)
}

/// Uses a pre-allocated buffer for constructing the schema path.
pub fn new_checked_with_buffer(buffer: &mut String, schema: &str) -> Option<Settings> {
    if schema_exists_with_buffer(buffer, schema) {
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
/// let buf = &mut String::with_capacity(64)
/// if settings::schema_exists_with_buffer(buf, schema) {
///     println!("settings for {} exists", schema);
/// }
/// ```
pub fn schema_exists(schema: &str) -> bool {
    schema_exists_with_buffer(&mut String::with_capacity(64), schema)
}

/// Uses a pre-allocated buffer for constructing the schema path.
pub fn schema_exists_with_buffer(buffer: &mut String, schema: &str) -> bool {
    buffer.clear();
    buffer.push_str(SCHEMA_PATH);
    buffer.push_str(schema);
    buffer.push_str(".gschema.xml");

    Path::new(buffer.as_str()).exists()
}

/// Convenience type for `org.gnome.gedit.preferences.editor`
pub struct GeditPreferencesEditor(pub Settings);

impl GeditPreferencesEditor {
    pub fn new() -> Self { Self(Settings::new("org.gnome.gedit.preferences.editor")) }

    /// Get the active scheme
    pub fn scheme(&self) -> Option<GString> { self.0.get_string("scheme") }

    /// Set the active scheme
    pub fn set_scheme(&self, scheme: &str) {
        self.0.set_string("scheme", scheme);
        Settings::sync();
    }
}

/// Convenience type for `org.gnome.desktop.interface`
pub struct GnomeDesktopInterface(pub Settings);

impl GnomeDesktopInterface {
    pub fn new() -> Self { Self(Settings::new("org.gnome.desktop.interface")) }

    /// Get the active GTK theme
    pub fn gtk_theme(&self) -> Option<GString> { self.0.get_string("gtk-theme") }

    /// Set the active GTK theme
    pub fn set_gtk_theme(&self, theme: &str) {
        self.0.set_string("gtk-theme", theme);
        Settings::sync();
    }
}
