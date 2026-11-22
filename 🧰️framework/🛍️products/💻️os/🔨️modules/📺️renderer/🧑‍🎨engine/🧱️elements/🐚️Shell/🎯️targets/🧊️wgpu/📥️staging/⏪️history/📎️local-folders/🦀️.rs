//! ⏪️ `shell-history` staging of `local-folders`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;
use semio_os_config::opening_config::mutations::{apply_local_folders_config_mutation, attach_local_folder, detach_local_folder, LocalFolderBinding, LocalFolderBindings, LocalFolderRef, LocalFoldersConfigMutation, LOCAL_FOLDERS_CONFIG_SCHEMA};

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

//#region 🔖️LocalCatalog
/// 🗂️ The device's local document catalog (`os.config.local-catalog`, React's `recordLocalCatalogRef`): every document this device keeps, folded from `admitLocalDocument` and `retireLocalDocument` mutations and stored local-only as its snapshot.
#[cfg(any(test, feature = "testkit"))]
thread_local! {
    static TEST_LOCAL_CATALOG: std::cell::RefCell<Option<String>> = const { std::cell::RefCell::new(None) };
}

fn local_catalog_read() -> semio_os_config::opening_config::mutations::admit_local_document::LocalCatalog {
    #[cfg(any(test, feature = "testkit"))]
    let raw = TEST_LOCAL_CATALOG.with(|slot| slot.borrow().clone());
    #[cfg(not(any(test, feature = "testkit")))]
    let raw = prefs_get(semio_os_config::opening_config::mutations::admit_local_document::LOCAL_CATALOG_CONFIG_SCHEMA);
    raw.and_then(|raw| semio_os_config::io::text::snapshot::decode_local_catalog_json(&raw).ok()).unwrap_or_default()
}

fn local_catalog_commit(mutation: &semio_os_config::opening_config::mutations::LocalCatalogConfigMutation) -> bool {
    let mut catalog = local_catalog_read();
    if semio_os_config::opening_config::apply_local_catalog_config_mutation(&mut catalog, mutation).is_err() {
        return false;
    }
    let raw = semio_os_config::io::text::snapshot::encode_local_catalog_json(&catalog);
    #[cfg(any(test, feature = "testkit"))]
    TEST_LOCAL_CATALOG.with(|slot| *slot.borrow_mut() = Some(raw));
    #[cfg(not(any(test, feature = "testkit")))]
    prefs_set(semio_os_config::opening_config::mutations::admit_local_document::LOCAL_CATALOG_CONFIG_SCHEMA, &raw);
    true
}

impl HistoryState {
    /// 🗂️ Admits one kept document (its folder, selected surface and admission time) to the local catalog; `false` when the catalog refuses it (already kept, or invalid).
    pub fn record_local_document(&mut self, document_id: &str, schema: &str, name: &str, target: &str, plugin_id: &str, app_id: &str, admitted_at_ms: u64) -> bool {
        use semio_os_config::opening_config::mutations::admit_local_document::{admit_local_document, LocalDocument, LocalDocumentStorage, LocalDocumentSurface};
        let admitted = local_catalog_commit(&admit_local_document(LocalDocument {
            document_id: document_id.into(), schema: schema.into(), name: name.into(), storage: LocalDocumentStorage::Folder, target: target.into(), admitted_at_ms,
            surface: Some(LocalDocumentSurface { plugin_id: plugin_id.into(), app_id: app_id.into() }),
        }));
        self.refresh_local_files_from_catalog();
        admitted
    }

    /// 🗂️ Retires a kept document from the local catalog (React's `unlist`); `false` when it was not listed.
    pub fn unlist_local_document(&mut self, document_id: &str) -> bool {
        let retired = local_catalog_commit(&semio_os_config::opening_config::mutations::retire_local_document::retire_local_document(document_id));
        self.refresh_local_files_from_catalog();
        retired
    }

    /// 📖️ Whether the local catalog lists `document_id`.
    pub fn local_document_listed(&self, document_id: &str) -> bool {
        local_catalog_read().documents.iter().any(|document| document.document_id == document_id)
    }
}
//#endregion 🔖️LocalCatalog

//#region 🔖️LocalFiles
/// 🗂️ One kept document the shell lists and reopens: its identity, the name a person knows it by, the folder holding its archive and the surface (program and app) it opens on.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LocalFileRow {
    pub document_id: String,
    pub schema: String,
    pub name: String,
    pub target: String,
    pub plugin_id: String,
    pub app_id: String,
    pub admitted_at_ms: u64,
}

/// 🗂️ React's `LocalFilesV1` ordering: newest admitted first, the document id breaking a tie.
fn sorted_local_file_rows(mut rows: Vec<LocalFileRow>) -> Vec<LocalFileRow> {
    rows.sort_by(|left, right| right.admitted_at_ms.cmp(&left.admitted_at_ms).then_with(|| left.document_id.cmp(&right.document_id)));
    rows
}

/// 🗂️ The rows of a catalog snapshot JSON (`{documents: [...]}`, React's `LocalCatalog`) that name a surface, newest first and by document id within one instant — React's `LocalFilesV1` ordering.
pub fn local_file_rows_of_json(json: &str) -> Vec<LocalFileRow> {
    let Ok(value) = serde_json::from_str::<Value>(json) else { return Vec::new() };
    let text = |row: &Value, key: &str| row.get(key).and_then(Value::as_str).map(str::to_string);
    sorted_local_file_rows(value.get("documents").and_then(Value::as_array).into_iter().flatten().filter_map(|row| {
        let surface = row.get("surface")?;
        Some(LocalFileRow {
            document_id: text(row, "documentId")?, schema: text(row, "schema")?, name: text(row, "name")?, target: text(row, "target")?,
            plugin_id: text(surface, "pluginId")?, app_id: text(surface, "appId")?, admitted_at_ms: row.get("admittedAtMs").and_then(Value::as_u64).unwrap_or(0),
        })
    }).collect())
}

/// 🗂️ The same rows of the native local catalog (`os.config.local-catalog` folded from its mutations): every document that names a surface, in React's order.
pub fn local_file_rows_of_catalog(catalog: &semio_os_config::opening_config::mutations::admit_local_document::LocalCatalog) -> Vec<LocalFileRow> {
    sorted_local_file_rows(catalog.documents.iter().filter_map(|document| {
        let surface = document.surface.as_ref()?;
        Some(LocalFileRow {
            document_id: document.document_id.clone(), schema: document.schema.clone(), name: document.name.clone(), target: document.target.clone(),
            plugin_id: surface.plugin_id.clone(), app_id: surface.app_id.clone(), admitted_at_ms: document.admitted_at_ms,
        })
    }).collect())
}

/// 🗂️ The listing a fresh shell starts with: the native local catalog's documents; the browser's comes from the page's facet client.
pub fn initial_local_files() -> Vec<LocalFileRow> {
    #[cfg(target_arch = "wasm32")]
    return Vec::new();
    #[cfg(not(target_arch = "wasm32"))]
    local_file_rows_of_catalog(&local_catalog_read())
}

impl HistoryState {
    /// 🗂️ Replaces the listed local files with a catalog snapshot the page's facet client published.
    pub fn set_local_files_from_json(&mut self, json: &str) {
        self.local_files = local_file_rows_of_json(json);
    }

    /// 🗂️ Lists the native local catalog's documents: read at boot and after every admission or retirement, so the listing is the catalog's own state.
    pub fn refresh_local_files_from_catalog(&mut self) {
        self.local_files = local_file_rows_of_catalog(&local_catalog_read());
    }
}
//#endregion 🔖️LocalFiles
