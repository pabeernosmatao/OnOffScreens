use crate::model::{AppState, ProfileInput};
use crate::{autostart, displays, lang, notify, profiles, settings, state, tray};
use tauri::AppHandle;

async fn blocking<T, F>(f: F) -> Result<T, String>
where
    T: Send + 'static,
    F: FnOnce() -> Result<T, String> + Send + 'static,
{
    tauri::async_runtime::spawn_blocking(f)
        .await
        .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn get_state(app: AppHandle) -> Result<AppState, String> {
    let app2 = app.clone();
    blocking(move || Ok(state::build(&app2))).await
}

#[tauri::command]
pub async fn save_profile(app: AppHandle, profile: ProfileInput) -> Result<AppState, String> {
    blocking(move || profiles::save_profile(profile)).await?;
    tray::refresh(&app);
    Ok(state::build(&app))
}

#[tauri::command]
pub async fn delete_profile(app: AppHandle, id: String) -> Result<AppState, String> {
    let id2 = id.clone();
    blocking(move || profiles::delete_profile(&id2)).await?;
    let mut stored = settings::load();
    if stored.last_profile.as_deref() == Some(id.as_str()) {
        stored.last_profile = None;
        settings::save(&stored)?;
    }
    tray::refresh(&app);
    Ok(state::build(&app))
}

#[tauri::command]
pub async fn apply_profile(app: AppHandle, id: String) -> Result<AppState, String> {
    let id2 = id.clone();
    let result = blocking(move || {
        let name = state::apply_profile(&id2)?;
        notify::profile_applied(&name);
        Ok(())
    })
    .await;

    if let Err(err) = result {
        notify::profile_failed(&err);
        return Err(err);
    }
    tray::refresh(&app);
    Ok(state::build(&app))
}

#[tauri::command]
pub async fn set_option(app: AppHandle, key: String, value: bool) -> Result<AppState, String> {
    match key.as_str() {
        "autostart" => {
            let value2 = value;
            blocking(move || autostart::set(value2)).await?;
        }
        "tray" => {
            if value {
                tray::build_threadsafe(&app)?;
            } else {
                tray::remove_threadsafe(&app)?;
            }
            let mut stored = settings::load();
            stored.tray = value;
            settings::save(&stored)?;
        }
        _ => return Err(format!("Unknown option: {key}")),
    }
    Ok(state::build(&app))
}

#[tauri::command]
pub async fn set_language(app: AppHandle, lang: String) -> Result<AppState, String> {
    let value = match lang.as_str() {
        "auto" | "es" | "ca" | "en" => lang,
        _ => return Err(format!("Unknown language: {lang}")),
    };
    let mut stored = settings::load();
    stored.lang = value;
    settings::save(&stored)?;
    tray::refresh(&app);
    Ok(state::build(&app))
}

#[tauri::command]
pub async fn reconfigure_displays(app: AppHandle) -> Result<AppState, String> {
    blocking(move || {
        let outputs = displays::read_outputs()?;
        displays::force_reconfigure(&outputs)
    })
    .await?;
    Ok(state::build(&app))
}

#[tauri::command]
pub async fn reassign_numbers(app: AppHandle) -> Result<AppState, String> {
    blocking(move || {
        let outputs = displays::read_outputs()?;
        displays::renumber(&outputs)?;
        Ok(())
    })
    .await?;
    tray::refresh(&app);
    Ok(state::build(&app))
}

pub fn open_display_settings_inner() -> Result<(), String> {
    for command in [
        "kcmshell6",
        "kcmshell5",
        "systemsettings6",
        "systemsettings",
    ] {
        if std::process::Command::new(command)
            .arg("kcm_kscreen")
            .spawn()
            .is_ok()
        {
            return Ok(());
        }
    }
    Err(lang::strings(lang::current()).kde_settings_failed.to_string())
}

#[tauri::command]
pub async fn open_display_settings() -> Result<(), String> {
    open_display_settings_inner()
}

#[tauri::command]
pub async fn open_config_folder() -> Result<(), String> {
    std::process::Command::new("xdg-open")
        .arg(crate::paths::config_dir())
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok(())
}
