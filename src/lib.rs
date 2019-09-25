//! Curated dumping ground for misc. useful GTK widgets and traits.
//!
//! Contains an assortment of unofficial GTK widgets and traits for your GTK Rust projects.
//!
//! [Contributions welcome]()!

#[macro_use]
extern crate shrinkwraprs;

mod keybindings;
mod uuid_entry;

pub use self::uuid_entry::UuidEntry;
