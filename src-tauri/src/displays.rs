use crate::fsutil::write_atomic;
use crate::lang;
use crate::model::{MapFile, MapScreen};
use crate::paths;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::process::Command;
use std::sync::{Mutex, MutexGuard};
use std::thread::sleep;
use std::time::Duration;

/// KScreen/KWin are sensitive to concurrent reconfiguration requests. Serialize
/// every kscreen-doctor invocation made by the app so reads never overlap with
/// an apply (or with another read).
static KSCREEN_LOCK: Mutex<()> = Mutex::new(());

fn kscreen_lock() -> MutexGuard<'static, ()> {
    KSCREEN_LOCK.lock().unwrap_or_else(|err| err.into_inner())
}

#[derive(Clone, Debug)]
pub struct Output {
    pub uuid: String,
    pub name: String,
    pub connected: bool,
    pub enabled: bool,
    pub priority: i64,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub scale: f64,
    pub primary: bool,
}

fn strip_ansi(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\x1b' && chars.peek() == Some(&'[') {
            chars.next();
            for next in chars.by_ref() {
                if next.is_ascii_alphabetic() {
                    break;
                }
            }
            continue;
        }
        out.push(c);
    }
    out
}

fn output_uuids() -> HashMap<String, String> {
    let Ok(result) = Command::new("kscreen-doctor").arg("-o").output() else {
        return HashMap::new();
    };
    let text = strip_ansi(&String::from_utf8_lossy(&result.stdout));
    let mut map = HashMap::new();
    for line in text.lines() {
        let line = line.trim();
        if !line.starts_with("Output:") {
            continue;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 4 {
            continue;
        }
        let candidate = parts[3].trim_matches(|c| c == '(' || c == ')');
        if candidate.len() >= 8 && candidate.chars().all(|c| c.is_ascii_hexdigit() || c == '-') {
            map.insert(parts[2].to_string(), candidate.to_string());
        }
    }
    map
}

pub fn read_outputs() -> Result<Vec<Output>, String> {
    #[cfg(debug_assertions)]
    if std::env::var_os("ONOFFSCREENS_MOCK").is_some() {
        return Ok(mock_outputs());
    }

    let lang = lang::current();
    let strings = lang::strings(lang);
    let _guard = kscreen_lock();
    let result = Command::new("kscreen-doctor").arg("-j").output();
    let output = match result {
        Ok(output) => output,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            return Err(strings.no_kscreen.to_string())
        }
        Err(err) => return Err(format!("{}: {err}", strings.read_error)),
    };
    if !output.status.success() {
        let detail = String::from_utf8_lossy(&output.stderr);
        return Err(format!("{}: {}", strings.read_error, detail.trim()));
    }
    let json: serde_json::Value = serde_json::from_slice(&output.stdout)
        .map_err(|e| format!("{}: {e}", strings.read_error))?;
    let uuids = output_uuids();

    let mut result = Vec::new();
    let Some(items) = json.get("outputs").and_then(|v| v.as_array()) else {
        return Ok(result);
    };
    for item in items {
        let name = item
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string();
        if name.is_empty() || name.starts_with("Writeback") {
            continue;
        }
        let connected = item
            .get("connected")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        let enabled = item
            .get("enabled")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let priority = item.get("priority").and_then(|v| v.as_i64()).unwrap_or(99);
        let x = item
            .get("pos")
            .and_then(|v| v.get("x"))
            .and_then(|v| v.as_i64())
            .unwrap_or(0) as i32;
        let y = item
            .get("pos")
            .and_then(|v| v.get("y"))
            .and_then(|v| v.as_i64())
            .unwrap_or(0) as i32;
        let native_w = item
            .get("size")
            .and_then(|v| v.get("width"))
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);
        let native_h = item
            .get("size")
            .and_then(|v| v.get("height"))
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);
        let scale = item
            .get("scale")
            .and_then(|v| v.as_f64())
            .filter(|s| *s > 0.0)
            .unwrap_or(1.0);
        let width = (native_w / scale).round().max(0.0) as i32;
        let height = (native_h / scale).round().max(0.0) as i32;

        let uuid = uuids
            .get(&name)
            .cloned()
            .or_else(|| {
                item.get("uuid")
                    .or_else(|| item.get("edidHash"))
                    .and_then(|v| v.as_str())
                    .filter(|s| !s.is_empty())
                    .map(str::to_string)
            })
            .unwrap_or_else(|| format!("name:{name}"));

        result.push(Output {
            uuid,
            name,
            connected,
            enabled,
            priority,
            x,
            y,
            width,
            height,
            scale,
            primary: priority == 1,
        });
    }
    Ok(result)
}

