//! ⏪️ `shell-history` staging of `storage-prefs-and-i18n`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellChrome
//#region 🗄️PrefsStore
pub const OS_SHELL_CONFIG_STORAGE_KEY: &str = "semio.os.config";

pub const OS_SHELL_CONFIG_MAX_BYTES: usize = 64 * 1024;

#[cfg(not(target_arch = "wasm32"))]
static NATIVE_PREF_DOCUMENTS: std::sync::LazyLock<std::sync::Mutex<semio_framework_os_services::RetainedFixedFileDocuments>> = std::sync::LazyLock::new(|| std::sync::Mutex::new(semio_framework_os_services::RetainedFixedFileDocuments::default()));

#[cfg(not(target_arch = "wasm32"))]
static NATIVE_PREF_DOCUMENT_GENERATION: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);

/// 📁️ Resolves the native prefs file path: `$SEMIO_PREFS_DIR/ui-prefs.json` when set, else a
/// per-OS config-home fallback (XDG on linux/devcontainer, `%APPDATA%` on windows, `~/.config` on
/// macOS) — no new dependency (no `dirs` crate), just `std::env`.
#[cfg(not(target_arch = "wasm32"))]
pub fn native_prefs_file_path() -> std::path::PathBuf {
    if let Ok(dir) = std::env::var("SEMIO_PREFS_DIR") {
        return std::path::PathBuf::from(dir).join("ui-prefs.json");
    }
    let base = std::env::var("XDG_CONFIG_HOME").or_else(|_| std::env::var("APPDATA")).or_else(|_| std::env::var("HOME").map(|home| format!("{home}/.config"))).unwrap_or_else(|_| ".".to_string());
    std::path::PathBuf::from(base).join("semio").join("ui-prefs.json")
}

