use std::env;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lang {
    Es,
    Ca,
    En,
}

impl Lang {
    pub fn code(self) -> &'static str {
        match self {
            Lang::Es => "es",
            Lang::Ca => "ca",
            Lang::En => "en",
        }
    }

    pub fn from_code(code: &str) -> Option<Lang> {
        match code {
            "es" => Some(Lang::Es),
            "ca" => Some(Lang::Ca),
            "en" => Some(Lang::En),
            _ => None,
        }
    }
}

fn locale_code(value: &str) -> Option<String> {
    let code = value
        .split(':')
        .next()?
        .split('@')
        .next()?
        .split('.')
        .next()?
        .split('_')
        .next()?
        .split('-')
        .next()?
        .to_ascii_lowercase();
    (code.len() == 2 && code.chars().all(|c| c.is_ascii_alphabetic())).then_some(code)
}

/// ONOFFSCREENS_IDIOMA wins, then LANGUAGE / LC_ALL / LC_MESSAGES / LANG.
/// Spanish variants use Spanish, Catalan variants use Catalan and anything
/// else falls back to English. The old PANTALLAS_IDIOMA variable is still
/// accepted for compatibility.
pub fn detect() -> Lang {
    for var in [
        "ONOFFSCREENS_IDIOMA",
        "PANTALLAS_IDIOMA",
        "LANGUAGE",
        "LC_ALL",
        "LC_MESSAGES",
        "LANG",
    ] {
        let Some(value) = env::var_os(var) else {
            continue;
        };
        let value = value.to_string_lossy();
        let Some(code) = locale_code(&value) else {
            continue;
        };
        return Lang::from_code(&code).unwrap_or(Lang::En);
    }
    Lang::En
}

/// Resolve a stored preference ("auto" by default) into a language.
pub fn resolve(preference: &str) -> Lang {
    Lang::from_code(preference).unwrap_or_else(detect)
}

/// Language currently in use, honoring the user preference in settings.
pub fn current() -> Lang {
    resolve(&crate::settings::load().lang)
}

pub struct Strings {
    pub open_app: &'static str,
    pub new_profile: &'static str,
    pub show_screens: &'static str,
    pub kde_settings: &'static str,
    pub quit: &'static str,
    pub tooltip: &'static str,
    pub profile_applied: &'static str,
    pub profile_failed: &'static str,
    pub notification_title: &'static str,
    pub no_kscreen: &'static str,
    pub read_error: &'static str,
    pub empty_profile: &'static str,
    pub empty_name: &'static str,
    pub no_screens: &'static str,
    pub duplicate_name: &'static str,
    pub profile_not_found: &'static str,
    pub screens_on: &'static str,
    pub screens_off: &'static str,
    pub renumbered: &'static str,
    pub reconfigure: &'static str,
    pub reconfigured: &'static str,
    pub reconfigure_failed: &'static str,
    pub no_screens_on: &'static str,
    pub kde_settings_failed: &'static str,
}

