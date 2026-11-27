//! 🧰️ Native extension store door: the native twin of the browser's `semioWgpuHostIo` `extension-store-*` ops and `install_js_extension`. It answers the SAME requests with the SAME answers (`{extensions}`, `{extension}`, `{extensionId}`, `{cancelled}`) over the SAME on-disk store layout the TypeScript store (`🔌️plugin/🏪️store`, `nativeMaterialize`) writes — `<root>/<directoryName>/component.wasm` beside `📥️install.json`, plus the watch marker — so the shell's marketplace flow, records and events do not know which backend answered. Installed programs are admitted as bridge entries and registered with [`register_extension_program`](super::register_extension_program). See [`super`].

use super::*;
use std::path::{Path, PathBuf};

pub const NATIVE_EXTENSION_STORE_DIRECTORY: &str = "🧩️extension-modules";
pub const NATIVE_EXTENSION_INSTALL_META: &str = "📥️install.json";
pub const NATIVE_EXTENSION_COMPONENT_FILE: &str = "component.wasm";
pub const NATIVE_EXTENSION_WATCH_MARKER: &str = "👀️extension-watch.json";
/// 🔐️ The capability names the package manifest requests, one JSON array beside `📥️install.json` (the shared install record has no field for them; absent means none).
pub const NATIVE_EXTENSION_CAPABILITIES_FILE: &str = "🔐️capabilities.json";
/// 📏️ The store's own list bound (the shell ledger and the page door refuse more).
pub const NATIVE_EXTENSION_LIST_LIMIT: usize = 256;
/// 📏️ The largest `.sxt` package this door reads.
pub const NATIVE_EXTENSION_PACKAGE_MAX_BYTES: u64 = 64 * 1024 * 1024;

/// 🗂️ The install root beside the plugin modules root (the TypeScript store's default: "beside `🔌️plugin-modules`").
pub fn native_extension_install_root(modules_root: &Path) -> PathBuf {
    modules_root.parent().unwrap_or(modules_root).join(NATIVE_EXTENSION_STORE_DIRECTORY)
}

/// 📇️ One installed extension exactly as the store records it (`InstalledExtensionRecord` of the TypeScript store and `ShellExtensionStoreRecord` of the shell).
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NativeExtensionRecord {
    pub extension_id: String,
    pub directory_name: String,
    pub version: String,
    /// 🏷️ The extension name in every terminology and locale — no default language.
    pub label: semio_os_extension::LocalizedLabel,
    #[serde(rename = "extends")]
    pub extends_host: String,
    pub module_url: String,
    pub package_hash: String,
    pub installed_at: u64,
}

//#region 📇️Records
fn refuse_symlink(path: &Path) -> Result<(), String> {
    match std::fs::symlink_metadata(path) {
        Ok(meta) if meta.file_type().is_symlink() => Err(format!("extension store path must not be a symlink: {}", path.display())),
        _ => Ok(()),
    }
}

/// 🔎️ Every installed extension under `root`, sorted by extension id: each non-hidden directory carrying `📥️install.json` whose `directoryName` is its own name.
pub fn native_extension_records(root: &Path) -> Result<Vec<NativeExtensionRecord>, String> {
    refuse_symlink(root)?;
    let Ok(entries) = std::fs::read_dir(root) else { return Ok(Vec::new()) };
    let mut rows = Vec::new();
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().to_string();
        if name.starts_with('_') || name.starts_with('.') || !entry.file_type().is_ok_and(|kind| kind.is_dir()) {
            continue;
        }
        let meta = entry.path().join(NATIVE_EXTENSION_INSTALL_META);
        if !meta.exists() {
            continue;
        }
        refuse_symlink(&entry.path())?;
        refuse_symlink(&meta)?;
        let record: NativeExtensionRecord = serde_json::from_slice(&std::fs::read(&meta).map_err(|error| error.to_string())?).map_err(|error| format!("extension installation metadata {}: {error}", meta.display()))?;
        if record.directory_name != name || record.extension_id.is_empty() {
            return Err("Extension installation metadata identity mismatch".to_string());
        }
        rows.push(record);
    }
    rows.sort_by(|left, right| left.extension_id.cmp(&right.extension_id));
    if rows.len() > NATIVE_EXTENSION_LIST_LIMIT {
        return Err("extension store returned an invalid or oversized list".to_string());
    }
    Ok(rows)
}

