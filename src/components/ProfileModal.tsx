import { useEffect, useRef, useState, type FormEvent } from "react";
import type { Profile, ProfileInput, Screen } from "../types";
import { messages } from "../i18n";
import { Modal } from "./Modal";
import { ScreenMap } from "./ScreenMap";
import {
  CheckIcon,
  CloseIcon,
  MonitorIcon,
  PlusIcon,
  SparkIcon,
} from "./Icons";

type Mode = "create" | "edit" | "view";

interface Props {
  mode: Mode;
  profile?: Profile;
  screens: Screen[];
  lang: string;
  onClose: () => void;
  onSave?: (input: ProfileInput) => Promise<void>;
  onCreateFromView?: () => void;
}

export function ProfileModal({
  mode,
  profile,
  screens,
  lang,
  onClose,
  onSave,
  onCreateFromView,
}: Props) {
  const t = messages(lang);
  const [name, setName] = useState(profile?.name ?? "");
  const [selected, setSelected] = useState<Set<string>>(() => {
    if (mode === "edit" && profile) return new Set(profile.screens);
    return new Set(screens.map((screen) => screen.uuid));
  });
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const inputRef = useRef<HTMLInputElement>(null);
  const interactive = mode !== "view";

  useEffect(() => {
    if (mode === "view") return;
    const timer = window.setTimeout(() => {
      inputRef.current?.focus();
      inputRef.current?.select();
    }, 80);
    return () => window.clearTimeout(timer);
  }, [mode]);

  const connected = new Set(screens.map((screen) => screen.uuid));
  const missing =
    mode === "edit" && profile
      ? profile.screens.filter((uuid) => !connected.has(uuid)).length
      : 0;
  const selectedVisible = screens.filter((screen) => selected.has(screen.uuid)).length;

  const toggle = (uuid: string) => {
    setSelected((current) => {
      const next = new Set(current);
      if (next.has(uuid)) next.delete(uuid);
      else next.add(uuid);
      return next;
    });
  };

  const submit = async (event: FormEvent) => {
    event.preventDefault();
    if (saving || !onSave) return;
    if (!name.trim() || selected.size === 0) return;
    setSaving(true);
    setError(null);
    try {
      await onSave({
        id: profile?.id,
        name: name.trim(),
        screens: [...selected],
      });
    } catch (err) {
      setError(String(err));
      setSaving(false);
    }
  };

  const title =
    mode === "create"
      ? t.modalNewTitle
      : mode === "edit"
        ? t.modalEditTitle
        : t.modalViewTitle;

  const subtitle =
    mode === "view"
      ? `${screens.length} ${t.connected} · ${
          screens.filter((screen) => screen.enabled).length
        } ${t.activeScreens}`
      : `${t.chooseScreens}. ${t.chooseHint}`;

  return (
    <Modal onClose={onClose} label={title} wide>
      <form className="modal-form" onSubmit={submit}>
        <header className="modal-head">
          <div className="modal-title">
            <span className="modal-kicker">
              <MonitorIcon size={14} />
              {mode === "view" ? t.detected : t.profileLabel}
            </span>
            <h2>{title}</h2>
            <p>{subtitle}</p>
          </div>
          <button
            type="button"
            className="icon-btn"
            onClick={onClose}
            aria-label={t.close}
          >
            <CloseIcon size={18} />
          </button>
        </header>

        <div className="modal-body">
          {mode !== "view" && (
            <label className="field">
              <span className="field-label">{t.nameLabel}</span>
              <input
                ref={inputRef}
                className="text-input"
                value={name}
                onChange={(event) => setName(event.target.value)}
                placeholder={t.namePlaceholder}
                maxLength={64}
                required
              />
            </label>
          )}

          <div className="map-heading">
            <div>
              <h3>{interactive ? t.chooseScreens : t.detected}</h3>
              <p>
                {interactive ? t.chooseHint : `${screens.length} ${t.connected}`}
              </p>
            </div>
            {interactive && (
              <span className="selection-pill">
                <strong>{selectedVisible}</strong>/{screens.length} {t.selected}
              </span>
            )}
          </div>

          <ScreenMap
            screens={screens}
            lang={lang}
            selected={interactive ? selected : undefined}
            interactive={interactive}
            onToggle={toggle}
          />

          {missing > 0 && (
            <p className="inline-note">
              <SparkIcon size={15} />
              {t.keepDisconnected(missing)}
            </p>
          )}

          <div className="screen-selection-list">
            {screens.map((screen) => {
              const isSelected = selected.has(screen.uuid);
              const content = (
                <>
                  <span className="list-screen-number">{screen.number ?? "?"}</span>
                  <span className="list-screen-info">
                    <strong>{screen.name}</strong>
                    <span>
                      {screen.width} × {screen.height} ·{" "}
                      {screen.enabled ? t.screenOn : t.screenOff}
                    </span>
                  </span>
                  {interactive ? (
                    <span className={`screen-checkbox ${isSelected ? "is-on" : ""}`}>
                      {isSelected && <CheckIcon size={13} strokeWidth={2.8} />}
                    </span>
                  ) : (
                    <span
                      className={`state-pill ${screen.enabled ? "is-on" : "is-off"}`}
                    >
                      <i />
                      {screen.enabled ? t.screenOn : t.screenOff}
                    </span>
                  )}
                </>
              );
              return interactive ? (
                <button
                  key={screen.uuid}
                  type="button"
                  className={`screen-list-row ${isSelected ? "is-selected" : ""}`}
                  onClick={() => toggle(screen.uuid)}
                  aria-pressed={isSelected}
                >
                  {content}
                </button>
              ) : (
                <div key={screen.uuid} className="screen-list-row is-static">
                  {content}
                </div>
              );
            })}
          </div>

          {error && <div className="form-error">{error}</div>}
        </div>

        <footer className="modal-foot">
          {mode === "view" ? (
            <>
              <button type="button" className="btn btn-ghost" onClick={onClose}>
                {t.close}
              </button>
              <button
                type="button"
                className="btn btn-primary"
                onClick={onCreateFromView}
              >
                <PlusIcon size={17} />
                {t.createFromView}
              </button>
            </>
          ) : (
            <>
              <button
                type="button"
                className="btn btn-ghost"
                onClick={onClose}
                disabled={saving}
              >
                {t.cancel}
              </button>
              <button
                type="submit"
                className="btn btn-primary"
                disabled={saving || !name.trim() || selected.size === 0}
              >
                {mode === "edit" ? <CheckIcon size={17} /> : <PlusIcon size={17} />}
                {saving ? "…" : mode === "edit" ? t.save : t.create}
              </button>
            </>
          )}
        </footer>
      </form>
    </Modal>
  );
}
