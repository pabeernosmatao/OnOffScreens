use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub fn config_dir() -> PathBuf {
    dirs::config_dir()
        .or_else(|| dirs::home_dir().map(|h| h.join(".config")))
        .unwrap_or_else(|| PathBuf::from("."))
        .join("onoffscreens")
}

pub fn state_dir() -> PathBuf {
    dirs::state_dir()
        .or_else(|| dirs::home_dir().map(|h| h.join(".local/state")))
        .unwrap_or_else(|| PathBuf::from("."))
        .join("onoffscreens")
}

pub fn profiles_path() -> PathBuf {
    config_dir().join("perfiles.json")
}

pub fn legacy_profiles_path() -> PathBuf {
    config_dir().join("perfiles.conf")
}

pub fn settings_path() -> PathBuf {
    config_dir().join("config.json")
}

pub fn map_path() -> PathBuf {
    state_dir().join("mapa.json")
}

pub fn autostart_path() -> PathBuf {
    autostart_dir().join("onoffscreens.desktop")
}

fn autostart_dir() -> PathBuf {
    dirs::config_dir()
        .or_else(|| dirs::home_dir().map(|h| h.join(".config")))
        .unwrap_or_else(|| PathBuf::from("."))
        .join("autostart")
}

/// Previous name of the application. Used only to migrate existing data.
pub fn old_config_dir() -> PathBuf {
    dirs::config_dir()
        .or_else(|| dirs::home_dir().map(|h| h.join(".config")))
        .unwrap_or_else(|| PathBuf::from("."))
        .join("pantallas")
}

pub fn old_state_dir() -> PathBuf {
    dirs::state_dir()
        .or_else(|| dirs::home_dir().map(|h| h.join(".local/state")))
        .unwrap_or_else(|| PathBuf::from("."))
        .join("pantallas")
}

pub fn old_autostart_paths() -> Vec<PathBuf> {
    vec![
        autostart_dir().join("pantallas.desktop"),
        autostart_dir().join("pantallas-tray.desktop"),
    ]
}

fn copy_dir_all(source: &Path, target: &Path) -> io::Result<()> {
    fs::create_dir_all(target)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let source_path = entry.path();
        let target_path = target.join(entry.file_name());
        if source_path.is_dir() {
            copy_dir_all(&source_path, &target_path)?;
        } else {
            fs::copy(&source_path, &target_path)?;
        }
    }
    Ok(())
}

/// One-time migration from the old "pantallas" data folders. User files are
/// copied, never deleted, so downgrading remains possible.
pub fn migrate_old_data() {
    for (old, new) in [
        (old_config_dir(), config_dir()),
        (old_state_dir(), state_dir()),
    ] {
        if new.exists() || !old.is_dir() {
            continue;
        }
        let _ = copy_dir_all(&old, &new);
    }
}
