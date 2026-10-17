//! ⏪️ `shell-history` staging of `local-folders`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;
use semio_framework_os_config::opening_config::mutations::{apply_local_folders_config_mutation, attach_local_folder, detach_local_folder, LocalFolderBinding, LocalFolderBindings, LocalFolderRef, LocalFoldersConfigMutation, LOCAL_FOLDERS_CONFIG_SCHEMA};

//#region 🔖️EventLog
/// 🧾️ The one version of the persisted event log `{version, events}`.
const LOCAL_FOLDERS_EVENT_LOG_VERSION: u64 = 1;

/// 📏️ The payload schemas' bounds (`📎️attach-local-folder`, `✂️detach-local-folder`).
const LOCAL_FOLDER_DOCUMENT_ID_MAX: usize = 512;

const LOCAL_FOLDER_PROGRAM_ID_MAX: usize = 256;

const LOCAL_FOLDER_PATH_MAX: usize = 4096;

fn bounded_text(value: &Value, key: &str, maximum: usize) -> Option<String> {
    value.get(key)?.as_str().filter(|text| !text.is_empty() && text.chars().count() <= maximum).map(str::to_string)
}

fn has_exactly(value: &Value, keys: &[&str]) -> bool {
    value.as_object().is_some_and(|object| object.len() == keys.len() && keys.iter().all(|key| object.contains_key(*key)))
}

/// 🔎️ One persisted event read against the payload schemas; `None` when it is not one — React's
/// `localFoldersMutationOfValueV1`, field for field (exact keys, bounded non-empty texts, a `path` folder).
pub fn local_folders_mutation_of_value(value: &Value) -> Option<LocalFoldersConfigMutation> {
    match value.get("mutation")?.as_str()? {
        "detachLocalFolder" if has_exactly(value, &["mutation", "documentId"]) => Some(detach_local_folder(&bounded_text(value, "documentId", LOCAL_FOLDER_DOCUMENT_ID_MAX)?)),
        "attachLocalFolder" if has_exactly(value, &["mutation", "documentId", "pluginId", "appId", "folder"]) => {
            let folder = value.get("folder")?;
            if !has_exactly(folder, &["kind", "path"]) || folder.get("kind")?.as_str()? != "path" {
                return None;
            }
            Some(attach_local_folder(LocalFolderBinding {
                document_id: bounded_text(value, "documentId", LOCAL_FOLDER_DOCUMENT_ID_MAX)?,
                plugin_id: bounded_text(value, "pluginId", LOCAL_FOLDER_PROGRAM_ID_MAX)?,
                app_id: bounded_text(value, "appId", LOCAL_FOLDER_PROGRAM_ID_MAX)?,
                folder: LocalFolderRef::Path { path: bounded_text(folder, "path", LOCAL_FOLDER_PATH_MAX)? },
            }))
        }
        _ => None,
    }
}

/// 📖️ The facet's committed events in a stored log; a log that does not read whole as one is empty, so nothing is
/// reattached from it — React's `readLocalFolderEventsV1`.
pub fn local_folder_events_of_log(raw: Option<&str>) -> Vec<LocalFoldersConfigMutation> {
    let Some(value) = raw.and_then(|raw| serde_json::from_str::<Value>(raw).ok()) else { return Vec::new() };
    if value.get("version").and_then(Value::as_u64) != Some(LOCAL_FOLDERS_EVENT_LOG_VERSION) {
        return Vec::new();
    }
    value.get("events").and_then(Value::as_array).and_then(|events| events.iter().map(local_folders_mutation_of_value).collect::<Option<Vec<_>>>()).unwrap_or_default()
}

/// 🧮️ The bindings committed events fold to — React's `replayLocalFolderEventsV1`.
pub fn replay_local_folder_events(events: &[LocalFoldersConfigMutation]) -> LocalFolderBindings {
    let mut bindings = LocalFolderBindings::default();
    for event in events {
        let _ = apply_local_folders_config_mutation(&mut bindings, event);
    }
    bindings
}

