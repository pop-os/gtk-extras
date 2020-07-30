use cairo::Rectangle;
use gio::{self, MemoryInputStream};
use glib::Bytes;
use gtk::prelude::*;
use librsvg::{CairoRenderer, Loader, LoadingError, SvgHandle};
use std::path::Path;

/// Renders SVG images into a GTK DrawingArea via rsvg
///
/// [rsvg]: https://gitlab.gnome.org/GNOME/librsvg
#[derive(AsRef, Deref)]
#[as_ref]
#[deref]
pub struct SvgImage(gtk::DrawingArea);

impl SvgImage {
    pub fn new(handle: SvgHandle) -> Self {
        let drawing_area = cascade! {
            gtk::DrawingArea::new();
            ..connect_draw(move |w, cr| {
                let rect = Rectangle {
                    x: 0.0,
                    y: 0.0,
                    width: w.get_allocated_width() as f64,
                    height: w.get_allocated_height() as f64,
                };

                let renderer = CairoRenderer::new(&handle);
                renderer.render_document(cr, &rect).unwrap();
                Inhibit(false)
            });
        };

        SvgImage(drawing_area)
    }

    /// Renders a SVG image which is already stored in-memory.
    pub fn from_bytes(image: &[u8]) -> Result<Self, LoadingError> {
        // Is there a way to avoid either cloning or unsafe cast to 'static?
        let bytes = Bytes::from_owned(image.to_owned());
        let stream = MemoryInputStream::new_from_bytes(&bytes);
        let handle = Loader::new().read_stream(&stream,
                                               None::<&gio::File>,
                                               None::<&gio::Cancellable>)?;
        Ok(Self::new(handle))
    }

    /// Renders a SVG image found at the given path.
    pub fn from_file<P: AsRef<Path>>(image: P) -> Result<Self, LoadingError> {
        let handle = Loader::new().read_path(image)?;
        Ok(Self::new(handle))
    }
}

impl Into<gtk::DrawingArea> for SvgImage {
    fn into(self) -> gtk::DrawingArea { self.0 }
}
