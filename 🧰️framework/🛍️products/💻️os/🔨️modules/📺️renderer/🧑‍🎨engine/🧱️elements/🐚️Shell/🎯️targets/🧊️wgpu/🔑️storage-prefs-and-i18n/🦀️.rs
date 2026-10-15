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
/// 🔑️ Byte-identical to `UI_COMPUTE_WORKER_COUNT_STORAGE_KEY` (`ui/js/react/index.tsx:2267`).
pub(crate) const UI_COMPUTE_WORKER_COUNT_STORAGE_KEY: &str = "ui.compute.workerCount";
/// 🔑️ Byte-identical to `UI_INTRODUCTION_SEEN_STORAGE_KEY_PREFIX` (`ui/js/react/index.tsx:2305`).
pub(crate) const UI_INTRODUCTION_SEEN_STORAGE_KEY_PREFIX: &str = "ui.introduction.seen.";

/// 🆔️ The introduction chrome's control ids, byte-identical to React's own
/// (`UIIntroduction`'s `close={{ id: "ui.introduction.skip" }}`, `<Button id="ui.introduction.back">`,
/// `<Button id="ui.introduction.next">`, and the keybinding rows in
/// `🖱️ui/🔨️modules/🕹️control-keybinding-context/🟦️.tsx`).
///
/// 🩸️ This renderer used to mint `shell.tour.{skip,next,back}` — ids React has never had, so a parity
/// probe driving ONE journey across both renderers had to carry an alias table and every tour step read
/// as a difference (`📓️w5b-interaction-parity-probe.md` §4.1).
pub(crate) const UI_INTRODUCTION_SKIP_CONTROL_ID: &str = "ui.introduction.skip";
pub(crate) const UI_INTRODUCTION_NEXT_CONTROL_ID: &str = "ui.introduction.next";
pub(crate) const UI_INTRODUCTION_BACK_CONTROL_ID: &str = "ui.introduction.back";
/// 🧯️ The blocking veil itself. React's scrim is a real `pointer-events-auto` element with its own
/// `onPointerDown={skip}`; giving it a control id is what lets `pointer_press_belongs_to_shell_chrome`
/// claim the press before any 3D surface under it can.
pub(crate) const UI_INTRODUCTION_VEIL_CONTROL_ID: &str = "ui.introduction.veil";
/// 🆔️ The tour as ONE dialog-level surface in the chrome census. React's `UIIntroduction` renders its
/// veil and card as several `[data-level="dialog"]` nodes (one of them anonymous), which is a DOM
/// composition, not a behaviour — a probe compares dialog PRESENCE, so one row states it.
pub(crate) const UI_INTRODUCTION_DIALOG_ELEMENT_ID: &str = "ui.introduction";
/// 🔑️ Byte-identical to `UI_TERMINOLOGY_NATIVE` (`ui/js/react/index.tsx:2183`).
const UI_TERMINOLOGY_NATIVE: &str = "native";
//#endregion 🔑️StorageKeys

//#region 🗄️PrefsStore
pub(crate) const OS_SHELL_CONFIG_STORAGE_KEY: &str = "semio.os.config";

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

pub(crate) const OS_SHELL_CONFIG_MAX_BYTES: usize = 64 * 1024;
#[cfg(test)]
const OS_SHELL_PREFS_FILE_MAX_BYTES: usize = OS_SHELL_CONFIG_MAX_BYTES + 4 * 1024;
#[cfg(not(target_arch = "wasm32"))]
static NATIVE_PREF_DOCUMENTS: std::sync::LazyLock<std::sync::Mutex<semio_framework_os_services::RetainedFixedFileDocuments>> = std::sync::LazyLock::new(|| std::sync::Mutex::new(semio_framework_os_services::RetainedFixedFileDocuments::default()));
#[cfg(not(target_arch = "wasm32"))]
static NATIVE_PREF_DOCUMENT_GENERATION: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);

/// 📁️ Resolves the native prefs file path: `$SEMIO_PREFS_DIR/ui-prefs.json` when set, else a
/// per-OS config-home fallback (XDG on linux/devcontainer, `%APPDATA%` on windows, `~/.config` on
/// macOS) — no new dependency (no `dirs` crate), just `std::env`.
#[cfg(not(target_arch = "wasm32"))]
fn native_prefs_file_path() -> std::path::PathBuf {
    if let Ok(dir) = std::env::var("SEMIO_PREFS_DIR") {
        return std::path::PathBuf::from(dir).join("ui-prefs.json");
    }
    let base = std::env::var("XDG_CONFIG_HOME").or_else(|_| std::env::var("APPDATA")).or_else(|_| std::env::var("HOME").map(|home| format!("{home}/.config"))).unwrap_or_else(|_| ".".to_string());
    std::path::PathBuf::from(base).join("semio").join("ui-prefs.json")
}

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

pub(crate) fn empty_os_shell_config() -> Value {
    serde_json::json!({
        "version": 1,
        "preferences": {},
        "dockLayouts": { "apps": {} },
        "dockUi": { "apps": {} },
        "windowPanes": { "apps": {} }
    })
}

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

#[cfg(not(target_arch = "wasm32"))]
fn native_pref_field_path(key: &str) -> Option<std::path::PathBuf> {
    if key.len() > SHELL_CHROME_IO_FIELD_BYTES {
        return None;
    }
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    key.hash(&mut hasher);
    Some(native_prefs_file_path().with_file_name("ui-prefs.fields").join(format!("{:016x}.field", hasher.finish())))
}

#[cfg(not(target_arch = "wasm32"))]
fn native_pref_read_page(key: &str) -> Option<String> {
    let path = native_pref_field_path(key)?;
    let page = if key == OS_SHELL_CONFIG_STORAGE_KEY {
        semio_framework_os_services::storage_worker_read_fixed_file_document(&path, OS_SHELL_CONFIG_MAX_BYTES).ok()?
    } else {
        semio_framework_os_services::storage_worker_read_fixed_file_page(&path, SHELL_CHROME_IO_FIELD_BYTES).ok()?
    };
    String::from_utf8(page).ok()
}

#[cfg(not(target_arch = "wasm32"))]
fn native_pref_write_page(key: &str, value: &str) -> bool {
    let Some(path) = native_pref_field_path(key) else { return false };
    if key == OS_SHELL_CONFIG_STORAGE_KEY {
        if value.len() > OS_SHELL_CONFIG_MAX_BYTES {
            return false;
        }
        let generation = NATIVE_PREF_DOCUMENT_GENERATION.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let mut documents = NATIVE_PREF_DOCUMENTS.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Ok(token) = documents.begin_write(&path, 1, generation, value.as_bytes().to_vec(), OS_SHELL_CONFIG_MAX_BYTES) else { return false };
        loop {
            match documents.write_step(token) {
                Ok(step) if step.ready_to_publish => break,
                Ok(_) => {}
                Err(_) => {
                    let _ = documents.cancel(token);
                    let _ = documents.cancel_step(token);
                    return false;
                }
            }
        }
        return match documents.publish(token) {
            Ok(()) => true,
            Err(_) => {
                let _ = documents.cancel(token);
                let _ = documents.cancel_step(token);
                false
            }
        };
    }
    semio_framework_os_services::storage_worker_write_fixed_file_page(&path, value.as_bytes(), SHELL_CHROME_IO_FIELD_BYTES).is_ok()
}

/// 🗝️ One TOP-LEVEL storage key — React's `StoragePort.get` (`🖥️platform/🟦️.ts`), the flat
/// `localStorage` entry, NOT a field of the `semio.os.config` document.
pub(crate) fn raw_prefs_get(key: &str) -> Option<String> {
    #[cfg(target_arch = "wasm32")]
    {
        host_storage_get(key)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        native_pref_read_page(key)
    }
}

/// 🗝️ React's `StoragePort.set`. On the browser build the write lands in this isolate's cache FIRST
/// and crosses the door afterwards, so a read that follows a write in the same frame answers the
/// written value — the door is asynchronous and a frame is not.
pub(crate) fn raw_prefs_set(key: &str, value: &str) {
    #[cfg(target_arch = "wasm32")]
    host_storage_set(key, value);
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = native_pref_write_page(key, value);
    }
}

/// 🗝️ One bounded top-level key, the shape every per-frame chrome reader uses. It is deliberately NOT
/// [`prefs_get`]: React keeps `ui.compute.workerCount` and `ui.introduction.seen.<appId>` as their own
/// flat entries and only `os.config.ui-preferences` inside `semio.os.config`'s `preferences` map, so a
/// renderer that nested them would answer nothing to a profile React had written.
pub(crate) fn stored_field_get(key: &str) -> Option<String> {
    if key.len() > SHELL_CHROME_IO_FIELD_BYTES {
        return None;
    }
    raw_prefs_get(key).filter(|value| value.len() <= SHELL_CHROME_IO_FIELD_BYTES)
}

/// 🗝️ The writer twin of [`stored_field_get`].
pub(crate) fn stored_field_set(key: &str, value: &str) {
    if key.len() <= SHELL_CHROME_IO_FIELD_BYTES && value.len() <= SHELL_CHROME_IO_FIELD_BYTES {
        raw_prefs_set(key, value);
    }
}

/// 🎚️ One field of the `semio.os.config` document's `preferences` map — React's
/// `OsShellConfig.getPreference` (`🖥️platform/🟦️.ts`). Its ONE key is `os.config.ui-preferences`, the
/// UI-preferences event log; every other durable key React keeps is a flat entry reached through
/// [`stored_field_get`].
pub(crate) fn prefs_get(key: &str) -> Option<String> {
    let raw = raw_prefs_get(OS_SHELL_CONFIG_STORAGE_KEY)?;
    if raw.len() > OS_SHELL_CONFIG_MAX_BYTES {
        return None;
    }
    serde_json::from_str::<Value>(&raw).ok()?.get("preferences")?.get(key)?.as_str().map(ToOwned::to_owned)
}

/// 🎚️ React's `OsShellConfig.setPreference`: re-read the complete document, apply one field, write it
/// back — so a sibling projection (`dockLayouts`, `dockUi`, `windowPanes`) is never
/// clobbered by a preference write.
pub(crate) fn prefs_set(key: &str, value: &str) {
    let mut config = raw_prefs_get(OS_SHELL_CONFIG_STORAGE_KEY)
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
            raw_prefs_set(OS_SHELL_CONFIG_STORAGE_KEY, &raw);
        }
    }
}

//#endregion 🗄️PrefsStore

