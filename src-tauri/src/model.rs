use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

fn default_version() -> u32 {
    1
}

fn tray_default() -> bool {
    true
}

fn lang_default() -> String {
    "auto".to_string()
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub screens: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfilesFile {
    #[serde(default = "default_version")]
    pub version: u32,
    #[serde(default)]
    pub profiles: Vec<Profile>,
}

impl Default for ProfilesFile {
    fn default() -> Self {
        Self {
            version: 1,
            profiles: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileInput {
    pub id: Option<String>,
    pub name: String,
    pub screens: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    #[serde(default = "tray_default")]
    pub tray: bool,
    #[serde(default)]
    pub last_profile: Option<String>,
    /// "auto", "es", "ca" or "en".
    #[serde(default = "lang_default")]
    pub lang: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            tray: true,
            last_profile: None,
            lang: lang_default(),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Screen {
    pub uuid: String,
    pub name: String,
    pub number: Option<u32>,
    pub connected: bool,
    pub enabled: bool,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub scale: f64,
    pub primary: bool,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppState {
    pub screens: Vec<Screen>,
    pub profiles: Vec<Profile>,
    pub active_profile_id: Option<String>,
    pub settings: Settings,
    pub autostart: bool,
    pub tray_running: bool,
    pub lang: String,
    pub backend_error: Option<String>,
    pub version: String,
    pub config_path: String,
    pub map_path: String,
    pub config_dir: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MapScreen {
    pub num: u32,
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MapFile {
    #[serde(default = "default_version")]
    pub version: u32,
    #[serde(default)]
    pub screens: BTreeMap<String, MapScreen>,
}

impl Default for MapFile {
    fn default() -> Self {
        Self {
            version: 1,
            screens: BTreeMap::new(),
        }
    }
}
