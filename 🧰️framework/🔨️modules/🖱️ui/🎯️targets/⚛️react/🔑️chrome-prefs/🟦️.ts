import type { StoragePort } from "@semio-tech/ui/modules/storage-port";
import { type UiTheme, parseUiTheme, serializeUiTheme } from "@semio-tech/ui-styling";

/** 🔑️ Storage key for the active theme id (builtin or `custom.<slug>`). */
export const UI_CHROME_THEME_ID_STORAGE_KEY = "ui.chrome.theme";

/** 📖️ Reads the persisted active theme id from the given shell's storage, if any. */
export function readStoredUiChromeThemeId(storage: StoragePort): string | null {
  return storage.get(UI_CHROME_THEME_ID_STORAGE_KEY);
}

/** 📝️ Persists the active theme id to the given shell's storage. */
export function writeStoredUiChromeThemeId(storage: StoragePort, id: string): void {
  storage.set(UI_CHROME_THEME_ID_STORAGE_KEY, id);
}

/** 🗝️ Storage key for a full snapshot of the active theme (boot-time fallback before builtin/custom lookup resolves). */
export const UI_CHROME_THEME_SNAPSHOT_STORAGE_KEY = "ui.chrome.theme.snapshot";

/** 💾️ Reads the persisted active theme snapshot from the given shell's storage; discards it silently if invalid. */
export function readStoredUiChromeThemeSnapshot(storage: StoragePort): UiTheme | null {
  const raw = storage.get(UI_CHROME_THEME_SNAPSHOT_STORAGE_KEY);
  if (!raw) return null;
  try {
    return parseUiTheme(JSON.parse(raw));
  } catch {
    return null;
  }
}

/** 📸️ Persists a full snapshot of the active theme to the given shell's storage. */
export function writeStoredUiChromeThemeSnapshot(storage: StoragePort, theme: UiTheme): void {
  storage.set(UI_CHROME_THEME_SNAPSHOT_STORAGE_KEY, serializeUiTheme(theme));
}

/** 🎨️ Storage key for the user's saved custom themes, keyed by theme id. */
export const UI_CUSTOM_THEMES_STORAGE_KEY = "ui.themes.custom";

/** 🗃️ Reads the user's saved custom themes from the given shell's storage; discards any entry that fails to parse. */
export function readStoredUiCustomThemes(storage: StoragePort): Record<string, UiTheme> {
  const raw = storage.get(UI_CUSTOM_THEMES_STORAGE_KEY);
  if (!raw) return {};
  try {
    const parsed = JSON.parse(raw) as Record<string, unknown>;
    const out: Record<string, UiTheme> = {};
    for (const [id, value] of Object.entries(parsed)) {
      try {
        out[id] = parseUiTheme(value);
      } catch {}
    }
    return out;
  } catch {
    return {};
  }
}

/** 🗄️ Persists the user's saved custom themes to the given shell's storage. */
export function writeStoredUiCustomThemes(storage: StoragePort, themes: Record<string, UiTheme>): void {
  storage.set(UI_CUSTOM_THEMES_STORAGE_KEY, JSON.stringify(themes));
}

/** 🧵️ Storage key for WASM compute worker thread count (`ui.chrome.*` namespace). */
export const UI_COMPUTE_WORKER_COUNT_STORAGE_KEY = "ui.compute.workerCount";

/** 🐝️ Default compute workers: `navigator.hardwareConcurrency` or 1. */
export function defaultComputeWorkerCount(): number {
  if (typeof navigator !== "undefined" && typeof navigator.hardwareConcurrency === "number" && navigator.hardwareConcurrency > 0) {
    return navigator.hardwareConcurrency;
  }
  return 1;
}

/** 🏪️ Reads persisted compute worker count from the given shell's storage. */
export function readStoredComputeWorkerCount(storage: StoragePort): number {
  const raw = storage.get(UI_COMPUTE_WORKER_COUNT_STORAGE_KEY);
  if (raw == null || raw === "") return defaultComputeWorkerCount();
  const parsed = Number.parseInt(raw, 10);
  if (!Number.isFinite(parsed) || parsed < 1) return defaultComputeWorkerCount();
  return parsed;
}

/** ✍️ Persists compute worker count to the given shell's storage. */
export function writeStoredComputeWorkerCount(storage: StoragePort, count: number): void {
  storage.set(UI_COMPUTE_WORKER_COUNT_STORAGE_KEY, String(Math.max(1, Math.floor(count))));
}

/** 🏝️ True when SharedArrayBuffer thread pools are available. */
export function isCrossOriginIsolatedRuntime(): boolean {
  return typeof crossOriginIsolated !== "undefined" && crossOriginIsolated === true;
}

/** 👷️ Effective worker count after cross-origin isolation fallback. */
export function effectiveComputeWorkerCount(storage: StoragePort, requested = readStoredComputeWorkerCount(storage)): number {
  if (!isCrossOriginIsolatedRuntime()) return 1;
  return Math.max(1, Math.floor(requested));
}

/** 🎓️ Storage key prefix for whether an app's introduction has already been shown on this device. */
export const UI_INTRODUCTION_SEEN_STORAGE_KEY_PREFIX = "ui.introduction.seen.";

/** 👀️ Reads whether `appId`'s introduction has already been shown — auto-start checks this once
 * per app; replaying stays available via the `startIntroduction` action regardless of this flag. */
export function readStoredIntroductionSeen(storage: StoragePort, appId: string): boolean {
  if (!appId) return false;
  return storage.get(`${UI_INTRODUCTION_SEEN_STORAGE_KEY_PREFIX}${appId}`) === "true";
}

/** ✔️ Marks `appId`'s introduction as shown so it stops auto-starting on future launches. */
export function writeStoredIntroductionSeen(storage: StoragePort, appId: string): void {
  if (!appId) return;
  storage.set(`${UI_INTRODUCTION_SEEN_STORAGE_KEY_PREFIX}${appId}`, "true");
}
