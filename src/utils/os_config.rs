
use regex::Captures;
use regex::Regex;
use std::env;
use std::process::Command;
use std::{
    cell::RefCell,
    fs,
    io::{self, Error},
    path::Path,
    rc::Rc,
};

//Function to find the config file path
pub fn find_config_path() -> std::result::Result<String, Error> {
    let paths = [
        "$XDG_CONFIG_HOME/pipewire/pipewire.conf",
        "/etc/pipewire/pipewire.conf",
        "/usr/share/pipewire/pipewire.conf",
        "/usr/share/pipewire/pipewire.conf.d/",
        "/etc/pipewire/pipewire.conf.d/",
        "$XDG_CONFIG_HOME/pipewire/pipewire.conf.d/",
    ];

    for path in &paths {
        if Path::new(path).exists() {
            return Ok(path.to_string());
        }
    }

    return Err(io::Error::new(
        io::ErrorKind::NotFound,
        "No config file found",
    ));
}

pub fn find_config() -> std::result::Result<String, Error> {
    let config_file_path = find_config_path().unwrap();

    let contents =
        fs::read_to_string(config_file_path).expect("Something went wrong reading the file");

    if contents.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "No config file found",
        ));
    } else {
        return Ok(contents);
    }
}

pub fn find_config_value(config_string: String, config_to_search: String) -> String {
    let custom_regex = format!(
        r"(\s*#\s*{}\s*=\s*)(.*?)$",
        config_to_search.replace(".", "\\.")
    );

    //regex to find the config property
    let re = Regex::new(&custom_regex).unwrap();
    let config_lines = config_string.lines();

    let mut return_value = String::new();

    for line in config_lines {
        if line.contains(&config_to_search) {
            let captures = re.captures(&line).unwrap();
            return_value = captures[2].to_string();
        }
    }

    return return_value;
}

//Function for changing the adapter config value to the one provided
pub fn change_adapter_configs(
    config_string: &Rc<RefCell<String>>,
    config_to_change: String,
    config_value: String,
) {
    let mut config_string = config_string.borrow_mut();
    let custom_regex = format!(
        r"(\s*#\s*{}\s*=\s*)(.*?)$",
        config_to_change.replace(".", "\\.")
    );

    //regex to find the config property
    let re = Regex::new(&custom_regex).unwrap();
    let mut config_lines: Vec<&str> = config_string.lines().collect();

    //var to hold the old and new adapter config
    let mut _new_adapter_config = String::new();

    //for loop for iterating through the config lines and finding and replacing
    //the config prop
    for (i, line) in config_lines.iter().enumerate() {
        if line.contains(&config_to_change) {
            let _captures = re.captures(&line).unwrap();

            _new_adapter_config = re
                .replace(line, |caps: &Captures| {
                    format!("{}{}", &caps[1], config_value)
                })
                .to_string();
            config_lines[i] = &_new_adapter_config;
            break;
        }
    }

    //let new_config = config_string.replace(&old_adapter_config, &new_adapter_config);
    *config_string = Into::into(config_lines.join("\n"));
}

//Function to create a new config file
pub fn create_new_config_file(config_string: String) -> Result<(), Error>{
    let config_file_path = find_config_path().unwrap();
    //Create temporary dir
    let temp_dir = env::temp_dir();

    //Add the temporary file's path to the temporary dir
    let temp_path = temp_dir.join("new_config.tmp");

    //Write the temporary file
    let _temp_file = fs::write(&temp_path, config_string.as_bytes());

    //Execute the move command with pkexec to move the temporaty config file to the original config file's path
    let command_status = Command::new("pkexec")
        .arg("mv")
        .arg(temp_path.to_str().unwrap())
        .arg(config_file_path)
        .status();

    if !command_status.is_ok() {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "failed to run command"))
    }

    return Ok(());
}
