import { useCallback, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { getCurrentWindow } from "@tauri-apps/api/window";
import type { AppState, Profile, ProfileInput, Tab } from "./types";
import { messages } from "./i18n";
import { ScreenMap } from "./components/ScreenMap";
import { ProfileModal } from "./components/ProfileModal";
import { ConfirmDialog } from "./components/ConfirmDialog";
import { Switch } from "./components/Switch";
import { Toast, type ToastData } from "./components/Toast";
import {
  AlertIcon,
  CheckIcon,
  ChevronIcon,
  EditIcon,
  ExternalIcon,
  FolderIcon,
  GlobeIcon,
  GridIcon,
  InfoIcon,
  MonitorIcon,
  PlusIcon,
  PowerIcon,
  RefreshIcon,
  SettingsIcon,
  SparkIcon,
  TrashIcon,
  TrayIcon,
} from "./components/Icons";
import logo from "./assets/logo.svg";

type Dialog =
  | { type: "create" }
  | { type: "edit"; profile: Profile }
  | { type: "view" }
  | { type: "delete"; profile: Profile }
  | { type: "renumber" }
  | null;

function initialLang(): string {
  const language = navigator.language.toLowerCase();
  if (language.startsWith("es")) return "es";
  if (language.startsWith("ca")) return "ca";
  return "en";
}

export default function App() {
  const [state, setState] = useState<AppState | null>(null);
  const [loadError, setLoadError] = useState<string | null>(null);
  const [tab, setTab] = useState<Tab>("profiles");
  const [dialog, setDialog] = useState<Dialog>(null);
  const [busy, setBusy] = useState<string | null>(null);
  const [toast, setToast] = useState<ToastData | null>(null);
  const [screensOpen, setScreensOpen] = useState(() => {
    try {
      return window.localStorage.getItem("onoffscreens.screensOpen") !== "0";
    } catch {
      return true;
    }
  });
  const t = messages(state?.lang ?? initialLang());

  useEffect(() => {
    try {
      window.localStorage.setItem(
        "onoffscreens.screensOpen",
        screensOpen ? "1" : "0",
      );
    } catch {
      // localStorage can be unavailable; the toggle still works for this run.
    }
  }, [screensOpen]);

  const refresh = useCallback(async (silent = false) => {
    try {
      const next = await invoke<AppState>("get_state");
      setState(next);
      setLoadError(null);
    } catch (err) {
      if (!silent) setLoadError(String(err));
    }
  }, []);

  // Load display state only when the window is actually visible. In tray-only
  // or autostart mode the webview is hidden, so KScreen is left alone until the
  // user opens the window or runs an action.
  useEffect(() => {
    let disposed = false;
    void (async () => {
      let visible = true;
      try {
        visible = (await getCurrentWindow().isVisible()) && !document.hidden;
      } catch {
        visible = !document.hidden;
      }
      if (!disposed && visible) await refresh();
    })();
    return () => {
      disposed = true;
    };
  }, [refresh]);

  // No background polling: refresh only when the window comes back or the tray
  // sends an explicit state change.
  useEffect(() => {
    const onFocus = () => void refresh(true);
    const onVisibility = () => {
      if (!document.hidden) void refresh(true);
    };
    window.addEventListener("focus", onFocus);
    document.addEventListener("visibilitychange", onVisibility);
    return () => {
      window.removeEventListener("focus", onFocus);
      document.removeEventListener("visibilitychange", onVisibility);
    };
  }, [refresh]);

  useEffect(() => {
    if (!toast) return;
    const timer = window.setTimeout(() => setToast(null), 4200);
    return () => window.clearTimeout(timer);
  }, [toast]);

  useEffect(() => {
    if (state) document.documentElement.lang = state.lang;
  }, [state?.lang]);

  useEffect(() => {
    let disposed = false;
    const offs: UnlistenFn[] = [];
    void (async () => {
      const screens = await listen("open-screens", () => {
        setTab("profiles");
        setDialog({ type: "view" });
      });
      const create = await listen("open-new-profile", () => {
        setTab("profiles");
        setDialog({ type: "create" });
      });
      const changed = await listen("state-changed", () => {
        if (!document.hidden) void refresh(true);
      });
      if (disposed) {
        screens();
        create();
        changed();
      } else {
        offs.push(screens, create, changed);
      }
    })();
    return () => {
      disposed = true;
      offs.forEach((off) => off());
    };
  }, []);

  const notify = (kind: ToastData["kind"], text: string) =>
    setToast({ kind, text });

  const applyProfile = async (profile: Profile) => {
    setBusy(`apply:${profile.id}`);
    try {
      const next = await invoke<AppState>("apply_profile", { id: profile.id });
      setState(next);
      notify("success", t.toastApplied(profile.name));
    } catch (err) {
      notify("error", `${t.errorApply}: ${err}`);
    } finally {
      setBusy(null);
    }
  };

  const saveProfile = async (input: ProfileInput) => {
    try {
      const next = await invoke<AppState>("save_profile", { profile: input });
      setState(next);
      setDialog(null);
      notify(
        "success",
        input.id ? t.toastUpdated(input.name) : t.toastCreated(input.name),
      );
    } catch (err) {
      throw `${t.errorSave}: ${err}`;
    }
  };

  const deleteProfile = async (profile: Profile) => {
    setBusy("delete");
    try {
      const next = await invoke<AppState>("delete_profile", { id: profile.id });
      setState(next);
      setDialog(null);
      notify("success", t.toastDeleted(profile.name));
    } catch (err) {
      notify("error", `${t.errorDelete}: ${err}`);
    } finally {
      setBusy(null);
    }
  };

  const setOption = async (key: "autostart" | "tray", value: boolean) => {    setBusy(`option:${key}`);
    try {
      const next = await invoke<AppState>("set_option", { key, value });
      setState(next);
      if (key === "autostart") {
        notify("success", value ? t.toastAutostartOn : t.toastAutostartOff);
      } else {
        notify("success", value ? t.toastTrayOn : t.toastTrayOff);
      }
    } catch (err) {
      notify("error", `${t.errorOption}: ${err}`);
      await refresh(true);
    } finally {
      setBusy(null);
    }
  };

  const setLanguage = async (lang: "auto" | "es" | "ca" | "en") => {
    setBusy("language");
    try {
      const next = await invoke<AppState>("set_language", { lang });
      setState(next);
      notify("success", messages(next.lang).toastLanguage);
    } catch (err) {
      notify("error", `${t.errorLanguage}: ${err}`);
    } finally {
      setBusy(null);
    }
  };

  const reconfigureDisplays = async () => {
    setBusy("reconfigure");
    try {
      const next = await invoke<AppState>("reconfigure_displays");
      setState(next);
      notify("success", t.toastReconfigured);
    } catch (err) {
      notify("error", `${t.errorReconfigure}: ${err}`);
    } finally {
      setBusy(null);
    }
  };

  const renumber = async () => {
    setBusy("renumber");
    try {
      const next = await invoke<AppState>("reassign_numbers");
      setState(next);
      setDialog(null);
      notify("success", t.toastRenumbered);
    } catch (err) {
      notify("error", `${t.errorRenumber}: ${err}`);
    } finally {
      setBusy(null);
    }
  };

  const openKdeSettings = () => {
    void invoke("open_display_settings").catch((err) =>
      notify("error", String(err)),
    );
  };

  const openConfigFolder = () => {
    void invoke("open_config_folder").catch((err) =>
      notify("error", String(err)),
    );
  };

  if (!state) {
    return (
      <div className="splash">
        <img src={logo} alt="" className="splash-logo" />
        <h1>OnOffScreens</h1>
        {loadError ? (
          <>
            <p className="splash-error">{loadError}</p>
            <button className="btn btn-primary" onClick={() => void refresh()}>
              <RefreshIcon size={17} />
              {t.refresh}
            </button>
          </>
        ) : (
          <>
            <p>{t.loading}</p>
            <span className="spinner" />
          </>
        )}
      </div>
    );
  }

  const screenByUuid = new Map(state.screens.map((screen) => [screen.uuid, screen]));
  const activeProfile =
    state.profiles.find((profile) => profile.id === state.activeProfileId) ?? null;
  const enabledCount = state.screens.filter((screen) => screen.enabled).length;

  return (
    <div className="app-shell">
      <header className="topbar">
        <div className="brand">
          <img src={logo} alt="" className="brand-logo" />
          <div className="brand-text">
            <h1>
              {t.appName}
              <span className="version-badge" title={`${t.version} ${state.version}`}>
                v{state.version}
              </span>
            </h1>
            <p>{t.tagline}</p>
          </div>
        </div>
        <div className="topbar-actions">
          <span className="status-pill">
            <i />
            {state.screens.length} {t.connected}
          </span>
          <button
            type="button"
            className="icon-btn"
            onClick={() => void refresh()}
            title={t.refresh}
            aria-label={t.refresh}
          >
            <RefreshIcon size={18} />
          </button>
        </div>
      </header>

      <nav className="tabs" role="tablist" aria-label={t.appName}>
        <button
          type="button"
          role="tab"
          aria-selected={tab === "profiles"}
          className={tab === "profiles" ? "is-active" : ""}
          onClick={() => setTab("profiles")}
        >
          <GridIcon size={17} />
          {t.tabProfiles}
          <span className="tab-count">{state.profiles.length}</span>
        </button>
        <button
          type="button"
          role="tab"
          aria-selected={tab === "options"}
          className={tab === "options" ? "is-active" : ""}
          onClick={() => setTab("options")}
        >
          <SettingsIcon size={17} />
          {t.tabOptions}
        </button>
      </nav>

      <main className="content">
        {state.backendError && (
          <div className="error-banner" role="alert">
            <span className="error-banner-icon">
              <AlertIcon size={20} />
            </span>
            <div>
              <strong>{t.backendErrorTitle}</strong>
              <p>{t.backendErrorHint}</p>
              <code>{state.backendError}</code>
            </div>
          </div>
        )}

        {tab === "profiles" ? (
          <div className="view">
            <div className="view-head">
              <div>
                <h2>{t.profilesTitle}</h2>
                <p>{t.profilesHint}</p>
              </div>
              <button
                type="button"
                className="btn btn-primary"
                onClick={() => setDialog({ type: "create" })}
                disabled={state.screens.length === 0}
              >
                <PlusIcon size={17} />
                {t.newProfile}
              </button>
            </div>

            <section
              className={`card layout-card ${screensOpen ? "" : "is-collapsed"}`}
            >
              <div className="card-head">
                <div className="card-title">
                  <span className="icon-badge">
                    <MonitorIcon size={19} />
                  </span>
                  <div>
                    <h3>{t.detected}</h3>
                    <p>
                      {state.screens.length} {t.connected} · {enabledCount}{" "}
                      {t.activeScreens}
                    </p>
                  </div>
                </div>
                <div className="card-head-actions">
                  <button
                    type="button"
                    className="btn btn-ghost btn-sm"
                    onClick={() => void refresh()}
                  >
                    <RefreshIcon size={15} />
                    {t.refresh}
                  </button>
                  <button
                    type="button"
                    className={`icon-btn icon-btn-sm collapse-btn ${
                      screensOpen ? "is-open" : ""
                    }`}
                    onClick={() => setScreensOpen((open) => !open)}
                    title={screensOpen ? t.collapse : t.expand}
                    aria-label={screensOpen ? t.collapse : t.expand}
                    aria-expanded={screensOpen}
                    aria-controls="layout-body"
                  >
                    <ChevronIcon size={17} />
                  </button>
                </div>
              </div>
              {screensOpen && (
                <div className="layout-body" id="layout-body">
                  <ScreenMap
                    screens={state.screens}
                    lang={state.lang}
                    selected={
                      activeProfile ? new Set(activeProfile.screens) : undefined
                    }
                  />
                  <div className="map-legend">
                    <span>
                      <i className="legend-dot is-on" />
                      {t.screenOn}
                    </span>
                    <span>
                      <i className="legend-dot is-off" />
                      {t.screenOff}
                    </span>
                    <span className="legend-active">
                      {t.activeProfile}:{" "}
                      <strong>{activeProfile?.name ?? t.noActiveProfile}</strong>
                    </span>
                  </div>
                </div>
              )}
            </section>

            {state.profiles.length === 0 ? (
              <div className="empty-state">
                <span className="empty-icon">
                  <MonitorIcon size={30} />
                </span>
                <h3>{t.noProfiles}</h3>
                <p>{t.noProfilesHint}</p>
                <button
                  type="button"
                  className="btn btn-primary"
                  onClick={() => setDialog({ type: "create" })}
                  disabled={state.screens.length === 0}
                >
                  <PlusIcon size={17} />
                  {t.createFirst}
                </button>
              </div>
            ) : (
              <section className="profiles-section">
                <div className="section-title">
                  <h3>{t.tabProfiles}</h3>
                  <span className="count-badge">{state.profiles.length}</span>
                </div>
                <div className="profile-grid">
                  {state.profiles.map((profile) => {
                    const active = profile.id === state.activeProfileId;
                    const missing = profile.screens.filter(
                      (uuid) => !screenByUuid.has(uuid),
                    ).length;
                    return (
                      <article
                        key={profile.id}
                        className={`profile-card ${active ? "is-active" : ""}`}
                      >
                        <div className="profile-top">
                          <span className="profile-icon">
                            <MonitorIcon size={19} />
                          </span>
                          <div className="profile-heading">
                            <h4>{profile.name}</h4>
                            <p>
                              {t.screenCount(profile.screens.length)}
                              {missing > 0 && ` · ${t.screensMissing(missing)}`}
                            </p>
                          </div>
                          {active && (
                            <span className="active-badge">
                              <i />
                              {t.active}
                            </span>
                          )}
                        </div>

                        <div className="profile-screens">
                          {profile.screens.map((uuid) => {
                            const screen = screenByUuid.get(uuid);
                            return screen ? (
                              <span className="screen-chip" key={uuid}>
                                <b>{screen.number ?? "?"}</b>
                                {screen.name}
                              </span>
                            ) : (
                              <span
                                className="screen-chip is-missing"
                                key={uuid}
                                title={t.unavailable}
                              >
                                <i />
                                {t.unavailable}
                              </span>
                            );
                          })}
                        </div>

                        <div className="profile-actions">
                          <button
                            type="button"
                            className="btn btn-primary btn-sm"
                            disabled={active || busy !== null}
                            onClick={() => void applyProfile(profile)}
                          >
                            {busy === `apply:${profile.id}` ? (
                              <span className="spinner spinner-sm" />
                            ) : active ? (
                              <CheckIcon size={16} strokeWidth={2.4} />
                            ) : (
                              <PowerIcon size={16} />
                            )}
                            {active ? t.applied : t.apply}
                          </button>
                          <button
                            type="button"
                            className="btn btn-ghost btn-sm"
                            onClick={() => setDialog({ type: "edit", profile })}
                            disabled={busy !== null}
                          >
                            <EditIcon size={15} />
                            {t.edit}
                          </button>
                          <button
                            type="button"
                            className="btn btn-ghost btn-sm destructive"
                            onClick={() => setDialog({ type: "delete", profile })}
                            disabled={busy !== null}
                          >
                            <TrashIcon size={15} />
                            {t.delete}
                          </button>
                        </div>
                      </article>
                    );
                  })}
                </div>
              </section>
            )}
          </div>
        ) : (
          <div className="view">
            <div className="view-head">
              <div>
                <h2>{t.optionsTitle}</h2>
                <p>{t.optionsHint}</p>
              </div>
            </div>

            <section className="card options-card">
              <div className="card-head">
                <div className="card-title">
                  <span className="icon-badge">
                    <PowerIcon size={19} />
                  </span>
                  <div>
                    <h3>{t.startupSection}</h3>
                  </div>
                </div>
              </div>

              <div className="option-row">
                <span className="option-icon">
                  <PowerIcon size={19} />
                </span>
                <div className="option-text">
                  <strong>{t.autostart}</strong>
                  <p>{t.autostartHint}</p>
                </div>
                <Switch
                  checked={state.autostart}
                  onChange={(value) => void setOption("autostart", value)}
                  disabled={busy !== null}
                  label={t.autostart}
                />
              </div>

              <div className="option-row">
                <span className="option-icon">
                  <TrayIcon size={19} />
                </span>
                <div className="option-text">
                  <strong>{t.tray}</strong>
                  <p>{t.trayHint}</p>
                  {state.settings.tray && !state.trayRunning && (
                    <p className="option-warning">{t.trayUnavailable}</p>
                  )}
                </div>
                <Switch
                  checked={state.settings.tray}
                  onChange={(value) => void setOption("tray", value)}
                  disabled={busy !== null}
                  label={t.tray}
                />
              </div>
            </section>

            <section className="card options-card">
              <div className="card-head">
                <div className="card-title">
                  <span className="icon-badge">
                    <GlobeIcon size={19} />
                  </span>
                  <div>
                    <h3>{t.languageSection}</h3>
                  </div>
                </div>
              </div>

              <div className="option-row">
                <span className="option-icon">
                  <GlobeIcon size={19} />
                </span>
                <div className="option-text">
                  <strong>{t.languageSection}</strong>
                  <p>{t.languageHint}</p>
                  {state.settings.lang === "auto" && (
                    <p className="option-note">{t.languageAutoHint}</p>
                  )}
                </div>
                <div
                  className="segmented"
                  role="group"
                  aria-label={t.languageSection}
                >
                  {(
                    [
                      ["auto", t.languageAuto],
                      ["es", t.languageEs],
                      ["ca", t.languageCa],
                      ["en", t.languageEn],
                    ] as const
                  ).map(([value, label]) => (
                    <button
                      key={value}
                      type="button"
                      className={
                        state.settings.lang === value ? "is-active" : ""
                      }
                      aria-pressed={state.settings.lang === value}
                      disabled={busy !== null}
                      onClick={() => void setLanguage(value)}
                    >
                      {label}
                    </button>
                  ))}
                </div>
              </div>
            </section>

            <section className="card options-card">
              <div className="card-head">
                <div className="card-title">
                  <span className="icon-badge">
                    <SparkIcon size={19} />
                  </span>
                  <div>
                    <h3>{t.advancedSection}</h3>
                  </div>
                </div>
              </div>

              <button
                type="button"
                className="action-row"
                onClick={() => setDialog({ type: "renumber" })}
              >
                <span className="option-icon">
                  <GridIcon size={19} />
                </span>
                <span className="option-text">
                  <strong>{t.renumber}</strong>
                  <p>{t.renumberHint}</p>
                </span>
                <ChevronIcon size={18} className="action-chevron" />
              </button>

              <button
                type="button"
                className="action-row"
                onClick={() => void reconfigureDisplays()}
                disabled={busy !== null}
              >
                <span className="option-icon">
                  <RefreshIcon size={19} />
                </span>
                <span className="option-text">
                  <strong>{t.reconfigure}</strong>
                  <p>{t.reconfigureHint}</p>
                </span>
                <RefreshIcon size={17} className="action-chevron" />
              </button>

              <button
                type="button"
                className="action-row"
                onClick={openKdeSettings}
              >
                <span className="option-icon">
                  <MonitorIcon size={19} />
                </span>
                <span className="option-text">
                  <strong>{t.kdeSettings}</strong>
                  <p>{t.kdeSettingsHint}</p>
                </span>
                <ExternalIcon size={17} className="action-chevron" />
              </button>

              <button
                type="button"
                className="action-row"
                onClick={openConfigFolder}
              >
                <span className="option-icon">
                  <FolderIcon size={19} />
                </span>
                <span className="option-text">
                  <strong>{t.openFolder}</strong>
                  <p>{t.openFolderHint}</p>
                </span>
                <ExternalIcon size={17} className="action-chevron" />
              </button>
            </section>

            <section className="card about-card">
              <div className="about-head">
                <img src={logo} alt="" />
                <div>
                  <h3>{t.appName}</h3>
                  <p>
                    {t.version} {state.version}
                  </p>
                </div>
                <InfoIcon size={18} className="about-info" />
              </div>
              <p className="about-note">
                <SparkIcon size={15} />
                {t.dataNote}
              </p>
              <div className="path-list">
                <div>
                  <span>{t.profileLabel}</span>
                  <code>{state.configPath}</code>
                </div>
                <div>
                  <span>{t.screensLabel}</span>
                  <code>{state.mapPath}</code>
                </div>
              </div>
            </section>
          </div>
        )}
      </main>

      {dialog?.type === "create" && (
        <ProfileModal
          mode="create"
          screens={state.screens}
          lang={state.lang}
          onClose={() => setDialog(null)}
          onSave={saveProfile}
        />
      )}
      {dialog?.type === "edit" && (
        <ProfileModal
          mode="edit"
          profile={dialog.profile}
          screens={state.screens}
          lang={state.lang}
          onClose={() => setDialog(null)}
          onSave={saveProfile}
        />
      )}
      {dialog?.type === "view" && (
        <ProfileModal
          mode="view"
          screens={state.screens}
          lang={state.lang}
          onClose={() => setDialog(null)}
          onCreateFromView={() => setDialog({ type: "create" })}
        />
      )}
      {dialog?.type === "delete" && (
        <ConfirmDialog
          title={t.deleteTitle}
          text={t.deleteText(dialog.profile.name)}
          confirmLabel={t.deleteConfirm}
          cancelLabel={t.cancel}
          busy={busy === "delete"}
          onConfirm={() => void deleteProfile(dialog.profile)}
          onClose={() => setDialog(null)}
        />
      )}
      {dialog?.type === "renumber" && (
        <ConfirmDialog
          title={t.renumberConfirmTitle}
          text={t.renumberConfirmText}
          confirmLabel={t.renumber}
          cancelLabel={t.cancel}
          busy={busy === "renumber"}
          onConfirm={() => void renumber()}
          onClose={() => setDialog(null)}
        />
      )}

      {toast && (
        <Toast
          toast={toast}
          onClose={() => setToast(null)}
          closeLabel={t.close}
        />
      )}
    </div>
  );
}
