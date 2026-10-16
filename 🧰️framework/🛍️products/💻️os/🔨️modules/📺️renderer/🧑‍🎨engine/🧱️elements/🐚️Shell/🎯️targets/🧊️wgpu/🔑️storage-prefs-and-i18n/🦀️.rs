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
/// 🔑️ Byte-identical to `UI_INTRODUCTION_SEEN_STORAGE_KEY_PREFIX` (`ui/js/react/index.tsx:2305`).
pub(crate) const UI_INTRODUCTION_SEEN_STORAGE_KEY_PREFIX: &str = "ui.introduction.seen.";

/// 🔑️ Byte-identical to `UI_TERMINOLOGY_NATIVE` (`ui/js/react/index.tsx:2183`).
const UI_TERMINOLOGY_NATIVE: &str = "native";
//#endregion 🔑️StorageKeys

//#region 🗄️PrefsStore

/// 🗄️ Cross-platform key-value persistence for uiPrefs, over React's OWN key spellings. The native
/// backend is a bounded file page per key; the browser backend is the page-serviced [`🚪️StorageDoor`]
/// below. No new dependency either way (`serde_json` is already a dep) — zero-touch across
/// devcontainer/win/mac/linux.
///
/// 🩸️ What the browser backend replaces: a `WebLocalStorage` shim that resolved `localStorage` through
/// `web_sys::window()`. The wgpu shell runs inside the frame Worker, whose realm owns NO `window`, so
/// the shim silently resolved to `None` and EVERY `prefs_get`/`prefs_set` on the browser build was a
/// no-op — appearance, locale, terminology, themes, custom drivers, keybinding overrides, the compute
/// worker count, the dock skeleton and `ui.introduction.seen.*` all read as empty and written to
/// nothing (`📓️w4a-boot-appearance-and-tour.md` §6, hand-off 1).
#[cfg(test)]
trait PrefsStore {
    fn get(&self, key: &str) -> Option<String>;
    fn set(&mut self, key: &str, value: &str);
}

#[cfg(all(not(target_arch = "wasm32"), test))]
struct FilePrefsStore {
    pub(crate) path: std::path::PathBuf,
    cache: std::cell::RefCell<HashMap<String, String>>,
    pending_load: std::cell::RefCell<Option<std::sync::mpsc::Receiver<HashMap<String, String>>>>,
    pub(crate) dirty: std::cell::Cell<bool>,
    flush_state: std::sync::Arc<std::sync::Mutex<FilePrefsFlushState>>,
}

#[cfg(all(not(target_arch = "wasm32"), test))]
#[derive(Default)]
struct FilePrefsFlushState {
    pub(crate) latest: Option<HashMap<String, String>>,
    pub(crate) running: bool,
}

#[cfg(test)]
const OS_SHELL_PREFS_FILE_MAX_BYTES: usize = OS_SHELL_CONFIG_MAX_BYTES + 4 * 1024;

#[cfg(all(not(target_arch = "wasm32"), test))]
impl FilePrefsStore {
    pub(crate) fn new() -> Self {
        Self::with_path(native_prefs_file_path())
    }

    fn with_path(path: std::path::PathBuf) -> Self {
        let (sender, receiver) = std::sync::mpsc::channel();
        let load_path = path.clone();
        crate::renderer_worker_pool().submit(
            Lane::Io,
            Box::new(move || {
                use std::fs as system_fs;
                use std::io::Read;
                let cache = system_fs::File::open(load_path)
                    .ok()
                    .filter(|file| file.metadata().ok().is_some_and(|metadata| metadata.len() <= OS_SHELL_PREFS_FILE_MAX_BYTES as u64))
                    .and_then(|file| {
                        let mut raw = String::new();
                        file.take((OS_SHELL_PREFS_FILE_MAX_BYTES + 1) as u64).read_to_string(&mut raw).ok()?;
                        (raw.len() <= OS_SHELL_PREFS_FILE_MAX_BYTES).then_some(raw)
                    })
                    .and_then(|raw| serde_json::from_str::<HashMap<String, String>>(&raw).ok())
                    .unwrap_or_default();
                let _ = sender.send(cache);
            }),
        );
        Self { path, cache: std::cell::RefCell::new(HashMap::new()), pending_load: std::cell::RefCell::new(Some(receiver)), dirty: std::cell::Cell::new(false), flush_state: std::sync::Arc::new(std::sync::Mutex::new(FilePrefsFlushState::default())) }
    }

