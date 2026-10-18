import { ephemeralBox, ephemeralSet, ephemeralMap } from "@semio-tech/framework/transient";
import { type StylingAppearanceName, STYLING_SEMIO_THEME } from "../../🤖️generated/🔤️tokens/🟦️.ts";
import type { UiTheme } from "../🏛️model/🟦️.ts";

export const _stylingAppearanceRoot = ephemeralBox<HTMLElement | null>("framework.modules.ui.styling.packages.typescript.index.ts._stylingAppearanceRoot", null);
export const _stylingAppearanceRootSubscribers = ephemeralSet<() => void>("framework.modules.ui.styling.packages.typescript.index.ts._stylingAppearanceRootSubscribers");

/** 🌓️ The registered appearance root while it is still in the document, else `documentElement`. */
export function stylingAppearanceRootElement(): HTMLElement | null {
  if (typeof document === "undefined") {
    return null;
  }
  const registered = _stylingAppearanceRoot.current;
  return registered?.isConnected ? registered : document.documentElement;
}

/** 🌓️ Resolves the active styling appearance name from the registered appearance root. */
export function currentStylingAppearanceName(): StylingAppearanceName {
  if (stylingAppearanceRootElement()?.classList.contains("dark")) {
    return "dark";
  }
  return "light";
}

//#region 🔑️Premades
/** 🎨️ The default "semio" theme, built from 🔣️.json at generate time. */
export function semioTheme(): UiTheme {
  return STYLING_SEMIO_THEME as unknown as UiTheme;
}
//#endregion 🔑️Premades

export const _activeUiTheme = ephemeralBox<UiTheme | undefined>("framework.modules.ui.styling.packages.typescript.index.ts._activeUiTheme", undefined);
export const _activeUiThemeSubscribers = ephemeralSet<(theme: UiTheme) => void>("framework.modules.ui.styling.packages.typescript.index.ts._activeUiThemeSubscribers");
/** 🐚️ Per-root applied CSS var names — lets N co-mounted shells each carry their own theme's tokens
 * without clobbering each other's `<div>` inline overrides (only `document.documentElement` is also
 * "the page", for the single page-owning shell / `setActiveUiTheme` callers). */
export const _appliedThemeCssPropsByRoot = ephemeralMap<HTMLElement, Set<string>>("framework.modules.ui.styling.packages.typescript.index.ts._appliedThemeCssPropsByRoot");

/** 🎨️ The currently active theme (defaults to semio before any theme is set). */
export function activeUiTheme(): UiTheme {
  return _activeUiTheme.current ?? semioTheme();
}

/** 🎨️ Registers a callback invoked whenever the active theme changes. Returns an unsubscribe function. */
export function subscribeActiveUiTheme(callback: (theme: UiTheme) => void): () => void {
  _activeUiThemeSubscribers.add(callback);
  return () => _activeUiThemeSubscribers.delete(callback);
}
