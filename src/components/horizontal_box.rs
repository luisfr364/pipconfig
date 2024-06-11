use std::{cell::RefCell, rc::Rc};

use crate::utils::os_config::{change_adapter_configs, find_config_value};
use gtk::prelude::*;
use gtk4 as gtk;

pub fn new(
    configuration_file: &Rc<RefCell<String>>,
    config_to_search: String,
    label_text: String,
) -> gtk::Box {
    let label = gtk::Label::new(Some(&label_text));
    label.set_width_chars(23);

    let entry = gtk::Entry::new();

    let configuration_file_clone = Rc::clone(&configuration_file);
    let config_search = config_to_search.clone();

    entry.add_css_class("entry");

    entry.set_width_chars(15);
    entry.set_text(&find_config_value(
        configuration_file_clone.borrow().to_string(),
        config_to_search,
    ));
    entry.connect_changed(move |entry| {
        let text = entry.text();

        change_adapter_configs(
            &configuration_file_clone,
            config_search.to_string(),
            text.to_string(),
        );
    });

    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 1);
    hbox.append(&label);
    hbox.append(&entry);
    hbox.set_margin_bottom(10);
    hbox.set_margin_end(20);

    return hbox;
}
