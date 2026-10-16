//! 📄️ `shell-docs` staging of `local-folders`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;
use super::local_folders::*;
use semio_framework_os_config::opening_config::mutations::{apply_local_folders_config_mutation, attach_local_folder, detach_local_folder, LocalFolderBinding, LocalFolderBindings, LocalFolderRef, LocalFoldersConfigMutation, LOCAL_FOLDERS_CONFIG_SCHEMA};

//#region 🔖️Reconnect
/// 🪪️ The program and the document identity a booted session holds.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct LocalFolderIdentity {
    pub document_id: String,
    pub plugin_id: String,
    pub app_id: String,
}

/// 📎️ The binding to bring back: the one remembered for this document in this program, unless the document is attached
/// already — React's `localFolderReconnectOfferV1`.
pub(crate) fn local_folder_reconnect_offer(bindings: &LocalFolderBindings, identity: Option<&LocalFolderIdentity>, attached_document_id: Option<&str>) -> Option<LocalFolderBinding> {
    let identity = identity.filter(|identity| attached_document_id != Some(identity.document_id.as_str()))?;
    bindings.bindings.iter().find(|binding| binding.document_id == identity.document_id && binding.plugin_id == identity.plugin_id && binding.app_id == identity.app_id).cloned()
}

//#endregion 🔖️Reconnect
//#region 🔖️HostRouteFolder
/// 🚪️ The host-io door op the browser folder door rides — the one spelling `🚪️host-io/🟦️.ts` services.
#[cfg(any(target_arch = "wasm32", test))]
pub(crate) const BACKBONE_FOLDER_DOOR_OP: &str = "backbone-folder";

/// 🗃️ What one folder-door hop does: read the stored archive, or write the document's archive.
#[cfg(any(target_arch = "wasm32", test))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum BackboneFolderVerb {
    Read,
    Write,
}

/// 📨️ One folder-door request as JSON — a write names the artifact schema its archive is stored under.
#[cfg(any(target_arch = "wasm32", test))]
pub(crate) fn backbone_folder_request(verb: BackboneFolderVerb, uri: &str, document_id: &str, schema: Option<&str>) -> String {
    let mut request = serde_json::json!({ "op": BACKBONE_FOLDER_DOOR_OP, "verb": match verb { BackboneFolderVerb::Read => "read", BackboneFolderVerb::Write => "write" }, "uri": uri, "documentId": document_id });
    if let (BackboneFolderVerb::Write, Some(schema)) = (verb, schema) {
        request["schema"] = Value::from(schema);
    }
    request.to_string()
}

/// 📬️ A write's answer: acknowledged on a success status, else the refusal as a read tells it.
#[cfg(any(target_arch = "wasm32", test))]
pub(crate) fn backbone_folder_write_answer(answer: &str) -> Result<(), String> {
    let answer: Value = serde_json::from_str(answer).map_err(|error| error.to_string())?;
    if let Some(error) = answer.get("error").and_then(Value::as_str) {
        return Err(error.to_string());
    }
    match answer.get("status").and_then(Value::as_u64) {
        Some(status) if (200..300).contains(&status) => Ok(()),
        Some(status) => Err(format!("backbone-folder.status-{status}")),
        None => Err("backbone-folder: an answer without a status".into()),
    }
}

/// 🚪️ One folder-door hop: the page's `semioWgpuHostIo` on the browser build, the law's host under `cfg(test)`.
#[cfg(any(target_arch = "wasm32", test))]
pub(super) async fn backbone_folder_call(request: &str, bytes: Option<&[u8]>) -> Result<String, String> {
    #[cfg(target_arch = "wasm32")]
    return host_io_call(request, bytes).await;
    #[cfg(not(target_arch = "wasm32"))]
    TEST_BACKBONE_FOLDER_HOST.with(|host| {
        let request: Value = serde_json::from_str(request).map_err(|error| error.to_string())?;
        let mut host = host.borrow_mut();
        host.1.push(request.clone());
        let key = (request["uri"].as_str().unwrap_or_default().to_string(), request["documentId"].as_str().unwrap_or_default().to_string());
        Ok(match (request["verb"].as_str(), bytes) {
            (Some("write"), Some(bytes)) => {
                host.0.insert(key, bytes.to_vec());
                serde_json::json!({ "status": 200 })
            }
            (Some("read"), _) => match host.0.get(&key) {
                Some(bytes) => serde_json::json!({ "status": 200, "bodyBase64": semio_framework_io_base64::base64_standard_encode(bytes) }),
                None => serde_json::json!({ "status": 204 }),
            },
            _ => serde_json::json!({ "error": "backbone-folder: a write carries no archive" }),
        }
        .to_string())
    })
}

