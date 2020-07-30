use gtk;
use gtk::prelude::*;
use gtk_extras::SvgImage;
use std::env;

fn main() {
    gtk::init().unwrap();

    let path = env::args().skip(1).next().unwrap();
    let svg_image = SvgImage::from_file(path).unwrap();
    let drawing_area: gtk::DrawingArea = svg_image.into();

    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.add(&drawing_area);
    window.show_all();

    gtk::main();
}
