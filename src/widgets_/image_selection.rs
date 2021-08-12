use gtk::prelude::*;
use std::{collections::HashMap, rc::Rc};

/// A list of selections based on radio buttons, with optional images.
#[derive(AsRef, Deref)]
pub struct ImageSelection {
    #[as_ref]
    #[deref]
    container: gtk::FlowBox,
}

impl ImageSelection {
    pub fn new<T: Copy + 'static>(
        variants: &[SelectionVariant<T>],
        placeholder: ImageSrc,
        event_cb: impl Fn(T) + 'static,
    ) -> Self {
        let event_cb = Rc::new(event_cb);

        let container = gtk::FlowBoxBuilder::new()
            .can_focus(true)
            .focus_on_click(false)
            .homogeneous(true)
            .selection_mode(gtk::SelectionMode::None)
            .build();

        let mut last_radio = None::<gtk::RadioButton>;
        let mut active_radio = None::<gtk::RadioButton>;
        let mut row_association = HashMap::new();

        for variant in variants {
            let event = variant.event;
            let event_cb_ = event_cb.clone();

            let radio = cascade! {
                gtk::RadioButton::new();
                ..set_can_focus(false);
                ..set_halign(gtk::Align::Center);
                ..join_group(last_radio.as_ref());
                ..connect_active_notify(move |_| {
                    event_cb_(event);
                });
            };

            if variant.active {
                active_radio = Some(radio.clone());
            }

            let image_path = variant.image.unwrap_or(placeholder);

            let mut ib = gtk::ImageBuilder::new();

            match image_path {
                ImageSrc::File(path) => ib = ib.file(path),
                ImageSrc::Resource(res) => ib = ib.resource(res)
            }

            let image = ib.build().upcast::<gtk::Widget>();

            if let Some((width, height)) = variant.size_request {
                image.set_size_request(width, height);
            };

            let widget = cascade! {
                gtk::Box::new(gtk::Orientation::Vertical, 12);
                ..add(&image);
                ..add(&gtk::LabelBuilder::new().label(variant.name).xalign(0.0).halign(gtk::Align::Center).build());
                ..add(&radio);
            };

            let child = cascade! {
                gtk::FlowBoxChild::new();
                ..add(&widget);
            };

            container.add(&child);

            row_association.insert(child, radio.clone());

            last_radio = Some(radio);
        }

        if let Some(radio) = active_radio {
            radio.set_active(true);
        }

        container.connect_child_activated(move |_, child| {
            if let Some(radio) = row_association.get(child) {
                radio.set_active(true);
            }
        });

        Self { container }
    }
}

#[derive(Clone, Copy)]
pub enum ImageSrc<'a> {
    File(&'a str),
    Resource(&'a str)
}

pub struct SelectionVariant<'a, T> {
    pub name:         &'a str,
    pub image:        Option<ImageSrc<'a>>,
    pub size_request: Option<(i32, i32)>,
    pub active:       bool,
    pub event:        T,
}
