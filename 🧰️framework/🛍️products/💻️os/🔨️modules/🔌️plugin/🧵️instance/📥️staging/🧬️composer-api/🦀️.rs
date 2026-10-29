//! 🧬 The composer API of the mounted artifact instance: the public readers and entry points that delegate to a component.
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_artifact_reference::ArtifactDialect;
use semio_framework_plugin_document_media::{ArtifactEnvelopeDecodeOperationHandle, ArtifactEnvelopeDecodeOperationPoll};
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use pack::snapshot::ArtifactPack;
use store::{ArtifactStore, MemberFactory, Mutation, ReferenceSpaceMember};
use semio_framework_plugin_operation::original_catalog_constructor::{ArtifactStoreConstructorKind, ArtifactStoreConstructorPhase};
use semio_framework_plugin_decl_views::child_view::ChildContentView;
use semio_framework_plugin_child::{ChildMemberRegistrationError, OwnedDocumentMemberIngress};
use semio_framework_plugin_decl_emission::Emit;
use semio_framework::{Fault, FaultFrom, ViewModel};
use semio_framework_plugin_job::reserved_jobs::FrameworkReservedCommitProgress;
use std::future::Future;
use semio_framework_plugin_instance_extension::InstanceExtension;
use protocol::OpText;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};
use semio_framework_plugin_operation::ToolCancellationHandle;
use crate::VcsArtifactApp;
use semio_framework_plugin_window_transient::window_transient::WindowTransientSnapshot;
use semio_framework_ui_contract::*;