fn order_by_position<'a>(outputs: &[&'a Output]) -> Vec<&'a Output> {
    let mut rows: BTreeMap<i32, Vec<&Output>> = BTreeMap::new();
    for output in outputs {
        rows.entry(output.y).or_default().push(output);
    }
    if rows.is_empty() {
        return Vec::new();
    }

    // Main row: the one with most screens, then the widest, then the lowest
    // priority value. Mirrors the original Python algorithm.
    let main_y = rows
        .iter()
        .max_by_key(|(_, row)| {
            let width: i32 = row.iter().map(|o| o.width.max(1)).sum();
            let priority = row.iter().map(|o| o.priority).min().unwrap_or(99);
            (row.len(), width, -priority)
        })
        .map(|(y, _)| *y)
        .unwrap_or(0);

    let mut ordered = Vec::new();
    let mut main_row = rows.get(&main_y).cloned().unwrap_or_default();
    main_row.sort_by_key(|o| (o.x, o.name.clone()));
    ordered.extend(main_row);
    for (y, row) in rows.iter_mut() {
        if *y == main_y {
            continue;
        }
        row.sort_by_key(|o| (o.x, o.name.clone()));
        ordered.extend(row.iter().copied());
    }
    ordered
}

pub fn load_map() -> MapFile {
    std::fs::read_to_string(paths::map_path())
        .ok()
        .and_then(|text| serde_json::from_str::<MapFile>(&text).ok())
        .unwrap_or_default()
}

fn save_map(map: &MapFile) -> Result<(), String> {
    let text = serde_json::to_string_pretty(map).map_err(|e| e.to_string())?;
    write_atomic(&paths::map_path(), &text).map_err(|e| e.to_string())
}

/// Refresh and persist the stable number <-> EDID UUID map. Existing screens
/// keep their number and their original geometry (used to pack profiles).
pub fn refresh_map(outputs: &[Output], rebuild: bool) -> Result<MapFile, String> {
    let mut map = if rebuild {
        MapFile::default()
    } else {
        load_map()
    };
    let before = serde_json::to_string(&map.screens).unwrap_or_default();

    let connected: Vec<&Output> = outputs
        .iter()
        .filter(|o| o.connected && !o.name.is_empty())
        .collect();

    for output in &connected {
        if let Some(entry) = map.screens.get_mut(&output.uuid) {
            if entry.name != output.name {
                entry.name = output.name.clone();
            }
        }
    }

    let fresh: Vec<&Output> = connected
        .iter()
        .copied()
        .filter(|o| !map.screens.contains_key(&o.uuid))
        .collect();

    if !fresh.is_empty() {
        let mut used: HashSet<u32> = map.screens.values().map(|s| s.num).collect();
        let mut free = 1u32;
        for output in order_by_position(&fresh) {
            while used.contains(&free) {
                free += 1;
            }
            map.screens.insert(
                output.uuid.clone(),
                MapScreen {
                    num: free,
                    name: output.name.clone(),
                    x: output.x,
                    y: output.y,
                    w: output.width.max(1),
                    h: output.height.max(1),
                },
            );
            used.insert(free);
        }
    }

    let after = serde_json::to_string(&map.screens).unwrap_or_default();
    if before != after || rebuild {
        save_map(&map)?;
    }
    Ok(map)
}

