//! Curated dumping ground for misc. useful GTK widgets and traits.
//!
//! Contains an assortment of unofficial GTK widgets and traits for your GTK Rust projects.
//!
//! [Contributions welcome]()!

#[macro_use]
extern crate cascade;

#[macro_use]
extern crate shrinkwraprs;

pub mod entries;
pub mod widgets;

mod dynamic_resize;
mod keybindings;
mod revealing_button;
mod uuid_entry;

pub use cascade::cascade;

pub use self::{
    dynamic_resize::DynamicResize, entries::EntriesExt, revealing_button::RevealingButton,
    uuid_entry::UuidEntry,
};
