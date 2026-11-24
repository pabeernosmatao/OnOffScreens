export interface Messages {
  appName: string;
  tagline: string;
  tabProfiles: string;
  tabOptions: string;
  detected: string;
  connected: string;
  activeScreens: string;
  refresh: string;
  newProfile: string;
  profilesTitle: string;
  profilesHint: string;
  noProfiles: string;
  noProfilesHint: string;
  createFirst: string;
  apply: string;
  applied: string;
  edit: string;
  delete: string;
  active: string;
  screenCount: (n: number) => string;
  screensMissing: (n: number) => string;
  optionsTitle: string;
  optionsHint: string;
  startupSection: string;
  autostart: string;
  autostartHint: string;
  tray: string;
  trayHint: string;
  trayUnavailable: string;
  advancedSection: string;
  renumber: string;
  renumberHint: string;
  renumberConfirmTitle: string;
  renumberConfirmText: string;
  kdeSettings: string;
  kdeSettingsHint: string;
  openFolder: string;
  openFolderHint: string;
  about: string;
  version: string;
  dataNote: string;
  backendErrorTitle: string;
  backendErrorHint: string;
  cancel: string;
  save: string;
  create: string;
  modalNewTitle: string;
  modalEditTitle: string;
  modalViewTitle: string;
  nameLabel: string;
  namePlaceholder: string;
  chooseScreens: string;
  chooseHint: string;
  selected: string;
  noScreensDetected: string;
  createFromView: string;
  close: string;
  deleteTitle: string;
  deleteText: (name: string) => string;
  deleteConfirm: string;
  toastApplied: (name: string) => string;
  toastCreated: (name: string) => string;
  toastUpdated: (name: string) => string;
  toastDeleted: (name: string) => string;
  toastRenumbered: string;
  toastAutostartOn: string;
  toastAutostartOff: string;
  toastTrayOn: string;
  toastTrayOff: string;
  toastLanguage: string;
  errorApply: string;
  errorSave: string;
  errorDelete: string;
  errorOption: string;
  errorRenumber: string;
  errorLanguage: string;
  loading: string;
  screenOn: string;
  screenOff: string;
  primary: string;
  unavailable: string;
  resultsOf: string;
  profileLabel: string;
  screensLabel: string;
  keepDisconnected: (n: number) => string;
  resolution: string;
  activeProfile: string;
  noActiveProfile: string;
  errorConnection: string;
  languageSection: string;
  languageHint: string;
  languageAuto: string;
  languageAutoHint: string;
  languageEs: string;
  languageCa: string;
  languageEn: string;
  expand: string;
  collapse: string;
  reconfigure: string;
  reconfigureHint: string;
  toastReconfigured: string;
  errorReconfigure: string;
}

