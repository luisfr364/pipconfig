mod build_ui;
mod pre_build;

pub mod utils {
    pub mod backup_file;
    pub mod config_file;
    pub mod os_config;
}

pub mod components {
    pub mod horizontal_box;
    pub mod restore_button;
    pub mod save_button;
}

use build_ui::{build_ui, load_css};

use gtk::prelude::*;
use gtk::{glib, Application};
use gtk4 as gtk;

fn main() -> glib::ExitCode {
    //build the application
    let app = Application::builder()
        .application_id("com.luisf364.pipconfig")
        .build();

    //Load css file on app startup
    app.connect_startup(|_| load_css());

    //Build the app Ui on app activation using module ui_build
    app.connect_activate(|app| {
        build_ui(app);
    });

    app.run()
}