    fn poll_load(&self) {
        let result = self.pending_load.borrow().as_ref().map(std::sync::mpsc::Receiver::try_recv);
        match result {
            Some(Ok(loaded)) => {
                self.pending_load.borrow_mut().take();
                if !self.dirty.get() {
                    *self.cache.borrow_mut() = loaded;
                }
            }
            Some(Err(std::sync::mpsc::TryRecvError::Disconnected)) => {
                self.pending_load.borrow_mut().take();
            }
            Some(Err(std::sync::mpsc::TryRecvError::Empty)) | None => {}
        }
    }

    fn flush(&self) {
        let mut state = self.flush_state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        state.latest = Some(self.cache.borrow().clone());
        if state.running {
            return;
        }
        state.running = true;
        drop(state);
        let path = self.path.clone();
        let flush_state = std::sync::Arc::clone(&self.flush_state);
        crate::renderer_worker_pool().submit(
            Lane::Io,
            Box::new(move || loop {
                let snapshot = {
                    let mut state = flush_state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                    match state.latest.take() {
                        Some(snapshot) => snapshot,
                        None => {
                            state.running = false;
                            break;
                        }
                    }
                };
                use std::fs as system_fs;
                if let Some(parent) = path.parent() {
                    let _ = system_fs::create_dir_all(parent);
                }
                if let Ok(json) = serde_json::to_string_pretty(&snapshot) {
                    let _ = system_fs::write(&path, json);
                }
            }),
        );
    }
}

#[cfg(all(not(target_arch = "wasm32"), test))]
impl PrefsStore for FilePrefsStore {
    fn get(&self, key: &str) -> Option<String> {
        self.poll_load();
        self.cache.borrow().get(key).cloned()
    }

    fn set(&mut self, key: &str, value: &str) {
        self.poll_load();
        self.cache.borrow_mut().insert(key.to_string(), value.to_string());
        self.dirty.set(true);
        self.flush();
    }
}

#[cfg(all(not(target_arch = "wasm32"), test))]
static PREFS_STORE: std::sync::OnceLock<std::sync::Mutex<std::cell::RefCell<FilePrefsStore>>> = std::sync::OnceLock::new();

#[cfg(test)]
fn prefs_get_from(store: &impl PrefsStore, key: &str) -> Option<String> {
    let raw = store.get(OS_SHELL_CONFIG_STORAGE_KEY)?;
    if raw.len() > OS_SHELL_CONFIG_MAX_BYTES {
        return None;
    }
    serde_json::from_str::<Value>(&raw).ok()?.get("preferences")?.get(key)?.as_str().map(ToOwned::to_owned)
}

#[cfg(test)]
fn prefs_set_in(store: &mut impl PrefsStore, key: &str, value: &str) {
    let mut config = store
        .get(OS_SHELL_CONFIG_STORAGE_KEY)
        .filter(|raw| raw.len() <= OS_SHELL_CONFIG_MAX_BYTES)
        .and_then(|raw| serde_json::from_str::<Value>(&raw).ok())
        .filter(|config| config.get("version").and_then(Value::as_u64) == Some(1))
        .unwrap_or_else(empty_os_shell_config);
    if !config.get("preferences").is_some_and(Value::is_object) {
        config["preferences"] = serde_json::json!({});
    }
    config["preferences"][key] = Value::String(value.to_string());
    if let Ok(raw) = serde_json::to_string(&config) {
        if raw.len() <= OS_SHELL_CONFIG_MAX_BYTES {
            store.set(OS_SHELL_CONFIG_STORAGE_KEY, &raw);
        }
    }
}

//#endregion 🗄️PrefsStore

//#region 🚪️StorageDoor

/// 📏️ Ceiling on the whole boot snapshot — it is structured-cloned into this isolate before the first
/// frame, so it is priced like any other boot payload instead of growing with the origin's store.
pub const HOST_STORAGE_SNAPSHOT_MAX_BYTES: usize = 128 * 1024;

/// 🗄️ Reads one boot snapshot — the page's `{key: value}` read of the carried census. Uncarried keys
/// and oversized values are DROPPED rather than failing the whole snapshot: one unreadable preference
/// must not cost a boot every other preference it could have served.
pub fn decode_host_storage_snapshot(snapshot_json: &str) -> BTreeMap<String, String> {
    let mut entries = BTreeMap::new();
    if snapshot_json.len() > HOST_STORAGE_SNAPSHOT_MAX_BYTES {
        return entries;
    }
    let Ok(Value::Object(object)) = serde_json::from_str::<Value>(snapshot_json) else { return entries };
    let mut bytes = 0usize;
    for (key, value) in object {
        let Some(text) = value.as_str() else { continue };
        if !host_storage_carries_key(&key) || text.len() > HOST_STORAGE_VALUE_MAX_BYTES {
            continue;
        }
        bytes += key.len() + text.len();
        if bytes > HOST_STORAGE_SNAPSHOT_MAX_BYTES {
            break;
        }
        entries.insert(key, text.to_string());
    }
    entries
}

