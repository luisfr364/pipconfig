use crate::utils::backup_file::create_backup_file;
use crate::utils::config_file;
use crate::utils::config_file::{read_config_file, Config};

// Function to run before building the UI containing the pre-build logic
pub fn pre_build() {
    config_file::config_file_exists();

    let config_file_data: Config = read_config_file();

    if config_file_data.first_initialization {
        create_backup_file();
        println!("Backup file created");
    }
}
