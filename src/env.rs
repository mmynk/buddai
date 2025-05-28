use std::{fs, path::PathBuf};

pub fn load_env() {
    let config_path = get_config_path();
    // If we can't read the file, just return silently
    let config = match fs::read_to_string(config_path) {
        Ok(content) => content,
        Err(_) => return,
    };

    for line in config.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with(['#', '/', '=']) {
            continue;
        }

        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim();

            std::env::set_var(key, value);
        }
    }
}


/// Get the path to the config file located in ~/.config/buddai.env
fn get_config_path() -> PathBuf {
    dirs::home_dir()
        .expect("Failed to get config directory")
        .join(".config")
        .join("buddai.env")
}
