use gtk::prelude::*;

/// A widget which reveals a child widget when clicked
///
/// The primary widget is displayed at all times, whereas the child widget is generated
/// on the first reveal.
#[derive(Shrinkwrap)]
pub struct RevealingButton {
    #[shrinkwrap(main_field)]
    container: gtk::Container,

    pub event_box: gtk::EventBox,
    pub revealer:  gtk::Revealer,
}

impl RevealingButton {
    pub fn new<M>(main_content: M) -> Self
    where
        M: FnOnce(&gtk::Image) -> gtk::Widget,
    {
        let dropdown_image = gtk::ImageBuilder::new()
            .icon_name("pan-end-symbolic")
            .icon_size(gtk::IconSize::Menu.into())
            .halign(gtk::Align::Start)
            .valign(gtk::Align::Center)
            .build();

        let dropdown_image_ = dropdown_image.downgrade();
        let revealer = cascade! {
            gtk::Revealer::new();
            ..connect_property_reveal_child_notify(move |revealer| {
                dropdown_image_.upgrade()
                    .expect("dropdown image did not exist")
                    .set_from_icon_name(
                        Some(if revealer.get_reveal_child() {
                            "pan-down-symbolic"
                        } else {
                            "pan-end-symbolic"
                        }),
                        gtk::IconSize::Menu
                    );
            });
        };

        let event_box = cascade! {
            gtk::EventBoxBuilder::new()
                .can_focus(false)
                .hexpand(true)
                .events(gdk::EventMask::BUTTON_PRESS_MASK)
                .build();
            ..add(&main_content(&dropdown_image));
        };

        let container = cascade! {
            gtk::Box::new(gtk::Orientation::Vertical, 4);
            ..set_border_width(12);
            ..set_can_focus(false);
            ..add(&event_box);
            ..add(&revealer);
        };

        Self { container: container.upcast::<gtk::Container>(), event_box, revealer }
    }

    /// Activates when the widget's container is clicked.
    pub fn connect_clicked<F: Fn(gtk::Revealer) + 'static>(&self, func: F) {
        let revealer = self.revealer.downgrade();
        self.event_box.connect_button_press_event(move |_, _| {
            func(revealer.upgrade().expect("revealer for device did not exist"));
            gtk::Inhibit(true)
        });
    }

    /// Reveals an inner child, and generates it if it is missing.
    pub fn reveal<F: FnMut() -> gtk::Widget>(&self, mut func: F) -> bool {
        let reveal = if self.revealer.get_reveal_child() {
            false
        } else {
            if self.revealer.get_child().is_none() {
                self.revealer.add(&func());
            }

            true
        };

        self.revealer.set_reveal_child(reveal);
        reveal
    }

    /// Defines the revealed status of the child widget.
    pub fn set_reveal_child(&self, reveal: bool) { self.revealer.set_reveal_child(reveal); }

    /// If the button has already generated a child widget, destroy it.
    pub fn destroy_revealed(&self) {
        if let Some(child) = self.revealer.get_child() {
            child.destroy();
        }
    }
}
