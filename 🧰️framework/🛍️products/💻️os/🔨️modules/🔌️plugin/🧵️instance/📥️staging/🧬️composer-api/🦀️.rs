//! 🧬 The composer API of the mounted artifact instance: the public readers and entry points that delegate to a component.
use semio_plugin_decl_app::ArtifactApp;
use semio_framework_artifact_reference::ArtifactDialect;
use semio_plugin_document_media::{ArtifactEnvelopeDecodeOperationHandle, ArtifactEnvelopeDecodeOperationPoll};
use semio_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use pack::snapshot::ArtifactPack;
use store::{ArtifactStore, MemberFactory, Mutation, ReferenceSpaceMember};
use semio_plugin_operation::original_catalog_constructor::{ArtifactStoreConstructorKind, ArtifactStoreConstructorPhase};
use semio_plugin_decl_views::child_view::ChildContentView;
use semio_plugin_child::{ChildMemberRegistrationError, OwnedDocumentMemberIngress};
use semio_plugin_decl_emission::Emit;
use semio_framework::{Fault, FaultFrom, ViewModel};
use semio_plugin_job::reserved_jobs::FrameworkReservedCommitProgress;
use std::future::Future;
use semio_plugin_instance_extension::InstanceExtension;
use protocol::OpText;
use semio_plugin_reactor_app::PluginApp;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};
use semio_plugin_operation::ToolCancellationHandle;
use crate::VcsArtifactApp;
use semio_framework_job::{Generation, OperationId, StepContext};
use semio_plugin_ownership::bounded_ownership::PluginLifecycleFailure;
use semio_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority;
use store::os_store::owned_schema_decode::{ArtifactEnvelopeDecodePage, OwnedSchemaDecodePages};
use semio_plugin_window_transient::window_transient::WindowTransientSnapshot;
use semio_ui_contract::*;

