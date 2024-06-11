use gtk::prelude::*;
use gtk4::{self as gtk, ApplicationWindow, DialogFlags, MessageDialog};
use std::{cell::RefCell, rc::Rc};

use crate::utils::os_config;

pub fn new(btn_label: String, configuration_file: &Rc<RefCell<String>>) -> gtk::Button {
    let button = gtk::Button::with_label(&btn_label);

    let configuration_file_clone = Rc::clone(&configuration_file);

    button.connect_clicked(move |_| {
        let configuration_file = Rc::clone(&configuration_file_clone);
        show_confirmation_dialog(&configuration_file, "Are you sure you want to proceed?");
    });
    return button;
}

pub fn show_confirmation_dialog(configuration_file: &Rc<RefCell<String>>, message: &str) {
    let dialog = MessageDialog::new(
        None::<&ApplicationWindow>,
        DialogFlags::MODAL,
        gtk4::MessageType::Question,
        gtk4::ButtonsType::OkCancel,
        &message.to_string(),
    );
    dialog.show();
    let configuration_file_clone = configuration_file.clone();

    dialog.connect_response(move |dialog, response| {
        if response == gtk4::ResponseType::Ok {
            save_configs(&configuration_file_clone) // Pass the `args` argument to the `callback` function.
        }
        dialog.close();
    });
}

pub fn save_configs(configuration_file: &Rc<RefCell<String>>) {
    let config = configuration_file.borrow();
    let _configs = os_config::create_new_config_file(config.to_string());
}