const es: Messages = {
  appName: "OnOffScreens",
  tagline: "Perfiles de monitores para KDE Plasma",
  tabProfiles: "Perfiles",
  tabOptions: "Opciones",
  detected: "Pantallas detectadas",
  connected: "conectadas",
  activeScreens: "encendidas",
  refresh: "Actualizar",
  newProfile: "Nuevo perfil",
  profilesTitle: "Tus perfiles",
  profilesHint: "Aplica un perfil para encender solo las pantallas que elijas.",
  noProfiles: "Todavía no hay perfiles",
  noProfilesHint:
    "Crea tu primer perfil eligiendo las pantallas que quieres encender.",
  createFirst: "Crear mi primer perfil",
  apply: "Aplicar",
  applied: "Aplicado",
  edit: "Editar",
  delete: "Borrar",
  active: "Activo",
  screenCount: (n) => `${n} pantalla${n === 1 ? "" : "s"}`,
  screensMissing: (n) => `${n} no conectada${n === 1 ? "" : "s"}`,
  optionsTitle: "Opciones",
  optionsHint: "Ajusta cómo se comporta OnOffScreens con tu sistema.",
  startupSection: "Inicio y bandeja",
  autostart: "Iniciar con el sistema",
  autostartHint: "Abre OnOffScreens automáticamente al iniciar sesión.",
  tray: "Icono en la bandeja del sistema",
  trayHint: "Ten tus perfiles siempre a mano desde la bandeja.",
  trayUnavailable:
    "El icono no está activo. En Plasma puede tardar unos segundos; si no aparece, revisa la bandeja del sistema.",
  advancedSection: "Avanzado",
  renumber: "Reasignar numeración",
  renumberHint:
    "Recalcula los números según la posición actual. Enciende todas las pantallas antes de hacerlo.",
  renumberConfirmTitle: "¿Reasignar la numeración?",
  renumberConfirmText:
    "Los números se asignarán de nuevo por posición. Tus perfiles no cambian, porque se guardan por monitor.",
  kdeSettings: "Configuración de pantallas de KDE",
  kdeSettingsHint: "Abre el panel de pantallas de Plasma.",
  openFolder: "Carpeta de configuración",
  openFolderHint: "Abre la carpeta donde se guardan perfiles y ajustes.",
  about: "Acerca de",
  version: "Versión",
  dataNote:
    "Los perfiles se guardan por UUID del monitor: siguen funcionando aunque cambies de puerto.",
  backendErrorTitle: "No se pudieron detectar las pantallas",
  backendErrorHint:
    "OnOffScreens necesita KDE Plasma y el paquete kscreen. Los ajustes y la bandeja siguen disponibles.",
  cancel: "Cancelar",
  save: "Guardar perfil",
  create: "Crear perfil",
  modalNewTitle: "Nuevo perfil",
  modalEditTitle: "Editar perfil",
  modalViewTitle: "Pantallas y distribución",
  nameLabel: "Nombre del perfil",
  namePlaceholder: "p. ej. Trabajo, Cine, Solo la tele…",
  chooseScreens: "Elige las pantallas que quieres encender",
  chooseHint: "Pulsa una pantalla del mapa para incluirla o quitarla.",
  selected: "seleccionadas",
  noScreensDetected: "No hay pantallas conectadas que mostrar.",
  createFromView: "Crear un perfil con esta distribución",
  close: "Cerrar",
  deleteTitle: "¿Borrar el perfil?",
  deleteText: (name) => `«${name}» se eliminará de forma permanente.`,
  deleteConfirm: "Sí, borrar",
  toastApplied: (name) => `Perfil aplicado: ${name}`,
  toastCreated: (name) => `Perfil creado: ${name}`,
  toastUpdated: (name) => `Perfil actualizado: ${name}`,
  toastDeleted: (name) => `Perfil borrado: ${name}`,
  toastRenumbered: "Numeración recalculada",
  toastAutostartOn: "Arranque automático activado",
  toastAutostartOff: "Arranque automático desactivado",
  toastTrayOn: "Icono de bandeja activado",
  toastTrayOff: "Icono de bandeja desactivado",
  toastLanguage: "Idioma cambiado",
  errorApply: "No se pudo aplicar el perfil",
  errorSave: "No se pudo guardar el perfil",
  errorDelete: "No se pudo borrar el perfil",
  errorOption: "No se pudo cambiar la opción",
  errorRenumber: "No se pudo reasignar la numeración",
  errorLanguage: "No se pudo cambiar el idioma",
  loading: "Detectando pantallas…",
  screenOn: "Encendida",
  screenOff: "Apagada",
  primary: "Principal",
  unavailable: "No conectada",
  resultsOf: "de",
  profileLabel: "Perfil",
  screensLabel: "Pantallas",
  keepDisconnected: (n) =>
    `${n} pantalla${n === 1 ? "" : "s"} no conectada${n === 1 ? "" : "s"} se conservará${n === 1 ? "" : "n"}.`,
  resolution: "Resolución",
  activeProfile: "Perfil activo",
  noActiveProfile: "Ninguno",
  errorConnection: "No se pudo conectar con el motor de OnOffScreens.",
  languageSection: "Idioma",
  languageHint: "Elige el idioma de la interfaz.",
  languageAuto: "Automático",
  languageAutoHint: "Según el idioma del sistema.",
  languageEs: "Español",
  languageCa: "Català",
  languageEn: "English",
  expand: "Mostrar",
  collapse: "Ocultar",
  reconfigure: "Reconfigurar pantallas",
  reconfigureHint:
    "Vuelve a aplicar la configuración actual de pantallas. Útil si los colores se ven mal o la imagen queda rara tras iniciar sesión.",
  toastReconfigured: "Pantallas reconfiguradas",
  errorReconfigure: "No se pudieron reconfigurar las pantallas",
};