fn write_watch_marker(root: &Path, event: serde_json::Value) -> Result<(), String> {
    std::fs::create_dir_all(root).map_err(|error| error.to_string())?;
    let emitted_at = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |elapsed| elapsed.as_millis() as u64);
    let mut event = event;
    event["emittedAt"] = emitted_at.into();
    std::fs::write(root.join(NATIVE_EXTENSION_WATCH_MARKER), format!("{event}\n")).map_err(|error| error.to_string())
}

/// 🏷️ A directory name the store would accept: one plain path segment, neither hidden nor reserved.
fn directory_name_is_plain(name: &str) -> bool {
    !name.is_empty() && !name.starts_with('.') && !name.starts_with('_') && !name.contains(['/', '\\', '\0']) && name != ".." && !name.chars().any(char::is_control)
}
//#endregion 📇️Records

//#region 📥️Install
/// 📥️ Verifies and unpacks `.sxt` bytes (the same `semio-os-extension` verification the plugin host runs, including the `extends == dependencies[0]` gate), materializes the raw `component.wasm` and records the install — the Rust twin of the store's `install` with `nativeMaterialize`.
pub async fn native_extension_install_package(root: &Path, bytes: &[u8]) -> Result<NativeExtensionRecord, String> {
    refuse_symlink(root)?;
    let package = semio_os_extension::unpack(bytes).await.map_err(|error| format!("extension package: {error}"))?;
    let manifest = package.manifest;
    if !manifest.extends_matches_primary_dependency().await {
        return Err(format!("extension {:?} declares extends={:?} that differs from its first dependency", manifest.extension_id, manifest.extends));
    }
    if manifest.extension_id.is_empty() || !directory_name_is_plain(&manifest.directory_name) {
        return Err("extension package names an invalid identity or directory".to_string());
    }
    if native_extension_records(root)?.iter().any(|record| record.extension_id == manifest.extension_id && record.directory_name != manifest.directory_name) {
        return Err("Extension identity already owns a different directory".to_string());
    }
    let out = root.join(&manifest.directory_name);
    std::fs::create_dir_all(&out).map_err(|error| error.to_string())?;
    refuse_symlink(&out)?;
    std::fs::write(out.join(NATIVE_EXTENSION_COMPONENT_FILE), &package.component_wasm).map_err(|error| error.to_string())?;
    let installed_at = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |elapsed| elapsed.as_millis() as u64);
    let record = NativeExtensionRecord {
        module_url: format!("/{NATIVE_EXTENSION_STORE_DIRECTORY}/{}/{NATIVE_EXTENSION_COMPONENT_FILE}", manifest.directory_name),
        package_hash: semio_os_extension::content_hash(bytes),
        extension_id: manifest.extension_id,
        directory_name: manifest.directory_name,
        version: manifest.version,
        label: manifest.label,
        extends_host: manifest.extends,
        installed_at,
    };
    std::fs::write(out.join(NATIVE_EXTENSION_CAPABILITIES_FILE), format!("{}\n", serde_json::to_string(&manifest.capabilities).map_err(|error| error.to_string())?)).map_err(|error| error.to_string())?;
    std::fs::write(out.join(NATIVE_EXTENSION_INSTALL_META), format!("{}\n", serde_json::to_string_pretty(&record).map_err(|error| error.to_string())?)).map_err(|error| error.to_string())?;
    write_watch_marker(root, serde_json::json!({ "kind": "installed", "extensionId": record.extension_id, "version": record.version, "installedAt": record.installed_at }))?;
    Ok(record)
}

fn native_extension_uninstall(root: &Path, extension_id: &str) -> Result<(), String> {
    if extension_id.is_empty() {
        return Err("extensionId is required".to_string());
    }
    refuse_symlink(root)?;
    if let Some(record) = native_extension_records(root)?.into_iter().find(|record| record.extension_id == extension_id) {
        std::fs::remove_dir_all(root.join(&record.directory_name)).map_err(|error| error.to_string())?;
    }
    write_watch_marker(root, serde_json::json!({ "kind": "uninstalled", "extensionId": extension_id }))
}

