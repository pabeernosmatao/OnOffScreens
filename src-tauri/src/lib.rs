mod autostart;
mod cli;
mod commands;
mod displays;
mod fsutil;
mod lang;
mod model;
mod notify;
mod paths;
mod profiles;
mod settings;
mod state;
mod tray;
mod window;

use tauri::Manager;

pub fn run() {
    paths::migrate_old_data();
    autostart::migrate_legacy_autostart();
    let args: Vec<String> = std::env::args().skip(1).collect();
    if cli::handle(&args) {
        return;
    }
    let hidden_launch = args
        .iter()
        .any(|arg| arg == "--autostart" || arg == "--tray");

    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
            window::show(app);
        }))
        .setup(move |app| {
            autostart::refresh_if_enabled();
            let handle = app.handle().clone();
            window::apply_scale(&handle);
            let settings = settings::load();
            let mut tray_ok = false;
            if settings.tray {
                match tray::build(&handle) {
                    Ok(()) => tray_ok = true,
                    Err(err) => eprintln!("No se pudo crear el icono de la bandeja: {err}"),
                }
            }
            if !(hidden_launch && settings.tray && tray_ok) {
                window::show(&handle);
            }
            Ok(())
        })
        .on_menu_event(|app, event| {
            tray::handle_menu_event(app, event.id().as_ref());
        })
        .on_page_load(|webview, _payload| {
            let _ = webview.set_zoom(window::UI_ZOOM);
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                let app = window.app_handle();
                let settings = settings::load();
                if settings.tray && app.tray_by_id("main").is_some() {
                    api.prevent_close();
                    let _ = window.hide();
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_state,
            commands::save_profile,
            commands::delete_profile,
            commands::apply_profile,
            commands::set_option,
            commands::set_language,
            commands::reconfigure_displays,
            commands::reassign_numbers,
            commands::open_display_settings,
            commands::open_config_folder,
        ])
        .run(tauri::generate_context!())
        .expect("error while running OnOffScreens");
}