pub fn empty_os_shell_config() -> Value {
    serde_json::json!({
        "version": 1,
        "preferences": {},
        "dockLayouts": { "apps": {} },
        "dockUi": { "apps": {} },
        "windowPanes": { "apps": {} }
    })
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
pub fn raw_prefs_get(key: &str) -> Option<String> {
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
pub fn raw_prefs_set(key: &str, value: &str) {
    #[cfg(target_arch = "wasm32")]
    host_storage_set(key, value);
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = native_pref_write_page(key, value);
    }
}

/// 🎚️ One field of the `semio.os.config` document's `preferences` map — React's
/// `OsShellConfig.getPreference` (`🖥️platform/🟦️.ts`). Its ONE key is `os.config.ui-preferences`, the
/// UI-preferences event log; every other durable key React keeps is a flat entry reached through
/// [`stored_field_get`].
pub fn prefs_get(key: &str) -> Option<String> {
    let raw = raw_prefs_get(OS_SHELL_CONFIG_STORAGE_KEY)?;
    if raw.len() > OS_SHELL_CONFIG_MAX_BYTES {
        return None;
    }
    serde_json::from_str::<Value>(&raw).ok()?.get("preferences")?.get(key)?.as_str().map(ToOwned::to_owned)
}

/// 🎚️ React's `OsShellConfig.setPreference`: re-read the complete document, apply one field, write it
/// back — so a sibling projection (`dockLayouts`, `dockUi`, `windowPanes`) is never
/// clobbered by a preference write.
pub fn prefs_set(key: &str, value: &str) {
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

#[cfg(target_arch = "wasm32")]
pub fn host_storage_write_through(verb: StorageDoorVerb, key: &str, value: Option<&str>) {
    let request = match encode_storage_door_request(verb, StorageDoorScope::Local, key, value) {
        Ok(request) => request,
        Err(error) => {
            debug_log(&format!("[TRACE] wgpu-shell storage write refused: {error}"));
            return;
        }
    };
    let key = key.to_string();
    semio_framework_os_renderer_wgpu_host_base::spawn_app_task(async move {
        match host_io_call(&request, None).await.map_err(|error| error).and_then(|answer| decode_storage_door_answer(&answer)) {
            Ok(_) => {}
            Err(error) => debug_log(&format!("[TRACE] wgpu-shell storage {key} not persisted: {error}")),
        }
    });
}

#[cfg(not(target_arch = "wasm32"))]
pub fn host_storage_write_through(_verb: StorageDoorVerb, _key: &str, _value: Option<&str>) {}

//#endregion 🚪️StorageDoor
//#endregion ShellChrome

//#region ShellChrome
//#region 🎨️ThemeRegistry
//#endregion 🎨️ThemeRegistry
//#endregion ShellChrome

//#region ShellChrome
//#region 🌉️AgentBridgeConfigDoor
//#endregion 🌉️AgentBridgeConfigDoor
//#endregion ShellChrome

//#region ShellChrome
//#region 🛰️AgentBridgeOfferScope
//#endregion 🛰️AgentBridgeOfferScope
//#endregion ShellChrome

//#region ShellChrome
//#region 🚪️StorageDoor
thread_local! {
    pub static HOST_STORAGE: std::cell::RefCell<BTreeMap<String, String>> = const { std::cell::RefCell::new(BTreeMap::new()) };
}
//#endregion 🚪️StorageDoor
//#endregion ShellChrome

//#region ShellChrome
//#region 🗄️PrefsStore
#[cfg(any(test, feature = "testkit"))]
pub fn prefs_set_in(store: &mut impl PrefsStore, key: &str, value: &str) {
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

#[cfg(any(test, feature = "testkit"))]
pub fn prefs_get_from(store: &impl PrefsStore, key: &str) -> Option<String> {
    let raw = store.get(OS_SHELL_CONFIG_STORAGE_KEY)?;
    if raw.len() > OS_SHELL_CONFIG_MAX_BYTES {
        return None;
    }
    serde_json::from_str::<Value>(&raw).ok()?.get("preferences")?.get(key)?.as_str().map(ToOwned::to_owned)
}

#[cfg(any(test, feature = "testkit"))]
pub const OS_SHELL_PREFS_FILE_MAX_BYTES: usize = OS_SHELL_CONFIG_MAX_BYTES + 4 * 1024;

#[cfg(all(not(target_arch = "wasm32"), any(test, feature = "testkit")))]
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

#[cfg(all(not(target_arch = "wasm32"), any(test, feature = "testkit")))]
impl FilePrefsStore {
    pub fn new() -> Self {
        Self::with_path(native_prefs_file_path())
    }

    pub fn with_path(path: std::path::PathBuf) -> Self {
        let (sender, receiver) = std::sync::mpsc::channel();
        let load_path = path.clone();
        semio_framework_os_renderer_wgpu_host_base::renderer_worker_pool().submit(
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

    pub fn poll_load(&self) {
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

    pub fn flush(&self) {
        let mut state = self.flush_state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        state.latest = Some(self.cache.borrow().clone());
        if state.running {
            return;
        }
        state.running = true;
        drop(state);
        let path = self.path.clone();
        let flush_state = std::sync::Arc::clone(&self.flush_state);
        semio_framework_os_renderer_wgpu_host_base::renderer_worker_pool().submit(
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

#[cfg(all(not(target_arch = "wasm32"), any(test, feature = "testkit")))]
pub struct FilePrefsStore {
    pub path: std::path::PathBuf,
    pub cache: std::cell::RefCell<HashMap<String, String>>,
    pub pending_load: std::cell::RefCell<Option<std::sync::mpsc::Receiver<HashMap<String, String>>>>,
    pub dirty: std::cell::Cell<bool>,
    pub flush_state: std::sync::Arc<std::sync::Mutex<FilePrefsFlushState>>,
}
//#endregion 🗄️PrefsStore
//#endregion ShellChrome

//#region ShellChrome
//#region 🚪️StorageDoor
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
//#endregion 🚪️StorageDoor
//#endregion ShellChrome
