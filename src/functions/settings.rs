//! Convenience types for interacting with common GSettings parameters.

use gio::{Settings, SettingsExt};
use glib::GString;

/// Convenience type for `org.gnome.gedit.preferences.editor`
pub struct GeditPreferencesEditor(pub Settings);

impl GeditPreferencesEditor {
    pub fn new() -> Self { Self(Settings::new("org.gnome.gedit.preferences.editor")) }

    /// Get the active scheme
    pub fn scheme(&self) -> Option<GString> { self.0.get_string("scheme") }

    /// Set the active scheme
    pub fn set_scheme(&self, scheme: &str) { self.0.set_string("scheme", scheme); }
}

/// Convenience type for `org.gnome.desktop.interface`
pub struct GnomeDesktopInterface(pub Settings);

impl GnomeDesktopInterface {
    pub fn new() -> Self { Self(Settings::new("org.gnome.desktop.interface")) }

    /// Get the active GTK theme
    pub fn gtk_theme(&self) -> Option<GString> { self.0.get_string("gtk-theme") }

    /// Set the active GTK theme
    pub fn set_gtk_theme(&self, theme: &str) { self.0.set_string("gtk-theme", theme); }
}