thread_local! {
    pub(super) static HOST_STORAGE: std::cell::RefCell<BTreeMap<String, String>> = const { std::cell::RefCell::new(BTreeMap::new()) };
}

/// 🗄️ Installs the page's read of the carried census as this isolate's synchronous store, replacing
/// whatever it held: the page's store IS the authority, and the only re-seed after boot is a `storage`
/// event, which fires for OTHER documents on the origin only — this isolate's own writes went out
/// through the door and are already in both halves. Answers how many entries it accepted.
pub fn seed_host_storage(snapshot_json: &str) -> usize {
    let entries = decode_host_storage_snapshot(snapshot_json);
    let accepted = entries.len();
    HOST_STORAGE.with(|store| *store.borrow_mut() = entries);
    accepted
}

/// 🗄️ The remover of React's `StoragePort` trio, mirrored whole so the door is the port rather than a
/// subset of it. No shell lane deletes a preference today — React's own `OsShellConfig.reset()` has no
/// wgpu twin, because the ephemeral brand does not (`📓️w1d` gap) — so this is the door's surface, not
/// a live path.
pub fn host_storage_remove(key: &str) {
    if !host_storage_carries_key(key) {
        return;
    }
    HOST_STORAGE.with(|store| store.borrow_mut().remove(key));
    host_storage_write_through(StorageDoorVerb::Remove, key, None);
}

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

/// ⏱️ How often a standing scope re-reads its delegations — the offer watcher's own maximum discovery interval
/// (`BRIDGE_DISCOVERY_MAX_INTERVAL_MS`), so a revoked or expired delegation leaves the scope within one poll of it.
pub const AGENT_BRIDGE_OFFER_SCOPE_REFRESH_MS: f64 = 30_000.0;

thread_local! {
    pub(super) static PUBLISHED_AGENT_BRIDGE_OFFER_SCOPE: std::cell::RefCell<String> = std::cell::RefCell::new("null".to_string());
}

/// 🛰️ The scope the page's offer watcher asks the supervisor with (`🔗️AgentBridge/🛰️offer` `agentBridgeOfferScopeFromJsonV1`):
/// `{hubOrigin, spaceId, agentPrincipalIds}` or `null` — read-only, and nothing else of the session crosses.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(js_name = dumpAgentBridgeOfferScope)]
pub fn dump_agent_bridge_offer_scope() -> String {
    PUBLISHED_AGENT_BRIDGE_OFFER_SCOPE.with(|slot| slot.borrow().clone())
}
//#endregion 🛰️AgentBridgeOfferScope

//#region 🌉️AgentBridgeConfigDoor
/// 🔗️ The one config slot a supervisor may fill before (or after) the shell exists — packet W15e.
///
/// ⚖️ Why a slot and not a boot-descriptor axis: bridge admission is a PROTECTED in-memory value the
/// local supervisor hands over, never a url/`<meta>`/env carrier — React makes exactly that call in
/// `discoverAgentBridgeConfig`'s own docstring and returns `null` unconditionally rather than reading
/// `VITE_SEMIO_BRIDGE_*`. Putting it on the boot descriptor would put it in the address bar and in
/// every referrer; putting it here keeps it where React keeps it.
#[cfg(target_arch = "wasm32")]
thread_local! {
    pub(super) static PENDING_AGENT_BRIDGE_CONFIG: std::cell::RefCell<Option<Option<crate::agent_bridge::AgentBridgeConfig>>> = const { std::cell::RefCell::new(None) };
}

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

#[cfg(target_arch = "wasm32")]
thread_local! {
    pub(super) static CHROME_PREFS: std::cell::RefCell<Option<ChromePrefsState>> = std::cell::RefCell::new(None);
}

#[cfg(test)]
pub(crate) fn active_theme_id() -> String {
    with_chrome_prefs(|prefs| prefs.theme_id.clone())
}

#[cfg(test)]
pub(crate) fn active_ui_layout() -> String {
    with_chrome_prefs(|prefs| prefs.ui_layout.clone())
}

#[cfg(all(test, not(target_arch = "wasm32")))]
pub(crate) fn set_active_ui_layout(layout: &str) {
    let value = if layout == "tablet" { "tablet" } else { "desktop" };
    with_chrome_prefs(|prefs| prefs.ui_layout = value.to_string());
}

#[cfg(test)]
pub(crate) fn custom_theme_ids() -> Vec<String> {
    with_chrome_prefs(|prefs| prefs.custom_themes.keys().cloned().collect())
}

