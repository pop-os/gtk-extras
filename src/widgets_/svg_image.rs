use gtk::prelude::*;
use resvg::{backend_cairo, usvg, Options, ScreenSize};
use std::path::Path;

/// Renders SVG images into a GTK DrawingArea via resvg
///
/// [resvg]: https://github.com/RazrFalcon/resvg
#[derive(Shrinkwrap)]
pub struct SvgImage(gtk::DrawingArea);

impl SvgImage {
    pub fn new(tree: usvg::Tree, opt: Options) -> Self {
        let drawing_area = cascade! {
            gtk::DrawingArea::new();
            ..connect_draw(move |w, cr| {
                let s = ScreenSize::new(
                    w.get_allocated_width() as u32,
                    w.get_allocated_height() as u32,
                ).expect("failed to create ScreenSize");

                backend_cairo::render_to_canvas(&tree, &opt, s, cr);
                Inhibit(false)
            });
        };

        SvgImage(drawing_area)
    }

    /// Renders a SVG image which is already stored in-memory.
    pub fn from_bytes(image: &[u8]) -> Result<Self, usvg::Error> {
        let opt = Options::default();
        let tree = usvg::Tree::from_data(image, &opt.usvg)?;

        Ok(Self::new(tree, opt))
    }

    /// Renders a SVG image found at the given path.
    pub fn from_file<P: AsRef<Path>>(image: P) -> Result<Self, usvg::Error> {
        let opt = Options::default();
        let tree = usvg::Tree::from_file(image, &opt.usvg)?;

        Ok(Self::new(tree, opt))
    }
}

impl Into<gtk::DrawingArea> for SvgImage {
    fn into(self) -> gtk::DrawingArea { self.0 }
}