//#region 🚪️StorageDoor
/// 🚪️ The browser backend of [`🗄️PrefsStore`]: one bounded storage door on the SAME page mailbox the
/// file and directory doors already use (`🚪️host-io/🟦️.ts`, `🎞️frame-worker/🟦️.ts` ⇄
/// `🚚️browser-frame-transport/🟦️.ts` ⇄ `🚀️browser-boot/🟦️.ts`), plus the synchronous cache a frame can
/// actually read from.
///
/// ⚖️ Why a cache AND a door, not just a door: `prefs_get` is called from the frame build — the shell
/// resolves appearance, `ui.introduction.seen.<appId>` and the dock skeleton while it paints — and the
/// door is a `Promise`. So the page reads the whole carried census ONCE, before the shell boots, and
/// hands it over with the boot descriptor ([`seed_host_storage`]); every later write updates this cache
/// first and crosses the door afterwards. A read that follows a write in the same frame therefore
/// answers the written value, and a boot never paints a default it has to correct a frame later.
///
/// 🔑️ Every key is React's own spelling, because both renderers share one browser origin: a host that
/// chose Light, dismissed a tour or rearranged its dock under React must find all three again under
/// this renderer. See [`HOST_STORAGE_KEY_CENSUS`].
pub const STORAGE_DOOR_OP: &str = "storage";

/// 📏️ Per-value ceiling, the same 64 KiB the shell already refuses a larger `semio.os.config` document
/// at, so the door cannot become the wider hole.
pub const HOST_STORAGE_VALUE_MAX_BYTES: usize = OS_SHELL_CONFIG_MAX_BYTES;

/// 📏️ Ceiling on the whole boot snapshot — it is structured-cloned into this isolate before the first
/// frame, so it is priced like any other boot payload instead of growing with the origin's store.
pub const HOST_STORAGE_SNAPSHOT_MAX_BYTES: usize = 128 * 1024;

/// 🔑️ What one durable browser key is to THIS renderer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HostStorageKeyKind {
    /// 🗄️ Carried: one fixed key the boot snapshot holds and the door may address.
    Carried,
    /// 🗄️ Carried FAMILY: a prefix whose tail is a runtime app id, so the page scans for members.
    CarriedFamily,
    /// 🧩️ The standalone `🖱️ui` surface library's own key (`useUiTerminology`, `UiDriver`, the locale
    /// detector). React's OS shell does not read it either — it persists through
    /// `semio.os.config`'s `os.config.ui-preferences` event log, and so does this renderer — so
    /// carrying it would be dead payload in the frame Worker.
    UiLibrary,
    /// 👥️ `sessionStorage`, and credential-bearing (a presence identity pack). Off the boot snapshot
    /// on purpose; reachable only through an explicit [`StorageDoorScope::Session`] call, which
    /// nothing makes yet because document presence is native-only (`📓️w1e` gap 2).
    Session,
}

/// 🔑️ EVERY durable key React's shell stack spells, classified. This is the parity ledger: the law
/// `the_key_census_covers_reacts_own_key_list` re-reads React's own constants out of
/// `🖱️ui/🎯️targets/⚛️react/🟦️.tsx`, `🖥️platform/🟦️.ts`, `💾️keybinding-persistence/🟦️.ts`,
/// `🚗️UiDriver/🟦️.tsx`, `🏛️ShellHost/🟦️.tsx` and `🛠️ShellHelpers/🟦️.tsx` and refuses any key this
/// table does not classify — so a key React adds cannot quietly go unserved here.
pub const HOST_STORAGE_KEY_CENSUS: &[(&str, HostStorageKeyKind)] = &[
    ("SEMIO_RUNTIME_DIAGNOSTICS", HostStorageKeyKind::Carried),
    ("semio.os.config", HostStorageKeyKind::Carried),
    ("ui.compute.workerCount", HostStorageKeyKind::Carried),
    ("ui.introduction.seen.", HostStorageKeyKind::CarriedFamily),
    ("semio.presence.client", HostStorageKeyKind::Session),
    ("ui.chrome.appearance", HostStorageKeyKind::UiLibrary),
    ("ui.chrome.driver", HostStorageKeyKind::UiLibrary),
    ("ui.chrome.layout", HostStorageKeyKind::UiLibrary),
    ("ui.chrome.locale", HostStorageKeyKind::UiLibrary),
    ("ui.chrome.terminology", HostStorageKeyKind::UiLibrary),
    ("ui.chrome.theme", HostStorageKeyKind::UiLibrary),
    ("ui.chrome.theme.snapshot", HostStorageKeyKind::UiLibrary),
    ("ui.drivers.custom", HostStorageKeyKind::UiLibrary),
    ("ui.keybindings.overrides", HostStorageKeyKind::UiLibrary),
    ("ui.themes.custom", HostStorageKeyKind::UiLibrary),
];

/// 🔑️ Whether the door and the snapshot carry `key` at all. The door is an ALLOWLIST, not a general
/// `localStorage` proxy: the shell is its only caller and its census is closed, so an unknown key is a
/// defect worth refusing loudly rather than a value worth serving.
pub fn host_storage_carries_key(key: &str) -> bool {
    HOST_STORAGE_KEY_CENSUS.iter().any(|(name, kind)| match kind {
        HostStorageKeyKind::Carried => *name == key,
        HostStorageKeyKind::CarriedFamily => key.len() > name.len() && key.starts_with(name),
        HostStorageKeyKind::UiLibrary | HostStorageKeyKind::Session => false,
    })
}

/// 🚪️ What one door call does to the addressed key — React's `StoragePort`, whole.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StorageDoorVerb {
    Get,
    Set,
    Remove,
}

impl StorageDoorVerb {
    pub fn as_id(self) -> &'static str {
        match self {
            Self::Get => "get",
            Self::Set => "set",
            Self::Remove => "remove",
        }
    }
}

/// 🗄️ Which of the browser's two stores one call addresses — React uses both, so the door speaks both.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum StorageDoorScope {
    #[default]
    Local,
    Session,
}

impl StorageDoorScope {
    pub fn as_id(self) -> &'static str {
        match self {
            Self::Local => "local",
            Self::Session => "session",
        }
    }
}

/// 📨️ Seals one storage hop for the page. Refused HERE — never sent and silently dropped there — when
/// the key is outside the census or the value outside the bound, because both are caller defects and a
/// door that answered them would be a wider hole than the store it fronts.
pub fn encode_storage_door_request(verb: StorageDoorVerb, scope: StorageDoorScope, key: &str, value: Option<&str>) -> Result<String, String> {
    if !host_storage_carries_key(key) {
        return Err(format!("storage door refuses uncarried key {key:?}"));
    }
    if value.is_some_and(|value| value.len() > HOST_STORAGE_VALUE_MAX_BYTES) {
        return Err(format!("storage door refuses {key} over {HOST_STORAGE_VALUE_MAX_BYTES} bytes"));
    }
    let mut request = serde_json::json!({ "op": STORAGE_DOOR_OP, "verb": verb.as_id(), "scope": scope.as_id(), "key": key });
    if let Some(value) = value {
        request["value"] = Value::String(value.to_string());
    }
    Ok(request.to_string())
}

/// 📬️ Reads one door answer. `Ok(None)` is "the store holds nothing under this key" and is the answer
/// a `set`/`remove` also gives; `Err` is "the call was refused". The two are different outcomes and a
/// refusal is never flattened into an empty read.
pub fn decode_storage_door_answer(answer: &str) -> Result<Option<String>, String> {
    let value: Value = serde_json::from_str(answer).map_err(|error| format!("storage door answer unreadable: {error}"))?;
    if let Some(error) = value.get("error").and_then(Value::as_str) {
        return Err(error.to_string());
    }
    match value.get("value") {
        Some(Value::String(text)) => Ok(Some(text.clone())),
        Some(Value::Null) | None => Ok(None),
        Some(other) => Err(format!("storage door answered a non-string value: {other}")),
    }
}

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
    static HOST_STORAGE: std::cell::RefCell<BTreeMap<String, String>> = const { std::cell::RefCell::new(BTreeMap::new()) };
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

/// 🗄️ One carried key, synchronously.
pub fn host_storage_get(key: &str) -> Option<String> {
    HOST_STORAGE.with(|store| store.borrow().get(key).cloned())
}

/// 🗄️ Writes one carried key into this isolate's store and then through the door to the page's. The
/// cache write lands first so the next read in this frame sees it; the door hop is fire-and-forget
/// because a preference write must never fault a surface, and a refusal is logged rather than swallowed.
pub fn host_storage_set(key: &str, value: &str) {
    if !host_storage_carries_key(key) || value.len() > HOST_STORAGE_VALUE_MAX_BYTES {
        return;
    }
    HOST_STORAGE.with(|store| store.borrow_mut().insert(key.to_string(), value.to_string()));
    host_storage_write_through(StorageDoorVerb::Set, key, Some(value));
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

#[cfg(target_arch = "wasm32")]
fn host_storage_write_through(verb: StorageDoorVerb, key: &str, value: Option<&str>) {
    let request = match encode_storage_door_request(verb, StorageDoorScope::Local, key, value) {
        Ok(request) => request,
        Err(error) => {
            ShellState::debug_log(&format!("[TRACE] wgpu-shell storage write refused: {error}"));
            return;
        }
    };
    let key = key.to_string();
    crate::spawn_app_task(async move {
        match host_io_call(&request, None).await.map_err(|error| error).and_then(|answer| decode_storage_door_answer(&answer)) {
            Ok(_) => {}
            Err(error) => ShellState::debug_log(&format!("[TRACE] wgpu-shell storage {key} not persisted: {error}")),
        }
    });
}

#[cfg(not(target_arch = "wasm32"))]
fn host_storage_write_through(_verb: StorageDoorVerb, _key: &str, _value: Option<&str>) {}

/// 🗄️ wasm storage hook — the twin of `semioWgpuSetHostAppearance`. The frame Worker owns no
/// `localStorage`, so the page reads the carried census and hands it here with the boot descriptor, and
/// again whenever another document on this origin rewrites one of those keys.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(js_name = semioWgpuSetHostStorage)]
pub fn semio_wgpu_set_host_storage(snapshot_json: String) {
    let accepted = seed_host_storage(&snapshot_json);
    ShellState::debug_log(&format!("[TRACE] wgpu-shell host storage seeded entries={accepted}"));
}
//#endregion 🚪️StorageDoor

//#region 🛰️AgentBridgeOfferScope
/// 🎯️ Who this shell is for the agent-bridge supervisor's scoped offer (ticket 26/09/23, G12 × WG11 session 14c): the hub it
/// signed in to, the space it is in and the agents ITS human delegated there — ShellHost's `offerScope`. Only these three
/// values cross into the page realm (`dumpAgentBridgeOfferScope`); the session capability that lists them never does.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentBridgeOfferScopeV1 {
    pub hub_origin: String,
    pub space_id: String,
    pub agent_principal_ids: Vec<String>,
}

/// ⏱️ How often a standing scope re-reads its delegations — the offer watcher's own maximum discovery interval
/// (`BRIDGE_DISCOVERY_MAX_INTERVAL_MS`), so a revoked or expired delegation leaves the scope within one poll of it.
pub const AGENT_BRIDGE_OFFER_SCOPE_REFRESH_MS: f64 = 30_000.0;