fn plan_layout(targets: &[&Output], map: &MapFile) -> HashMap<String, (i32, i32)> {
    let mut rows: BTreeMap<i32, Vec<(i32, i32, i32, String)>> = BTreeMap::new();
    for output in targets {
        let (x, y, width, height) = map
            .screens
            .get(&output.uuid)
            .map(|s| (s.x, s.y, s.w.max(1), s.h.max(1)))
            .unwrap_or((
                output.x,
                output.y,
                output.width.max(1),
                output.height.max(1),
            ));
        rows.entry(y)
            .or_default()
            .push((x, width, height, output.name.clone()));
    }

    let widths: Vec<i32> = rows
        .values()
        .map(|row| row.iter().map(|(_, w, _, _)| *w).sum())
        .collect();
    let max_width = widths.iter().copied().max().unwrap_or(0);

    let mut positions = HashMap::new();
    let mut cursor_y = 0;
    for row in rows.values_mut() {
        row.sort_by(|a, b| (a.0, &a.3).cmp(&(b.0, &b.3)));
        let row_width: i32 = row.iter().map(|(_, w, _, _)| *w).sum();
        let row_height = row.iter().map(|(_, _, h, _)| *h).max().unwrap_or(1);
        let mut cursor_x = (max_width - row_width) / 2;
        for (_, width, _, name) in row.iter() {
            positions.insert(name.clone(), (cursor_x, cursor_y));
            cursor_x += width;
        }
        cursor_y += row_height;
    }
    positions
}

/// Apply a set of screen UUIDs. Everything not selected is turned off and the
/// selected screens are re-packed without gaps, preserving rows.
pub fn apply_targets(all_outputs: &[Output], uuids: &[String]) -> Result<(), String> {
    let selected: HashSet<&str> = uuids.iter().map(String::as_str).collect();
    let connected: Vec<&Output> = all_outputs
        .iter()
        .filter(|o| o.connected && !o.name.is_empty())
        .collect();
    let targets: Vec<&Output> = connected
        .iter()
        .copied()
        .filter(|o| selected.contains(o.uuid.as_str()))
        .collect();

    if targets.is_empty() {
        return Err(lang::strings(lang::current()).empty_profile.to_string());
    }

    let map = load_map();
    let positions = plan_layout(&targets, &map);
    let args = plan_commands(&connected, &targets, &positions, &selected);

    // If every screen is already in the requested state and position, do not
    // call kscreen-doctor again. Re-enabling outputs that are already on forces
    // KWin to renegotiate modes/HDR and can freeze the session on some setups.
    if args.is_empty() {
        eprintln!("onoffscreens: displays already match; skipping KWin reconfiguration");
        return Ok(());
    }

    eprintln!("onoffscreens: kscreen-doctor {}", args.join(" "));
    let result = {
        let _guard = kscreen_lock();
        Command::new("kscreen-doctor").args(&args).output()
    }
    .map_err(|e| e.to_string())?;
    if !result.status.success() {
        return Err(format!(
            "kscreen-doctor: {}{}",
            String::from_utf8_lossy(&result.stdout).trim(),
            String::from_utf8_lossy(&result.stderr).trim()
        ));
    }

    // Give KWin a moment to apply the configuration, then verify it with a
    // light read (`kscreen-doctor -j` only). The short, bounded loop avoids
    // hammering KScreen while it is reconfiguring outputs.
    let target_names: HashSet<String> = targets.iter().map(|o| o.name.clone()).collect();
    sleep(Duration::from_millis(350));
    let mut matched = false;
    for _ in 0..12 {
        match enabled_output_names() {
            Some(current) if current == target_names => {
                matched = true;
                break;
            }
            _ => sleep(Duration::from_millis(250)),
        }
    }
    if !matched {
        eprintln!("onoffscreens: outputs did not reach the requested state after apply");
    }
    Ok(())
}

