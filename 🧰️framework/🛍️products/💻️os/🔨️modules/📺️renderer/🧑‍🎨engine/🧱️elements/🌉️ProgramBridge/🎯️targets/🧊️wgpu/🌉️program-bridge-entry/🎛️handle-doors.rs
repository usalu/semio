//! 🎛️ The native `PluginWasmHandle` doors of the program bridge entry: React's `🔌️PluginRuntime/🧷️handle-contract` methods that `ProgramBridgeEntry` lacked, each a thin method over [`super::native_handle_doors`]. The browser build keeps the TypeScript handle as the owner. See [`super`].

use super::*;

impl ProgramBridgeEntry {
    fn client(&self) -> &KernelClient {
        let ProgramBridgeBackend::Wasm { client, .. } = &self.backend;
        client
    }

    /// 📥️ `applyMutations` with the merge report and conflict roster the ingest pushed.
    pub async fn apply_mutations_reported(&self, instance_id: u32, operations: &[u8]) -> Result<ProgramMergeOutcome, ProgramFault> {
        native_handle_doors::apply_mutations_reported(self.client(), instance_id, operations).await
    }

    /// ⚔️ `resolveConflict` with the merge report and the fresh roster.
    pub async fn resolve_conflict_reported(&self, instance_id: u32, conflict_id: &str, accept: bool) -> Result<ProgramMergeOutcome, ProgramFault> {
        native_handle_doors::resolve_conflict_reported(self.client(), instance_id, conflict_id, accept).await
    }

    /// ⚖️ `setMergePolicy`.
    pub async fn set_merge_policy(&self, instance_id: u32, policy: protocol::MergePolicy) -> Result<(), ProgramFault> {
        let policy = match policy {
            protocol::MergePolicy::LaissezFaire => 0,
            protocol::MergePolicy::Normal => 1,
            protocol::MergePolicy::Vigilant => 2,
        };
        native_handle_doors::set_merge_policy(self.client(), instance_id, policy).await
    }

    /// 📖️ `readAppDocumentPack`.
    pub async fn read_app_document_pack(&self, instance_id: u32) -> Result<Option<ProgramDocumentPack>, ProgramFault> {
        native_handle_doors::read_document_pack(self.client(), instance_id).await
    }

    /// 📦️ `documentPack`: the pack and history the instance last read or loaded.
    pub fn document_pack(&self, instance_id: u32) -> Option<(Vec<u8>, Vec<u8>)> {
        native_handle_doors::document_pack(instance_id)
    }

    /// 📤️ `exportAppMedia`.
    pub async fn export_app_media(&self, instance_id: u32, port: &str) -> Result<Option<ProgramMediaOut>, ProgramFault> {
        native_handle_doors::export_media(self.client(), instance_id, port).await
    }

    /// 📂️ `importAppMedia` over an already pack-encoded descriptor; `cancelled` is read before the send.
    pub async fn import_app_media(&self, instance_id: u32, port: &str, descriptor: Vec<u8>, data: Vec<u8>, cancelled: impl Fn() -> bool) -> Result<(), ProgramFault> {
        native_handle_doors::import_media(self.client(), instance_id, port, descriptor, data, &cancelled).await
    }

    /// 🚀️ `submitMediaExport`.
    pub async fn submit_media_export(&self, instance_id: u32, port: &str, parent_document_id: &str, revision: u64, checkpoint_id: Option<String>) -> Result<protocol::MediaExportHandleWire, ProgramFault> {
        native_handle_doors::submit_media_export(self.client(), instance_id, port, parent_document_id, revision, checkpoint_id).await
    }

    /// ⏱️ `pollMediaExport`.
    pub async fn poll_media_export(&self, instance_id: u32, handle: &protocol::MediaExportHandleWire) -> Result<ProgramMediaExportStatus, ProgramFault> {
        native_handle_doors::poll_media_export(self.client(), instance_id, handle).await
    }

    /// 🛑️ `cancelMediaExport`.
    pub async fn cancel_media_export(&self, instance_id: u32, handle: &protocol::MediaExportHandleWire) -> Result<(), ProgramFault> {
        native_handle_doors::cancel_media_export(self.client(), instance_id, handle).await
    }

    /// 📥️ `takeMediaExportChunk` (and React's `takeSegmentedDownloadChunk`, the same bounded page).
    pub async fn take_media_export_chunk(&self, instance_id: u32, handle: &protocol::MediaExportHandleWire) -> Result<ProgramMediaExportChunk, ProgramFault> {
        native_handle_doors::take_media_export_chunk(self.client(), instance_id, handle).await
    }

