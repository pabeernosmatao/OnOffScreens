use crate::fsutil::write_atomic;
use crate::model::Settings;
use crate::paths;

pub fn load() -> Settings {
    std::fs::read_to_string(paths::settings_path())
        .ok()
        .and_then(|text| serde_json::from_str::<Settings>(&text).ok())
        .unwrap_or_default()
}

pub fn save(settings: &Settings) -> Result<(), String> {
    let text = serde_json::to_string_pretty(settings).map_err(|e| e.to_string())?;
    write_atomic(&paths::settings_path(), &text).map_err(|e| e.to_string())
}