/// Build the minimal set of kscreen-doctor operations needed to reach the
/// requested profile. Screens that are already on and correctly positioned are
/// left untouched.
fn plan_commands(
    connected: &[&Output],
    targets: &[&Output],
    positions: &HashMap<String, (i32, i32)>,
    selected: &HashSet<&str>,
) -> Vec<String> {
    let mut args = Vec::new();
    for output in connected {
        if !selected.contains(output.uuid.as_str()) && output.enabled {
            args.push(format!("output.{}.disable", output.name));
        }
    }
    for output in targets {
        let Some(&(x, y)) = positions.get(&output.name) else {
            continue;
        };
        if !output.enabled {
            args.push(format!("output.{}.position.{},{}", output.name, x, y));
            args.push(format!("output.{}.enable", output.name));
        } else if output.x != x || output.y != y {
            args.push(format!("output.{}.position.{},{}", output.name, x, y));
        }
    }
    args
}

/// Lightweight enabled-output check used after an apply. It avoids the extra
/// UUID lookup (`kscreen-doctor -o`) that `read_outputs` performs.
fn enabled_output_names() -> Option<HashSet<String>> {
    let _guard = kscreen_lock();
    let output = Command::new("kscreen-doctor").arg("-j").output().ok()?;
    if !output.status.success() {
        return None;
    }
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).ok()?;
    let mut names = HashSet::new();
    for item in json.get("outputs")?.as_array()? {
        let name = item.get("name").and_then(|v| v.as_str()).unwrap_or_default();
        if name.is_empty() || name.starts_with("Writeback") {
            continue;
        }
        let connected = item
            .get("connected")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        let enabled = item
            .get("enabled")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        if connected && enabled {
            names.insert(name.to_string());
        }
    }
    Some(names)
}

/// Force KWin to re-apply the current enabled outputs. This is a recovery
/// action for wrong colors or a glitched picture after login: it re-sends the
/// current positions and enables the outputs that are already on, so KDE
/// rebuilds the output configuration. Disabled screens stay off.
pub fn force_reconfigure(outputs: &[Output]) -> Result<(), String> {
    let enabled: Vec<&Output> = outputs
        .iter()
        .filter(|o| o.connected && o.enabled && !o.name.is_empty())
        .collect();
    if enabled.is_empty() {
        return Err(lang::strings(lang::current()).no_screens_on.to_string());
    }

    let mut args = Vec::new();
    for output in &enabled {
        args.push(format!(
            "output.{}.position.{},{}",
            output.name, output.x, output.y
        ));
        args.push(format!("output.{}.enable", output.name));
    }

    eprintln!("onoffscreens: kscreen-doctor {}", args.join(" "));
    let result = {
        let _guard = kscreen_lock();
        Command::new("kscreen-doctor").args(&args).output()
    }
    .map_err(|e| e.to_string())?;
    if !result.status.success() {
        return Err(format!(
            "kscreen-doctor: {}{}",
            String::from_utf8_lossy(&result.stdout).trim(),
            String::from_utf8_lossy(&result.stderr).trim()
        ));
    }
    Ok(())
}

pub fn renumber(outputs: &[Output]) -> Result<MapFile, String> {
    refresh_map(outputs, true)
}

