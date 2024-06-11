use crate::components::restore_button::restore_button;
use crate::components::{horizontal_box, save_button};
use crate::pre_build;
use crate::utils::{config_file, os_config};

use gtk::gdk::Display;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
use gtk4 as gtk;
use std::{cell::RefCell, rc::Rc};

// Funtion to load the css file
pub fn load_css() {
    let provider = gtk::CssProvider::new();
    provider.load_from_path("./src/styles.css");

    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Couldn't load the css file"),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

// Function to build the UI
pub fn build_ui(app: &Application) {
    //Execute code inside the pre_build function
    pre_build::pre_build();
    config_file::config_file_exists();

    let configuration_file = Rc::new(RefCell::new(
        os_config::find_config().expect("No config file found"),
    ));

    //-------------- Main Container --------------//
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 1);
    vbox.add_css_class("vbox");
    vbox.set_halign(gtk::Align::Center);
    vbox.set_valign(gtk::Align::Center);

    //-------------- Entry Boxes --------------//

    let entry_with_label_1 = horizontal_box::new(
        &configuration_file,
        "default.clock.rate".to_string(),
        "Default Clock Rate".to_string(),
    );

    let entry_with_label_2 = horizontal_box::new(
        &configuration_file,
        "default.clock.allowed-rates".to_string(),
        "Default Clock Allowed-Rates".to_string(),
    );
    let entry_with_label_3 = horizontal_box::new(
        &configuration_file,
        "audio.format".to_string(),
        "Audio Format".to_string(),
    );

    //-------------- Buttons --------------//

    let restore_button = restore_button();

    let confirmation_button = save_button::new("Save".to_string(), &configuration_file);

    //-------------- Append to the main container --------------//
    vbox.append(&entry_with_label_1);
    vbox.append(&entry_with_label_2);
    vbox.append(&entry_with_label_3);
    vbox.append(&confirmation_button);
    vbox.append(&restore_button);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("PipConfig")
        .default_width(110)
        .default_height(300)
        .resizable(false)
        .build();

    window.set_child(Some(&vbox));

    window.present();
}
