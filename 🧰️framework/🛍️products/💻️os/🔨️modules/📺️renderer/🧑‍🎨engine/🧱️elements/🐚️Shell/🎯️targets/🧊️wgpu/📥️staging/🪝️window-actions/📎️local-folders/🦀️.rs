//! 🪝️ `shell-window-actions` staging of `local-folders`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;
use super::local_folders::*;
use semio_framework_os_config::opening_config::mutations::{apply_local_folders_config_mutation, attach_local_folder, detach_local_folder, LocalFolderBinding, LocalFolderBindings, LocalFolderRef, LocalFoldersConfigMutation, LOCAL_FOLDERS_CONFIG_SCHEMA};

//#region 🔖️HostRouteFolder
/// 📬️ A read's answer: the stored archive's bytes, `None` when nothing was written yet (`204`), else the refusal — the
/// HTTP status as `backbone-folder.status-<n>`, or the page's own transport error.
#[cfg(any(target_arch = "wasm32", test))]
pub(crate) fn backbone_folder_read_answer(answer: &str) -> Result<Option<Vec<u8>>, String> {
    let answer: Value = serde_json::from_str(answer).map_err(|error| error.to_string())?;
    if let Some(error) = answer.get("error").and_then(Value::as_str) {
        return Err(error.to_string());
    }
    match answer.get("status").and_then(Value::as_u64) {
        Some(200) => semio_framework_io_base64::base64_standard_decode(answer.get("bodyBase64").and_then(Value::as_str).ok_or("backbone-folder: a stored archive without bytes")?).map(Some).map_err(|error| error.to_string()),
        Some(204) => Ok(None),
        Some(status) => Err(format!("backbone-folder.status-{status}")),
        None => Err("backbone-folder: an answer without a status".into()),
    }
}

//#endregion 🔖️HostRouteFolder
impl<'ctx> WindowActionsCtx<'ctx> {
    /// 🗃️ Restores the archive a native folder holds for `document_id` into the session's program like a fresh load —
    /// React's `restoreDocumentArchiveV1`: once the program holds it, its history is re-read (rows, edits, head) and every
    /// surface refreshes. `true` when an archive was restored.
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) async fn restore_folder_archive(&mut self, path: &str, document_id: &str) -> Result<bool, String> {
        let Some(bytes) = store_sync::sync::FolderEventLogStorage::new(std::path::PathBuf::from(path)).read_archive(document_id).await.map_err(|error| error.to_string())? else { return Ok(false) };
        self.restore_document_archive_bytes(&bytes).await?;
        Ok(true)
    }

    /// 🗃️ Loads one encoded document archive into the session's program like a fresh load, then re-reads its history
    /// (rows, edits, head) and refreshes every surface — React's `restoreDocumentArchiveV1`, on both builds.
    pub(crate) async fn restore_document_archive_bytes(&mut self, bytes: &[u8]) -> Result<(), String> {
        let archive = protocol::decode_document_archive_bytes(bytes).await.map_err(|error| error.to_string())?;
        let session = self.docs.session.clone().ok_or("session missing")?;
        let plugin = self.plugins.plugins.iter().find(|entry| entry.plugin_id == session.plugin_id).cloned().ok_or("plugin missing")?;
        plugin.load_app_document_archive(session.instance_id, &archive).await?;
        self.as_session().refresh_history_snapshot().await;
        self.refresh_ui(UiDirtyScope::Full).await
    }

    /// 🌐️ The browser build's folder or file attach — React's worker folder transport over the dev host's backbone route:
    /// the stored archive is read and restored like a fresh load, or, when nothing was written yet, the document's own
    /// archive is written there first; the document then stays bound until detached and every history change writes its
    /// archive again ([`Self::flush_host_route_folder`]). A folder attach is remembered for this device.
    #[cfg(any(target_arch = "wasm32", test))]
    pub(crate) async fn attach_host_route_backbone(&mut self, uri: String, identity: LocalFolderIdentity) -> Result<(), String> {
        let schema = self.docs.session.as_ref().map(|session| session.app.io.artifact_schema.clone()).ok_or("session missing")?;
        let stored = backbone_folder_read_answer(&backbone_folder_call(&backbone_folder_request(BackboneFolderVerb::Read, &uri, &identity.document_id, None), None).await?)?;
        let written = match stored {
            Some(bytes) => {
                self.restore_document_archive_bytes(&bytes).await?;
                archive_digest(&bytes)
            }
            None => self.docs.write_host_route_archive(&uri, &identity.document_id, &schema, &self.plugins).await?,
        };
        self.history.host_route_folder = Some(HostRouteFolder { uri: uri.clone(), document_id: identity.document_id.clone(), schema, written, written_cursor: self.history.history_cursor });
        self.sync.sync_backbone_uri = Some(uri.clone());
        if let Some(path) = uri.strip_prefix("folder://") {
            self.history.ensure_local_folder_bindings();
            self.history.remember_local_folder(LocalFolderBinding { document_id: identity.document_id, plugin_id: identity.plugin_id, app_id: identity.app_id, folder: LocalFolderRef::Path { path: path.to_string() } });
        }
        Ok(())
    }
}

