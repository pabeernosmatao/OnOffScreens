use crate::lang;
use crate::model::{AppState, MapFile, Profile, Screen, Settings};
use crate::{autostart, displays, paths, profiles, settings};
use std::collections::HashSet;
use std::sync::Mutex;
use tauri::AppHandle;

static APPLY_LOCK: Mutex<()> = Mutex::new(());

pub fn active_profile_id(
    profiles: &[Profile],
    outputs: &[displays::Output],
    settings: &Settings,
) -> Option<String> {
    let connected: HashSet<&str> = outputs
        .iter()
        .filter(|o| o.connected)
        .map(|o| o.uuid.as_str())
        .collect();
    let enabled: HashSet<&str> = outputs
        .iter()
        .filter(|o| o.connected && o.enabled)
        .map(|o| o.uuid.as_str())
        .collect();

    let matches: Vec<&Profile> = profiles
        .iter()
        .filter(|profile| {
            let target: HashSet<&str> = profile
                .screens
                .iter()
                .map(String::as_str)
                .filter(|uuid| connected.contains(uuid))
                .collect();
            !target.is_empty() && target == enabled
        })
        .collect();

    if let Some(last) = settings.last_profile.as_deref() {
        if let Some(profile) = matches.iter().find(|p| p.id == last) {
            return Some(profile.id.clone());
        }
    }
    matches.first().map(|p| p.id.clone())
}

fn build_screens(outputs: &[displays::Output], map: &MapFile) -> Vec<Screen> {
    let mut screens: Vec<Screen> = outputs
        .iter()
        .filter(|o| o.connected)
        .map(|output| {
            let saved = map.screens.get(&output.uuid);
            let use_saved_geometry = !output.enabled || output.width <= 0 || output.height <= 0;
            let width = if use_saved_geometry {
                saved.map(|s| s.w).unwrap_or(output.width).max(1)
            } else {
                output.width.max(1)
            };
            let height = if use_saved_geometry {
                saved.map(|s| s.h).unwrap_or(output.height).max(1)
            } else {
                output.height.max(1)
            };
            Screen {
                uuid: output.uuid.clone(),
                name: output.name.clone(),
                number: saved.map(|s| s.num),
                connected: output.connected,
                enabled: output.enabled,
                x: if use_saved_geometry {
                    saved.map(|s| s.x).unwrap_or(output.x)
                } else {
                    output.x
                },
                y: if use_saved_geometry {
                    saved.map(|s| s.y).unwrap_or(output.y)
                } else {
                    output.y
                },
                width,
                height,
                scale: output.scale,
                primary: output.primary,
            }
        })
        .collect();
    screens.sort_by_key(|s| (s.number.unwrap_or(u32::MAX), s.name.clone()));
    screens
}

pub fn build(app: &AppHandle) -> AppState {
    let settings = settings::load();
    let language = lang::resolve(&settings.lang);
    let tray_running = app.tray_by_id("main").is_some();
    let mut backend_error: Option<String> = None;
    let mut screens: Vec<Screen> = Vec::new();
    let mut profile_list: Vec<Profile> = profiles::load_file().profiles;
    let mut active = None;

    match displays::read_outputs() {
        Ok(outputs) => match displays::refresh_map(&outputs, false) {
            Ok(map) => {
                match profiles::ensure(&outputs, &map) {
                    Ok(loaded) => profile_list = loaded,
                    Err(err) => {
                        backend_error = Some(err);
                    }
                }
                screens = build_screens(&outputs, &map);
                active = active_profile_id(&profile_list, &outputs, &settings);
            }
            Err(err) => {
                backend_error = Some(err);
            }
        },
        Err(err) => {
            backend_error = Some(err);
        }
    }

    AppState {
        screens,
        profiles: profile_list,
        active_profile_id: active,
        settings,
        autostart: autostart::is_enabled(),
        tray_running,
        lang: language.code().to_string(),
        backend_error,
        version: env!("CARGO_PKG_VERSION").to_string(),
        config_path: paths::profiles_path().to_string_lossy().to_string(),
        map_path: paths::map_path().to_string_lossy().to_string(),
        config_dir: paths::config_dir().to_string_lossy().to_string(),
    }
}

/// Blocking core used by the GUI and the tray. Returns the profile name.
pub fn apply_profile(id: &str) -> Result<String, String> {
    let _guard = APPLY_LOCK.lock().map_err(|e| e.to_string())?;
    let profile = profiles::find(id)
        .ok_or_else(|| lang::strings(lang::current()).profile_not_found.to_string())?;
    let outputs = displays::read_outputs()?;
    displays::apply_targets(&outputs, &profile.screens)?;
    let mut stored = settings::load();
    stored.last_profile = Some(profile.id.clone());
    settings::save(&stored)?;
    Ok(profile.name)
}

/// Used by `--apply` in the CLI, where no AppHandle exists.
pub fn apply_profile_cli(id: &str) -> Result<String, String> {
    apply_profile(id)
}