const ca: Messages = {
  appName: "OnOffScreens",
  tagline: "Perfils de monitors per a KDE Plasma",
  tabProfiles: "Perfils",
  tabOptions: "Opcions",
  detected: "Pantalles detectades",
  connected: "connectades",
  activeScreens: "enceses",
  refresh: "Actualitza",
  newProfile: "Perfil nou",
  profilesTitle: "Els teus perfils",
  profilesHint: "Aplica un perfil per encendre només les pantalles que triïs.",
  noProfiles: "Encara no hi ha perfils",
  noProfilesHint:
    "Crea el teu primer perfil triant les pantalles que vols encendre.",
  createFirst: "Crea el meu primer perfil",
  apply: "Aplica",
  applied: "Aplicat",
  edit: "Edita",
  delete: "Elimina",
  active: "Actiu",
  screenCount: (n) => `${n} pantalla${n === 1 ? "" : "s"}`,
  screensMissing: (n) => `${n} no connectada${n === 1 ? "" : "s"}`,
  optionsTitle: "Opcions",
  optionsHint: "Ajusta com es comporta OnOffScreens amb el sistema.",
  startupSection: "Inici i safata",
  autostart: "Inicia amb el sistema",
  autostartHint: "Obre OnOffScreens automàticament en iniciar la sessió.",
  tray: "Icona a la safata del sistema",
  trayHint: "Tingues els teus perfils sempre a mà des de la safata.",
  trayUnavailable:
    "La icona no està activa. A Plasma pot trigar uns segons; si no apareix, revisa la safata del sistema.",
  advancedSection: "Avançat",
  renumber: "Reassigna la numeració",
  renumberHint:
    "Recalcula els números segons la posició actual. Encén totes les pantalles abans de fer-ho.",
  renumberConfirmTitle: "Voleu reassignar la numeració?",
  renumberConfirmText:
    "Els números s'assignaran de nou per posició. Els perfils no canvien, perquè es desen per monitor.",
  kdeSettings: "Configuració de pantalles del KDE",
  kdeSettingsHint: "Obre el tauler de pantalles del Plasma.",
  openFolder: "Carpeta de configuració",
  openFolderHint: "Obre la carpeta on es desen els perfils i els ajustos.",
  about: "Quant a",
  version: "Versió",
  dataNote:
    "Els perfils es desen per UUID del monitor: continuen funcionant encara que canviïs de port.",
  backendErrorTitle: "No s'han pogut detectar les pantalles",
  backendErrorHint:
    "OnOffScreens necessita KDE Plasma i el paquet kscreen. Els ajustos i la safata continuen disponibles.",
  cancel: "Cancel·la",
  save: "Desa el perfil",
  create: "Crea el perfil",
  modalNewTitle: "Perfil nou",
  modalEditTitle: "Edita el perfil",
  modalViewTitle: "Pantalles i distribució",
  nameLabel: "Nom del perfil",
  namePlaceholder: "p. ex. Feina, Cinema, Només la tele…",
  chooseScreens: "Tria les pantalles que vols encendre",
  chooseHint: "Fes clic en una pantalla del mapa per incloure-la o treure-la.",
  selected: "seleccionades",
  noScreensDetected: "No hi ha pantalles connectades per mostrar.",
  createFromView: "Crea un perfil amb aquesta distribució",
  close: "Tanca",
  deleteTitle: "Voleu eliminar el perfil?",
  deleteText: (name) => `«${name}» s'eliminarà de manera permanent.`,
  deleteConfirm: "Sí, elimina'l",
  toastApplied: (name) => `Perfil aplicat: ${name}`,
  toastCreated: (name) => `Perfil creat: ${name}`,
  toastUpdated: (name) => `Perfil actualitzat: ${name}`,
  toastDeleted: (name) => `Perfil eliminat: ${name}`,
  toastRenumbered: "Numeració recalculada",
  toastAutostartOn: "Inici automàtic activat",
  toastAutostartOff: "Inici automàtic desactivat",
  toastTrayOn: "Icona de safata activada",
  toastTrayOff: "Icona de safata desactivada",
  toastLanguage: "Idioma canviat",
  errorApply: "No s'ha pogut aplicar el perfil",
  errorSave: "No s'ha pogut desar el perfil",
  errorDelete: "No s'ha pogut eliminar el perfil",
  errorOption: "No s'ha pogut canviar l'opció",
  errorRenumber: "No s'ha pogut reassignar la numeració",
  errorLanguage: "No s'ha pogut canviar l'idioma",
  loading: "Detectant pantalles…",
  screenOn: "Encesa",
  screenOff: "Apagada",
  primary: "Principal",
  unavailable: "No connectada",
  resultsOf: "de",
  profileLabel: "Perfil",
  screensLabel: "Pantalles",
  keepDisconnected: (n) =>
    `${n} pantalla${n === 1 ? "" : "s"} no connectada${n === 1 ? "" : "s"} es conservarà.`,
  resolution: "Resolució",
  activeProfile: "Perfil actiu",
  noActiveProfile: "Cap",
  errorConnection: "No s'ha pogut connectar amb el motor d'OnOffScreens.",
  languageSection: "Idioma",
  languageHint: "Tria l'idioma de la interfície.",
  languageAuto: "Automàtic",
  languageAutoHint: "Segons l'idioma del sistema.",
  languageEs: "Español",
  languageCa: "Català",
  languageEn: "English",
  expand: "Desplega",
  collapse: "Plega",
  reconfigure: "Reconfigura les pantalles",
  reconfigureHint:
    "Torna a aplicar la configuració actual de pantalles. Útil si els colors es veuen malament o la imatge queda estranya després d'iniciar la sessió.",
  toastReconfigured: "Pantalles reconfigurades",
  errorReconfigure: "No s'han pogut reconfigurar les pantalles",
};

