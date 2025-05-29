use std::{fs, path::PathBuf};

use dirs::config_dir;

use crate::gpa::GPA;

const APP_NAME: &str = "gpa-calc";
const CONFIG_FILE: &str = "gpa.json";

pub fn get_config_path() -> PathBuf {
    let mut path = config_dir().expect("Could not find gpa-calc directory");
    path.push(APP_NAME);
    path.push(CONFIG_FILE);
    path
}

pub fn ensure_config_dir() -> std::io::Result<()> {
    let mut path = config_dir().expect("Could not find gpa-calc directory");
    path.push(APP_NAME);
    fs::create_dir_all(&path)
}

pub fn load_or_create_config() -> Result<GPA, Box<dyn std::error::Error>> {
    let config_path = get_config_path();

    if !config_path.exists() {
        ensure_config_dir()?;
        let default_gpa = GPA::default();
        let json = serde_json::to_string_pretty(&default_gpa)?;
        fs::write(&config_path, json)?;
    }

    let json = fs::read_to_string(config_path)?;
    Ok(serde_json::from_str(&json)?)
}