/// 🎨️ Resolves built-ins and canonical custom theme documents through one format.
pub fn resolve_theme_for_ids(theme_id: &str, appearance_id: &str) -> Theme {
    if let Some(theme) = document_theme_for_ids(theme_id, crate::appearance_is_dark(appearance_id)) {
        return theme;
    }
    match theme_id {
        "" | "semio" => crate::resolve_theme(appearance_id),
        "mono" => Theme::mono(crate::appearance_is_dark(appearance_id)),
        _ => crate::resolve_theme(appearance_id),
    }
}
//#endregion 🎨️ThemeRegistry


//#region 💾️PrefsSync

/// 🗣️ The ONE locale-resolution law, mirroring `🏛️ShellHost/🟦️.tsx`'s
/// `locks?.locale ?? readUiPreferences(storage).locale ?? detectShellLocale(navigator.language)` term
/// for term — a lock wins, then the persisted preference, then the host's own language read published
/// through `semioWgpuSetHostLocale`, then the caller's already admitted locale when no preference source speaks.
/// 🌐️ Admits the host selected language and terminology before shell construction ([`resolve_language_axes`] over the
/// process's locks, its stored preferences and the host's language read).
pub(crate) fn shell_language_axes() -> Result<(Locale, Terminology), String> {
    resolve_language_axes(env_lock("SEMIO_LOCKED_LOCALE"), env_lock("SEMIO_LOCKED_TERMINOLOGY"), &read_ui_preferences(), crate::host_locale())
}

/// 🌱️ The shell's two language axes from their sources, in React's order (`🏛️ShellHost/🟦️.tsx`: locks, then
/// `resolveUiPreferences`, then its seeds): a lock wins, then the stored preference, then — on a first visit, with an empty
/// profile — the seed. The locale's seed is the host's own language read (the page negotiates it from the browser's
/// languages; no language is assumed here, so a locale nobody named refuses); the terminology's seed is React's
/// `UI_TERMINOLOGY_NATIVE`, which is no language and always answers, so a fresh profile boots. A stored terminology this
/// build does not know falls back to the seed; an unsupported lock is a configuration fault and refuses.
pub(crate) fn resolve_language_axes(locked_locale: Option<String>, locked_terminology: Option<String>, preferences: &UiPreferences, host_locale: Option<&str>) -> Result<(Locale, Terminology), String> {
    let locale_id = locked_locale
        .or_else(|| preferences.locale.map(|locale| match locale { OsUiLocale::En => "en".to_string(), OsUiLocale::De => "de".to_string() }))
        .or_else(|| host_locale.map(ToOwned::to_owned))
        .ok_or_else(|| "missing explicit shell locale authority".to_string())?;
    let terminology_id = locked_terminology.or_else(|| preferences.terminology.clone().filter(|stored| Terminology::parse(stored).is_some())).unwrap_or_else(|| Terminology::Native.as_str().to_string());
    let locale = Locale::parse(&locale_id).ok_or_else(|| "unsupported shell locale authority".to_string())?;
    let terminology = Terminology::parse(&terminology_id).ok_or_else(|| "unsupported shell terminology authority".to_string())?;
    Ok((locale, terminology))
}

//#endregion 💾️PrefsSync

//#region 🧪️UiPrefsThemesI18nTests
#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../🧪️tests/🔬️wgpu-ui-prefs-themes-i18n/🦀️.rs"]
mod ui_prefs_themes_i18n_tests;
//#endregion 🧪️UiPrefsThemesI18nTests
// #endregion 💾️🎨️🌐️ UiPrefsThemesI18n

//#region 🗄️BrowserPrefsPersistenceTests
/// 🧪️ WGPU-RENDERER-REACT-PARITY packet W5a — the page-serviced preference door, its key census
/// against React's own, the boot snapshot and the encodings both renderers share.
#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../🧪️tests/🗄️browser-prefs-persistence/🦀️.rs"]
mod browser_prefs_persistence_tests;
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
#[cfg(test)]
#[path = "../../../🧪️tests/🔬️wgpu-chrome-overlays-tour/🦀️.rs"]
mod chrome_overlays_tour_tests;

/// 🧪️ WGPU-RENDERER-REACT-PARITY packet W1j — transient notice, agent approvals modal, agent
/// presence in the chat panel header, and the tooltip-delay alignment.
#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../🧪️tests/🔬️wgpu-agent-overlays/🦀️.rs"]
mod agent_overlays_tests;

/// 🧪️ Design §20.12 — app fault notices told from the structured guest refusal, mirrored as the polite status.
#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../🧪️tests/🧪️wgpu-fault-notices/🦀️.rs"]
mod fault_notices_tests;
//#endregion 🧪️ChromeOverlaysAndTourTests

//#endregion ShellChrome
