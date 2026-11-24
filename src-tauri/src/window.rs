use tauri::{AppHandle, Manager};

/// The UI is designed with generous type and spacing; a slight zoom-out keeps
/// it comfortable on 1080p and 1440p desktops.
pub const UI_ZOOM: f64 = 0.88;

pub fn apply_scale(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.set_zoom(UI_ZOOM);
    }
}

pub fn show(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.set_zoom(UI_ZOOM);
        let _ = window.show();
        let _ = window.unminimize();
        let _ = window.set_focus();
    }
}
