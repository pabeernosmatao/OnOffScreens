use crate::displays::Output;
use crate::fsutil::write_atomic;
use crate::lang::{self, Lang};
use crate::model::{MapFile, Profile, ProfileInput, ProfilesFile};
use crate::paths;
use std::collections::HashSet;

pub fn load_file() -> ProfilesFile {
    std::fs::read_to_string(paths::profiles_path())
        .ok()
        .and_then(|text| serde_json::from_str::<ProfilesFile>(&text).ok())
        .unwrap_or_default()
}

fn save_file(file: &ProfilesFile) -> Result<(), String> {
    let text = serde_json::to_string_pretty(file).map_err(|e| e.to_string())?;
    write_atomic(&paths::profiles_path(), &text).map_err(|e| e.to_string())
}

pub fn find(id: &str) -> Option<Profile> {
    load_file().profiles.into_iter().find(|p| p.id == id)
}

fn default_profiles(outputs: &[Output], map: &MapFile) -> Vec<Profile> {
    let lang = lang::current();
    let mut profiles = Vec::new();
    let connected: Vec<&Output> = outputs.iter().filter(|o| o.connected).collect();
    if connected.is_empty() {
        return profiles;
    }

    let all_screens: Vec<String> = connected.iter().map(|o| o.uuid.clone()).collect();
    profiles.push(Profile {
        id: uuid::Uuid::new_v4().to_string(),
        name: match lang {
            Lang::Es => "Todas".to_string(),
            Lang::Ca => "Totes".to_string(),
            Lang::En => "All".to_string(),
        },
        screens: all_screens,
    });

    for output in &connected {
        let number = map
            .screens
            .get(&output.uuid)
            .map(|s| s.num.to_string())
            .unwrap_or_else(|| "?".to_string());
        let name = match lang {
            Lang::Es => format!("Solo {} ({})", number, output.name),
            Lang::Ca => format!("Només {} ({})", number, output.name),
            Lang::En => format!("Only {} ({})", number, output.name),
        };
        profiles.push(Profile {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            screens: vec![output.uuid.clone()],
        });
    }
    profiles
}

/// Parse the old `perfiles.conf` so existing users keep their profiles.
fn migrate_legacy(outputs: &[Output], map: &MapFile) -> Vec<Profile> {
    let Ok(text) = std::fs::read_to_string(paths::legacy_profiles_path()) else {
        return Vec::new();
    };
    let mut profiles = Vec::new();
    for raw in text.lines() {
        let line = raw.split('#').next().unwrap_or("").trim();
        if line.is_empty() || !line.contains('=') {
            continue;
        }
        let (name, value) = line.split_once('=').unwrap();
        let name = name.trim();
        if name.is_empty() {
            continue;
        }
        let mut screens: Vec<String> = Vec::new();
        for item in value.replace(';', ",").split(',') {
            let item = item.trim();
            if item.is_empty() {
                continue;
            }
            let uuid = if let Ok(number) = item.parse::<u32>() {
                map.screens
                    .iter()
                    .find(|(_, screen)| screen.num == number)
                    .map(|(uuid, _)| uuid.clone())
            } else {
                map.screens
                    .iter()
                    .find(|(_, screen)| screen.name == item)
                    .map(|(uuid, _)| uuid.clone())
                    .or_else(|| {
                        outputs
                            .iter()
                            .find(|o| o.name == item)
                            .map(|o| o.uuid.clone())
                    })
                    .or_else(|| map.screens.contains_key(item).then(|| item.to_string()))
            };
            if let Some(uuid) = uuid {
                if !screens.contains(&uuid) {
                    screens.push(uuid);
                }
            }
        }
        if !screens.is_empty() {
            profiles.push(Profile {
                id: uuid::Uuid::new_v4().to_string(),
                name: name.to_string(),
                screens,
            });
        }
    }
    profiles
}

/// Load profiles, creating the JSON file on first run. If an old
/// `perfiles.conf` exists its profiles are imported once.
pub fn ensure(outputs: &[Output], map: &MapFile) -> Result<Vec<Profile>, String> {
    if paths::profiles_path().exists() {
        return Ok(load_file().profiles);
    }

    let mut profiles = migrate_legacy(outputs, map);
    if profiles.is_empty() {
        profiles = default_profiles(outputs, map);
    }
    // Do not persist an empty list: if there are no connected screens yet we
    // want to retry the migration/default generation on a later launch.
    if profiles.is_empty() {
        return Ok(profiles);
    }
    let file = ProfilesFile {
        version: 1,
        profiles: profiles.clone(),
    };
    save_file(&file)?;
    Ok(profiles)
}

pub fn save_profile(input: ProfileInput) -> Result<Profile, String> {
    let lang = lang::current();
    let strings = lang::strings(lang);
    let name = input.name.trim();
    if name.is_empty() {
        return Err(strings.empty_name.to_string());
    }
    if input.screens.is_empty() {
        return Err(strings.no_screens.to_string());
    }

    let mut file = load_file();
    let mut screens: Vec<String> = Vec::new();
    for uuid in input.screens {
        if !screens.contains(&uuid) {
            screens.push(uuid);
        }
    }

    let id = input.id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
    if let Some(existing) = file.profiles.iter_mut().find(|p| p.id == id) {
        existing.name = name.to_string();
        existing.screens = screens;
    } else {
        if file
            .profiles
            .iter()
            .any(|p| p.name.eq_ignore_ascii_case(name))
        {
            return Err(strings.duplicate_name.to_string());
        }
        file.profiles.push(Profile {
            id: id.clone(),
            name: name.to_string(),
            screens,
        });
    }

    // An edit must not create a duplicate either.
    let mut seen: HashSet<String> = HashSet::new();
    for profile in &file.profiles {
        if !seen.insert(profile.name.to_lowercase()) && profile.id != id {
            return Err(strings.duplicate_name.to_string());
        }
    }

    save_file(&file)?;
    Ok(file
        .profiles
        .into_iter()
        .find(|p| p.id == id)
        .expect("profile just saved"))
}

pub fn delete_profile(id: &str) -> Result<(), String> {
    let mut file = load_file();
    let before = file.profiles.len();
    file.profiles.retain(|p| p.id != id);
    if file.profiles.len() == before {
        return Err(lang::strings(lang::current()).profile_not_found.to_string());
    }
    save_file(&file)
}