/// 🚪️ Answers one `extension-store-*` request exactly as the browser page door does. `extension-store-install-file` reads the package at the request's `path` (no `path`: `{ "cancelled": true }`, like a cancelled pick); `extension-store-install-url` takes a `file://` URL (a remote URL is refused: a native host moves no package bytes over the network here).
pub async fn native_extension_store_answer(request: &serde_json::Value, root: &Path) -> Result<serde_json::Value, String> {
    let text = |key: &str| request.get(key).and_then(serde_json::Value::as_str).map(str::trim).filter(|value| !value.is_empty());
    match request.get("op").and_then(serde_json::Value::as_str) {
        Some("extension-store-list") => Ok(serde_json::json!({ "extensions": native_extension_records(root)? })),
        Some("extension-store-uninstall") => {
            let id = text("extensionId").ok_or("extensionId is required")?;
            native_extension_uninstall(root, id)?;
            Ok(serde_json::json!({ "extensionId": id }))
        }
        Some(op @ ("extension-store-install-file" | "extension-store-install-url")) => {
            let path = if op == "extension-store-install-file" {
                text("path").map(PathBuf::from)
            } else {
                match text("url") {
                    None => None,
                    Some(url) => Some(PathBuf::from(url.strip_prefix("file://").ok_or("native extension installation takes a file:// URL or a file path")?)),
                }
            };
            let Some(path) = path else { return Ok(serde_json::json!({ "cancelled": true })) };
            let meta = std::fs::metadata(&path).map_err(|error| format!("extension package {}: {error}", path.display()))?;
            if meta.len() > NATIVE_EXTENSION_PACKAGE_MAX_BYTES {
                return Err("extension package exceeds the install size bound".to_string());
            }
            let bytes = std::fs::read(&path).map_err(|error| error.to_string())?;
            Ok(serde_json::json!({ "extension": native_extension_install_package(root, &bytes).await? }))
        }
        _ => Err("unknown extension store operation".to_string()),
    }
}
//#endregion 📥️Install

//#region 🔌️Admission
/// 🔌️ Admits one installed extension as a program — the native twin of `install_js_extension`: the store's `component.wasm` becomes a bridge entry whose identity must equal the record's, registered as the host of its `Component::Extension` nodes. The kernel activates it LIVE (`KernelRequest::ActivateExtension`, no parent re-creation) beside a live instance of its DECLARED host plugin, with the package's requested capabilities scoped to that host's grants; unregistering retires it.
pub fn install_native_extension(root: &Path, record: &NativeExtensionRecord) -> Result<ProgramBridgeEntry, String> {
    let component = root.join(&record.directory_name).join(NATIVE_EXTENSION_COMPONENT_FILE);
    refuse_symlink(&component)?;
    if !component.is_file() {
        return Err(format!("extension {}: {} is missing", record.extension_id, component.display()));
    }
    let manifest: PluginManifest = serde_json::from_value(serde_json::json!({ "pluginId": record.extension_id, "label": record.label, "version": record.version, "apps": [], "examples": [] })).map_err(|error| format!("extension {}: {error}", record.extension_id))?;
    let entry = ProgramBridgeEntry::from_wasm(record.extension_id.clone(), None, Some(record.package_hash.clone()), component, manifest)?;
    if entry.manifest.plugin_id != record.extension_id || entry.manifest.version != record.version {
        return Err(format!("extension {}: loaded manifest identity/version mismatch", record.extension_id));
    }
    let capabilities = std::fs::read(root.join(&record.directory_name).join(NATIVE_EXTENSION_CAPABILITIES_FILE)).ok().and_then(|bytes| serde_json::from_slice::<Vec<String>>(&bytes).ok()).unwrap_or_default();
    register_extension_program_extending(&record.extension_id, entry.clone(), None, Some(record.extends_host.clone()), capabilities);
    Ok(entry)
}

/// 🗑️ Retires an admitted extension — the native twin of `retire_js_extension`.
pub fn retire_native_extension(extension_id: &str) {
    unregister_extension_program(extension_id);
}
//#endregion 🔌️Admission

#[cfg(test)]
#[path = "../../../🧪️tests/🧰️native-extension-store/🦀️.rs"]
mod native_extension_store_tests;