/// ✍️ The stored log after appending `mutation`, or `None` when the mutation changes nothing (it is then not recorded) —
/// React's `commitLocalFoldersConfigMutationV1`.
pub fn local_folders_log_after(raw: Option<&str>, mutation: &LocalFoldersConfigMutation) -> Option<String> {
    let mut events = local_folder_events_of_log(raw);
    let before = replay_local_folder_events(&events);
    let mut after = before.clone();
    apply_local_folders_config_mutation(&mut after, mutation).ok()?;
    if after == before {
        return None;
    }
    events.push(mutation.clone());
    serde_json::to_string(&serde_json::json!({ "version": LOCAL_FOLDERS_EVENT_LOG_VERSION, "events": events })).ok()
}

//#endregion 🔖️EventLog
//#region 🔖️Store
/// 📖️ The stored event log.
fn local_folders_log_read() -> Option<String> {
    #[cfg(any(test, feature = "testkit"))]
    return TEST_LOCAL_FOLDERS_LOG.with(|slot| slot.borrow().clone());
    #[cfg(not(any(test, feature = "testkit")))]
    prefs_get(LOCAL_FOLDERS_CONFIG_SCHEMA)
}

/// ✍️ Replaces the stored event log.
fn local_folders_log_write(raw: &str) {
    #[cfg(any(test, feature = "testkit"))]
    TEST_LOCAL_FOLDERS_LOG.with(|slot| *slot.borrow_mut() = Some(raw.to_string()));
    #[cfg(not(any(test, feature = "testkit")))]
    prefs_set(LOCAL_FOLDERS_CONFIG_SCHEMA, raw);
}

//#endregion 🔖️Store
impl HistoryState {
    /// 📖️ Loads the remembered bindings once; every commit keeps the cache current.
    pub fn ensure_local_folder_bindings(&mut self) {
        if self.local_folder_bindings.is_none() {
            self.local_folder_bindings = Some(replay_local_folder_events(&local_folder_events_of_log(local_folders_log_read().as_deref())));
        }
    }

    /// ✍️ Appends one mutation to the facet's local-only log; a mutation that changes nothing is not recorded.
    pub fn commit_local_folders_mutation(&mut self, mutation: &LocalFoldersConfigMutation) {
        let raw = local_folders_log_read();
        if let Some(next) = local_folders_log_after(raw.as_deref(), mutation) {
            local_folders_log_write(&next);
        }
        self.local_folder_bindings = Some(replay_local_folder_events(&local_folder_events_of_log(local_folders_log_read().as_deref())));
    }

    /// 📎️ Remembers the folder a program's document was attached to.
    pub fn remember_local_folder(&mut self, binding: LocalFolderBinding) {
        self.commit_local_folders_mutation(&attach_local_folder(binding));
    }

    /// ✂️ Forgets a document's folder.
    pub fn forget_local_folder(&mut self, document_id: &str) {
        self.commit_local_folders_mutation(&detach_local_folder(document_id));
    }
}

//#region 🔖️HostRouteFolder
/// 🗃️ A document kept through the dev host's backbone route: its uri, identity and artifact schema, the digest of the
/// archive last written there and the history cursor it was written at.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HostRouteFolder {
    pub uri: String,
    pub document_id: String,
    pub schema: String,
    pub written: u64,
    pub written_cursor: u64,
}

//#endregion 🔖️HostRouteFolder

//#region 🔖️Store
#[cfg(any(test, feature = "testkit"))]
thread_local! {
    /// 🧪️ The preference slot a law's shells share, outliving each of them like the real store outlives a restart.
    pub static TEST_LOCAL_FOLDERS_LOG: std::cell::RefCell<Option<String>> = const { std::cell::RefCell::new(None) };
}
//#endregion 🔖️Store