impl<A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + 'static> VcsArtifactApp<A, M> {
    /// 📷️ Copies persisted state for structural inspection.
    pub fn snapshot(&self) -> Result<A::Snapshot, Fault> {
        Ok(self.stores.snapshot_ref().clone())
    }

    /// 📸️ Borrows the current persisted state.
    pub fn snapshot_ref(&self) -> &A::Snapshot {
        self.stores.snapshot_ref()
    }

    /// 🧪️ Reads the store revision observed by rendering and admitted commands.
    #[cfg(any(test, feature = "artifact-app-testing"))]
    pub fn test_document_revision(&self) -> [u8; 32] {
        self.stores.test_document_revision()
    }

    /// 🔗️ Borrows the current backbone descriptor.
    pub fn backbone_ref(&self) -> Option<&store::os_store::store_schema::ArtifactBackboneRef> {
        self.stores.backbone_ref()
    }

    /// 🕹️ Joins persisted local interaction and ephemeral hover for presence assembly.
    pub async fn interaction_state(&self) -> protocol::InteractionState {
        self.interaction.interaction_state(&self.stores).await
    }

    pub fn take_constructor_receipt(&mut self)->Option<(ArtifactStoreConstructorKind,ArtifactStoreConstructorPhase,RetainedCloneGrant,RetainedCloneProgress)> {
        self.constructor.take_constructor_receipt()
    }

    pub fn store_constructor_is_pending(&self)->bool {
        self.constructor.store_constructor_is_pending()
    }

    pub fn artifact_generation_now(&self) -> Generation {
        self.stores.artifact_generation_now()
    }

    /// 🫧️ Reads the immutable projection of one live window.
    pub fn window_transient_snapshot(&mut self, view_state: &ViewModel) -> Result<Option<WindowTransientSnapshot>, Fault> {
        self.stores.window_transient_snapshot(view_state)
    }

    pub fn window_transient_generation(&mut self, view_state: &ViewModel) -> Result<Option<u64>, Fault> {
        self.stores.window_transient_generation(view_state)
    }

    pub async fn window_config_generation(&mut self, view_state: &ViewModel) -> Result<Option<u64>, Fault> {
        self.stores.window_config_generation(view_state).await
    }

    pub fn tool_cancellation_handle(&self) -> &ToolCancellationHandle {
        self.tools.tool_cancellation_handle()
    }

    /// 📊️ Reads progress of the first reserved commit.
    pub fn framework_reserved_commit_progress(&self) -> Option<FrameworkReservedCommitProgress> {
        self.pending.framework_reserved_commit_progress()
    }

    /// 🎟️ Reserves exact page and byte limits before constructing envelope input.
    pub fn begin_artifact_envelope_ingress(&mut self, maximum_pages: usize, maximum_bytes: usize) -> Result<ArtifactEnvelopeDecodeOperationHandle, Fault> {
        self.ingest.begin_artifact_envelope_ingress(&mut self.stores, maximum_pages, maximum_bytes)
    }

    /// 🔎️ Validates the live ingress slot and byte allowance before copying a page.
    pub fn preflight_artifact_envelope_ingress_page(&self, handle: ArtifactEnvelopeDecodeOperationHandle, page_bytes: usize) -> Result<(), Fault> {
        self.ingest.preflight_artifact_envelope_ingress_page(&self.stores, handle, page_bytes)
    }

    /// 📨️ Constructs a page only after its original ingress accepts the extent.
    pub fn construct_and_admit_artifact_envelope_ingress_page<Build>(&mut self, handle: ArtifactEnvelopeDecodeOperationHandle, page_bytes: usize, build: Build) -> Result<(), Fault>
    where
        Build: FnOnce() -> ArtifactEnvelopeDecodePage, {
        self.ingest.construct_and_admit_artifact_envelope_ingress_page(&mut self.stores, handle, page_bytes, build)
    }

    /// 📥️ Donates an exact fixed page or returns it unchanged with its refusal.
    #[expect(clippy::result_large_err, reason = "Refusal returns the original fixed-capacity input so its caller can retry or retire that exact owner without an extra allocation.")]
    pub fn admit_artifact_envelope_ingress_page(&mut self, handle: ArtifactEnvelopeDecodeOperationHandle, page: ArtifactEnvelopeDecodePage) -> Result<(), (Fault, ArtifactEnvelopeDecodePage)> {
        self.ingest.admit_artifact_envelope_ingress_page(&mut self.stores, handle, page)
    }

    /// 🔒️ Seals the original pages; saturation retains them for retry.
    pub fn seal_artifact_envelope_ingress(&mut self, handle: ArtifactEnvelopeDecodeOperationHandle,cx:&mut StepContext<'_>) -> Result<bool, Fault> {
        self.ingest.seal_artifact_envelope_ingress(&mut self.stores, handle, cx)
    }

    /// 🔄️ Preserves decoder output until its initializer accepts the handoff.
    pub fn advance_artifact_envelope_load(&mut self, handle: ArtifactEnvelopeDecodeOperationHandle,cx:&mut StepContext<'_>) -> Result<ArtifactEnvelopeDecodeOperationPoll, PluginLifecycleFailure> {
        self.ingest.advance_artifact_envelope_load(&mut self.stores, &mut self.child_composition, handle, cx)
    }

    pub fn cancel_artifact_envelope_load(&mut self, handle: ArtifactEnvelopeDecodeOperationHandle) -> Result<(), Fault> {
        self.ingest.cancel_artifact_envelope_load(handle)
    }

    /// 📦️ Admits original fixed pages or returns them unchanged if the slot or domain catalog is absent.
    pub fn submit_artifact_envelope_decode(
        &mut self,
        operation: OperationId,
        generation: Generation,
        pages: OwnedSchemaDecodePages,
        cx:&mut StepContext<'_>,
    ) -> Result<ArtifactEnvelopeDecodeOperationHandle, OwnedSchemaDecodePages> {
        self.ingest.submit_artifact_envelope_decode(operation, generation, pages, cx)
    }

    pub fn poll_artifact_envelope_decode(&self, handle: ArtifactEnvelopeDecodeOperationHandle) -> ArtifactEnvelopeDecodeOperationPoll {
        self.ingest.poll_artifact_envelope_decode(handle)
    }

    /// 🧹️ Tests worker terminal emptiness for the live handle.
    pub fn artifact_envelope_decode_worker_owners_are_terminal(&self, handle: ArtifactEnvelopeDecodeOperationHandle) -> bool {
        self.ingest.artifact_envelope_decode_worker_owners_are_terminal(handle)
    }

    /// 📤️ Publishes after generation validation; backpressure retains the exact record and operation.
    pub fn try_publish_artifact_envelope_decode(&mut self, handle: ArtifactEnvelopeDecodeOperationHandle, target: &mut dyn store::os_store::owned_schema_decode::ArtifactEnvelopeCompletedRecordTarget<A::Snapshot, A::Mutation>) -> Result<bool, Fault> {
        self.ingest.try_publish_artifact_envelope_decode(&mut self.stores, handle, target)
    }

    pub fn cancel_artifact_envelope_decode(&mut self, handle: ArtifactEnvelopeDecodeOperationHandle) -> Result<(), Fault> {
        self.ingest.cancel_artifact_envelope_decode(handle)
    }

    /// 🏗️ Transfers the decoded envelope; refusal retains its original completed slot.
    pub fn try_begin_artifact_store_replacement(&mut self, handle: ArtifactEnvelopeDecodeOperationHandle,cx:&mut StepContext<'_>) -> Result<bool, PluginLifecycleFailure> {
        self.ingest.try_begin_artifact_store_replacement(&mut self.stores, &mut self.child_composition, handle, cx)
    }

    /// 🌳️ Reserves the recursive member count after the parent initializer closes.
    pub fn try_begin_owned_document_members(&mut self, handle: ArtifactEnvelopeDecodeOperationHandle, expected: usize, expires_at_us: u64) -> Result<bool, Fault> {
        self.ingest.try_begin_owned_document_members(handle, expected, expires_at_us)
    }

    /// 🧳️ Donates a member request into its admitted ordinal.
    #[expect(clippy::result_large_err, reason = "Rejected admission returns the exact retained member request and identity owner unchanged.")]
    pub fn admit_owned_document_member(&mut self, handle: ArtifactEnvelopeDecodeOperationHandle, ingress: OwnedDocumentMemberIngress) -> Result<(), (Fault, OwnedDocumentMemberIngress)> {
        self.ingest.admit_owned_document_member(handle, ingress)
    }

    /// 🔏️ Freezes the admitted ordinals before opening members.
    pub fn seal_owned_document_members(&mut self, handle: ArtifactEnvelopeDecodeOperationHandle) -> Result<(), Fault> {
        self.ingest.seal_owned_document_members(handle)
    }

    pub fn poll_artifact_store_replacement(&self, handle: ArtifactEnvelopeDecodeOperationHandle) -> ArtifactEnvelopeDecodeOperationPoll {
        self.ingest.poll_artifact_store_replacement(handle)
    }

    /// 🧭️ Reads the exact replacement refusal before acknowledgment retires its evidence.
    pub fn artifact_store_replacement_refusal(&self, handle: ArtifactEnvelopeDecodeOperationHandle) -> Option<Fault> {
        self.ingest.artifact_store_replacement_refusal(handle)
    }

    pub fn acknowledge_artifact_store_replacement(&mut self, handle: ArtifactEnvelopeDecodeOperationHandle) -> Result<bool, Fault> {
        self.ingest.acknowledge_artifact_store_replacement(handle)
    }

    pub fn cancel_artifact_store_replacement(&mut self, handle: ArtifactEnvelopeDecodeOperationHandle) -> Result<(), Fault> {
        self.ingest.cancel_artifact_store_replacement(handle)
    }

    /// 🎰️ Counts live typed-operation and segmented-output slots; zero proves all were released.
    pub fn live_typed_operation_slots(&self) -> usize {
        self.media.live_typed_operation_slots(&self.pending, &self.tools)
    }

    /// 🕸️ Publishes the snapshot's child references into the ownership graph.
    pub async fn sync_composition_graph(&mut self) -> Result<(), Fault>
    where
        A::Snapshot: store::ArtifactRefs, {
        self.child_composition.sync_composition_graph(&mut self.stores).await
    }

    /// 🌱️ Restores a member with its graph, map and snapshot.
    pub async fn open_child(&mut self, slot: impl Into<String>, child_id: impl Into<String>, dialect: ArtifactDialect, envelope_pack: &[u8], identity: &mut EntityIdentityAuthority<'_>) -> Result<(), Fault> {
        self.child_composition.open_child(&mut self.stores, slot, child_id, dialect, envelope_pack, identity).await
    }

    /// 🔍️ Borrows an adopted or created child by slot and identity.
    pub async fn child_store(&self, slot: &str, child_id: &str) -> Option<&M> {
        self.child_composition.child_store(slot, child_id).await
    }

    /// 🪴️ Registers the member and ownership edge before group dispatch; see [Self::open_child].
    pub async fn register_child(&mut self, slot: impl Into<String>, child_id: impl Into<String>, dialect: ArtifactDialect, mut member: M, identity: &mut EntityIdentityAuthority<'_>) -> Result<(), ChildMemberRegistrationError<M>> {
        self.child_composition.register_child(&mut self.stores, slot, child_id, dialect, member, identity).await
    }
}

