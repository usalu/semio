import { type LayeredOffset, type LayeredCell, type LayeredVeil, veilClipPath, type LayeredWindow } from "../../../🔨️modules/🥞️layered-overview-geometry/🟦️.ts";
import * as React from "react";

// #region 🖼️LayeredPoster
/** 🖼️ Composites every `<canvas>` inside `container` into one PNG data URL, synchronously (before a `preserveDrawingBuffer: false` back buffer is
 * cleared); `null` when the container holds no drawable canvas, so a DOM-only page never gets a poster. */
export function capturePosterFromCanvases(container: HTMLElement): string | null {
  const canvases = [...container.querySelectorAll("canvas")].filter((canvas) => canvas.width > 0 && canvas.height > 0);
  const box = container.getBoundingClientRect();
  if (canvases.length === 0 || box.width <= 0 || box.height <= 0) return null;
  const poster = document.createElement("canvas");
  poster.width = Math.round(box.width);
  poster.height = Math.round(box.height);
  const context = poster.getContext("2d");
  if (!context) return null;
  const drawn = canvases.filter((canvas) => {
    const rect = canvas.getBoundingClientRect();
    try {
      context.drawImage(canvas, rect.left - box.left, rect.top - box.top, rect.width, rect.height);
      return true;
    } catch {
      return false;
    }
  });
  if (drawn.length === 0) return null;
  try {
    return poster.toDataURL("image/png");
  } catch {
    return null;
  }
}
// #endregion 🖼️LayeredPoster

const REDUCED_MOTION_QUERY = "(prefers-reduced-motion: reduce)";
export const FOCUSABLE = 'a[href], button:not([disabled]), input:not([disabled]), select:not([disabled]), textarea:not([disabled]), [tabindex]:not([tabindex="-1"])';

export type LayeredDrive = { readonly kind: "follow" } | { readonly kind: "glide" | "settle"; readonly from: LayeredOffset; readonly to: LayeredOffset; readonly startedAt: number };
export const FOLLOW: LayeredDrive = { kind: "follow" };

/** 🐢️ Whether the user asks for reduced motion, unless the app forces it on or off. */
export function useReducedMotion(setting: "auto" | "always" | "never"): boolean {
  const [prefers, setPrefers] = React.useState(() => typeof window !== "undefined" && typeof window.matchMedia === "function" && window.matchMedia(REDUCED_MOTION_QUERY).matches);
  React.useEffect(() => {
    if (setting !== "auto" || typeof window.matchMedia !== "function") return;
    const media = window.matchMedia(REDUCED_MOTION_QUERY);
    const sync = () => setPrefers(media.matches);
    sync();
    media.addEventListener("change", sync);
    return () => media.removeEventListener("change", sync);
  }, [setting]);
  return setting === "always" || (setting === "auto" && prefers);
}

/** 📍️ The strip cell of pane `id`; every pane must have one. */
export function requireCell(cells: Readonly<Record<string, LayeredCell>>, id: string): LayeredCell {
  const cell = cells[id];
  if (!cell) throw new Error(`LayeredOverview: pane "${id}" has no cell`);
  return cell;
}

/** 🔗️ The pane a location hash names, if any. */
export function hashPaneId(ids: readonly string[]): string | null {
  const raw = window.location.hash.replace(/^#/, "");
  return ids.includes(raw) ? raw : null;
}

/** 🔗️ Mirrors the opened pane into the hash with `replaceState` (no history entries); a hash naming no pane, such as a skip link, is left alone. */
export function syncHash(opened: string | null, ids: readonly string[]): void {
  const current = window.location.hash.replace(/^#/, "");
  if (opened === null ? !ids.includes(current) : current === opened) return;
  window.history.replaceState(window.history.state, "", opened === null ? window.location.pathname + window.location.search : `#${opened}`);
}

/** ⌨️ Whether focus landed by keyboard (`:focus-visible`); a browser that cannot answer counts as keyboard. */
export function focusVisible(target: EventTarget): boolean {
  if (!(target instanceof Element)) return true;
  try {
    return target.matches(":focus-visible");
  } catch {
    return true;
  }
}

/** 🕳️ Writes a veil onto the glass element. */
export function applyVeil(element: HTMLElement, veil: LayeredVeil): void {
  element.style.clipPath = veilClipPath(veil);
  element.style.visibility = veil.kind === "clear" ? "hidden" : "visible";
  element.dataset.veil = veil.kind;
}

/** 🪟️ Whether two windows cover the same cells. */
export function sameWindow(a: LayeredWindow, b: LayeredWindow): boolean {
  return a.firstColumn === b.firstColumn && a.lastColumn === b.lastColumn && a.firstRow === b.firstRow && a.lastRow === b.lastRow;
}

/** 🧷️ The first value seen for a key: equal ids and cells keep one identity across renders, so an app re-rendering with fresh arrays never
 * restarts a glide or a warm-boot timer. */
export function useStableByKey<T>(value: T, key: string): T {
  const kept = React.useRef({ key, value });
  if (kept.current.key !== key) kept.current = { key, value };
  return kept.current.value;
}
