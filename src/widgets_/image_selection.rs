use gtk::prelude::*;
use std::rc::Rc;

/// A list of selections based on radio buttons, with optional images.
#[derive(Shrinkwrap)]
pub struct ImageSelection {
    #[shrinkwrap(main_field)]
    container: gtk::FlowBox,
}

impl ImageSelection {
    pub fn new<T: Copy + 'static>(
        variants: &[SelectionVariant<T>],
        placeholder: &str,
        event_cb: impl Fn(T) + 'static,
    ) -> Self {
        let event_cb = Rc::new(event_cb);

        let container = gtk::FlowBoxBuilder::new()
            .homogeneous(true)
            .selection_mode(gtk::SelectionMode::None)
            .build();

        let mut last_radio = None::<gtk::RadioButton>;
        let mut active_radio = None::<gtk::RadioButton>;

        for variant in variants {
            let event = variant.event;
            let event_cb_ = event_cb.clone();

            let radio = cascade! {
                radio: gtk::RadioButton::new();
                ..set_halign(gtk::Align::Center);
                ..join_group(last_radio.as_ref());
                ..connect_property_active_notify(move |_| {
                    event_cb_(event);
                });
            };

            if variant.active {
                active_radio = Some(radio.clone());
            }

            let image_path = variant.image.unwrap_or(placeholder);

            let widget = cascade! {
                gtk::Box::new(gtk::Orientation::Vertical, 12);
                ..add(
                    &gtk::ImageBuilder::new()
                        .file(image_path)
                        .build()
                );
                ..add(&gtk::LabelBuilder::new().label(variant.name).xalign(0.0).halign(gtk::Align::Center).build());
                ..add(&radio);
            };

            container.add(&widget);

            last_radio = Some(radio);
        }

        if let Some(radio) = active_radio {
            radio.set_active(true);
        }

        Self { container }
    }
}

pub struct SelectionVariant<'a, T> {
    pub name:   &'a str,
    pub image:  Option<&'a str>,
    pub active: bool,
    pub event:  T,
}
