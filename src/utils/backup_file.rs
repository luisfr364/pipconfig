use std::fs;

use super::os_config::find_config;

pub fn create_backup_file() {
    let configs = find_config().expect("no config found");

    if !configs.is_empty() {
        fs::File::create("../../pipewire.conf").expect("Unable to create file");
        fs::write("../../pipewire.conf", configs.to_string()).expect("error creating the backup");
    }
}
