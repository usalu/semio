// #region 🧲️Header
/// <reference types="vite/client" />
/** 🎨️ `@semio-tech/ui-styling` centralizes palette CSS, theme tokens and the shared typography preset for ui consumers.
 * Browser-only by construction: this barrel is served to the browser through `📦️packages/🟦️typescript/🟦️.ts`, so it MUST NOT
 * reach `../💨️tailwind/🟦️.ts` (the build-time Tailwind config, whose `@tailwindcss/typography` → `@tailwindcss/node` →
 * `@tailwindcss/oxide` closure ends at a `.node` binary Vite's dependency optimizer cannot load). Build tooling imports the
 * config from its declared owner (`🖱️ui/🎨️styling/💨️tailwind/🟦️.ts`) directly.
 * @see 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧹️config/🟦️.ts — "browser entry module graph" */
// #endregion 🧲️Header

export {
  STYLING_BOARD_PALETTES,
  STYLING_CANVAS_FONTS,
  STYLING_CANVAS_PALETTES,
  STYLING_MAP_PALETTES,
  STYLING_METRICS,
  STYLING_OPACITIES,
  STYLING_PRESENCE_PALETTES,
  STYLING_RADII,
  STYLING_SEMIO_THEME,
  STYLING_STROKES,
  STYLING_TOKENS,
  type StylingAppearanceName,
  type StylingTokenKey,
} from "../🤖️generated/🔤️tokens/🟦️.ts";
import { ephemeralBox } from "@semio-tech/framework/transient";
import { STYLING_BOARD_PALETTES, type StylingAppearanceName } from "../🤖️generated/🔤️tokens/🟦️.ts";

export * from "./🏛️model/🟦️.ts";
import { parseUiTheme, resolveThemeGeometry, resolveThemeAppearancePalettes, resolveThemePaint, type UiTheme } from "./🏛️model/🟦️.ts";
import { _activeUiTheme, _activeUiThemeSubscribers, _appliedThemeCssPropsByRoot, _stylingAppearanceRoot, _stylingAppearanceRootSubscribers, activeUiTheme, currentStylingAppearanceName, semioTheme } from "./🎛️active-theme/🟦️.ts";
import { clearColorResolveCache } from "./🌈️color/🟦️.ts";

/** 🌓️ Registers the element that carries the active appearance class and the appearance's
 * `--base`/`--foreground` values — a mounted shell's own `.semio-scope` root, NOT `documentElement`.
 * `applyElementsSurfaceChromeAppearanceDom` paints `.dark` on that root and leaves `documentElement`
 * on the light palette, so every canvas/WASM surface that resolves paints through this module read the
 * LIGHT palette while the shell rendered dark (measured 2026-09-14 on the generation3d React playground:
 * the flow node graph painted dark-on-cream inside a dark shell). Pass `null` to fall back to
 * `documentElement`, which IS the appearance root for a page that owns itself. */
export function setStylingAppearanceRoot(root: HTMLElement | null): void {
  if (_stylingAppearanceRoot.current === root) {
    return;
  }
  _stylingAppearanceRoot.current = root;
  clearColorResolveCache();
  for (const subscriber of _stylingAppearanceRootSubscribers) {
    subscriber();
  }
}

/** 🌓️ Notifies when the appearance root changes, so a canvas surface that mounted before the
 * shell registered its `.semio-scope` root re-observes and re-resolves instead of keeping the paints it
 * resolved against `documentElement`'s light palette. Returns an unsubscribe function. */
export function subscribeStylingAppearanceRoot(callback: () => void): () => void {
  _stylingAppearanceRootSubscribers.add(callback);
  return () => {
    _stylingAppearanceRootSubscribers.delete(callback);
  };
}

/** 🌓️ Unregisters `root` — and ONLY `root`. A page whose `documentElement` lease is released
 * moments after a shell registered its own `.semio-scope` root must not drag the live registration down
 * with it, which an unconditional `setStylingAppearanceRoot(null)` did (measured: the flow canvas
 * re-resolved the light palette ~40 s after the shell had already gone dark). */
