use crate::lang;
use crate::{displays, notify, profiles, settings, state, window};
use tauri::image::Image;
use tauri::menu::{CheckMenuItem, Menu, MenuItem, PredefinedMenuItem};
use tauri::tray::TrayIconBuilder;
use tauri::{AppHandle, Emitter};

/// Build the tray menu from local files only. Display state is intentionally
/// not queried here: while the app lives in the tray we must not touch KScreen
/// until the user opens the window or runs an action.
pub fn build(app: &AppHandle) -> Result<(), String> {
    let strings = lang::strings(lang::current());
    let file = profiles::load_file();
    let stored = settings::load();
    let active = stored.last_profile.as_deref();
    let menu = Menu::new(app).map_err(|e| e.to_string())?;

    for profile in &file.profiles {
        let id = format!("profile:{}", profile.id);
        let checked = active == Some(profile.id.as_str());
        let item = CheckMenuItem::with_id(app, id, &profile.name, true, checked, None::<&str>)
            .map_err(|e| e.to_string())?;
        menu.append(&item).map_err(|e| e.to_string())?;
    }

    if file.profiles.is_empty() {
        let item = MenuItem::with_id(app, "new", strings.new_profile, true, None::<&str>)
            .map_err(|e| e.to_string())?;
        menu.append(&item).map_err(|e| e.to_string())?;
    }

    menu.append(&PredefinedMenuItem::separator(app).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    menu.append(
        &MenuItem::with_id(app, "open", strings.open_app, true, None::<&str>)
            .map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    menu.append(
        &MenuItem::with_id(app, "screens", strings.show_screens, true, None::<&str>)
            .map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    menu.append(
        &MenuItem::with_id(
            app,
            "reconfigure",
            strings.reconfigure,
            true,
            None::<&str>,
        )
        .map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    menu.append(
        &MenuItem::with_id(app, "kde", strings.kde_settings, true, None::<&str>)
            .map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    menu.append(&PredefinedMenuItem::separator(app).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    menu.append(
        &MenuItem::with_id(app, "quit", strings.quit, true, None::<&str>)
            .map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;

    let tip = strings.tooltip.to_string();

    if let Some(tray) = app.tray_by_id("main") {
        tray.set_menu(Some(menu)).map_err(|e| e.to_string())?;
        tray.set_tooltip(Some(tip)).map_err(|e| e.to_string())?;
        return Ok(());
    }

    let icon =
        Image::from_bytes(include_bytes!("../icons/64x64.png")).map_err(|e| e.to_string())?;
    TrayIconBuilder::with_id("main")
        .icon(icon)
        .title(strings.tooltip)
        .tooltip(tip)
        .menu(&menu)
        .show_menu_on_left_click(true)
        .build(app)
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn remove(app: &AppHandle) {
    let _ = app.remove_tray_by_id("main");
}

/// Rebuild the menu from a worker thread, running the UI work on the main loop.
pub fn refresh(app: &AppHandle) {
    if !crate::settings::load().tray {
        return;
    }
    let (tx, rx) = std::sync::mpsc::channel();
    let app2 = app.clone();
    if app
        .run_on_main_thread(move || {
            let _ = tx.send(build(&app2));
        })
        .is_ok()
    {
        let _ = rx.recv();
    }
}

pub fn build_threadsafe(app: &AppHandle) -> Result<(), String> {
    let (tx, rx) = std::sync::mpsc::channel();
    let app2 = app.clone();
    app.run_on_main_thread(move || {
        let _ = tx.send(build(&app2));
    })
    .map_err(|e| e.to_string())?;
    rx.recv().map_err(|e| e.to_string())?
}

pub fn remove_threadsafe(app: &AppHandle) -> Result<(), String> {
    let (tx, rx) = std::sync::mpsc::channel();
    let app2 = app.clone();
    app.run_on_main_thread(move || {
        remove(&app2);
        let _ = tx.send(());
    })
    .map_err(|e| e.to_string())?;
    rx.recv().map_err(|e| e.to_string())
}

fn apply_from_tray(app: &AppHandle, id: &str) {
    let app = app.clone();
    let id = id.to_string();
    std::thread::spawn(move || {
        match state::apply_profile(&id) {
            Ok(name) => notify::profile_applied(&name),
            Err(err) => notify::profile_failed(&err),
        }
        refresh(&app);
        let _ = app.emit("state-changed", ());
    });
}

fn reconfigure_from_tray(app: &AppHandle) {
    let app = app.clone();
    std::thread::spawn(move || {
        let result = (|| -> Result<(), String> {
            let outputs = displays::read_outputs()?;
            displays::force_reconfigure(&outputs)
        })();
        match result {
            Ok(()) => notify::reconfigured(),
            Err(err) => notify::reconfigure_failed(&err),
        }
        refresh(&app);
        let _ = app.emit("state-changed", ());
    });
}

pub fn handle_menu_event(app: &AppHandle, id: &str) {
    match id {
        "open" => window::show(app),
        "new" => {
            window::show(app);
            let _ = app.emit("open-new-profile", ());
        }
        "screens" => {
            window::show(app);
            let _ = app.emit("open-screens", ());
        }
        "reconfigure" => reconfigure_from_tray(app),
        "kde" => {
            let _ = crate::commands::open_display_settings_inner();
        }
        "quit" => app.exit(0),
        _ if id.starts_with("profile:") => apply_from_tray(app, &id["profile:".len()..]),
        _ => {}
    }
}