impl<A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + 'static> VcsArtifactApp<A, M> {
    /// 📸️ Materializes and returns the current snapshot — the typed counterpart to
    /// `render`'s `UiNode` output, for callers (host code, downstream plugin crates' own tests) that
    /// need direct structural access to document state instead of a rendered node.
    pub fn snapshot(&self) -> Result<A::Snapshot, Fault> {
        Ok(self.stores.snapshot_ref().clone())
    }

    /// 📸️ Borrows the current persisted read root without copying its state.
    pub fn snapshot_ref(&self) -> &A::Snapshot {
        self.stores.snapshot_ref()
    }

    /// 🧪️ Returns the fixed store revision that a host render or admitted command observes.
    #[cfg(any(test, feature = "artifact-app-testing"))]
    pub fn test_document_revision(&self) -> [u8; 32] {
        self.stores.test_document_revision()
    }

    /// 🔗️ The store's current backbone descriptor, `None` when unattached (the default).
    pub fn backbone_ref(&self) -> Option<&store::os_store::store_schema::ArtifactBackboneRef> {
        self.stores.backbone_ref()
    }

    /// 🕹️ Combines the persisted-local `interaction_store` snapshot with the ephemeral
    /// `interaction_hover` map into one `InteractionState` — the "app-side source" a host's presence
    /// heartbeat reads for THIS app instance before calling `assemble_presence_interaction`
    /// (`🏪️store/🔄️sync/🦀️.rs`, wave 2a) with it plus this app's declared hover/selection
    /// specs (from `AppDefinition.interactions`, already available wherever a heartbeat is built).
    pub async fn interaction_state(&self) -> protocol::InteractionState {
        self.interaction.interaction_state(&self.stores).await
    }

    pub fn take_constructor_receipt(&mut self)->Option<(ArtifactStoreConstructorKind,ArtifactStoreConstructorPhase,RetainedCloneGrant,RetainedCloneProgress)> {
        self.constructor.take_constructor_receipt()
    }

    pub fn store_constructor_is_pending(&self)->bool {
        self.constructor.store_constructor_is_pending()
    }

    pub fn artifact_generation_now(&self) -> semio_framework_job::Generation {
        self.stores.artifact_generation_now()
    }

    /// 🫧️ Reads the immutable transient projection of one exact live window.
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

    /// 📊️ Progress of the commit at the head of the reserved-commit queue.
    pub fn framework_reserved_commit_progress(&self) -> Option<FrameworkReservedCommitProgress> {
        self.pending.framework_reserved_commit_progress()
    }

    /// 🎟️ Pre-admits the exact page and byte extent before a producer constructs any
    /// envelope input owner. Occupied fixed slots and allocation failure leave no input owner.
    pub fn begin_artifact_envelope_ingress(&mut self, maximum_pages: usize, maximum_bytes: usize) -> Result<ArtifactEnvelopeDecodeOperationHandle, Fault> {
        self.ingest.begin_artifact_envelope_ingress(&mut self.stores, maximum_pages, maximum_bytes)
    }

    /// 🔎️ Rejects stale, closed, full, or over-byte page ingress before the producer copies its raw owner.
    pub fn preflight_artifact_envelope_ingress_page(&self, handle: ArtifactEnvelopeDecodeOperationHandle, page_bytes: usize) -> Result<(), Fault> {
        self.ingest.preflight_artifact_envelope_ingress_page(&self.stores, handle, page_bytes)
    }

    /// 📨️ Runs the bounded page producer only after the same mutable ingress authority accepts its extent.
    pub fn construct_and_admit_artifact_envelope_ingress_page<Build>(&mut self, handle: ArtifactEnvelopeDecodeOperationHandle, page_bytes: usize, build: Build) -> Result<(), Fault>
    where
        Build: FnOnce() -> store::os_store::owned_schema_decode::ArtifactEnvelopeDecodePage, {
        self.ingest.construct_and_admit_artifact_envelope_ingress_page(&mut self.stores, handle, page_bytes, build)
    }

    /// 📥️ Moves one exact fixed page into a pre-admitted operation or returns that page
    /// untouched with the fault. No page clone or aggregate command buffer is constructed.
    #[expect(clippy::result_large_err, reason = "Refusal returns the original fixed-capacity input so its caller can retry or retire that exact owner without an extra allocation.")]
    pub fn admit_artifact_envelope_ingress_page(&mut self, handle: ArtifactEnvelopeDecodeOperationHandle, page: store::os_store::owned_schema_decode::ArtifactEnvelopeDecodePage) -> Result<(), (Fault, store::os_store::owned_schema_decode::ArtifactEnvelopeDecodePage)> {
        self.ingest.admit_artifact_envelope_ingress_page(&mut self.stores, handle, page)
    }

    /// 🔒️ Seals and transfers the exact page authority into the retained decoder. A
    /// saturated decoder keeps the sealed ingress owner in its original fixed slot for retry.
    pub fn seal_artifact_envelope_ingress(&mut self, handle: ArtifactEnvelopeDecodeOperationHandle,cx:&mut semio_framework_job::StepContext<'_>) -> Result<bool, Fault> {
        self.ingest.seal_artifact_envelope_ingress(&mut self.stores, handle, cx)
    }

    /// 🔄️ Advances decode-to-initializer handoff without running either job inline. A decode that
    /// has not reached [`ArtifactEnvelopeDecodeOperationPoll::Ready`] reports its OWN poll — the
    /// store replacement it will hand to does not exist yet, and reading that absent job here
    /// turned every in-progress decode into a terminal `Fault` for all eight plugin loops
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub fn advance_artifact_envelope_load(&mut self, handle: ArtifactEnvelopeDecodeOperationHandle,cx:&mut semio_framework_job::StepContext<'_>) -> Result<ArtifactEnvelopeDecodeOperationPoll, semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
        self.ingest.advance_artifact_envelope_load(&mut self.stores, &mut self.child_composition, handle, cx)
    }

    pub fn cancel_artifact_envelope_load(&mut self, handle: ArtifactEnvelopeDecodeOperationHandle) -> Result<(), Fault> {
        self.ingest.cancel_artifact_envelope_load(handle)
    }

    /// 📥️ Admits one exact fixed-page envelope owner into app maintenance. A missing
    /// domain catalog or occupied operation slot returns every page untouched.
    pub fn submit_artifact_envelope_decode(
        &mut self,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
        pages: store::os_store::owned_schema_decode::OwnedSchemaDecodePages,
        cx:&mut semio_framework_job::StepContext<'_>,
    ) -> Result<ArtifactEnvelopeDecodeOperationHandle, store::os_store::owned_schema_decode::OwnedSchemaDecodePages> {
        self.ingest.submit_artifact_envelope_decode(operation, generation, pages, cx)
    }

    pub fn poll_artifact_envelope_decode(&self, handle: ArtifactEnvelopeDecodeOperationHandle) -> ArtifactEnvelopeDecodeOperationPoll {
        self.ingest.poll_artifact_envelope_decode(handle)
    }

    /// 🧹️ Whether the decode worker session/outcome ladder is fully retired for one live handle.
    pub fn artifact_envelope_decode_worker_owners_are_terminal(&self, handle: ArtifactEnvelopeDecodeOperationHandle) -> bool {
        self.ingest.artifact_envelope_decode_worker_owners_are_terminal(handle)
    }

    /// 📤️ Applies the sole non-suspending completed-record publication after exact live
    /// generation validation. Backpressure leaves both the record and active operation retained.
    pub fn try_publish_artifact_envelope_decode(&mut self, handle: ArtifactEnvelopeDecodeOperationHandle, target: &mut dyn store::os_store::owned_schema_decode::ArtifactEnvelopeCompletedRecordTarget<A::Snapshot, A::Mutation>) -> Result<bool, Fault> {
        self.ingest.try_publish_artifact_envelope_decode(&mut self.stores, handle, target)
    }

    pub fn cancel_artifact_envelope_decode(&mut self, handle: ArtifactEnvelopeDecodeOperationHandle) -> Result<(), Fault> {
        self.ingest.cancel_artifact_envelope_decode(handle)
    }

    /// 🏗️ Atomically transfers one decoded envelope into the app-owned retained store
    /// initializer. Capacity or factory rejection leaves the completed record in its exact slot.
    pub fn try_begin_artifact_store_replacement(&mut self, handle: ArtifactEnvelopeDecodeOperationHandle,cx:&mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
        self.ingest.try_begin_artifact_store_replacement(&mut self.stores, &mut self.child_composition, handle, cx)
    }

    /// 🌳️ Pre-admits the exact recursive member count after the retained parent initializer
    /// is terminal and before any member input may enter the candidate transaction.
    pub fn try_begin_owned_document_members(&mut self, handle: ArtifactEnvelopeDecodeOperationHandle, expected: usize, expires_at_us: u64) -> Result<bool, Fault> {
        self.ingest.try_begin_owned_document_members(handle, expected, expires_at_us)
    }

    /// 📥️ Transfers one admitted member-open request into its exact retained ordinal.
    #[expect(clippy::result_large_err, reason = "Rejected admission returns the exact retained member request and identity owner unchanged.")]
    pub fn admit_owned_document_member(&mut self, handle: ArtifactEnvelopeDecodeOperationHandle, ingress: OwnedDocumentMemberIngress) -> Result<(), (Fault, OwnedDocumentMemberIngress)> {
        self.ingest.admit_owned_document_member(handle, ingress)
    }

    /// 🔒️ Freezes the complete ordinal set so bounded typed opens can begin.
    pub fn seal_owned_document_members(&mut self, handle: ArtifactEnvelopeDecodeOperationHandle) -> Result<(), Fault> {
        self.ingest.seal_owned_document_members(handle)
    }

    pub fn poll_artifact_store_replacement(&self, handle: ArtifactEnvelopeDecodeOperationHandle) -> ArtifactEnvelopeDecodeOperationPoll {
        self.ingest.poll_artifact_store_replacement(handle)
    }

    /// 🧭️ The exact leg that refused this replacement, minted as a typed fault. Read BEFORE
    /// {@link acknowledge_artifact_store_replacement} retires the operation — after that the
    /// evidence is gone and only the generic collapse remains.
    pub fn artifact_store_replacement_refusal(&self, handle: ArtifactEnvelopeDecodeOperationHandle) -> Option<Fault> {
        self.ingest.artifact_store_replacement_refusal(handle)
    }

    pub fn acknowledge_artifact_store_replacement(&mut self, handle: ArtifactEnvelopeDecodeOperationHandle) -> Result<bool, Fault> {
        self.ingest.acknowledge_artifact_store_replacement(handle)
    }

    pub fn cancel_artifact_store_replacement(&mut self, handle: ArtifactEnvelopeDecodeOperationHandle) -> Result<(), Fault> {
        self.ingest.cancel_artifact_store_replacement(handle)
    }

    /// 🎰️ How many of the fixed residue classes currently own a live typed operation or segmented
    /// output — the exact complement of what [`Self::admit_typed_operation_slot`] can still hand out,
    /// so `0` is the only value that proves every admitted slot reached its release path.
    pub fn live_typed_operation_slots(&self) -> usize {
        self.media.live_typed_operation_slots(&self.pending, &self.tools)
    }

    /// 🕸️ Re-syncs `self.composition`'s ownership/link graph from the parent's own live
    /// `ArtifactRefs` projection (`child_refs()`/`links()`) — the mechanism
    /// `store::os_store::composition_coordinator::CompositionGraph::sync_member`'s own doc comment names as the required follow-up
    /// "after any dispatch that might have changed an artifact's own `ArtifactRefs`". Deliberately
    /// NOT called automatically by `dispatch_emit`/`dispatch_emit_group`: doing so would force
    /// `A::Snapshot: store::ArtifactRefs` onto EVERY `ArtifactApp` impl in the workspace (today
    /// zero of them implement it — `ArtifactRefs`-bearing snapshots are new, later-wave-scoped
    /// schema work), breaking every existing plugin's compile for a graph no leaf artifact needs
    /// synced. A composed artifact's own app calls this explicitly wherever its `handle` might
    /// have changed a `#[child(...)]`/`#[link_slot(...)]` field — see B2's own
    /// `📓️wave1-reports/b2-store-composition-report.md` sharedFileRequest #6.
    pub async fn sync_composition_graph(&mut self) -> Result<(), Fault>
    where
        A::Snapshot: store::ArtifactRefs, {
        self.child_composition.sync_composition_graph(&mut self.stores).await
    }

    /// 🌱️ Restores one exact owned member and publishes its graph, map and snapshot together.
    pub async fn open_child(&mut self, slot: impl Into<String>, child_id: impl Into<String>, dialect: ArtifactDialect, envelope_pack: &[u8], identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), Fault> {
        self.child_composition.open_child(&mut self.stores, slot, child_id, dialect, envelope_pack, identity).await
    }

    /// 🔎️ The live child store for `(slot, child_id)`, if adopted/created — the read half of
    /// `open_child`/`register_child`.
    pub async fn child_store(&self, slot: &str, child_id: &str) -> Option<&M> {
        self.child_composition.child_store(slot, child_id).await
    }

    /// 🌱️ Adopts an already-live `M` member into the child-store map directly — the
    /// counterpart `dispatch_emit_group` uses to absorb a `store::os_store::composition_coordinator::GroupReceipt::created_children`
    /// entry (a `ChildGenesis`-minted member has no prior caller-held reference an `open_child`
    /// call could have produced it from) and the general escape hatch for a caller that already
    /// holds a constructed member from elsewhere (e.g. a host-level `SpaceHost`).
    /// 🌱️ Adopts an already-built child store under `(slot, child_id)`. Like [`Self::open_child`]
    /// it MUST seed the ownership edge into `self.composition`'s graph: `dispatch_group`'s phase 1
    /// ownership check (`CompositionGraph::owner_of`) is fail-closed, so a child present in
    /// `self.children` but absent from the graph is rejected as an `OwnershipViolation` the first
    /// time anything dispatches against it. Registering in one place and validating from the other
    /// is precisely the inconsistency this seeds away.
    pub async fn register_child(&mut self, slot: impl Into<String>, child_id: impl Into<String>, dialect: ArtifactDialect, mut member: M, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), ChildMemberRegistrationError<M>> {
        self.child_composition.register_child(&mut self.stores, slot, child_id, dialect, member, identity).await
    }
}

//#region 🔖️Driver
impl<A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + 'static> VcsArtifactApp<A, M> {
    /// 🧪️ The committed generations of the config lanes — the app config, and window `window_id` of kind `window_kind_id` once
    /// its partition exists — so a law counts the config edits a press published.
    #[cfg(any(test, feature = "artifact-app-testing"))]
    pub fn config_generations(&self, window_kind_id: &str, window_id: &str) -> (u64, Option<u64>) {
        self.stores.config_generations(window_kind_id, window_id)
    }

    /// 🧪️ The `TransactionRef` each edit of the log carries, oldest first (`None`: an edit no tool transaction authored) — a
    /// law counts edits and reads which run or press authored each.
    #[cfg(any(test, feature = "artifact-app-testing"))]
    pub fn edit_transactions(&self) -> Vec<Option<protocol::TransactionRef>> {
        self.stores.edit_transactions()
    }
}
//#endregion 🔖️Driver

impl<A:ArtifactApp,M:store::ReferenceSpaceMember+store::MemberFactory+'static> VcsArtifactApp<A,M>{
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