/// Development-only fake setup for working on the UI without KDE.
#[cfg(debug_assertions)]
fn mock_outputs() -> Vec<Output> {
    vec![
        Output {
            uuid: "1a2b3c4d5e6f708192a3b4c5d6e7f801".to_string(),
            name: "DP-1".to_string(),
            connected: true,
            enabled: true,
            priority: 1,
            x: 0,
            y: 0,
            width: 2560,
            height: 1440,
            scale: 1.0,
            primary: true,
        },
        Output {
            uuid: "1a2b3c4d5e6f708192a3b4c5d6e7f802".to_string(),
            name: "DP-2".to_string(),
            connected: true,
            enabled: true,
            priority: 2,
            x: 2560,
            y: 0,
            width: 1920,
            height: 1080,
            scale: 1.0,
            primary: false,
        },
        Output {
            uuid: "1a2b3c4d5e6f708192a3b4c5d6e7f803".to_string(),
            name: "DP-3".to_string(),
            connected: true,
            enabled: false,
            priority: 3,
            x: 2560,
            y: 1080,
            width: 1920,
            height: 1080,
            scale: 1.0,
            primary: false,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn output(name: &str, uuid: &str, x: i32, y: i32, width: i32, height: i32) -> Output {
        Output {
            uuid: uuid.to_string(),
            name: name.to_string(),
            connected: true,
            enabled: true,
            priority: 99,
            x,
            y,
            width,
            height,
            scale: 1.0,
            primary: false,
        }
    }

    #[test]
    fn main_row_is_numbered_first() {
        let a = output("A", "a", 0, 0, 1920, 1080);
        let b = output("B", "b", 0, 1080, 1920, 1080);
        let c = output("C", "c", 1920, 1080, 1920, 1080);
        let d = output("D", "d", 3840, 1080, 1920, 1080);
        let ordered = order_by_position(&[&a, &b, &c, &d]);
        let names: Vec<&str> = ordered.iter().map(|o| o.name.as_str()).collect();
        assert_eq!(names, ["B", "C", "D", "A"]);
    }

    #[test]
    fn plan_keeps_rows_and_removes_gaps() {
        let a = output("A", "a", 0, 0, 1920, 1080);
        let b = output("B", "b", 1920, 0, 1920, 1080);
        let c = output("C", "c", 960, 1080, 1920, 1080);

        let mut map = MapFile::default();
        for o in [&a, &b, &c] {
            map.screens.insert(
                o.uuid.clone(),
                MapScreen {
                    num: 1,
                    name: o.name.clone(),
                    x: o.x,
                    y: o.y,
                    w: o.width,
                    h: o.height,
                },
            );
        }

        let all = plan_layout(&[&a, &b, &c], &map);
        assert_eq!(all["A"], (0, 0));
        assert_eq!(all["B"], (1920, 0));
        assert_eq!(all["C"], (960, 1080));

        let subset = plan_layout(&[&b, &c], &map);
        assert_eq!(subset["B"], (0, 0));
        assert_eq!(subset["C"], (0, 1080));
    }

    #[test]
    fn apply_commands_skip_screens_already_in_place() {
        let a = output("A", "a", 0, 0, 1920, 1080);
        let b = output("B", "b", 1920, 0, 1920, 1080);
        let selected: HashSet<&str> = ["a"].into_iter().collect();
        let mut positions = HashMap::new();
        positions.insert("A".to_string(), (0, 0));
        positions.insert("B".to_string(), (0, 0));

        let args = plan_commands(&[&a, &b], &[&a], &positions, &selected);
        assert_eq!(args, vec!["output.B.disable"]);
    }

    #[test]
    fn apply_commands_only_enable_disabled_screens() {
        let mut a = output("A", "a", 960, 0, 1920, 1080);
        let mut b = output("B", "b", 0, 0, 1920, 1080);
        a.enabled = true;
        b.enabled = false;
        let selected: HashSet<&str> = ["a", "b"].into_iter().collect();
        let mut positions = HashMap::new();
        positions.insert("A".to_string(), (0, 0));
        positions.insert("B".to_string(), (1920, 0));

        let args = plan_commands(&[&a, &b], &[&a, &b], &positions, &selected);
        assert_eq!(
            args,
            vec!["output.A.position.0,0", "output.B.position.1920,0", "output.B.enable"]
        );
    }

    #[test]
    fn apply_commands_are_empty_when_nothing_changes() {
        let a = output("A", "a", 0, 0, 1920, 1080);
        let selected: HashSet<&str> = ["a"].into_iter().collect();
        let mut positions = HashMap::new();
        positions.insert("A".to_string(), (0, 0));

        assert!(plan_commands(&[&a], &[&a], &positions, &selected).is_empty());
    }
}