export function clearStylingAppearanceRoot(root: HTMLElement): void {
  if (_stylingAppearanceRoot.current === root) {
    setStylingAppearanceRoot(null);
  }
}

/** 🖼️ Serializes active theme icon overrides for hosts that rasterize catalog icons at runtime. */
export function serializeThemeIconOverridesJson(): string | undefined {
  const icons = activeUiTheme().icons;
  return icons ? JSON.stringify(icons) : undefined;
}

/** 🎨️ Serializes the active theme's board palette paints for DAG/flow canvas WASM (`CanvasPalette` JSON). Falls back to the baked semio palette before a theme is set. */
export function serializeCanvasThemeJson(appearanceName: StylingAppearanceName = currentStylingAppearanceName()): string {
  if (_activeUiTheme.current) {
    return JSON.stringify(resolveThemeAppearancePalettes(_activeUiTheme.current, appearanceName).board);
  }
  return JSON.stringify(STYLING_BOARD_PALETTES[appearanceName]);
}

/** 🎨️ WASM session surface that accepts serialized canvas theme JSON. */
export interface CanvasThemeSession {
  setCanvasThemeJson(json: string): void;
}

/** 🌓️ Pushes the active theme's canvas palette into a canvas WASM session. */
export function syncSessionCanvasTheme(session: CanvasThemeSession | null | undefined): void {
  if (!session) return;
  try {
    clearColorResolveCache();
    session.setCanvasThemeJson(serializeCanvasThemeJson());
  } catch {}
}

const _builtinThemesCache = ephemeralBox<UiTheme[] | undefined>("framework.modules.ui.styling.packages.typescript.index.ts._builtinThemesCache", undefined);

/** 🎨️ Bundles the same JSON presets that the styling projection validates; standalone scripts use semio. */
export function builtinUiThemes(): readonly UiTheme[] {
  if (_builtinThemesCache.current) {
    return _builtinThemesCache.current;
  }
  const themes: UiTheme[] = [semioTheme()];
  try {
    const modules = import.meta.glob("./*.json", { eager: true, import: "default" }) as Record<string, unknown>;
    for (const raw of Object.values(modules)) {
      themes.push(parseUiTheme(raw));
    }
  } catch {}
  _builtinThemesCache.current = themes;
  return themes;
}

function setCssVar(root: HTMLElement, appliedNames: Set<string>, name: string, value: string): void {
  root.style.setProperty(name, value);
  appliedNames.add(name);
}

/** 🎨️ Applies a theme's colors/spacing/fonts/strokes/glass metrics as inline CSS var overrides on
 * `root`, clearing any previous overrides *this function* applied to that same root first. Applied
 * unconditionally (even for a pristine semio theme, whose values equal the generated CSS defaults) so a
 * semio-based *draft* with edits — which still carries `id: "semio"` until saved — is never mistaken for
 * the untouched default.
 *
 * Bounded scope: only the DOM chrome tokens below are per-root. `activeUiTheme().icons` (render-time icon
 * defaults) and the `resolveColorHex`/`resolveBackgroundColorHex` canvas-paint caches stay page-level —
 * they key off `currentStylingAppearanceName()` with no root, so a co-mounted shell's own theme correctly
 * repaints its DOM chrome (CSS vars inherit from its own root) while any canvas/GPU paint that resolves a
 * hex value still follows whichever theme was applied last, page-wide. */