/// 🔁️ Keeps [`AgentBridgeOfferScopeV1`] true to the shell: a sign-out or a space switch retires the scope AT ONCE (one space's
/// agents are never offered to another), a listing answered for a target the shell has since left is dropped, and the
/// listing is re-read every [`AGENT_BRIDGE_OFFER_SCOPE_REFRESH_MS`] while the target stands.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AgentBridgeOfferScopeOwnerV1 {
    pub(crate) target: Option<(String, String)>,
    pub(crate) scope: Option<AgentBridgeOfferScopeV1>,
    next_listing_at_ms: f64,
    listing: bool,
}

impl AgentBridgeOfferScopeOwnerV1 {
    /// 🎯️ Follows the shell's current `(hubOrigin, spaceId)`; answers the target whose listing is due now, if any.
    pub fn observe(&mut self, target: Option<(String, String)>, now_ms: f64) -> Option<(String, String)> {
        if self.target != target {
            self.target = target;
            self.scope = None;
            self.listing = false;
            self.next_listing_at_ms = now_ms;
        }
        if self.listing || now_ms < self.next_listing_at_ms {
            return None;
        }
        let due = self.target.clone()?;
        self.listing = true;
        Some(due)
    }

    /// 📥️ Settles the listing asked for `target`: its live principals become the scope, a failed listing leaves no scope
    /// (local offers only, as ShellHost's rejected `offerScope`), and an answer for a target the shell has left is dropped.
    pub fn settle(&mut self, target: &(String, String), principals: Option<Vec<String>>, now_ms: f64) {
        if self.target.as_ref() != Some(target) {
            return;
        }
        self.listing = false;
        self.next_listing_at_ms = now_ms + AGENT_BRIDGE_OFFER_SCOPE_REFRESH_MS;
        self.scope = principals.map(|agent_principal_ids| AgentBridgeOfferScopeV1 { hub_origin: target.0.clone(), space_id: target.1.clone(), agent_principal_ids });
    }

    /// 🎯️ The standing scope, if the shell is signed in and in a space whose listing answered.
    pub fn scope(&self) -> Option<&AgentBridgeOfferScopeV1> {
        self.scope.as_ref()
    }

    /// 📜️ The scope as the page realm reads it: the three values, or `null`.
    pub fn page_json(&self) -> String {
        self.scope.as_ref().and_then(|scope| serde_json::to_string(scope).ok()).unwrap_or_else(|| "null".to_string())
    }
}

thread_local! {
    static PUBLISHED_AGENT_BRIDGE_OFFER_SCOPE: std::cell::RefCell<String> = std::cell::RefCell::new("null".to_string());
}

/// 📤️ Publishes the shell's scope for the page realm's offer watcher.
pub(crate) fn publish_agent_bridge_offer_scope(json: String) {
    PUBLISHED_AGENT_BRIDGE_OFFER_SCOPE.with(|slot| *slot.borrow_mut() = json);
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
    static PENDING_AGENT_BRIDGE_CONFIG: std::cell::RefCell<Option<Option<crate::agent_bridge::AgentBridgeConfig>>> = const { std::cell::RefCell::new(None) };
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
                ShellState::debug_log(&format!("[TRACE] wgpu-shell agent bridge config refused: {error}"));
                return;
            }
        }
    };
    let armed = config.is_some();
    PENDING_AGENT_BRIDGE_CONFIG.with(|slot| *slot.borrow_mut() = Some(config));
    ShellState::debug_log(&format!("[TRACE] wgpu-shell agent bridge config armed={armed}"));
}

/// 🌉️ Takes whatever the hook left, exactly once.
#[cfg(target_arch = "wasm32")]
pub(crate) fn take_pending_agent_bridge_config() -> Option<Option<crate::agent_bridge::AgentBridgeConfig>> {
    PENDING_AGENT_BRIDGE_CONFIG.with(|slot| slot.borrow_mut().take())
}

/// 🖥️ Native has no equivalent hook and deliberately no environment discovery either: a native
/// embedder holds the shell directly and calls `ShellState::set_agent_bridge_config`.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn take_pending_agent_bridge_config() -> Option<Option<crate::agent_bridge::AgentBridgeConfig>> {
    None
}
//#endregion 🌉️AgentBridgeConfigDoor

//#region 🔒️PrefLocks
/// 🔒️ `SEMIO_LOCKED_*` env-driven pref locks — mirrors `os-shell.tsx`'s `FrameworkOsLocks`
/// (`:372-378`, read via `VITE_SEMIO_LOCKED_*` in `framework/product/os/dev/js/index.ts:19-23`):
/// appearance/locale/terminology/themeId may be locked; driver/layout/customThemes/customDrivers
/// deliberately stay unlocked in React too. A locked pref skips its localStorage write. Native-only
/// in practice (wasm32-unknown-unknown has no process env at runtime, so this is always empty
/// there — kiosk/demo locking is a native `semio-wgpu-native` deployment concern).
#[derive(Clone, Debug, Default)]
#[cfg(test)]
struct ShellPrefLocks {
    pub(crate) appearance: Option<String>,
    pub(crate) locale: Option<String>,
    pub(crate) terminology: Option<String>,
    pub(crate) theme_id: Option<String>,
}

/// 🔒️ Resolves one `SEMIO_LOCKED_*` lock through the BOOT DESCRIPTOR rather than the process env.
/// The descriptor already carries the native env seeds (`resolve_environment_boot_descriptor`), so a
/// native deployment behaves exactly as before — and a BROWSER build, which has no process env at all
/// and was therefore unlockable, now locks from `<meta name="semio-locked-*">`/the library door's
/// `locks` option through the same one vocabulary React reads as `FrameworkOsLocks`.
pub(crate) fn env_lock(name: &str) -> Option<String> {
    let locks = crate::boot_locks();
    let value = match name {
        "SEMIO_LOCKED_APPEARANCE" => locks.appearance,
        "SEMIO_LOCKED_LOCALE" => locks.locale,
        "SEMIO_LOCKED_TERMINOLOGY" => locks.terminology,
        "SEMIO_LOCKED_THEME" => locks.theme_id,
        "SEMIO_LOCKED_EXAMPLE" => locks.example_id,
        _ => String::new(),
    };
    (!value.is_empty()).then_some(value)
}

#[cfg(test)]
fn shell_pref_locks() -> ShellPrefLocks {
    ShellPrefLocks { appearance: env_lock("SEMIO_LOCKED_APPEARANCE"), locale: env_lock("SEMIO_LOCKED_LOCALE"), terminology: env_lock("SEMIO_LOCKED_TERMINOLOGY"), theme_id: env_lock("SEMIO_LOCKED_THEME") }
}
//#endregion 🔒️PrefLocks

//#region 🎨️ThemeRegistry
/// 🎛️ Owned preference snapshot consumed while constructing chrome.
#[derive(Clone)]
pub(crate) struct ChromePrefsState {
    pub(crate) ui_layout: String,
    pub(crate) theme_id: String,
    pub(crate) custom_drivers: HashMap<String, OsUiDriver>,
    pub(crate) custom_themes: HashMap<String, String>,
    pub(crate) keybinding_overrides: HashMap<String, String>,
    /// 🎨️ The UNSAVED theme document the settings panel is editing, mirrored here so the present
    /// side's `resolve_theme_for_ids` re-tokenises from it — React's `uiThemeDraft` reaching
    /// `applyUiThemeToRoot`.
    pub(crate) theme_document_draft: Option<ThemeDocument>,
    /// 🎨️ Bumped on every draft/registry write, so the cache below can never answer a stale theme.
    pub(crate) theme_document_generation: u64,
    /// 🧊️ `(theme id, dark, generation) -> resolved Theme` — one document walk per change, not one
    /// per frame.
    pub(crate) theme_document_resolved: Option<((String, bool, u64), Theme)>,
    pub(crate) worker_count: u32,
}

impl Default for ChromePrefsState {
    fn default() -> Self {
        Self {
            ui_layout: "desktop".to_string(),
            theme_id: "semio".to_string(),
            custom_drivers: HashMap::new(),
            custom_themes: HashMap::new(),
            keybinding_overrides: HashMap::new(),
            theme_document_draft: None,
            theme_document_generation: 0,
            theme_document_resolved: None,
            worker_count: default_compute_worker_count(),
        }
    }
}

#[cfg(target_arch = "wasm32")]
thread_local! {
    static CHROME_PREFS: std::cell::RefCell<Option<ChromePrefsState>> = std::cell::RefCell::new(None);
}

#[cfg(not(target_arch = "wasm32"))]
static CHROME_PREFS: crate::element_contracts::WorkerCell<Option<ChromePrefsState>> = crate::element_contracts::WorkerCell::new(Default::default);

pub(crate) fn default_compute_worker_count() -> u32 {
    std::thread::available_parallelism().map(|n| n.get() as u32).unwrap_or(1)
}

fn project_chrome_prefs(preferences: UiPreferences, worker_count: u32) -> ChromePrefsState {
    let custom_themes = custom_themes_from(&preferences);
    let custom_drivers = preferences.custom_drivers;
    let ui_layout = match preferences.layout {
        Some(OsUiChromeLayout::Tablet) => "tablet",
        _ => "desktop",
    }
    .to_string();
    let theme_id = preferences.theme_id.unwrap_or_else(|| "semio".to_string());
    let keybinding_overrides = preferences.keybinding_overrides;
    ChromePrefsState { ui_layout, theme_id, custom_drivers, custom_themes, keybinding_overrides, theme_document_draft: None, theme_document_generation: 0, theme_document_resolved: None, worker_count }
}

fn load_chrome_prefs() -> ChromePrefsState {
    let worker_count = stored_field_get(UI_COMPUTE_WORKER_COUNT_STORAGE_KEY).and_then(|raw| raw.parse::<u32>().ok()).filter(|count| *count >= 1).unwrap_or_else(default_compute_worker_count);
    project_chrome_prefs(read_ui_preferences(), worker_count)
}

pub(crate) fn with_chrome_prefs<R>(f: impl FnOnce(&mut ChromePrefsState) -> R) -> R {
    CHROME_PREFS.with(|cell| {
        let mut guard = cell.borrow_mut();
        if guard.is_none() {
            *guard = Some(load_chrome_prefs());
        }
        f(guard.as_mut().expect("chrome prefs just initialized"))
    })
}

#[cfg(test)]
pub(crate) fn active_theme_id() -> String {
    with_chrome_prefs(|prefs| prefs.theme_id.clone())
}

