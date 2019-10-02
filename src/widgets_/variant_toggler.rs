use gtk::prelude::*;
use std::{ops::Deref, rc::Rc};

/// A list box containing a collection of toggleable variants.
pub struct VariantToggler {
    container: gtk::Container,
}

impl VariantToggler {
    pub fn new<T: Copy + 'static>(
        variants: &[ToggleVariant<T>],
        event_cb: impl Fn(T, bool) + 'static,
    ) -> Self {
        let event_cb = Rc::new(event_cb);

        let container = gtk::ListBoxBuilder::new().selection_mode(gtk::SelectionMode::None).build();

        container.set_header_func(Some(Box::new(super::standard_header)));

        for variant in variants {
            let switch = gtk::SwitchBuilder::new()
                .halign(gtk::Align::End)
                .valign(gtk::Align::Center)
                .active(variant.active)
                .build();

            let event = variant.event;
            let event_cb_ = event_cb.clone();
            switch.connect_changed_active(move |switch| {
                event_cb_(event, switch.get_active());
            });

            let switch = switch.upcast::<gtk::Widget>();

            let title_label = gtk::LabelBuilder::new()
                .label(variant.name)
                .hexpand(true)
                .xalign(0.0)
                .use_underline(true)
                .mnemonic_widget(&switch)
                .build();

            let desc_label =
                gtk::LabelBuilder::new().xalign(0.0).label(variant.description).build();

            desc_label.get_style_context().add_class(&gtk::STYLE_CLASS_DIM_LABEL);

            let variant_container = gtk::GridBuilder::new()
                .row_spacing(2)
                .column_spacing(16)
                .margin_start(20)
                .margin_end(20)
                .margin_top(6)
                .margin_bottom(6)
                .valign(gtk::Align::Center)
                .build();

            variant_container.attach(&title_label, 0, 0, 1, 1);
            variant_container.attach(&desc_label, 0, 1, 1, 1);
            variant_container.attach(&switch, 1, 0, 1, 2);

            container.add(&variant_container);
        }

        Self { container: container.upcast::<gtk::Container>() }
    }
}

impl Deref for VariantToggler {
    type Target = gtk::Container;

    fn deref(&self) -> &Self::Target { &self.container }
}

impl Into<gtk::Container> for VariantToggler {
    fn into(self) -> gtk::Container {
        self.container
    }
}

/// A variant for the `VariantToggler` widget.
pub struct ToggleVariant<'a, T> {
    pub name:        &'a str,
    pub description: &'a str,
    pub active:      bool,
    pub event:       T,
}