export function applyUiThemeToRoot(root: HTMLElement, theme: UiTheme): void {
  const geometry = resolveThemeGeometry(theme);
  let appliedNames = _appliedThemeCssPropsByRoot.get(root);
  if (!appliedNames) {
    appliedNames = new Set();
    _appliedThemeCssPropsByRoot.set(root, appliedNames);
  }
  for (const name of appliedNames) {
    root.style.removeProperty(name);
  }
  appliedNames.clear();
  root.dataset.uiTheme = theme.id;
  for (const [key, hex] of Object.entries(theme.colors)) {
    setCssVar(root, appliedNames, `--color-${key.replaceAll("_", "-")}`, hex);
  }
  for (const appearance of ["light", "dark"] as const) {
    for (const [key, paint] of Object.entries(theme.appearances[appearance].chrome)) {
      const [red, green, blue, alpha] = resolveThemePaint(theme.colors, paint);
      const name = key.replace(/[A-Z]/g, (letter) => `-${letter.toLowerCase()}`);
      setCssVar(root, appliedNames, `--theme-${appearance}-chrome-${name}`, `rgba(${red}, ${green}, ${blue}, ${alpha / 255})`);
    }
  }
  for (const [key, value] of Object.entries(theme.spacing)) {
    setCssVar(root, appliedNames, `--spacing-${key.replaceAll("_", "-")}`, value);
  }
  for (const [name, value] of Object.entries(geometry.cssVars)) setCssVar(root, appliedNames, name, value);
  if (theme.fontStacks.sans) setCssVar(root, appliedNames, "--font-sans", theme.fontStacks.sans);
  if (theme.fontStacks.serif) setCssVar(root, appliedNames, "--font-serif", theme.fontStacks.serif);
  if (theme.fontStacks.mono) setCssVar(root, appliedNames, "--font-mono", theme.fontStacks.mono);
  const hairline = theme.strokes.chromeBorderHairline;
  if (typeof hairline === "number") setCssVar(root, appliedNames, "--stroke-hairline", `${hairline}px`);
  const chrome = theme.metrics.chrome;
  if (chrome) {
    if (typeof chrome.glassSaturate === "number") setCssVar(root, appliedNames, "--glass-saturate", `${chrome.glassSaturate}`);
    if (typeof chrome.shadeStepPercent === "number") setCssVar(root, appliedNames, "--level-shade-step", `${chrome.shadeStepPercent}%`);
    if (typeof chrome.elementStepPercent === "number") setCssVar(root, appliedNames, "--element-shade-step", `${chrome.elementStepPercent}%`);
    if (typeof chrome.hoverStepPercent === "number") setCssVar(root, appliedNames, "--hover-shade-step", `${chrome.hoverStepPercent}%`);
    if (typeof chrome.glassAlphaStep === "number") setCssVar(root, appliedNames, "--glass-alpha-step", `${chrome.glassAlphaStep}`);
    if (typeof chrome.glassBlurStepPx === "number") setCssVar(root, appliedNames, "--glass-blur-step", `${chrome.glassBlurStepPx / 16}rem`);
  }
  clearColorResolveCache();
}

/** 🎨️ Removes every CSS var {@link applyUiThemeToRoot} applied to `root` and forgets its registry
 * entry — call on a shell's unmount so a later, unrelated element reused at the same DOM position never
 * inherits a stale theme's inline overrides. */
export function clearUiThemeFromRoot(root: HTMLElement): void {
  const appliedNames = _appliedThemeCssPropsByRoot.get(root);
  if (!appliedNames) return;
  for (const name of appliedNames) {
    root.style.removeProperty(name);
  }
  delete root.dataset.uiTheme;
  _appliedThemeCssPropsByRoot.delete(root);
}

/** 🎨️ Applies a theme's colors/spacing/fonts/strokes/glass metrics as inline `documentElement` CSS
 * var overrides — the page-owning case of {@link applyUiThemeToRoot}. */
export function applyUiThemeToDocument(theme: UiTheme): void {
  if (typeof document === "undefined") {
    return;
  }
  applyUiThemeToRoot(document.documentElement, theme);
}

/** 🎨️ Sets the *page-global* active theme, applies it to `document.documentElement`, and notifies
 * subscribers — for the single page-owning shell (`ShellScope.ownsPage`) or a standalone (non-shell) host.
 * A co-mounted, non-page-owning shell must call {@link applyUiThemeToRoot} on its own root instead, or it
 * would fight every other mounted shell over the same document-wide tokens. */
export function setActiveUiTheme(theme: UiTheme): void {
  resolveThemeGeometry(theme);
  _activeUiTheme.current = theme;
  applyUiThemeToDocument(theme);
  for (const subscriber of _activeUiThemeSubscribers) {
    subscriber(theme);
  }
}