const en: Messages = {
  appName: "OnOffScreens",
  tagline: "Display profiles for KDE Plasma",
  tabProfiles: "Profiles",
  tabOptions: "Options",
  detected: "Detected screens",
  connected: "connected",
  activeScreens: "on",
  refresh: "Refresh",
  newProfile: "New profile",
  profilesTitle: "Your profiles",
  profilesHint: "Apply a profile to turn on only the screens you choose.",
  noProfiles: "No profiles yet",
  noProfilesHint: "Create your first profile by choosing the screens to turn on.",
  createFirst: "Create my first profile",
  apply: "Apply",
  applied: "Applied",
  edit: "Edit",
  delete: "Delete",
  active: "Active",
  screenCount: (n) => `${n} screen${n === 1 ? "" : "s"}`,
  screensMissing: (n) => `${n} not connected`,
  optionsTitle: "Options",
  optionsHint: "Choose how OnOffScreens behaves on your system.",
  startupSection: "Startup and tray",
  autostart: "Start with the system",
  autostartHint: "Open OnOffScreens automatically when you log in.",
  tray: "System tray icon",
  trayHint: "Keep your profiles one click away in the tray.",
  trayUnavailable:
    "The icon is not active. On Plasma it can take a few seconds; if it does not appear, check the system tray.",
  advancedSection: "Advanced",
  renumber: "Reassign numbering",
  renumberHint:
    "Recompute numbers from the current layout. Turn every screen on before doing this.",
  renumberConfirmTitle: "Reassign numbering?",
  renumberConfirmText:
    "Numbers will be assigned again by position. Your profiles stay the same because they are stored per monitor.",
  kdeSettings: "KDE display settings",
  kdeSettingsHint: "Open Plasma's display panel.",
  openFolder: "Configuration folder",
  openFolderHint: "Open the folder where profiles and settings are stored.",
  about: "About",
  version: "Version",
  dataNote:
    "Profiles are stored by monitor UUID, so they keep working even if you change ports.",
  backendErrorTitle: "Screens could not be detected",
  backendErrorHint:
    "OnOffScreens needs KDE Plasma and the kscreen package. Settings and the tray remain available.",
  cancel: "Cancel",
  save: "Save profile",
  create: "Create profile",
  modalNewTitle: "New profile",
  modalEditTitle: "Edit profile",
  modalViewTitle: "Screens and layout",
  nameLabel: "Profile name",
  namePlaceholder: "e.g. Work, Cinema, TV only…",
  chooseScreens: "Choose the screens to turn on",
  chooseHint: "Click a screen in the map to include or remove it.",
  selected: "selected",
  noScreensDetected: "No connected screens to show.",
  createFromView: "Create a profile with this layout",
  close: "Close",
  deleteTitle: "Delete this profile?",
  deleteText: (name) => `“${name}” will be permanently removed.`,
  deleteConfirm: "Yes, delete",
  toastApplied: (name) => `Profile applied: ${name}`,
  toastCreated: (name) => `Profile created: ${name}`,
  toastUpdated: (name) => `Profile updated: ${name}`,
  toastDeleted: (name) => `Profile deleted: ${name}`,
  toastRenumbered: "Numbering recomputed",
  toastAutostartOn: "Autostart enabled",
  toastAutostartOff: "Autostart disabled",
  toastTrayOn: "Tray icon enabled",
  toastTrayOff: "Tray icon disabled",
  toastLanguage: "Language changed",
  errorApply: "Could not apply the profile",
  errorSave: "Could not save the profile",
  errorDelete: "Could not delete the profile",
  errorOption: "Could not change the option",
  errorRenumber: "Could not reassign numbering",
  errorLanguage: "Could not change the language",
  loading: "Detecting screens…",
  screenOn: "On",
  screenOff: "Off",
  primary: "Primary",
  unavailable: "Not connected",
  resultsOf: "of",
  profileLabel: "Profile",
  screensLabel: "Screens",
  keepDisconnected: (n) =>
    `${n} disconnected screen${n === 1 ? "" : "s"} will be kept.`,
  resolution: "Resolution",
  activeProfile: "Active profile",
  noActiveProfile: "None",
  errorConnection: "Could not connect to the OnOffScreens engine.",
  languageSection: "Language",
  languageHint: "Choose the interface language.",
  languageAuto: "Automatic",
  languageAutoHint: "Follow the system language.",
  languageEs: "Español",
  languageCa: "Català",
  languageEn: "English",
  expand: "Expand",
  collapse: "Collapse",
  reconfigure: "Reconfigure displays",
  reconfigureHint:
    "Reapplies the current display configuration. Useful when colors look wrong or the image is odd after logging in.",
  toastReconfigured: "Displays reconfigured",
  errorReconfigure: "Could not reconfigure the displays",
};

export function messages(lang: string | undefined): Messages {
  if (lang === "es") return es;
  if (lang === "ca") return ca;
  return en;
}
