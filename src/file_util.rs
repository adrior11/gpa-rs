use std::{fs, path::PathBuf};

use anyhow::{Context, Result};

use crate::gpa::GPA;

const APP_NAME: &str = "gpa-rs";
const CONFIG_FILE: &str = "gpa.json";

pub fn get_config_path() -> PathBuf {
    let mut path = dirs::config_dir().expect("Could not find XDG config dir");
    path.push(APP_NAME);
    path.push(CONFIG_FILE);
    path
}

pub fn load_or_create_config() -> Result<GPA> {
    let config_path = get_config_path();

    if !config_path.exists() {
        ensure_config_dir()
            .with_context(|| format!("creating {:?}", config_path.parent().unwrap()))?;
        let default_gpa = GPA::default();
        let json = serde_json::to_string_pretty(&default_gpa)?;
        fs::write(&config_path, json)?;
    }

    let json: String =
        fs::read_to_string(&config_path).with_context(|| format!("reading {:?}", config_path))?;
    let gpa: GPA =
        serde_json::from_str(&json).with_context(|| format!("parsing {:?}", config_path))?;
    Ok(gpa)
}

fn ensure_config_dir() -> Result<()> {
    let mut path = dirs::config_dir().expect("Could not find XDG config dir");
    path.push(APP_NAME);
    Ok(fs::create_dir_all(&path)?)
}