/// 🔢️ The digest a written archive is compared by, so an unchanged document is never written again.
#[cfg(any(target_arch = "wasm32", test))]
pub(super) fn archive_digest(bytes: &[u8]) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    bytes.hash(&mut hasher);
    hasher.finish()
}

//#endregion 🔖️HostRouteFolder
impl DocumentState {
    /// 🪪️ The document identity the session's program holds, as its own store stamps it — `None` until the program
    /// answered `ReadDocumentIdentity` with a document.
    pub(crate) fn sync_program_identity(&self) -> Option<LocalFolderIdentity> {
        let session = self.session.as_ref()?;
        let document_id = self.app_document_identities.document(&(session.plugin_id.clone(), session.instance_id))?;
        Some(LocalFolderIdentity { document_id: document_id.to_string(), plugin_id: session.plugin_id.clone(), app_id: session.app.id.clone() })
    }

    /// 📎️ The remembered folder for the session's document, while that document is not attached. Asked every frame, so a
    /// device remembering no folder answers before reading the session's identity.
    pub(crate) fn folder_reconnect_offer(&self, history: &HistoryState, sync: &SyncState) -> Option<LocalFolderBinding> {
        let bindings = history.local_folder_bindings.as_ref().filter(|bindings| !bindings.bindings.is_empty())?;
        local_folder_reconnect_offer(bindings, self.sync_program_identity().as_ref(), sync.attached_document_id(history))
    }

    /// 📎️ The offer the band shows: on a build that waits for the person's gesture, the remembered folder itself; on a
    /// build that reopens folders by itself, one whose direct reattach already ran and left the document unattached.
    pub(crate) fn folder_reconnect_band_offer(&self, history: &HistoryState, sync: &SyncState) -> Option<LocalFolderBinding> {
        self.folder_reconnect_offer(history, sync).filter(|offer| !history.local_folder_direct_reattach || history.local_folder_reattach_tried.as_deref() == Some(offer.document_id.as_str()))
    }

    /// 🔁️ The folder the shell reopens by itself now: on a build that does so, the offer for the session's document, once
    /// per document.
    pub(crate) fn direct_reattach_candidate(&self, history: &HistoryState, sync: &SyncState) -> Option<LocalFolderBinding> {
        if !history.local_folder_direct_reattach {
            return None;
        }
        self.folder_reconnect_offer(history, sync).filter(|offer| history.local_folder_reattach_tried.as_deref() != Some(offer.document_id.as_str()))
    }

    /// ✍️ Writes the session program's document archive through the host route; answers the written archive's digest.
    #[cfg(any(target_arch = "wasm32", test))]
    pub(super) async fn write_host_route_archive(&self, uri: &str, document_id: &str, schema: &str, plugins: &PluginState) -> Result<u64, String> {
        let session = self.session.as_ref().ok_or("session missing")?;
        let plugin = plugins.plugins.iter().find(|entry| entry.plugin_id == session.plugin_id).ok_or("plugin missing")?;
        let archive = plugin.read_app_document_archive(session.instance_id).await?;
        let bytes = protocol::encode_document_archive_bytes(&archive).map_err(|error| error.to_string())?;
        backbone_folder_write_answer(&backbone_folder_call(&backbone_folder_request(BackboneFolderVerb::Write, uri, document_id, Some(schema)), Some(&bytes)).await?)?;
        Ok(archive_digest(&bytes))
    }
}

