//! 🏐️ Staged `archive-merge` members of the `instance-commands` use cases, as methods of the borrowed bundle (WP7 of the instance family).
//!
//! 🗃️ The whole-document archive merge.
use semio_framework_plugin_document_media::ActiveDocumentArchiveLoadState;
use semio_framework_plugin_decl_app::{ArtifactApp, DOCUMENT_LOAD_FAILED_CODE, DOCUMENT_LOAD_MEMBERS_DIFFER_CODE, DOCUMENT_LOAD_OPERATION_UNKNOWN_CODE, DOCUMENT_LOAD_OTHER_DOCUMENT_CODE, document_load_fault};
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use pack::snapshot::ArtifactPack;
use crate::command_bundle::CommandCtx;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework::{Fault, FaultFrom};
use std::future::Future;
use semio_framework_plugin_instance_extension::InstanceExtension;
use store::{MemberFactory, Mutation, ReferenceSpaceMember};
use protocol::OpText;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_ui_contract::*;

impl<'ctx, A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + 'static, X: InstanceExtension> CommandCtx<'ctx, A, M, X> {
    /// 🔀️ Merges a decoded read-back into the live document (design §22.22) in the host's own poll turn and requests the
    /// operation's terminal: `Ready` with what it took (`completed == total`) and is ahead by, or the fault that
    /// refused it. Nothing is replaced.
    pub async fn merge_document_archive(&mut self, operation: u64, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<(), Fault> {
        let merged = self.merge_decoded_document_archive(operation, identity, cx).await;
        let active = self.ingest.document_archive_loads.get_mut(operation).ok_or_else(|| document_load_fault(DOCUMENT_LOAD_OPERATION_UNKNOWN_CODE, "recursive document archive merge authority changed during its merge"))?;
        match merged {
            Ok((taken, ahead)) => {
                active.completed = taken;
                active.total = taken;
                active.ahead = ahead;
                active.request_terminal(ActiveDocumentArchiveLoadState::Ready);
            }
            Err(fault) => active.request_fault(&fault),
        }
        Ok(())
    }

    /// 🧬️ The merge itself, answering the events taken and the events this program holds that the archive lacks: the
    /// archive's members must be the live ones (a member merge is the member's own, not yet available), its decoded
    /// log joins the store's in one store call — an archive of another document is refused — and an open history edit
    /// hears the base move like after any remote edit.
    async fn merge_decoded_document_archive(&mut self, operation: u64, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<(u64, u64), Fault> {
        let live = self.child_composition.archive_member_entries().await?;
        let active = self.ingest.document_archive_loads.get_mut(operation).ok_or_else(|| document_load_fault(DOCUMENT_LOAD_OPERATION_UNKNOWN_CODE, "recursive document archive merge authority changed before its merge"))?;
        let archive = active.archive.as_mut().ok_or_else(|| document_load_fault(DOCUMENT_LOAD_FAILED_CODE, "recursive document archive merge input owner is absent"))?;
        if !live.iter().eq(archive.members.iter().rev()) {
            return Err(document_load_fault(DOCUMENT_LOAD_MEMBERS_DIFFER_CODE, "the archive's members differ from this document's live members; a member merge is not available"));
        }
        let pack = std::mem::take(&mut archive.parent_pack);
        let history = active.decoded_history.take().ok_or_else(|| document_load_fault(DOCUMENT_LOAD_FAILED_CODE, "recursive document archive decoded history owner is absent"))?;
        let generation = self.stores.store.generation();
        let merge = match self.stores.store.merge_persisted_history(&pack, history).await {
            Ok(merge) => merge,
            Err(store::VcsError::ValidationFailed(detail)) => return Err(document_load_fault(DOCUMENT_LOAD_OTHER_DOCUMENT_CODE, detail)),
            Err(error) => return Err(error.into_fault()),
        };
        self.history.cache = None;
        if self.stores.store.generation() != generation {
            self.as_dispatch().queue_base_moved();
        }
        self.child_composition.follow_derivable_children(&mut *self.stores, identity).await?;
        Ok((u64::try_from(merge.merged).unwrap_or(u64::MAX), u64::try_from(merge.ahead).unwrap_or(u64::MAX)))
    }
}
