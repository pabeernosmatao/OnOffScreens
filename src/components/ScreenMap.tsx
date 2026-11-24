import { useEffect, useRef, useState } from "react";
import type { Screen } from "../types";
import { messages } from "../i18n";
import { CheckIcon, MonitorIcon } from "./Icons";

interface Props {
  screens: Screen[];
  lang: string;
  selected?: Set<string>;
  interactive?: boolean;
  onToggle?: (uuid: string) => void;
}

const PADDING = 22;

export function ScreenMap({
  screens,
  lang,
  selected,
  interactive = false,
  onToggle,
}: Props) {
  const t = messages(lang);
  const ref = useRef<HTMLDivElement>(null);
  const [size, setSize] = useState({ width: 0, height: 0 });

  useEffect(() => {
    const element = ref.current;
    if (!element) return;
    const observer = new ResizeObserver(([entry]) => {
      setSize({
        width: entry.contentRect.width,
        height: entry.contentRect.height,
      });
    });
    observer.observe(element);
    return () => observer.disconnect();
  }, []);

  const visible = screens.filter((screen) => screen.width > 0 && screen.height > 0);
  if (visible.length === 0) {
    return (
      <div className="screen-map screen-map-empty" ref={ref}>
        <div>
          <MonitorIcon size={26} />
          <span>{t.noScreensDetected}</span>
        </div>
      </div>
    );
  }

  const minX = Math.min(...visible.map((screen) => screen.x));
  const minY = Math.min(...visible.map((screen) => screen.y));
  const maxX = Math.max(...visible.map((screen) => screen.x + screen.width));
  const maxY = Math.max(...visible.map((screen) => screen.y + screen.height));
  const totalWidth = Math.max(1, maxX - minX);
  const totalHeight = Math.max(1, maxY - minY);
  const availableWidth = Math.max(1, size.width - PADDING * 2);
  const availableHeight = Math.max(1, size.height - PADDING * 2);
  const scale = Math.min(availableWidth / totalWidth, availableHeight / totalHeight);
  const canvasWidth = totalWidth * scale;
  const canvasHeight = totalHeight * scale;

  const ordered = [...visible].sort(
    (a, b) =>
      (a.number ?? Number.MAX_SAFE_INTEGER) -
        (b.number ?? Number.MAX_SAFE_INTEGER) || a.x - b.x,
  );

  return (
    <div className="screen-map" ref={ref}>
      <div
        className="screen-map-canvas"
        style={{
          width: canvasWidth,
          height: canvasHeight,
          left: (size.width - canvasWidth) / 2,
          top: (size.height - canvasHeight) / 2,
        }}
      >
        {ordered.map((screen) => {
          const width = screen.width * scale;
          const height = screen.height * scale;
          const isSelected = selected?.has(screen.uuid) ?? false;
          const compact = width < 128 || height < 76;
          const classes = [
            "screen-tile",
            screen.enabled ? "is-on" : "is-off",
            isSelected ? "is-selected" : "",
            interactive ? "is-interactive" : "",
            compact ? "is-compact" : "",
          ]
            .filter(Boolean)
            .join(" ");

          const state = screen.enabled ? t.screenOn : t.screenOff;
          const label = `${t.screenCount(1)} ${screen.number ?? "?"} · ${screen.name} · ${
            screen.width
          }×${screen.height} · ${state}`;

          return (
            <button
              key={screen.uuid}
              type="button"
              className={classes}
              style={{
                left: (screen.x - minX) * scale,
                top: (screen.y - minY) * scale,
                width,
                height,
              }}
              onClick={() => interactive && onToggle?.(screen.uuid)}
              tabIndex={interactive ? 0 : -1}
              aria-pressed={interactive ? isSelected : undefined}
              aria-label={label}
            >
              <span className="screen-tile-head">
                <span className="screen-number">
                  {screen.number ?? "?"}
                  {screen.primary && <i className="screen-primary-dot" />}
                </span>
                {interactive && (
                  <span className={`screen-checkbox ${isSelected ? "is-on" : ""}`}>
                    {isSelected && <CheckIcon size={12} strokeWidth={2.6} />}
                  </span>
                )}
              </span>
              <span className="screen-tile-body">
                <strong>{screen.name}</strong>
                <span>
                  {screen.width} × {screen.height}
                </span>
              </span>
              <span className="screen-tile-foot">
                <i className="screen-state-dot" />
                {state}
              </span>
            </button>
          );
        })}
      </div>
    </div>
  );
}