pub fn strings(lang: Lang) -> Strings {
    match lang {        Lang::Es => Strings {
            open_app: "Abrir OnOffScreens",
            new_profile: "Crear perfil…",
            show_screens: "Pantallas y distribución…",
            kde_settings: "Configuración de pantallas de KDE…",
            quit: "Salir",
            tooltip: "OnOffScreens — perfiles de monitores",
            profile_applied: "Perfil aplicado",
            profile_failed: "No se pudo aplicar el perfil",
            notification_title: "OnOffScreens",
            no_kscreen: "No se encuentra kscreen-doctor. Instala KDE Plasma (paquete kscreen).",
            read_error: "No se pudo leer la configuración de pantallas",
            empty_profile: "Ninguna de las pantallas del perfil está conectada.",
            empty_name: "El nombre del perfil no puede estar vacío.",
            no_screens: "Selecciona al menos una pantalla.",
            duplicate_name: "Ya existe otro perfil con ese nombre.",
            profile_not_found: "Perfil no encontrado.",
            screens_on: "encendidas",
            screens_off: "apagada",
            renumbered: "Numeración de pantallas recalculada.",
            reconfigure: "Reconfigurar pantallas",
            reconfigured: "Pantallas reconfiguradas",
            reconfigure_failed: "No se pudieron reconfigurar las pantallas",
            no_screens_on: "No hay pantallas encendidas que reconfigurar.",
            kde_settings_failed: "No se pudo abrir la configuración de pantallas de KDE.",
        },
        Lang::Ca => Strings {
            open_app: "Obre OnOffScreens",
            new_profile: "Crea un perfil…",
            show_screens: "Pantalles i distribució…",
            kde_settings: "Configuració de pantalles del KDE…",
            quit: "Surt",
            tooltip: "OnOffScreens — perfils de monitors",
            profile_applied: "Perfil aplicat",
            profile_failed: "No s'ha pogut aplicar el perfil",
            notification_title: "OnOffScreens",
            no_kscreen: "No es troba kscreen-doctor. Instal·la KDE Plasma (paquet kscreen).",
            read_error: "No s'ha pogut llegir la configuració de pantalles",
            empty_profile: "Cap de les pantalles del perfil no està connectada.",
            empty_name: "El nom del perfil no pot estar buit.",
            no_screens: "Selecciona almenys una pantalla.",
            duplicate_name: "Ja existeix un altre perfil amb aquest nom.",
            profile_not_found: "Perfil no trobat.",
            screens_on: "enceses",
            screens_off: "apagada",
            renumbered: "Numeració de pantalles recalculada.",
            reconfigure: "Reconfigura les pantalles",
            reconfigured: "Pantalles reconfigurades",
            reconfigure_failed: "No s'han pogut reconfigurar les pantalles",
            no_screens_on: "No hi ha pantalles enceses per reconfigurar.",
            kde_settings_failed: "No s'ha pogut obrir la configuració de pantalles del KDE.",
        },
        Lang::En => Strings {
            open_app: "Open OnOffScreens",
            new_profile: "Create profile…",
            show_screens: "Screens and layout…",
            kde_settings: "KDE display settings…",
            quit: "Quit",
            tooltip: "OnOffScreens — display profiles",
            profile_applied: "Profile applied",
            profile_failed: "Could not apply the profile",
            notification_title: "OnOffScreens",
            no_kscreen: "Cannot find kscreen-doctor. Install KDE Plasma (kscreen package).",
            read_error: "Could not read the display configuration",
            empty_profile: "None of the screens in this profile is connected.",
            empty_name: "The profile name cannot be empty.",
            no_screens: "Select at least one screen.",
            duplicate_name: "Another profile already uses that name.",
            profile_not_found: "Profile not found.",
            screens_on: "on",
            screens_off: "off",
            renumbered: "Screen numbering recalculated.",
            reconfigure: "Reconfigure displays",
            reconfigured: "Displays reconfigured",
            reconfigure_failed: "Could not reconfigure the displays",
            no_screens_on: "There are no enabled screens to reconfigure.",
            kde_settings_failed: "Could not open KDE display settings.",
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_locale_variants() {
        assert_eq!(locale_code("es_ES.UTF-8").as_deref(), Some("es"));
        assert_eq!(locale_code("es_MX").as_deref(), Some("es"));
        assert_eq!(locale_code("ca_ES.UTF-8").as_deref(), Some("ca"));
        assert_eq!(locale_code("ca_ES@valencia").as_deref(), Some("ca"));
        assert_eq!(locale_code("en_US.UTF-8").as_deref(), Some("en"));
        assert_eq!(locale_code("C.UTF-8"), None);
    }

    #[test]
    fn resolves_explicit_preferences() {
        assert_eq!(resolve("es"), Lang::Es);
        assert_eq!(resolve("ca"), Lang::Ca);
        assert_eq!(resolve("en"), Lang::En);
    }
}
