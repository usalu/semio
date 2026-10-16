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

//#region 🔑️StorageKeys

/// 🔑️ Byte-identical to `UI_TERMINOLOGY_NATIVE` (`ui/js/react/index.tsx:2183`).
const UI_TERMINOLOGY_NATIVE: &str = "native";
//#endregion 🔑️StorageKeys

//#region 🗄️PrefsStore

#[cfg(all(not(target_arch = "wasm32"), test))]
static PREFS_STORE: std::sync::OnceLock<std::sync::Mutex<std::cell::RefCell<FilePrefsStore>>> = std::sync::OnceLock::new();

//#endregion 🗄️PrefsStore

//#region 🚪️StorageDoor

/// 🗄️ wasm storage hook — the twin of `semioWgpuSetHostAppearance`. The frame Worker owns no
/// `localStorage`, so the page reads the carried census and hands it here with the boot descriptor, and
/// again whenever another document on this origin rewrites one of those keys.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(js_name = semioWgpuSetHostStorage)]
pub fn semio_wgpu_set_host_storage(snapshot_json: String) {
    let accepted = seed_host_storage(&snapshot_json);
    debug_log(&format!("[TRACE] wgpu-shell host storage seeded entries={accepted}"));
}
//#endregion 🚪️StorageDoor

//#region 🛰️AgentBridgeOfferScope

/// 🛰️ The scope the page's offer watcher asks the supervisor with (`🔗️AgentBridge/🛰️offer` `agentBridgeOfferScopeFromJsonV1`):
/// `{hubOrigin, spaceId, agentPrincipalIds}` or `null` — read-only, and nothing else of the session crosses.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(js_name = dumpAgentBridgeOfferScope)]
pub fn dump_agent_bridge_offer_scope() -> String {
    PUBLISHED_AGENT_BRIDGE_OFFER_SCOPE.with(|slot| slot.borrow().clone())
}
//#endregion 🛰️AgentBridgeOfferScope

//#region 🌉️AgentBridgeConfigDoor

/// 🌉️ wasm bridge-config hook. An empty url CLEARS the config (React's `config: null`), which retires
/// the socket and parks the bridge back in `Disabled`.
///
/// 🔒️ The refusal names the FAULT, never the proof.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(js_name = semioWgpuSetAgentBridgeConfig)]
pub fn semio_wgpu_set_agent_bridge_config(url: String, admission_proof: String) {
    let config = if url.trim().is_empty() {
        None
    } else {
        match crate::agent_bridge::AgentBridgeConfig::admit(&url, &admission_proof) {
            Ok(config) => Some(config),
            Err(error) => {
                debug_log(&format!("[TRACE] wgpu-shell agent bridge config refused: {error}"));
                return;
            }
        }
    };
    let armed = config.is_some();
    PENDING_AGENT_BRIDGE_CONFIG.with(|slot| *slot.borrow_mut() = Some(config));
    debug_log(&format!("[TRACE] wgpu-shell agent bridge config armed={armed}"));
}

//#endregion 🌉️AgentBridgeConfigDoor


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

/// 🧪️ WGPU-RENDERER-REACT-PARITY packet W1j — transient notice, agent approvals modal, agent
/// presence in the chat panel header, and the tooltip-delay alignment.

/// 🧪️ Design §20.12 — app fault notices told from the structured guest refusal, mirrored as the polite status.
//#endregion 🧪️ChromeOverlaysAndTourTests

//#endregion ShellChrome