    /// 🗃️ `loadAppDocumentArchive` with its cancellation and per-status progress.
    pub async fn load_app_document_archive_observed(&self, instance_id: u32, archive: &protocol::DocumentArchivePack, cancelled: impl Fn() -> bool, mut progress: impl FnMut(&protocol::DocumentArchiveLoadStatus)) -> Result<(), ProgramFault> {
        native_handle_doors::load_archive(self.client(), instance_id, archive, &cancelled, &mut progress).await
    }

    /// 🔀️ `mergeAppDocumentArchive`: answers the events the instance is ahead by.
    pub async fn merge_app_document_archive(&self, instance_id: u32, archive: &protocol::DocumentArchivePack, cancelled: impl Fn() -> bool, mut progress: impl FnMut(&protocol::DocumentArchiveLoadStatus)) -> Result<u64, ProgramFault> {
        native_handle_doors::merge_archive(self.client(), instance_id, archive, &cancelled, &mut progress).await
    }

    /// 🪟️ `readWindowConfigPacks`.
    pub async fn read_window_config_packs(&self, instance_id: u32) -> Result<Vec<protocol::WindowConfigPackEntry>, ProgramFault> {
        native_handle_doors::read_window_configs(self.client(), instance_id).await
    }

    /// 🪟️ `loadWindowConfigPack`.
    pub async fn load_window_config_pack(&self, instance_id: u32, entry: protocol::WindowConfigPackEntry) -> Result<(), ProgramFault> {
        native_handle_doors::load_window_config(self.client(), instance_id, entry).await
    }

    /// 🎫️ `transactionPrepare`.
    pub async fn transaction_prepare(&self, instance_id: u32, txn_id: &str, request: ProgramTransactionRequest) -> Result<ProgramTransactionPrepared, ProgramFault> {
        native_handle_doors::transaction_prepare(self.client(), instance_id, txn_id, request).await
    }

    /// ✅️ `transactionCommit`.
    pub async fn transaction_commit(&self, instance_id: u32, txn_id: &str) -> Result<ProgramTransactionCommitted, ProgramFault> {
        native_handle_doors::transaction_commit(self.client(), instance_id, txn_id).await
    }

    /// ↩️ `transactionRollback`.
    pub async fn transaction_rollback(&self, instance_id: u32, txn_id: &str) -> Result<(), ProgramFault> {
        native_handle_doors::transaction_rollback(self.client(), instance_id, txn_id).await
    }

    /// ⏪️ `transactionUndo`.
    pub async fn transaction_undo(&self, instance_id: u32, group_id: &str) -> Result<(), ProgramFault> {
        native_handle_doors::transaction_group(self.client(), instance_id, group_id, false).await
    }

    /// ⏩️ `transactionRedo`.
    pub async fn transaction_redo(&self, instance_id: u32, group_id: &str) -> Result<(), ProgramFault> {
        native_handle_doors::transaction_group(self.client(), instance_id, group_id, true).await
    }

    /// 📂️ `openArtifact`.
    pub async fn open_artifact(&self, instance_id: u32, artifact_ref: &str, role: u8, plugin_id: &str, app_id: &str) -> Result<(), ProgramFault> {
        native_handle_doors::open_artifact(self.client(), instance_id, artifact_ref, role, plugin_id, app_id).await
    }

    /// 📌️ `setDefaultApp`.
    pub async fn set_default_app(&self, instance_id: u32, artifact_kind: &str, standard: &str, subset: &str, role: u8, plugin_id: &str, app_id: &str) -> Result<(), ProgramFault> {
        native_handle_doors::set_default_app(self.client(), instance_id, artifact_kind, standard, subset, role, plugin_id, app_id).await
    }

    /// 🧽️ `clearDefaultApp`.
    pub async fn clear_default_app(&self, instance_id: u32, artifact_kind: &str, standard: &str, subset: &str, role: u8) -> Result<(), ProgramFault> {
        native_handle_doors::clear_default_app(self.client(), instance_id, artifact_kind, standard, subset, role).await
    }

    /// 🏠️ `readLocalInteraction`: the instance's local interaction capture, paged and acknowledged, cancelled through `cancelled`.
    pub async fn read_local_interaction(&self, instance_id: u32, cancelled: impl Fn() -> bool) -> Result<replication::LocalInteractionCapture, ProgramFault> {
        native_handle_doors::read_local_interaction(self.client(), instance_id, &cancelled).await
    }
}
