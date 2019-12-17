//! Curated dumping ground for misc. useful GTK widgets and traits.
//!
//! Contains an assortment of unofficial GTK widgets and traits for your GTK Rust projects.
//!
//! [Contributions welcome]()!

#[macro_use]
extern crate cascade;
#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate log;

mod functions;
mod macros;
mod traits;
mod widgets_;

pub use self::{functions::*, macros::*, traits::*, widgets_::*};

pub use cascade::cascade;
