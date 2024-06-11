use gtk::prelude::*;
use gtk4::{self as gtk, ApplicationWindow, DialogFlags, MessageDialog};
use std::fs;

use crate::utils::os_config::find_config_path;

fn restore_config_from_backup() {
    let backup_file =
        fs::read_to_string("./../pipewire.conf").expect("Unable to read the backup file");

    fs::write(find_config_path().unwrap(), &backup_file).expect("Unable to write the backup file");
}

fn show_confirmation_dialog() {
    let dialog = MessageDialog::new(
        None::<&ApplicationWindow>,
        DialogFlags::MODAL,
        gtk4::MessageType::Question,
        gtk4::ButtonsType::OkCancel,
        "Want to proceed?",
    );

    dialog.show();

    let _response = dialog.connect_response(|dialog, response| {
        if response == gtk4::ResponseType::Ok {
            restore_config_from_backup();
        }
        dialog.close();
    });
}

pub fn restore_button() -> gtk::Button {
    let button = gtk::Button::new();

    button.set_label("Restore from backup");
    button.connect_clicked(move |_| {
        show_confirmation_dialog();
    });

    return button;
}
