//! 🔀️ `shell-actions` staging of `local-folders`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;
use semio_os_config::opening_config::mutations::{apply_local_folders_config_mutation, attach_local_folder, detach_local_folder, LocalFolderBinding, LocalFolderBindings, LocalFolderRef, LocalFoldersConfigMutation, LOCAL_FOLDERS_CONFIG_SCHEMA};

impl<'ctx> ActionsCtx<'ctx> {
    /// 🔗️ Reattaches a remembered folder through the same folder attach a person makes, which re-records it and restores
    /// the folder's archive.
    pub async fn reattach_local_folder(&mut self, binding: &LocalFolderBinding) -> Result<(), String> {
        self.attach_sync_backbone(format!("folder://{}", local_folder_path(binding))).await
    }
}

//#region 🔖️Reconnect
/// 🆔️ The notice code a folder attach of a program holding no document is told under.
pub const SYNC_DOCUMENT_UNIDENTIFIED_CODE: &str = "sync.attach.document-unidentified";
//#endregion 🔖️Reconnect

//#region 🔖️HostRouteFolder
/// 🚪️ Whether a sync-card uri names a document the browser keeps through the host route.
#[cfg(target_arch = "wasm32")]
pub fn is_host_route_uri(uri: &str) -> bool {
    uri.starts_with("folder://") || uri.starts_with("file://")
}
//#endregion 🔖️HostRouteFolder