//#region 🔖️Driver
impl<A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + 'static> VcsArtifactApp<A, M> {
    /// 🎛️ Reads app and window config generations for publication laws.
    #[cfg(any(test, feature = "artifact-app-testing"))]
    pub fn config_generations(&self, window_kind_id: &str, window_id: &str) -> (u64, Option<u64>) {
        self.stores.config_generations(window_kind_id, window_id)
    }

    /// 🧾️ Reads edit transaction references in log order for history laws.
    #[cfg(any(test, feature = "artifact-app-testing"))]
    pub fn edit_transactions(&self) -> Vec<Option<protocol::TransactionRef>> {
        self.stores.edit_transactions()
    }
}
//#endregion 🔖️Driver

impl<A:ArtifactApp,M:ReferenceSpaceMember+MemberFactory+'static> VcsArtifactApp<A,M>{
 #[cfg(any(test,feature="artifact-app-testing"))]
 pub fn test_parent_store_mut(&mut self)->&mut store::ArtifactStore<A::Snapshot,A::Mutation> {
        self.stores.test_parent_store_mut()
    }

 #[cfg(any(test,feature="artifact-app-testing"))]
 pub fn test_child_content_view(&self)->ChildContentView {
        self.child_composition.test_child_content_view()
    }

 #[cfg(any(test,feature="artifact-app-testing"))]
 pub fn test_child_member_mut(&mut self,key:&(String,String))->Option<&mut M> {
        self.child_composition.test_child_member_mut(key)
    }

 #[cfg(any(test,feature="artifact-app-testing"))]
 pub async fn test_publish_child_content(&mut self,slot:&str,child_id:&str)->Result<(),Fault> {
        self.child_composition.test_publish_child_content(slot, child_id).await
    }
}
