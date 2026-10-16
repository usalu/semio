//! 🔑️ Storage doors, preferences, themes registry and chrome i18n.
//!
//! Storage keys and doors, the prefs store, pref locks, `ChromePrefsState`, the theme registry, the chrome i18n
//! string table, the UI preferences snapshot and event log and the prefs-sync `impl ShellState`.
//!
//! One chunk of the former `shell` monolith (ticket CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT); [`ShellState`] and the
//! shared imports stay in the parent module and are reached through `use super::*;`.

use super::*;

//#region ShellChrome
// #region 💾️🎨️🌐️ UiPrefsThemesI18n
// WP14: uiPrefs persistence (byte-identical localStorage keys to `ui/js/react/index.tsx`
// :2100-2318 so both renderers share prefs on one browser origin), `SEMIO_LOCKED_*` pref locks
// (mirrors `os-shell.tsx`'s `FrameworkOsLocks`), a named/custom theme registry + minimal draft
// editor, and an EN/DE chrome-string bundle keyed to match `ui/js/react/index.tsx`'s
// `uiChromeTranslationBundles` (:2898-3975). Additive-only new region: shares `shell::ShellChrome`
// with `w3-overlays-chrome-polish` (tooltips/dialogs/tour/cursor/ribbon) — this region touches
// neither; it only adds new items after the last existing method.

//#region 🎨️ThemeRegistry

//#endregion 🎨️ThemeRegistry


//#region 💾️PrefsSync

//#endregion 💾️PrefsSync

//#region 🧪️UiPrefsThemesI18nTests
//#endregion 🧪️UiPrefsThemesI18nTests
// #endregion 💾️🎨️🌐️ UiPrefsThemesI18n

//#region 🗄️BrowserPrefsPersistenceTests
/// 🧪️ WGPU-RENDERER-REACT-PARITY packet W5a — the page-serviced preference door, its key census
/// against React's own, the boot snapshot and the encodings both renderers share.
//#endregion 🗄️BrowserPrefsPersistenceTests

//#region 🚪️HostDoorRemainderTests
/// 🧪️ WGPU-RENDERER-REACT-PARITY packet W15f — the host/boot-door remainder: the platform door behind
/// `command_host_platform`, the undo/redo chord gate, OS file-drop parity (by absence on both
/// renderers), `namedLayouts`/`windowPanes` persistence, the brand-scoped introduction key, and the
/// palette's arg-carrying command routes.
#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../🧪️tests/🚪️wgpu-host-door-remainder/🦀️.rs"]
mod host_door_remainder_tests;
//#endregion 🚪️HostDoorRemainderTests

//#region 🎓️TourOverlayAndChordGlyphTests
#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../🧪️tests/🎓️tour-overlay-and-chord-glyphs/🦀️.rs"]
mod tour_overlay_and_chord_glyph_tests;
//#endregion 🎓️TourOverlayAndChordGlyphTests

//#region 🧪️ChromeOverlaysAndTourTests
//#endregion 🧪️ChromeOverlaysAndTourTests

//#endregion ShellChrome
