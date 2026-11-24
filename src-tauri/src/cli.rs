use crate::lang::{self, Lang};
use crate::{displays, profiles, state};
use std::process::exit;

fn help(lang: Lang) {
    match lang {
        Lang::Es => println!(
            "OnOffScreens — perfiles de monitores para KDE Plasma\n\n\
             Uso:\n\
             \x20 onoffscreens                     abre la ventana\n\
             \x20 onoffscreens --tray              abre solo el icono de la bandeja\n\
             \x20 onoffscreens --list              lista los perfiles\n\
             \x20 onoffscreens --apply <n|nombre>  aplica un perfil\n\
             \x20 onoffscreens --status            muestra las pantallas detectadas\n\
             \x20 onoffscreens --renumber          recalcula la numeración\n\n\
             También puede ejecutar el AppImage directamente."
        ),
        Lang::Ca => println!(
            "OnOffScreens — perfils de monitors per a KDE Plasma\n\n\
             Ús:\n\
             \x20 onoffscreens                     obre la finestra\n\
             \x20 onoffscreens --tray              inicia només la icona de la safata\n\
             \x20 onoffscreens --list              llista els perfils\n\
             \x20 onoffscreens --apply <n|nom>     aplica un perfil\n\
             \x20 onoffscreens --status            mostra les pantalles detectades\n\
             \x20 onoffscreens --renumber          recalcula la numeració\n\n\
             També pots executar l'AppImage directament."
        ),
        Lang::En => println!(
            "OnOffScreens — display profiles for KDE Plasma\n\n\
             Usage:\n\
             \x20 onoffscreens                     open the window\n\
             \x20 onoffscreens --tray              start only the tray icon\n\
             \x20 onoffscreens --list              list profiles\n\
             \x20 onoffscreens --apply <n|name>    apply a profile\n\
             \x20 onoffscreens --status            show detected screens\n\
             \x20 onoffscreens --renumber          recompute screen numbering\n\n\
             You can also run the AppImage directly."
        ),
    }
}

struct Context {
    outputs: Vec<displays::Output>,
    profiles: Vec<crate::model::Profile>,
    active: Option<String>,
}

fn context() -> Result<Context, String> {
    let outputs = displays::read_outputs()?;
    let map = displays::refresh_map(&outputs, false)?;
    let profile_list = profiles::ensure(&outputs, &map)?;
    let settings = crate::settings::load();
    let active = state::active_profile_id(&profile_list, &outputs, &settings);
    Ok(Context {
        outputs,
        profiles: profile_list,
        active,
    })
}

fn list(lang: Lang) -> Result<(), String> {
    let ctx = context()?;
    if ctx.profiles.is_empty() {
        println!(
            "{}",
            match lang {
                Lang::Es => "(sin perfiles)",
                Lang::Ca => "(sense perfils)",
                Lang::En => "(no profiles)",
            }
        );
        return Ok(());
    }
    for (index, profile) in ctx.profiles.iter().enumerate() {
        let mark = if ctx.active.as_deref() == Some(profile.id.as_str()) {
            "●"
        } else {
            " "
        };
        println!("{mark} {}. {}", index + 1, profile.name);
    }
    Ok(())
}

fn status() -> Result<(), String> {
    let ctx = context()?;
    let strings = lang::strings(lang::current());
    for output in ctx.outputs.iter().filter(|o| o.connected) {
        let mark = if output.enabled {
            strings.screens_on
        } else {
            strings.screens_off
        };
        println!(
            "{:<14} {:>5}x{:<5} {}",
            output.name, output.width, output.height, mark
        );
    }
    if let Some(active) = ctx
        .active
        .as_deref()
        .and_then(|id| ctx.profiles.iter().find(|p| p.id == id))
    {
        println!("\n●: {}", active.name);
    }
    Ok(())
}

fn apply(selector: &str) -> Result<(), String> {
    let ctx = context()?;
    let strings = lang::strings(lang::current());
    let lowered = selector.trim().to_lowercase();

    let selected = selector
        .parse::<usize>()
        .ok()
        .and_then(|number| ctx.profiles.get(number.wrapping_sub(1)))
        .or_else(|| ctx.profiles.iter().find(|p| p.id == selector))
        .or_else(|| {
            ctx.profiles
                .iter()
                .find(|p| p.name.to_lowercase() == lowered)
        })
        .or_else(|| {
            let matches: Vec<_> = ctx
                .profiles
                .iter()
                .filter(|p| p.name.to_lowercase().contains(&lowered))
                .collect();
            (matches.len() == 1).then(|| matches[0])
        })
        .ok_or_else(|| strings.profile_not_found.to_string())?;

    let name = state::apply_profile_cli(&selected.id)?;
    println!("{}: {name}", strings.profile_applied);
    Ok(())
}

fn renumber() -> Result<(), String> {
    let outputs = displays::read_outputs()?;
    displays::renumber(&outputs)?;
    println!("{}", lang::strings(lang::current()).renumbered);
    Ok(())
}

/// Returns true when the process should exit (CLI mode) and false to start the GUI.
pub fn handle(args: &[String]) -> bool {
    let language = lang::current();
    if args.is_empty() {
        return false;
    }
    let result = match args[0].as_str() {
        "--autostart" | "--tray" => return false,
        "-h" | "--help" | "help" => {
            help(language);
            return true;
        }
        "-V" | "--version" | "version" => {
            println!("OnOffScreens {}", env!("CARGO_PKG_VERSION"));
            return true;
        }
        "--list" | "list" => list(language),
        "--status" | "status" => status(),
        "--renumber" | "--reassign" | "renumber" | "reassign" => renumber(),
        "--apply" | "apply" => match args.get(1) {
            Some(selector) => apply(selector),
            None => Err(match language {
                Lang::Es => "Uso: onoffscreens --apply <n|nombre>".to_string(),
                Lang::Ca => "Ús: onoffscreens --apply <n|nom>".to_string(),
                Lang::En => "Usage: onoffscreens --apply <n|name>".to_string(),
            }),
        },
        _ => return false,
    };
    match result {
        Ok(()) => true,
        Err(err) => {
            eprintln!("{err}");
            exit(2);
        }
    }
}
