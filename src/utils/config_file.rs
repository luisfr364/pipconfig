use serde::Deserialize;
use std::{
    fs::{self},
    path::Path,
};

use super::backup_file::create_backup_file;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub first_initialization: bool,
}

const INITIAL_CONFIG: &str = r#"
{
  "first_initialization": false
}
"#;

// Function to check if the config file exists and create it if it doesn't
pub fn config_file_exists() -> bool {
    if Path::new("../../config.json").exists() {
        return true;
    };

    fs::File::create("../../config.json").expect("Unable to create file");

    fs::write("../../config.json", &INITIAL_CONFIG).expect("file not found");
    create_backup_file();
    return false;
}

// Function to read the config.json file and return the data or a default value
pub fn read_config_file() -> Config {
    let config_file_json = fs::read_to_string(Path::new("../../config.json")).unwrap();

    let config_file_data = serde_json::from_str(&config_file_json);

    let data = match config_file_data {
        Ok(data) => data,
        Err(_) => Config {
            first_initialization: true,
        },
    };

    return data;
}
