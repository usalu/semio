/** ♿️ The platform preferences of the React shell: the web reports forced colours (high contrast), more contrast, dark scheme and reduced motion as media queries, and this module turns them into the same four values the native shell hands a plugin's `platform_prefs_changed` hook (`ShellPlatformPrefs`, `🛂️manifest/🧱️app-surface/📱️shell-hooks/🧬️schema`), reports them once per change like the Rust `ShellHookDriver::note_platform`, and states them on the shell root where the stylesheet (`PlatformPrefs` region) renders them: reduced motion stills the transitions, more contrast thickens the focus ring and the focus cross of the focused grip.
 *
 * 🧫️ The language-agnostic fixture `🐚️Shell/🧪️tests/🧫️fixtures/🎛️platform-prefs/🔣️.json` drives this module, the wgpu browser host (`🧊️wgpu/🧭️boot-descriptor`) and the Rust driver. */
import { useEffect } from "../../⚛️react-port/🟦️.ts";

/** 🎛️ Mirrors the Rust `ShellPlatformPrefs` (camelCase on the wire). */
export type ShellPlatformPrefsV1 = Readonly<{ forcedColors: boolean; prefersContrastMore: boolean; prefersDark: boolean; prefersReducedMotion: boolean }>;

/** 🎛️ The four media queries; `forced-colors` is the web's name for Windows high contrast and the Linux high-contrast themes. */
export const PLATFORM_PREFS_MEDIA_QUERIES_V1 = {
  forcedColors: "(forced-colors: active)",
  prefersContrastMore: "(prefers-contrast: more)",
  prefersDark: "(prefers-color-scheme: dark)",
  prefersReducedMotion: "(prefers-reduced-motion: reduce)",
} as const satisfies Record<keyof ShellPlatformPrefsV1, string>;

type MatchMediaViewV1 = Readonly<{ matchMedia?: (query: string) => MediaQueryList }>;

/** 🎛️ What the view reports right now; a view without `matchMedia` reports none of the four. */
export function resolvePlatformPrefsV1(view: MatchMediaViewV1): ShellPlatformPrefsV1 {
  const asked = (query: string): boolean => view.matchMedia?.(query).matches === true;
  return {
    forcedColors: asked(PLATFORM_PREFS_MEDIA_QUERIES_V1.forcedColors),
    prefersContrastMore: asked(PLATFORM_PREFS_MEDIA_QUERIES_V1.prefersContrastMore),
    prefersDark: asked(PLATFORM_PREFS_MEDIA_QUERIES_V1.prefersDark),
    prefersReducedMotion: asked(PLATFORM_PREFS_MEDIA_QUERIES_V1.prefersReducedMotion),
  };
}

/** 🔔️ Records the reports and says whether one differs from the last (the first always does), like `ShellHookDriver::note_platform`. */
export function createPlatformPrefsDriverV1(): Readonly<{ note: (prefs: ShellPlatformPrefsV1) => boolean; forget: () => void }> {
  let last: ShellPlatformPrefsV1 | undefined;
  return {
    note: (prefs) => {
      if (last !== undefined && last.forcedColors === prefs.forcedColors && last.prefersContrastMore === prefs.prefersContrastMore && last.prefersDark === prefs.prefersDark && last.prefersReducedMotion === prefs.prefersReducedMotion) return false;
      last = prefs;
      return true;
    },
    forget: () => {
      last = undefined;
    },
  };
}

export type PlatformPrefsAttributesV1 = Readonly<{ "data-forced-colors": "active" | null; "data-contrast": "more" | null; "data-reduced-motion": "reduce" | null; "data-color-scheme": "light" | "dark" }>;

/** 🎨️ The attributes the shell root states: forced colours imply more contrast; the colour scheme is always stated. */
export function platformPrefsAttributesV1(prefs: ShellPlatformPrefsV1): PlatformPrefsAttributesV1 {
  return {
    "data-forced-colors": prefs.forcedColors ? "active" : null,
    "data-contrast": prefs.forcedColors || prefs.prefersContrastMore ? "more" : null,
    "data-reduced-motion": prefs.prefersReducedMotion ? "reduce" : null,
    "data-color-scheme": prefs.prefersDark ? "dark" : "light",
  };
}

/** 🎨️ Writes the attributes on `root`; an absent preference removes its attribute. */
export function applyPlatformPrefsV1(root: Element, prefs: ShellPlatformPrefsV1): void {
  for (const [name, value] of Object.entries(platformPrefsAttributesV1(prefs))) {
    if (value === null) root.removeAttribute(name);
    else root.setAttribute(name, value);
  }
}

/** 👂️ Reports the current preferences to `onChange` at once and again on every change of any of the four queries (the driver silences a repeat); answers the function that stops listening. */
export function watchPlatformPrefsV1(view: MatchMediaViewV1, onChange: (prefs: ShellPlatformPrefsV1) => void): () => void {
  const driver = createPlatformPrefsDriverV1();
  const report = (): void => {
    const prefs = resolvePlatformPrefsV1(view);
    if (driver.note(prefs)) onChange(prefs);
  };
  const lists = Object.values(PLATFORM_PREFS_MEDIA_QUERIES_V1).flatMap((query) => (view.matchMedia === undefined ? [] : [view.matchMedia(query)]));
  for (const list of lists) list.addEventListener("change", report);
  report();
  return () => {
    for (const list of lists) list.removeEventListener("change", report);
  };
}

/** ⚛️ Keeps the platform preferences stated on the shell root for as long as it is mounted. */
export function useShellPlatformPrefs(rootRef: Readonly<{ current: Element | null }>, attached: unknown): void {
  useEffect(() => {
    const root = rootRef.current;
    if (root === null || typeof window === "undefined") return undefined;
    return watchPlatformPrefsV1(window, (prefs) => applyPlatformPrefsV1(root, prefs));
  }, [rootRef, attached]);
}