pub(crate) fn set_active_theme_id(id: &str) {
    with_chrome_prefs(|prefs| prefs.theme_id = id.to_string());
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

pub(crate) fn delete_custom_theme(id: &str) {
    with_chrome_prefs(|prefs| {
        prefs.custom_themes.remove(id);
        if prefs.theme_id == id {
            prefs.theme_id = "semio".to_string();
        }
    });
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

//#region 🗣️ChromeI18n
pub(crate) fn scene_chrome_labels(is_de: bool) -> SceneChromeLabels {
    SceneChromeLabels {
        german: is_de,
        virtual_file_system: VirtualFileSystemChromeLabels {
            name: shell_chrome_string("common.name", is_de),
            no_file_system_nodes: shell_chrome_string("common.noFileSystemNodes", is_de),
            expand: shell_chrome_string("common.expand", is_de),
            collapse: shell_chrome_string("common.collapse", is_de),
        },
        block_list: BlockListChromeLabels { steps: shell_chrome_string("blockList.steps", is_de), add_step: shell_chrome_string("blockList.addStep", is_de), delete: shell_chrome_string("common.delete", is_de) },
        icon_render: crate::scenes::IconRenderChromeLabels { empty_scene: shell_chrome_string("host.emptyScene", is_de), rendering: shell_chrome_string("host.rendering", is_de), failed: shell_chrome_string("host.iconRenderFailed", is_de) },
    }
}

/// 🗣️ EN/DE lookup for a curated subset of previously-hardcoded chrome strings, byte-identical to
/// `ui/js/react/index.tsx`'s `uiChromeTranslationBundles.{en,de}.translation.ui.*` "normal" labels
/// (`:2898-3975`) at the dotted paths named below (e.g. `"display.tab.windows"` ==
/// `ui.display.tab.windows`). Unknown keys fall back to the key itself rather than inventing text.
///
/// 🖥️ React's `ui.display.*` block (`🖱️ui/🎯️targets/⚛️react/🟦️.tsx`), verbatim on both sides.
///
/// ⚔️ React's `ui.conflict.*` block.
///
/// 🔌️ React's `ui.plugins.status.*` / `ui.plugins.action.*` blocks.
///
/// 🎨️ wgpu-only additions (not verified against the external `elements/ui` i18next resource
/// bundle React's `shellLabel`/`uiI18n.t` ultimately reads — that package isn't vendored in this
/// repo tree, so these are reasonable EN/DE pairs in the same terse register as the rest of this
/// curated subset, not a byte-identical trace like the entries above copied from `index.tsx:2898-3975`).
///
/// 🎨️ The theme EDITOR's own vocabulary (packet W15d) — React reads these from
/// `ui.settings.theme.*`/`ui.mutation.policy.*` in the external i18next bundle this repo does
/// not vendor, so these are EN/DE pairs in the same terse register as the wgpu-only additions
/// above, not a byte-identical trace.
///
/// 💬️ The `os.agent.chat.*` bundle React registers in `🔗️AgentBridge/🟦️.tsx`, en first then de,
/// string-for-string — this renderer resolves chrome copy from this table instead of i18next.
///
/// 🚦️ `#s-sync-status`'s vocabulary, term for term with `syncPillText`'s own `FrozenLabel`s
/// (`🛠️ShellHelpers/🟦️.tsx`). `sync.recovering` has no React twin — see `ShellSyncPill`.
pub(crate) fn shell_chrome_string(key: &'static str, is_de: bool) -> &'static str {
    match (key, is_de) {
        ("host.emptyScene", false) => "No scene",
        ("host.emptyScene", true) => "Keine Szene",
        ("host.rendering", false) => "Rendering…",
        ("host.rendering", true) => "Wird gerendert…",
        ("host.iconRenderFailed", false) => "Icon rendering failed",
        ("host.iconRenderFailed", true) => "Symbol konnte nicht gerendert werden",
        ("display.emptyShell", false) => "Drag windows from Display in the navbar, or restore a saved layout.",
        ("display.emptyShell", true) => "Fenster aus Anzeige in der Navigationsleiste hierher ziehen oder ein gespeichertes Layout wiederherstellen.",
        ("display.saveLayout", false) => "Save layout",
        ("display.saveLayout", true) => "Layout speichern",
        ("display.saveLayoutPlaceholder", false) => "Layout name",
        ("display.saveLayoutPlaceholder", true) => "Layoutname",
        ("display.saveCurrentLayout", false) => "Save current layout",
        ("display.saveCurrentLayout", true) => "Aktuelles Layout speichern",
        ("display.deleteLayout", false) => "Delete",
        ("display.deleteLayout", true) => "Löschen",
        ("display.layouts", false) => "Layouts",
        ("display.layouts", true) => "Layouts",
        ("display.saved", false) => "Saved",
        ("display.saved", true) => "Gespeichert",
        ("display.unavailable", false) => "Display unavailable",
        ("display.unavailable", true) => "Anzeige nicht verfügbar",
        ("common.name", false) => "Name",
        ("common.name", true) => "Name",
        ("common.noFileSystemNodes", false) => "No file system nodes",
        ("common.noFileSystemNodes", true) => "Keine Dateisystemknoten",
        ("common.expand", false) => "Expand",
        ("common.expand", true) => "Ausklappen",
        ("common.collapse", false) => "Collapse",
        ("common.collapse", true) => "Einklappen",
        ("common.save", false) => "Save",
        ("common.save", true) => "Speichern",
        ("common.delete", false) => "Delete",
        ("common.delete", true) => "Löschen",
        ("blockList.steps", false) => "Steps",
        ("blockList.steps", true) => "Schritte",
        ("blockList.addStep", false) => "Add Step",
        ("blockList.addStep", true) => "Schritt hinzufügen",
        ("conflict.accept", false) => "Accept",
        ("conflict.accept", true) => "Übernehmen",
        ("conflict.discard", false) => "Discard",
        ("conflict.discard", true) => "Verwerfen",
        ("conflict.quarantined", false) => "Held back",
        ("conflict.quarantined", true) => "Zurückgehalten",
        ("conflict.degraded", false) => "Degraded",
        ("conflict.degraded", true) => "Beeinträchtigt",
        ("conflict.hubRejected", false) => "Change refused by the hub",
        ("conflict.hubRejected", true) => "Änderung vom Hub abgelehnt",
        ("conflict.hubTransformed", false) => "Change adjusted",
        ("conflict.hubTransformed", true) => "Änderung angepasst",
        ("conflict.hubConcurrentEdit", false) => "Someone else changed the same part at the same time",
        ("conflict.hubConcurrentEdit", true) => "Jemand anderes hat gleichzeitig dieselbe Stelle geändert",
        ("conflict.hubConcurrentInvariant", false) => "Conflicts with a simultaneous change",
        ("conflict.hubConcurrentInvariant", true) => "Widerspricht einer gleichzeitigen Änderung",
        ("conflict.local.readOnly", false) => "Change not applied: this document is read-only here",
        ("conflict.local.readOnly", true) => "Änderung nicht übernommen: Dieses Dokument ist hier schreibgeschützt",
        ("conflict.local.queueFull", false) => "Change not applied: too many changes are waiting to be saved",
        ("conflict.local.queueFull", true) => "Änderung nicht übernommen: Zu viele Änderungen warten aufs Speichern",
        ("conflict.local.duplicate", false) => "Change not applied: it is already waiting to be saved",
        ("conflict.local.duplicate", true) => "Änderung nicht übernommen: Sie wartet bereits aufs Speichern",
        ("conflict.local.notReady", false) => "Change not applied: the document is not ready yet",
        ("conflict.local.notReady", true) => "Änderung nicht übernommen: Das Dokument ist noch nicht bereit",
        ("conflict.local.foreignDocument", false) => "Change not applied: it belongs to another document",
        ("conflict.local.foreignDocument", true) => "Änderung nicht übernommen: Sie gehört zu einem anderen Dokument",
        ("conflict.local.unreadable", false) => "Change not applied: it could not be read",
        ("conflict.local.unreadable", true) => "Änderung nicht übernommen: Sie konnte nicht gelesen werden",
        ("conflict.local.tooLarge", false) => "Change not applied: it is too large to send",
        ("conflict.local.tooLarge", true) => "Änderung nicht übernommen: Sie ist zu groß zum Senden",
        ("plugins.status.available", false) => "Available",
        ("plugins.status.available", true) => "Verfügbar",
        ("plugins.status.installing", false) => "Installing…",
        ("plugins.status.installing", true) => "Wird installiert…",
        ("plugins.action.install", false) => "Install",
        ("plugins.action.install", true) => "Installieren",
        ("plugins.action.uninstall", false) => "Uninstall",
        ("plugins.action.uninstall", true) => "Deinstallieren",
        ("plugins.action.reload", false) => "Reload",
        ("plugins.action.reload", true) => "Neu laden",
        ("plugins.reason.installing", false) => "The plugin is being installed.",
        ("plugins.reason.installing", true) => "Das Plugin wird gerade installiert.",
        ("plugins.reason.inUse", false) => "The open document uses this plugin.",
        ("plugins.reason.inUse", true) => "Das geöffnete Dokument verwendet dieses Plugin.",
        ("plugins.reason.extensionUnavailable", false) => "The extension did not load.",
        ("plugins.reason.extensionUnavailable", true) => "Die Erweiterung wurde nicht geladen.",
        ("display.tab.windows", false) => "Windows",
        ("display.tab.windows", true) => "Fenster",
        ("display.tab.layout", false) => "Layout",
        ("display.tab.layout", true) => "Layout",
        ("settings.tab.general", false) => "General",
        ("settings.tab.general", true) => "Allgemein",
        ("plugin.faulted", false) => "Plugin unavailable",
        ("plugin.faulted", true) => "Plugin nicht verfügbar",
        ("plugin.install.resolving", false) => "Preparing plugin",
        ("plugin.install.resolving", true) => "Plugin wird vorbereitet",
        ("plugin.install.loading", false) => "Loading plugin",
        ("plugin.install.loading", true) => "Plugin wird geladen",
        ("plugin.install.cancelled", false) => "Plugin load cancelled",
        ("plugin.install.cancelled", true) => "Plugin-Laden abgebrochen",
        ("plugin.install.failed", false) => "Plugin could not be loaded",
        ("plugin.install.failed", true) => "Plugin konnte nicht geladen werden",
        ("plugin.install.cancel", false) => "Cancel",
        ("plugin.install.cancel", true) => "Abbrechen",
        ("icon.export.loading", false) => "Loading export",
        ("icon.export.loading", true) => "Export wird geladen",
        ("icon.export.preparing", false) => "Preparing export",
        ("icon.export.preparing", true) => "Export wird vorbereitet",
        ("icon.export.rendering", false) => "Rendering export",
        ("icon.export.rendering", true) => "Export wird gerendert",
        ("icon.export.closing", false) => "Releasing export resources",
        ("icon.export.closing", true) => "Export-Ressourcen werden freigegeben",
        ("icon.export.saving", false) => "Saving export",
        ("icon.export.saving", true) => "Export wird gespeichert",
        ("icon.export.complete", false) => "Export complete",
        ("icon.export.complete", true) => "Export abgeschlossen",
        ("icon.export.cancelled", false) => "Export cancelled",
        ("icon.export.cancelled", true) => "Export abgebrochen",
        ("icon.export.failed", false) => "Export failed",
        ("icon.export.failed", true) => "Export fehlgeschlagen",
        ("icon.export.cancel", false) => "Cancel",
        ("icon.export.cancel", true) => "Abbrechen",
        ("document.open.resolving", false) => "Resolving the hub's component for",
        ("document.open.resolving", true) => "Hub-Komponente wird ermittelt für",
        ("document.open.resolving.lease", false) => "asking the hub which component runs it",
        ("document.open.resolving.lease", true) => "Hub wird nach der ausführenden Komponente gefragt",
        ("document.open.resolving.component", false) => "downloading the component",
        ("document.open.resolving.component", true) => "Komponente wird heruntergeladen",
        ("document.open.resolving.descriptor", false) => "downloading its descriptor",
        ("document.open.resolving.descriptor", true) => "Deskriptor wird heruntergeladen",
        ("document.open.resolving.verified", false) => "verified",
        ("document.open.resolving.verified", true) => "geprüft",
        ("document.open.instantiating", false) => "Starting",
        ("document.open.instantiating", true) => "Wird gestartet:",
        ("document.open.seeding", false) => "Loading the document in",
        ("document.open.seeding", true) => "Dokument wird geladen in",
        ("document.open.cancelled", false) => "Opening cancelled:",
        ("document.open.cancelled", true) => "Öffnen abgebrochen:",
        ("document.open.failed", false) => "Could not open",
        ("document.open.failed", true) => "Konnte nicht geöffnet werden:",
        ("document.open.cancel", false) => "Cancel",
        ("document.open.cancel", true) => "Abbrechen",
        ("document.open.busy", false) => "Another document is still opening",
        ("document.open.busy", true) => "Ein anderes Dokument wird noch geöffnet",
        ("open-artifact.document-failed", false) => "Opened the app, but its document could not be attached here",
        ("open-artifact.document-failed", true) => "App geöffnet, aber das Dokument konnte hier nicht angehängt werden",
        ("surface.faulted", false) => "Surface unavailable",
        ("surface.faulted", true) => "Fläche nicht verfügbar",
        ("settings.tab.theme", false) => "Theme",
        ("settings.tab.theme", true) => "Thema",
        ("settings.theme.colors", false) => "Colors",
        ("settings.theme.colors", true) => "Farben",
        ("settings.theme.spacing", false) => "Spacing",
        ("settings.theme.spacing", true) => "Abstände",
        ("settings.theme.fonts", false) => "Fonts",
        ("settings.theme.fonts", true) => "Schriften",
        ("settings.theme.strokes", false) => "Strokes",
        ("settings.theme.strokes", true) => "Striche",
        ("settings.theme.radii", false) => "Radii",
        ("settings.theme.radii", true) => "Radien",
        ("settings.theme.opacities", false) => "Opacities",
        ("settings.theme.opacities", true) => "Deckkraft",
        ("settings.theme.metrics", false) => "Metrics",
        ("settings.theme.metrics", true) => "Maße",
        ("settings.theme.appearances", false) => "Appearances",
        ("settings.theme.appearances", true) => "Darstellungen",
        ("settings.theme.appearance.light", false) => "Light",
        ("settings.theme.appearance.light", true) => "Hell",
        ("settings.theme.appearance.dark", false) => "Dark",
        ("settings.theme.appearance.dark", true) => "Dunkel",
        ("settings.theme.group.board", false) => "Board",
        ("settings.theme.group.board", true) => "Board",
        ("settings.theme.group.map", false) => "Map",
        ("settings.theme.group.map", true) => "Karte",
        ("settings.theme.group.canvas", false) => "Canvas",
        ("settings.theme.group.canvas", true) => "Leinwand",
        ("settings.theme.group.chrome", false) => "Chrome",
        ("settings.theme.group.chrome", true) => "Rahmen",
        ("settings.theme.group.outcome", false) => "Outcome",
        ("settings.theme.group.outcome", true) => "Ergebnis",
        ("settings.theme.group.diagram", false) => "Diagram",
        ("settings.theme.group.diagram", true) => "Diagramm",
        ("settings.theme.save", false) => "Save theme",
        ("settings.theme.save", true) => "Design speichern",
        ("settings.theme.export", false) => "Export theme",
        ("settings.theme.export", true) => "Design exportieren",
        ("settings.theme.import", false) => "Import theme",
        ("settings.theme.import", true) => "Design importieren",
        ("settings.theme.page", false) => "Page",
        ("settings.theme.page", true) => "Seite",
        ("settings.theme.pagePrevious", false) => "Previous",
        ("settings.theme.pagePrevious", true) => "Zurück",
        ("settings.theme.pageNext", false) => "Next",
        ("settings.theme.pageNext", true) => "Weiter",
        ("settings.mergePolicy", false) => "Merge policy",
        ("settings.mergePolicy", true) => "Zusammenführung",
        ("settings.mergePolicy.laissezFaire", false) => "Laissez-faire",
        ("settings.mergePolicy.laissezFaire", true) => "Laissez-faire",
        ("settings.mergePolicy.normal", false) => "Normal",
        ("settings.mergePolicy.normal", true) => "Normal",
        ("settings.mergePolicy.vigilant", false) => "Vigilant",
        ("settings.mergePolicy.vigilant", true) => "Wachsam",
        ("settings.tab.app", false) => "App",
        ("settings.tab.app", true) => "App",
        ("settings.app.name", false) => "Name",
        ("settings.app.name", true) => "Name",
        ("settings.app.id", false) => "Id",
        ("settings.app.id", true) => "Id",
        ("settings.app.controller", false) => "Controller",
        ("settings.app.controller", true) => "Controller",
        ("settings.app.plugin", false) => "Plugin",
        ("settings.app.plugin", true) => "Plugin",
        ("settings.tab.appearance", false) => "Appearance",
        ("settings.tab.appearance", true) => "Darstellung",
        ("settings.appearance.system", false) => "System",
        ("settings.appearance.system", true) => "System",
        ("settings.appearance.light", false) => "Light",
        ("settings.appearance.light", true) => "Hell",
        ("settings.appearance.dark", false) => "Dark",
        ("settings.appearance.dark", true) => "Dunkel",
        ("settings.tab.layout", false) => "Layout",
        ("settings.tab.layout", true) => "Layout",
        ("settings.layout.desktop", false) => "Desktop",
        ("settings.layout.desktop", true) => "Desktop",
        ("settings.layout.tablet", false) => "Tablet",
        ("settings.layout.tablet", true) => "Tablet",
        ("settings.tab.driver", false) => "Driver",
        ("settings.tab.driver", true) => "Steuerung",
        ("settings.driver.dirty", false) => "Modified",
        ("settings.driver.dirty", true) => "Geändert",
        ("settings.driver.default", false) => "Default",
        ("settings.driver.default", true) => "Standard",
        ("settings.driver.compact", false) => "Compact",
        ("settings.driver.compact", true) => "Kompakt",
        ("settings.driver.labels", false) => "Labels",
        ("settings.driver.labels", true) => "Beschriftungen",
        ("settings.driver.labelsOption.full", false) => "Full",
        ("settings.driver.labelsOption.full", true) => "Vollständig",
        ("settings.driver.labelsOption.icons", false) => "Icons",
        ("settings.driver.labelsOption.icons", true) => "Symbole",
        ("settings.driver.labelTier", false) => "Label detail",
        ("settings.driver.labelTier", true) => "Beschriftungsstufe",
        ("settings.driver.labelTierOption.beginner", false) => "Beginner",
        ("settings.driver.labelTierOption.beginner", true) => "Einsteiger",
        ("settings.driver.labelTierOption.normal", false) => "Normal",
        ("settings.driver.labelTierOption.normal", true) => "Normal",
        ("settings.driver.drag", false) => "Dragging",
        ("settings.driver.drag", true) => "Ziehen",
        ("settings.driver.dragOption.handle", false) => "Handle",
        ("settings.driver.dragOption.handle", true) => "Griff",
        ("settings.driver.dragOption.surface", false) => "Surface",
        ("settings.driver.dragOption.surface", true) => "Fläche",
        ("settings.driver.chrome", false) => "Chrome",
        ("settings.driver.chrome", true) => "Rahmen",
        ("settings.driver.gumball", false) => "Gumball",
        ("settings.driver.gumball", true) => "Manipulator",
        ("settings.driver.chromeOption.always", false) | ("settings.driver.gumballOption.always", false) => "Always",
        ("settings.driver.chromeOption.always", true) | ("settings.driver.gumballOption.always", true) => "Immer",
        ("settings.driver.chromeOption.hover", false) | ("settings.driver.gumballOption.hover", false) => "On hover",
        ("settings.driver.chromeOption.hover", true) | ("settings.driver.gumballOption.hover", true) => "Bei Zeigerkontakt",
        ("settings.driver.tooltips", false) => "Tooltips",
        ("settings.driver.tooltips", true) => "Tooltips",
        ("settings.driver.tooltipsOption.full", false) => "Full",
        ("settings.driver.tooltipsOption.full", true) => "Vollständig",
        ("settings.driver.tooltipsOption.minimal", false) => "Minimal",
        ("settings.driver.tooltipsOption.minimal", true) => "Minimal",
        ("settings.driver.tooltipsOption.none", false) => "None",
        ("settings.driver.tooltipsOption.none", true) => "Keine",
        ("settings.driver.hotkeys", false) => "Hotkeys",
        ("settings.driver.hotkeys", true) => "Tastenkürzel",
        ("settings.driver.hotkeysOption.inline", false) => "Inline",
        ("settings.driver.hotkeysOption.inline", true) => "Inline",
        ("settings.driver.hotkeysOption.tooltip", false) => "Tooltip",
        ("settings.driver.hotkeysOption.tooltip", true) => "Tooltip",
        ("settings.driver.hotkeysOption.none", false) => "None",
        ("settings.driver.hotkeysOption.none", true) => "Keine",
        ("settings.driver.savePlaceholder", false) => "Driver name",
        ("settings.driver.savePlaceholder", true) => "Steuerungsname",
        ("settings.driver.save", false) => "Save driver",
        ("settings.driver.save", true) => "Steuerung speichern",
        ("settings.driver.delete", false) => "Delete driver",
        ("settings.driver.delete", true) => "Steuerung löschen",
        ("settings.tab.language", false) => "Language",
        ("settings.tab.language", true) => "Sprache",
        ("settings.language.en", false) => "English",
        ("settings.language.en", true) => "Englisch",
        ("settings.language.de", false) => "German",
        ("settings.language.de", true) => "Deutsch",
        ("settings.tab.terminology", false) => "Terminology",
        ("settings.tab.terminology", true) => "Terminologie",
        ("settings.resetDock", false) => "Reset Dock",
        ("settings.resetDock", true) => "Dock zurücksetzen",
        ("settings.tab.keybindings", false) => "Hotkeys",
        ("settings.tab.keybindings", true) => "Tastenkürzel",
        ("settings.keybindings.capture", false) => "Change",
        ("settings.keybindings.capture", true) => "Ändern",
        ("settings.keybindings.pressKeys", false) => "Press keys…",
        ("settings.keybindings.pressKeys", true) => "Tasten drücken…",
        ("settings.keybindings.reset", false) => "Reset",
        ("settings.keybindings.reset", true) => "Zurücksetzen",
        ("settings.keybindings.conflict", false) => "conflict",
        ("settings.keybindings.conflict", true) => "Konflikt",
        ("settings.tab.defaultApps", false) => "Default Apps",
        ("settings.tab.defaultApps", true) => "Standard-Apps",
        ("settings.defaultApps.none", false) => "None",
        ("settings.defaultApps.none", true) => "Keine",
        ("settings.unavailable", false) => "Unavailable",
        ("settings.unavailable", true) => "Nicht verfügbar",
        ("conflict.panel", false) => "Conflicts",
        ("conflict.panel", true) => "Konflikte",
        ("marketplace.tab", false) => "Marketplace",
        ("marketplace.tab", true) => "Marktplatz",
        ("marketplace.unavailable", false) => "Marketplace unavailable",
        ("marketplace.unavailable", true) => "Marktplatz nicht verfügbar",
        ("plugins.source", false) => "Source",
        ("plugins.source", true) => "Quelle",
        ("plugins.status.loaded", false) => "loaded",
        ("plugins.status.loaded", true) => "geladen",
        ("plugins.status.failed", false) => "failed",
        ("plugins.status.failed", true) => "fehlgeschlagen",
        ("plugins.extension.install", false) => "Install extension",
        ("plugins.extension.install", true) => "Erweiterung installieren",
        ("plugins.extension.installFromFile", false) => "Install from file",
        ("plugins.extension.installFromFile", true) => "Aus Datei installieren",
        ("plugins.extension.installFromUrl", false) => "Install from URL",
        ("plugins.extension.installFromUrl", true) => "Von URL installieren",
        ("plugins.extension.urlPrompt", false) => "Extension package URL",
        ("plugins.extension.urlPrompt", true) => "URL des Erweiterungspakets",
        ("plugins.extension.enabled", false) => "enabled",
        ("plugins.extension.enabled", true) => "aktiviert",
        ("plugins.extension.disabled", false) => "disabled",
        ("plugins.extension.disabled", true) => "deaktiviert",
        ("plugins.extension.enable", false) => "Enable",
        ("plugins.extension.enable", true) => "Aktivieren",
        ("plugins.extension.disable", false) => "Disable",
        ("plugins.extension.disable", true) => "Deaktivieren",
        ("tool.armed", false) => "Options (armed)",
        ("tool.armed", true) => "Optionen (aktiv)",
        ("tool.idle", false) => "Options",
        ("tool.idle", true) => "Optionen",
        ("mobilePanel.app", false) => "App",
        ("mobilePanel.app", true) => "App",
        ("chat.empty", false) => "No agent activity yet. Messages you send appear here, along with every tool the agent runs.",
        ("chat.empty", true) => "Noch keine Agent-Aktivität. Gesendete Nachrichten erscheinen hier, ebenso jedes vom Agent ausgeführte Werkzeug.",
        ("chat.youRole", false) => "You",
        ("chat.youRole", true) => "Du",
        ("chat.toolCallRole", false) => "Tool call",
        ("chat.toolCallRole", true) => "Werkzeugaufruf",
        ("chat.agentRole", false) => "Agent",
        ("chat.agentRole", true) => "Agent",
        ("chat.replyStreaming", false) => "Still writing…",
        ("chat.replyStreaming", true) => "Schreibt noch…",
        ("chat.toolResultRole", false) => "Result",
        ("chat.toolResultRole", true) => "Ergebnis",
        ("chat.approvalRole", false) => "Approval",
        ("chat.approvalRole", true) => "Freigabe",
        ("chat.running", false) => "Running…",
        ("chat.running", true) => "Läuft…",
        ("chat.cancelling", false) => "Cancelling…",
        ("chat.cancelling", true) => "Wird abgebrochen…",
        ("chat.cancel", false) => "Cancel",
        ("chat.cancel", true) => "Abbrechen",
        ("chat.failed", false) => "Failed",
        ("chat.failed", true) => "Fehlgeschlagen",
        ("chat.succeeded", false) => "Done",
        ("chat.succeeded", true) => "Fertig",
        ("chat.approvalPending", false) => "Waiting for your decision",
        ("chat.approvalPending", true) => "Wartet auf deine Entscheidung",
        ("chat.disconnected", false) => "Not connected to an agent — messages cannot be sent.",
        ("chat.disconnected", true) => "Nicht mit einem Agent verbunden — Nachrichten können nicht gesendet werden.",
        ("chat.placeholder", false) => "Tell the agent what to do…",
        ("chat.placeholder", true) => "Sag dem Agent, was zu tun ist…",
        ("chat.draftLabel", false) => "Message to the agent",
        ("chat.draftLabel", true) => "Nachricht an den Agent",
        ("chat.send", false) => "Send",
        ("chat.send", true) => "Senden",
        ("common.clear", false) => "Clear",
        ("common.clear", true) => "Leeren",
        ("settings.tab.commands", false) => "Commands",
        ("settings.tab.commands", true) => "Befehle",
        ("settings.theme.select", false) => "Theme",
        ("settings.theme.select", true) => "Design",
        ("settings.theme.reset", false) => "Reset",
        ("settings.theme.reset", true) => "Zurücksetzen",
        ("settings.theme.delete", false) => "Delete",
        ("settings.theme.delete", true) => "Löschen",
        ("fullscreen.toggle", false) => "Fullscreen",
        ("fullscreen.toggle", true) => "Vollbild",
        ("fullscreen.exit", false) => "Exit Fullscreen",
        ("fullscreen.exit", true) => "Vollbild beenden",
        ("panelToggle.display", false) => "Display",
        ("panelToggle.display", true) => "Anzeige",
        ("panelToggle.workbench", false) => "Workbench",
        ("panelToggle.workbench", true) => "Arbeitsbereich",
        ("panelToggle.details", false) => "Details",
        ("panelToggle.details", true) => "Details",
        ("panelToggle.settings", false) => "Settings",
        ("panelToggle.settings", true) => "Einstellungen",
        ("panelToggle.chat", false) => "Chat",
        ("panelToggle.chat", true) => "Chat",
        ("panelToggle.hub", false) => "Hub",
        ("panelToggle.hub", true) => "Hub",
        ("panelToggle.command", false) => "Command",
        ("panelToggle.command", true) => "Befehl",
        ("panelToggle.tool", false) => "Tool",
        ("panelToggle.tool", true) => "Werkzeug",
        ("panelToggle.sync", false) => "Sync",
        ("panelToggle.sync", true) => "Abgleich",
        ("taskManager.title", false) => "Task Manager",
        ("taskManager.title", true) => "Task-Manager",
        ("panelToggle.taskManager", false) => "Tasks",
        ("panelToggle.taskManager", true) => "Aufgaben",
        ("taskManager.documentImport", false) => "Document import",
        ("taskManager.documentImport", true) => "Dokumentimport",
        ("taskManager.videoFrames", false) => "Video frames",
        ("taskManager.videoFrames", true) => "Videobilder",
        ("taskManager.runningTasks", false) => "Running tasks",
        ("taskManager.runningTasks", true) => "Laufende Aufgaben",
        ("taskManager.noRuntime", false) => "No actor runtime is attached to this shell.",
        ("taskManager.noRuntime", true) => "Mit dieser Shell ist keine Akteur-Laufzeit verbunden.",
        ("common.home", false) => "Home",
        ("common.home", true) => "Startseite",
        ("common.windowOptions", false) => "Window Options",
        ("common.windowOptions", true) => "Fensteroptionen",
        ("common.actions", false) => "Actions",
        ("common.actions", true) => "Aktionen",
        ("common.utilities", false) => "Utilities",
        ("common.utilities", true) => "Werkzeuge",
        ("common.projection", false) => "Projection",
        ("common.projection", true) => "Projektion",
        ("window.search.title", false) => "Search",
        ("window.search.title", true) => "Suchen",
        ("windowSearch.action", false) => "Action",
        ("windowSearch.action", true) => "Aktion",
        ("windowSearch.actionActive", false) => "Action or value",
        ("windowSearch.actionActive", true) => "Aktion oder Wert",
        ("windowSearch.suggestions", false) => "Suggestions",
        ("windowSearch.suggestions", true) => "Vorschläge",
        ("windowSearch.noMatches", false) => "No matches",
        ("windowSearch.noMatches", true) => "Keine Treffer",
        ("common.focus", false) => "Focus",
        ("common.focus", true) => "Fokussieren",
        ("common.unfocus", false) => "Unfocus",
        ("common.unfocus", true) => "Fokus aufheben",
        ("common.select", false) => "Select",
        ("common.select", true) => "Auswählen",
        ("common.execute", false) => "Execute",
        ("common.execute", true) => "Ausführen",
        ("common.reset", false) => "Reset",
        ("common.reset", true) => "Zurücksetzen",
        ("shellCommand.panelToggle", false) => "Toggle Panel",
        ("shellCommand.panelToggle", true) => "Panel umschalten",
        ("shellCommand.panelTab", false) => "Switch Panel Tab",
        ("shellCommand.panelTab", true) => "Panel-Tab wechseln",
        ("introduction.skip", false) => "Skip",
        ("introduction.skip", true) => "Überspringen",
        ("introduction.back", false) => "Back",
        ("introduction.back", true) => "Zurück",
        ("introduction.next", false) => "Next",
        ("introduction.next", true) => "Weiter",
        ("introduction.done", false) => "Done",
        ("introduction.done", true) => "Fertig",
        ("sync.file", false) => "File",
        ("sync.file", true) => "Datei",
        ("sync.folder", false) => "Folder",
        ("sync.folder", true) => "Ordner",
        ("sync.attach", false) => "Attach",
        ("sync.attach", true) => "Verbinden",
        ("sync.detach", false) => "Detach",
        ("sync.detach", true) => "Trennen",
        ("sync.browse", false) => "Browse",
        ("sync.browse", true) => "Durchsuchen",
        ("sync.persisted", false) => "Persisted",
        ("sync.persisted", true) => "Gespeichert",
        ("sync.pending", false) => "Pending",
        ("sync.pending", true) => "Ausstehend",
        ("sync.remote", false) => "Remote",
        ("sync.remote", true) => "Remote",
        ("sync.remote.connected", false) => "connected",
        ("sync.remote.connected", true) => "verbunden",
        ("sync.remote.connecting", false) => "connecting",
        ("sync.remote.connecting", true) => "verbindet",
        ("sync.remote.backoff", false) => "backoff",
        ("sync.remote.backoff", true) => "erneuter Versuch",
        ("sync.remote.detached", false) => "detached",
        ("sync.remote.detached", true) => "getrennt",
        ("sync.recovering", false) => "Recovering",
        ("sync.recovering", true) => "Wiederherstellung",
        ("hub.live", false) => "live",
        ("hub.live", true) => "verbunden",
        ("hub.connecting", false) => "connecting…",
        ("hub.connecting", true) => "verbinde…",
        ("hub.reconnecting", false) => "reconnecting…",
        ("hub.reconnecting", true) => "verbinde erneut…",
        ("hub.online", false) => "online",
        ("hub.online", true) => "online",
        ("hub.local", false) => "local only",
        ("hub.local", true) => "nur lokal",
        ("hub.signedOut", false) => "signed out",
        ("hub.signedOut", true) => "abgemeldet",
        ("hub.signIn", false) => "Sign in",
        ("hub.signIn", true) => "Anmelden",
        ("hub.peerOne", false) => "peer",
        ("hub.peerOne", true) => "Mitwirkender",
        ("hub.peerMany", false) => "peers",
        ("hub.peerMany", true) => "Mitwirkende",
        ("common.cancel", false) => "Cancel",
        ("common.cancel", true) => "Abbrechen",
        ("example.picker", false) => "Example",
        ("example.picker", true) => "Beispiel",
        ("example.overlay.title", false) => "Examples",
        ("example.overlay.title", true) => "Beispiele",
        ("nodeGraph.fitGraph", false) => "Fit graph",
        ("nodeGraph.fitGraph", true) => "Graph einpassen",
        ("surfaceRole.group", false) => "Surface role",
        ("surfaceRole.group", true) => "Oberflächenrolle",
        ("mode.group", false) => "Mode",
        ("mode.group", true) => "Modus",
        ("contextMenu.goHome", false) => "Go Home",
        ("contextMenu.goHome", true) => "Zur Startseite",
        ("common.loading", false) => "Loading",
        ("common.loading", true) => "Wird geladen",
        ("overlay.search.title", false) => "Search",
        ("overlay.search.title", true) => "Suchen",
        ("overlay.find.title", false) => "Find in page",
        ("overlay.find.title", true) => "Auf Seite suchen",
        ("search.title", false) => "Search",
        ("search.title", true) => "Suche",
        ("search.description", false) => "Search for items",
        ("search.description", true) => "Nach Elementen suchen",
        ("search.placeholder", false) => "Search...",
        ("search.placeholder", true) => "Suchen...",
        ("search.empty", false) => "No results found.",
        ("search.empty", true) => "Keine Ergebnisse gefunden.",
        ("find.title", false) => "Find",
        ("find.title", true) => "Finden",
        ("find.description", false) => "Find items in this view",
        ("find.description", true) => "Elemente in dieser Ansicht finden",
        ("find.placeholder", false) => "Find...",
        ("find.placeholder", true) => "Finden...",
        ("find.empty", false) => "No results found.",
        ("find.empty", true) => "Keine Ergebnisse gefunden.",
        ("common.close", false) => "Close",
        ("common.close", true) => "Schließen",
        ("tree.drag.sortTarget", false) => "Click and hold left click to drag {{target}}",
        ("tree.drag.sortTarget", true) => "Linksklick gedrückt halten, um {{target}} zu ziehen",
        ("command.activateWindow", false) => "Activate Window",
        ("command.activateWindow", true) => "Fenster aktivieren",
        ("command.applyLayout", false) => "Apply Layout",
        ("command.applyLayout", true) => "Layout anwenden",
        ("command.checkIn", false) => "Check In",
        ("command.checkIn", true) => "Einchecken",
        ("checkIn.running", false) => "Checking in…",
        ("checkIn.running", true) => "Wird eingecheckt…",
        ("checkIn.ready", false) => "Checked in",
        ("checkIn.ready", true) => "Eingecheckt",
        ("checkIn.cancelled", false) => "Check-in cancelled",
        ("checkIn.cancelled", true) => "Einchecken abgebrochen",
        ("checkIn.unknownHead", false) => "Check-in refused: the hub does not know this version",
        ("checkIn.unknownHead", true) => "Einchecken abgelehnt: Der Hub kennt diesen Stand nicht",
        ("checkIn.staleHead", false) => "Already checked in: a newer check-in exists",
        ("checkIn.staleHead", true) => "Bereits eingecheckt: Es gibt einen neueren Check-in",
        ("checkIn.activeCheckpointChanged", false) => "Check-in collided with another check-in; try again",
        ("checkIn.activeCheckpointChanged", true) => "Einchecken kollidierte mit einem anderen Check-in; bitte erneut versuchen",
        ("checkIn.ledgerNotReplayable", false) => "Check-in refused: an approval in this range is checked in on its own",
        ("checkIn.ledgerNotReplayable", true) => "Einchecken abgelehnt: Eine Freigabe in diesem Bereich wird separat eingecheckt",
        ("checkIn.codecRefused", false) => "Check-in refused: the document could not be rebuilt",
        ("checkIn.codecRefused", true) => "Einchecken abgelehnt: Das Dokument konnte nicht wiederhergestellt werden",
        ("checkIn.authorityChanged", false) => "Check-in stopped: you can no longer edit this document",
        ("checkIn.authorityChanged", true) => "Einchecken gestoppt: Sie dürfen dieses Dokument nicht mehr bearbeiten",
        ("checkIn.unavailable", false) => "Check-in unavailable; try again",
        ("checkIn.unavailable", true) => "Einchecken nicht verfügbar; bitte erneut versuchen",
        ("command.moveWindow", false) => "Move Window",
        ("command.moveWindow", true) => "Fenster verschieben",
        ("command.splitWindow", false) => "Split Window",
        ("command.splitWindow", true) => "Fenster teilen",
        ("command.resizeWindow", false) => "Resize Window",
        ("command.resizeWindow", true) => "Fenster anpassen",
        ("palette.undo", false) => "Undo",
        ("palette.undo", true) => "Rückgängig",
        ("palette.redo", false) => "Redo",
        ("palette.redo", true) => "Wiederholen",
        ("palette.goHome", false) => "Go Home",
        ("palette.goHome", true) => "Nach Hause",
        ("palette.spawnPrefix", false) => "Spawn",
        ("palette.spawnPrefix", true) => "Erzeugen",
        ("palette.commitCheckpoint", false) => "Commit Checkpoint",
        ("palette.commitCheckpoint", true) => "Prüfpunkt festschreiben",
        ("palette.group.panels", false) => "Panels",
        ("palette.group.panels", true) => "Panels",
        ("palette.group.windows", false) => "Windows",
        ("palette.group.windows", true) => "Fenster",
        ("palette.group.catalogue", false) => "Catalogue",
        ("palette.group.catalogue", true) => "Katalog",
        ("palette.group.navigation", false) => "Navigation",
        ("palette.group.navigation", true) => "Navigation",
        ("palette.group.actions", false) => "Actions",
        ("palette.group.actions", true) => "Aktionen",
        ("palette.group.hostApp", false) => "Space",
        ("palette.group.hostApp", true) => "Space",
        (other, _) => other,
    }
}

//#endregion 🗣️ChromeI18n

//#region 💾️PrefsSync
#[derive(Clone, Default, PartialEq)]
pub(crate) struct UiPrefsSnapshot {
    pub(crate) appearance_id: String,
    pub(crate) locale_id: String,
    pub(crate) terminology_id: String,
    pub(crate) driver_id: String,
    pub(crate) theme_id: String,
    pub(crate) ui_layout: String,
    pub(crate) custom_drivers: HashMap<String, OsUiDriver>,
    pub(crate) custom_themes: HashMap<String, String>,
    pub(crate) keybinding_overrides: HashMap<String, String>,
    pub(crate) worker_count: u32,
}

impl UiPrefsSnapshot {
    pub(crate) fn capture(state: &ShellState) -> Self {
        Self {
            appearance_id: state.appearance_id.clone(),
            locale_id: state.locale_id.clone(),
            terminology_id: state.terminology_id.clone(),
            driver_id: state.driver_id.clone(),
            theme_id: state.chrome_build.preferences.theme_id.clone(),
            ui_layout: state.chrome_build.preferences.ui_layout.clone(),
            custom_drivers: state.chrome_build.preferences.custom_drivers.clone(),
            custom_themes: state.chrome_build.preferences.custom_themes.clone(),
            keybinding_overrides: state.chrome_build.preferences.keybinding_overrides.clone(),
            worker_count: state.chrome_build.preferences.worker_count,
        }
    }
}

#[derive(Clone, Debug, Default)]
struct UiPreferencesEventLog {
    pub(crate) version: u8,
    pub(crate) events: Vec<UiPreferencesConfigMutation>,
}

/// 🚫️ A stored preference log this renderer cannot read whole — unparsable, another version, or carrying an event
/// it does not know (one a newer renderer wrote). Typed, never read as an empty log: it refuses every write, since a
/// rewrite from this renderer's own events would destroy what it could not read, while reads replay `readable`, the
/// events it did decode.
#[derive(Clone, Debug)]
struct UiPreferencesLogRefusal {
    pub(crate) reason: &'static str,
    event_index: Option<usize>,
    readable: UiPreferencesEventLog,
}

fn decode_ui_preferences_event_log(raw: &str) -> Result<UiPreferencesEventLog, UiPreferencesLogRefusal> {
    let refuse = |reason: &'static str, event_index: Option<usize>, events: Vec<UiPreferencesConfigMutation>| UiPreferencesLogRefusal { reason, event_index, readable: UiPreferencesEventLog { version: 1, events } };
    let Ok(value) = serde_json::from_str::<Value>(raw) else {
        return Err(refuse("json", None, Vec::new()));
    };
    if value.get("version").and_then(Value::as_u64) != Some(1) {
        return Err(refuse("version", None, Vec::new()));
    }
    let Some(stored) = value.get("events").and_then(Value::as_array) else {
        return Err(refuse("events", None, Vec::new()));
    };
    let mut events = Vec::with_capacity(stored.len());
    let mut undecodable = None;
    for (index, event) in stored.iter().enumerate() {
        match decode_ui_preferences_config_mutation_json(&event.to_string()) {
            Ok(mutation) => events.push(mutation),
            Err(_) => {
                undecodable.get_or_insert(index);
            }
        }
    }
    match undecodable {
        None => Ok(UiPreferencesEventLog { version: 1, events }),
        Some(index) => Err(refuse("event", Some(index), events)),
    }
}

fn encode_ui_preferences_event_log(log: &UiPreferencesEventLog) -> String {
    let events: Vec<Value> = log.events.iter().map(|event| dsl_value_as_json(&semio_framework_value::ToValue::to_value(event))).collect();
    serde_json::json!({ "version": log.version, "events": events }).to_string()
}

/// ✍️ The ONE write rule over a stored preference log: an absent log is an empty one, a stored one is decoded
/// whole or the write refuses (the stored text is never replaced by a log this renderer could not read); `append`
/// adds this write's events, and a write that added none stores nothing.
fn ui_preferences_log_write(stored: Option<&str>, append: impl FnOnce(&mut UiPreferencesEventLog)) -> Result<Option<String>, UiPreferencesLogRefusal> {
    let mut log = match stored {
        None => UiPreferencesEventLog { version: 1, events: Vec::new() },
        Some(raw) => decode_ui_preferences_event_log(raw)?,
    };
    let before = log.events.len();
    append(&mut log);
    Ok((log.events.len() > before).then(|| encode_ui_preferences_event_log(&log)))
}

/// 💾️ Commits one preference write through [`ui_preferences_log_write`]: stored, or one refusal line and the stored
/// log left exactly as it was.
pub(crate) fn commit_ui_preferences_write(append: impl FnOnce(&mut UiPreferencesEventLog)) {
    match ui_preferences_log_write(prefs_get(UI_PREFERENCES_CONFIG_SCHEMA).as_deref(), append) {
        Ok(Some(encoded)) => prefs_set(UI_PREFERENCES_CONFIG_SCHEMA, &encoded),
        Ok(None) => {}
        Err(refusal) => ShellState::debug_log(&format!("[wgpu-shell] preference write refused: the stored log is unreadable ({}{}) and stays untouched", refusal.reason, refusal.event_index.map(|index| format!(" at event {index}")).unwrap_or_default())),
    }
}

/// 📖️ The stored log for READING: an unreadable one still answers the events this renderer decoded.
fn read_ui_preferences_event_log() -> UiPreferencesEventLog {
    match prefs_get(UI_PREFERENCES_CONFIG_SCHEMA) {
        None => UiPreferencesEventLog { version: 1, events: Vec::new() },
        Some(raw) => decode_ui_preferences_event_log(&raw).unwrap_or_else(|refusal| refusal.readable),
    }
}

pub(crate) fn replay_ui_preferences(log: &UiPreferencesEventLog) -> UiPreferences {
    let mut preferences = UiPreferences::default();
    for mutation in &log.events {
        apply_ui_preferences_config_mutation(&mut preferences, mutation).expect("canonical OS UI-preferences mutation must apply");
    }
    preferences
}

pub(crate) fn read_ui_preferences() -> UiPreferences {
    replay_ui_preferences(&read_ui_preferences_event_log())
}

/// 🌓️ The persisted appearance this shell's OWN preference store answered, or `None` when it holds
/// no `setAppearance` event — React's `preferences.appearance`, before its `?? "system"` fallback,
/// so [`resolve_appearance_id`] can slot the host's read in between.
pub(crate) fn appearance_id(value: Option<OsUiAppearance>) -> Option<String> {
    match value? {
        OsUiAppearance::Light => Some("light".to_string()),
        OsUiAppearance::Dark => Some("dark".to_string()),
        OsUiAppearance::System => Some("system".to_string()),
    }
}

/// 🌓️ The ONE appearance-resolution law, mirroring `🏛️ShellHost/🟦️.tsx:2174`'s
/// `locks.appearance ?? resolveUiPreferences(preferences, { appearance: "system", … }).appearance`
/// term for term — a lock wins, then this process's own preference store, then the HOST's read of
/// that same store, then React's own `"system"` seed.
///
/// The host term is the page's own replay of that same log, published through
/// `semioWgpuSetHostAppearance`. It is a fallback and never an override, and since the
/// [`🚪️StorageDoor`] seeds this isolate's store before the shell boots, this process's own term now
/// answers on the browser too — the host term survives for the embeddable door and for a realm whose
/// snapshot was refused.
pub(crate) fn resolve_appearance_id(preferences: &UiPreferences) -> String {
    env_lock("SEMIO_LOCKED_APPEARANCE").or_else(|| appearance_id(preferences.appearance)).or_else(|| crate::host_appearance_preference().map(ToOwned::to_owned)).unwrap_or_else(|| "system".to_string())
}

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

pub(crate) fn resolve_locale_id(lock: Option<String>, preferences: &UiPreferences, selected: Locale) -> String {
    lock.or_else(|| {
        preferences.locale.map(|locale| match locale {
            OsUiLocale::De => "de".to_string(),
            OsUiLocale::En => "en".to_string(),
        })
    })
    .or_else(|| crate::host_locale().map(ToOwned::to_owned))
    .unwrap_or_else(|| selected.as_str().to_string())
}

pub(crate) fn custom_themes_from(preferences: &UiPreferences) -> HashMap<String, String> {
    preferences
        .custom_themes
        .iter()
        .filter_map(|(id, theme)| {
            let mut config = theme.config.as_object()?.to_vec();
            config.retain(|(key,_)|key!="id"&&key!="label");
            config.push(("id".to_owned(),DslValue::String(theme.theme_id.clone())));
            config.push(("label".to_owned(),DslValue::String(theme.label.clone())));
            Some((id.clone(),semio_framework_pack_json::to_json_string(&DslValue::Object(config))))
        })
        .collect()
}

fn canonical_custom_themes(preferences: &ChromePrefsState) -> HashMap<String, OsUiTheme> {
    preferences
        .custom_themes
        .iter()
        .filter_map(|(id, raw)| {
            let document = ThemeDocument::parse(raw)?;
            let label = document.label.clone();
            let DslValue::Object(mut entries)=document.to_value() else{return None};
            entries.retain(|(key,_)|key!="id"&&key!="label");
            let config=DslValue::Object(entries);
            Some((id.clone(), OsUiTheme { theme_id: id.clone(), label, config }))
        })
        .collect()
}

pub(crate) fn persist_ui_preferences(state: &ShellState, previous: Option<&UiPrefsSnapshot>) {
    commit_ui_preferences_write(|log| {
        let changed = |current: &str, prior: fn(&UiPrefsSnapshot) -> &str| previous.is_none_or(|snapshot| current != prior(snapshot));
        if env_lock("SEMIO_LOCKED_APPEARANCE").is_none() && changed(&state.appearance_id, |snapshot| &snapshot.appearance_id) {
            log.events.push(set_appearance(Some(match state.appearance_id.as_str() {
                "light" => OsUiAppearance::Light,
                "dark" => OsUiAppearance::Dark,
                _ => OsUiAppearance::System,
            })));
        }
        if changed(&state.chrome_build.preferences.ui_layout, |snapshot| &snapshot.ui_layout) {
            log.events.push(set_layout(Some(if state.chrome_build.preferences.ui_layout == "tablet" { OsUiChromeLayout::Tablet } else { OsUiChromeLayout::Desktop })));
        }
        if changed(&state.driver_id, |snapshot| &snapshot.driver_id) {
            log.events.push(set_driver(Some(state.driver_id.clone())));
        }
        if previous.is_none_or(|snapshot| snapshot.custom_drivers != state.chrome_build.preferences.custom_drivers) {
            let before = previous.map(|snapshot| &snapshot.custom_drivers).cloned().unwrap_or_default();
            let after = &state.chrome_build.preferences.custom_drivers;
            let ids: std::collections::BTreeSet<String> = before.keys().chain(after.keys()).cloned().collect();
            let projected = replay_ui_preferences(&log);
            for id in ids {
                let next = after.get(&id).cloned();
                if projected.custom_drivers.get(&id) != next.as_ref() {
                    log.events.push(set_custom_driver(id, next));
                }
            }
        }
        if env_lock("SEMIO_LOCKED_LOCALE").is_none() && changed(&state.locale_id, |snapshot| &snapshot.locale_id) {
            log.events.push(set_locale(Some(if state.locale_id == "de" { OsUiLocale::De } else { OsUiLocale::En })));
        }
        if env_lock("SEMIO_LOCKED_TERMINOLOGY").is_none() && changed(&state.terminology_id, |snapshot| &snapshot.terminology_id) {
            log.events.push(set_terminology(Some(state.terminology_id.clone())));
        }
        if env_lock("SEMIO_LOCKED_THEME").is_none() && changed(&state.chrome_build.preferences.theme_id, |snapshot| &snapshot.theme_id) {
            log.events.push(set_theme(Some(state.chrome_build.preferences.theme_id.clone())));
        }
        if previous.is_none_or(|snapshot| snapshot.custom_themes != state.chrome_build.preferences.custom_themes) {
            let before = previous.map(|snapshot| &snapshot.custom_themes).cloned().unwrap_or_default();
            let after = canonical_custom_themes(&state.chrome_build.preferences);
            let ids: std::collections::BTreeSet<String> = before.keys().chain(after.keys()).cloned().collect();
            let projected = replay_ui_preferences(&log);
            for id in ids {
                let next = after.get(&id).cloned();
                if projected.custom_themes.get(&id) != next.as_ref() {
                    log.events.push(set_custom_theme(id, next));
                }
            }
        }
        if previous.is_none_or(|snapshot| snapshot.keybinding_overrides != state.chrome_build.preferences.keybinding_overrides) {
            let before = previous.map(|snapshot| &snapshot.keybinding_overrides).cloned().unwrap_or_default();
            let after = &state.chrome_build.preferences.keybinding_overrides;
            let ids: std::collections::BTreeSet<String> = before.keys().chain(after.keys()).cloned().collect();
            let projected = replay_ui_preferences(&log);
            for id in ids {
                let next = after.get(&id).cloned();
                if projected.keybinding_overrides.get(&id) != next.as_ref() {
                    log.events.push(set_keybinding_override(id, next));
                }
            }
        }
    });
}

impl ShellState {
    /// 💾️ Loads persisted uiPrefs into `self` exactly once per process, mirroring `os-shell.tsx`'s
    /// boot-time `locks.appearance ?? readStoredUiChromeAppearance()` fallback chain (`:862-868`).
    /// A locked pref (`SEMIO_LOCKED_*`) wins over storage, matching `resolveShellLocks`.
    #[cfg(test)]
    fn load_ui_prefs_once(&mut self) {
        if self.chrome_present.preferences_loaded {
            return;
        }
        self.chrome_present.preferences_loaded = true;
        let locks = shell_pref_locks();
        let preferences = read_ui_preferences();
        self.appearance_id = resolve_appearance_id(&preferences);
        self.locale_id = resolve_locale_id(locks.locale.clone(), &preferences, self.active_locale());
        crate::interpreter::install_ui_engine_locale(self.active_locale());
        self.terminology_id = locks.terminology.clone().or(preferences.terminology).unwrap_or_else(|| self.active_terminology().as_str().to_string());
        self.driver_id = preferences.driver_id.unwrap_or_else(|| "default".to_string());
        self.chrome_build.preferences = with_chrome_prefs(|preferences| preferences.clone());
        self.chrome_build.driver = resolve_ui_driver_chrome(&self.driver_id, &self.chrome_build.preferences.custom_drivers);
        if let Some(locked_theme) = &locks.theme_id {
            set_active_theme_id(locked_theme);
            self.chrome_build.preferences.theme_id = locked_theme.clone();
        }
        self.chrome_present.last_synced_preferences = Some(UiPrefsSnapshot::capture(self));
    }

    /// 💾️ Writes any changed uiPrefs field to the store (skipping locked ones), mirroring
    /// `os-shell.tsx`'s persistence `useEffect` (`:3477-3491`): one combined-dependency effect that
    /// rewrites every non-locked pref whenever any of them changes, not a per-field diff.
    #[cfg(test)]
    fn persist_ui_prefs_if_changed(&mut self) {
        let snapshot = UiPrefsSnapshot::capture(self);
        if self.chrome_present.last_synced_preferences.as_ref() == Some(&snapshot) {
            return;
        }
        persist_ui_preferences(self, self.chrome_present.last_synced_preferences.as_ref());
        stored_field_set(UI_COMPUTE_WORKER_COUNT_STORAGE_KEY, &snapshot.worker_count.to_string());
        self.chrome_present.last_synced_preferences = Some(snapshot);
    }
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
