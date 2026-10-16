//! 🔀️ `shell-actions` staging of `local-folders`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;
use super::local_folders::*;
use semio_framework_os_config::opening_config::mutations::{apply_local_folders_config_mutation, attach_local_folder, detach_local_folder, LocalFolderBinding, LocalFolderBindings, LocalFolderRef, LocalFoldersConfigMutation, LOCAL_FOLDERS_CONFIG_SCHEMA};

impl<'ctx> ActionsCtx<'ctx> {
    /// 🔗️ Reattaches a remembered folder through the same folder attach a person makes, which re-records it and restores
    /// the folder's archive.
    pub(crate) async fn reattach_local_folder(&mut self, binding: &LocalFolderBinding) -> Result<(), String> {
        self.attach_sync_backbone(format!("folder://{}", local_folder_path(binding))).await
    }

    /// 🔁️ The boot reattach: once per document, the remembered folder of the session's document is reopened without a
    /// gesture. A failure is told through the dispatch-fault funnel and leaves the band offering it. `true` when it ran.
    pub(crate) async fn reattach_remembered_local_folder(&mut self) -> bool {
        self.history.ensure_local_folder_bindings();
        let Some(offer) = self.docs.direct_reattach_candidate(&self.history, &self.sync) else { return false };
        self.history.local_folder_reattach_tried = Some(offer.document_id.clone());
        if let Err(error) = self.reattach_local_folder(&offer).await {
            self.prefs.note_dispatch_fault(&error, &mut self.chrome);
        }
        true
    }
}

