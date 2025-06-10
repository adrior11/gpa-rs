use std::{fs, path::PathBuf};

use anyhow::Context;

use crate::gpa::GPA;

const APP_NAME: &str = "gpa-rs";
const CONFIG_FILE: &str = "gpa.json";

pub fn get_config_path() -> PathBuf {
    let mut path = dirs::config_dir().expect("Could not find XDG config dir");
    path.push(APP_NAME);
    path.push(CONFIG_FILE);
    path
}

pub fn load_or_create_config() -> anyhow::Result<GPA> {
    let config_path = get_config_path();

    if !config_path.exists() {
        ensure_config_dir()
            .with_context(|| format!("creating {:?}", config_path.parent().unwrap()))?;
        let default_gpa = GPA::default();
        let json = serde_json::to_string_pretty(&default_gpa)?;
        fs::write(&config_path, json).with_context(|| format!("writing {config_path:?}"))?;
    }

    let json: String =
        fs::read_to_string(&config_path).with_context(|| format!("reading {config_path:?}"))?;
    let gpa: GPA =
        serde_json::from_str(&json).with_context(|| format!("parsing {config_path:?}"))?;
    Ok(gpa)
}

fn ensure_config_dir() -> anyhow::Result<()> {
    let mut path = dirs::config_dir().expect("Could not find XDG config dir");
    path.push(APP_NAME);
    fs::create_dir_all(&path).context("creating config directory")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{env, os::unix::fs::PermissionsExt, path::Path};

    use tempfile::TempDir;

    use crate::gpa::Lecture;

    use super::*;

    fn redirect_config_home(tmp: &Path) {
        env::set_var("XDG_CONFIG_HOME", tmp);
        env::set_var("HOME", tmp);
        env::set_var("APPDATA", tmp);
    }

    #[test]
    #[serial_test::serial]
    fn creates_default_when_missing() {
        let tmp = TempDir::new().unwrap();
        redirect_config_home(tmp.path());

        let cfg = get_config_path();
        assert!(!cfg.exists());

        let gpa = load_or_create_config().unwrap();
        assert!(cfg.exists());
        assert_eq!(gpa.target_average, 2.0);
        assert_eq!(gpa.lectures.len(), 0);
    }

    #[test]
    #[serial_test::serial]
    fn loads_existing_config() {
        let tmp = TempDir::new().unwrap();
        redirect_config_home(tmp.path());

        let cfg = get_config_path();
        fs::create_dir_all(cfg.parent().unwrap()).unwrap();

        let gpa = GPA::default();
        let json = serde_json::to_string_pretty(&gpa).unwrap();
        fs::write(&cfg, json).unwrap();

        let loaded_gpa = load_or_create_config().unwrap();
        assert_eq!(loaded_gpa.target_average, gpa.target_average);
        assert_eq!(loaded_gpa.lectures.len(), gpa.lectures.len());
    }

    #[test]
    #[serial_test::serial]
    fn load_config_fails_on_write_error() {
        let tmp = tempfile::TempDir::new().unwrap();
        redirect_config_home(tmp.path());

        let cfg = get_config_path();
        let dir = cfg.parent().unwrap();
        fs::create_dir_all(dir).unwrap();
        assert!(!cfg.exists());

        // remove write bits (r-xr-xr-x)
        let mut p = fs::metadata(dir).unwrap().permissions();
        p.set_mode(0o555);
        fs::set_permissions(dir, p).unwrap();

        let err = load_or_create_config().unwrap_err();
        assert!(format!("{err:#}").contains("writing"));
    }

    #[test]
    #[serial_test::serial]
    fn load_config_fails_on_unreadable_file() {
        let tmp = TempDir::new().unwrap();
        redirect_config_home(tmp.path());

        let cfg = get_config_path();
        fs::create_dir_all(cfg.parent().unwrap()).unwrap();
        fs::create_dir(&cfg).unwrap();

        let err = load_or_create_config().unwrap_err();
        assert!(format!("{err:#}").contains("reading"));
    }

    #[test]
    #[serial_test::serial]
    fn load_config_fails_on_malformed_json() {
        let tmp = TempDir::new().unwrap();
        redirect_config_home(tmp.path());

        let cfg = get_config_path();
        fs::create_dir_all(cfg.parent().unwrap()).unwrap();
        fs::write(&cfg, "not valid json").unwrap();

        let err = load_or_create_config().unwrap_err();
        assert!(format!("{err:#}").contains("parsing"));
    }

    #[test]
    #[serial_test::serial]
    fn save_config_updates_existing_file() {
        let tmp = TempDir::new().unwrap();
        redirect_config_home(tmp.path());

        let cfg = get_config_path();
        assert!(!cfg.exists());

        let mut g = GPA::default();
        let loaded = load_or_create_config().unwrap();
        assert!(cfg.exists());
        assert_eq!(g, loaded);

        g.lectures.push(Lecture {
            title: "Calculus".into(),
            credits: 5,
            semester: 2,
            grade: None,
            completed: false,
        });
        g.lectures.push(Lecture {
            title: "Algorithms".into(),
            credits: 5,
            semester: 2,
            grade: None,
            completed: false,
        });

        g.save(&cfg).unwrap();
        assert!(cfg.exists());
        assert_ne!(g, loaded);
    }

    #[test]
    #[serial_test::serial]
    fn save_fails_when_path_is_directory() {
        let tmp = TempDir::new().unwrap();
        let dir_as_file: PathBuf = tmp.path().join("should_be_file");
        fs::create_dir_all(&dir_as_file).unwrap();

        let gpa = GPA::default();
        let err = gpa.save(&dir_as_file).unwrap_err();

        let msg = format!("{err:#}");
        assert!(msg.contains(&format!("writing GPA to {dir_as_file:?}")));
    }
}
