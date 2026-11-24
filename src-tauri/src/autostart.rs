use crate::fsutil::write_atomic;
use crate::paths;
use std::path::PathBuf;

fn quote_exec(arg: &str) -> String {
    let mut out = String::from("\"");
    for c in arg.chars() {
        match c {
            '\\' | '"' | '`' | '$' => {
                out.push('\\');
                out.push(c);
            }
            '%' => out.push_str("%%"),
            _ => out.push(c),
        }
    }
    out.push('"');
    out
}

fn executable() -> PathBuf {
    // Inside an AppImage, current_exe() points to the temporary mount. The
    // APPIMAGE variable is the real, persistent path and is what autostart
    // must launch.
    if let Some(path) = std::env::var_os("APPIMAGE") {
        let path = PathBuf::from(path);
        if path.is_file() {
            return path;
        }
    }
    std::env::current_exe().unwrap_or_else(|_| PathBuf::from("onoffscreens"))
}

pub fn is_enabled() -> bool {
    paths::autostart_path().is_file()
}

pub fn set(enabled: bool) -> Result<(), String> {
    let path = paths::autostart_path();
    if !enabled {
        if path.exists() {
            std::fs::remove_file(&path).map_err(|e| e.to_string())?;
        }
        return Ok(());
    }

    let exe = executable();
    let exec = format!("{} --autostart", quote_exec(&exe.to_string_lossy()));
    let content = format!(
        "[Desktop Entry]\n\
         Type=Application\n\
         Name=OnOffScreens\n\
         Comment=Perfiles de monitores\n\
         Exec={exec}\n\
         Icon=onoffscreens\n\
         Terminal=false\n\
         NoDisplay=true\n\
         X-GNOME-Autostart-enabled=true\n\
         X-KDE-autostart-after=panel\n\
         X-KDE-autostart-phase=2\n"
    );
    write_atomic(&path, &content).map_err(|e| e.to_string())
}

/// If autostart is enabled, keep the Exec path pointing at the current
/// AppImage/binary (the user may have moved the AppImage).
pub fn refresh_if_enabled() {
    if is_enabled() {
        let _ = set(true);
    }
}

/// Preserve the autostart choice made with any previous name of the app
/// ("pantallas" desktop entries) and remove the obsolete entries.
pub fn migrate_legacy_autostart() {
    let legacy: Vec<PathBuf> = paths::old_autostart_paths()
        .into_iter()
        .filter(|path| path.is_file())
        .collect();
    if legacy.is_empty() {
        return;
    }
    if !is_enabled() && std::env::var_os("APPIMAGE").is_some() {
        let _ = set(true);
    }
    for path in legacy {
        let _ = std::fs::remove_file(path);
    }
}
