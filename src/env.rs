use std::{fs, path::PathBuf};

pub fn load_env() {
    let config_path = get_config_path();
    let config = fs::read_to_string(config_path).unwrap_or_default();
    let config_lines: Vec<&str> = config.lines().collect();
    for line in config_lines {
        let parts: Vec<&str> = line.split('=').collect();
        if parts.len() != 2 {
            continue;
        }
        let key = parts[0];
        let value = parts[1];
        std::env::set_var(key, value);
    }
}

/// Get the path to the config file located in ~/.config/buddai.env
fn get_config_path() -> PathBuf {
    dirs::home_dir()
        .expect("Failed to get config directory")
        .join(".config")
        .join("buddai.env")
}
