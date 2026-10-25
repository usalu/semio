//! 🗄️ The test store wrapper, its member factory impls, the demo member store producer and the close helpers.
//!
//! 🧭️ Moved from the former store unit test module; the items are `pub` because the test crates of the Store use them through the public API only.

use semio_framework_os_store_testing_prelude::*;
use semio_framework_os_store_testing_fixtures::*;
use crate::*;
use semio_framework_os_store_testing_fixtures::demo::{AddN, AssignN, DeleteN, DemoMutation, SetN};
use semio_framework_os_store_testing_fixtures::lossy::{LossyMutation, SetN as LossySetN};
use semio_framework_os_store_testing_fixtures::severity::{SetErrorN, SetFatalN, SetN as SeveritySetN, SetWarningN, SeverityMutation};
use semio_framework_os_store_testing_fixtures::timestamped::{SetN as TimestampedSetN, TimestampedMutation};
use semio_framework_os_store_testing_fixtures::validated::{AssignN as ValidatedAssignN, SetN as ValidatedSetN, ValidatedMutation};
use pack::snapshot::ArtifactSqliteSnapshot as _;
use semio_framework_os_store_snapshot_clone::RetainedCloneEdit;
use semio_framework_os_store_snapshot_clone::RetainedCloneEditCursor;
use semio_framework_os_store_snapshot_clone::RetainedCloneEditStep;
use semio_framework_os_store_snapshot_clone::RetainedClonePreparationFactory;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef};
use semio_framework_pack_json_retained_schema::testing::{OWNED_SCHEMA_TEST_FIELDS, owned_schema_test_cursor, drive_owned_schema, schema_test_close_grant};
use semio_framework_value::testing::physical_test_close_grant;

pub fn drain_channel_for_test(remote: &ChannelBackboneRemote) -> Result<Vec<BackboneMessage>, VcsError> {
    let mut messages = Vec::new();
    while let Some(message) = remote.try_pop_front()? {
        messages.push(message);
    }
    Ok(messages)
}


/// 🧪️ A test store: `new` installs the document type's own retirement authorities (so it records
/// history like a production store) and the store closes itself on drop; `bare` leaves authorities to
/// the test, which then installs and closes them explicitly.
pub struct ArtifactStore<P, Mutation>(pub semio_framework_os_store_engine::lifted_store::ArtifactStore<P, Mutation>, pub Option<fn(&mut semio_framework_os_store_engine::lifted_store::ArtifactStore<P, Mutation>)>)
where
    P: Clone + ToValue + FromValue,
    Mutation: Clone + ToValue + FromValue + semio_framework_os_spr::Mutation<P> + semio_framework_os_spr::ColdMutation<P>;

impl<P, Mutation> ArtifactStore<P, Mutation>
where
    P: Clone + ToValue + FromValue + ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Send + Sync + 'static,
    Mutation: Clone + ToValue + FromValue + semio_framework_os_spr::Mutation<P> + semio_framework_os_spr::ColdMutation<P> + OpBinary + OpText + Send + 'static,
{
    pub async fn new<C:MemberStoreProducer<P,Mutation>>(envelope: ArtifactEnvelope<P, Mutation>,producer:&C) -> Self
    {
        Self::new_with_actor(envelope, ActorId(semio_framework_os_spr::LOCAL_ACTOR_ID.into()),producer).await
    }

    pub async fn new_with_actor<C:MemberStoreProducer<P,Mutation>>(envelope: ArtifactEnvelope<P, Mutation>, actor: ActorId,producer:&C) -> Self
    {
        let mut store = semio_framework_os_store_engine::lifted_store::ArtifactStore::new::<semio_framework_os_store_initialization_runtime::native_publication_producer::ArtifactSnapshotPublication>(envelope, actor).await.expect("test fixture history is valid");
        store.install_document_store_owners_exact(complete_test_catalog(producer.member_store_owners(physical_test_close_grant()))).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
        Self(store, Some(close_test_store::<P, Mutation>))
    }

    pub async fn bare(envelope: ArtifactEnvelope<P, Mutation>) -> Self {
        Self(semio_framework_os_store_engine::lifted_store::ArtifactStore::new::<semio_framework_os_store_initialization_runtime::native_publication_producer::ArtifactSnapshotPublication>(envelope, ActorId(semio_framework_os_spr::LOCAL_ACTOR_ID.into())).await.expect("test fixture history is valid"), None)
    }

    /// 🧹️ Arms the drop-close on a `bare` store once the test has installed its OWN owner catalog:
    /// a store that carries a disposer must still reach its terminal-empty witness before it drops.
    pub fn closes_on_drop(&mut self) {
        self.1 = Some(close_test_store::<P, Mutation>);
    }

    pub fn into_inner(self) -> semio_framework_os_store_engine::lifted_store::ArtifactStore<P, Mutation> {
        let this = std::mem::ManuallyDrop::new(self);
        unsafe { std::ptr::read(&this.0) }
    }

    pub async fn current_checkpoint_id(&self) -> Option<&str> {
        self.0.current_checkpoint_id()
    }
}

impl<P, Mutation> std::ops::Deref for ArtifactStore<P, Mutation>
where
    P: Clone + ToValue + FromValue,
    Mutation: Clone + ToValue + FromValue + semio_framework_os_spr::Mutation<P> + semio_framework_os_spr::ColdMutation<P>,
{
    type Target = semio_framework_os_store_engine::lifted_store::ArtifactStore<P, Mutation>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<P, Mutation> std::ops::DerefMut for ArtifactStore<P, Mutation>
where
    P: Clone + ToValue + FromValue,
    Mutation: Clone + ToValue + FromValue + semio_framework_os_spr::Mutation<P> + semio_framework_os_spr::ColdMutation<P>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub fn complete_test_catalog<P, M>(admitted: Result<(DocumentStoreOwners<P, M>, RetainedCloneProgress), DocumentStoreOwnersAdmissionError<P, M>>) -> DocumentStoreOwners<P, M>
where P: Clone + ToValue + FromValue, M: Clone + ToValue + FromValue + semio_framework_os_spr::Mutation<P> + semio_framework_os_spr::ColdMutation<P>,
{
    let grant=physical_test_close_grant();
    let (mut owners,progress)=admitted.unwrap_or_else(|error|panic!("test catalog ingress refused while retaining original custody: {}",error.error));
    assert!(progress.fits(grant));
    for _ in 0..512 {
        if owners.constructor_is_complete() { return owners; }
        owners.constructor_demands(grant.maximum_copy_bytes).expect("test catalog has genuine pending ticket demand");
        let progress=owners.admit_constructor(grant).unwrap_or_else(|(error,_)|panic!("test catalog ticket admission refused: {error}"));
        assert!(progress.fits(grant));assert!(progress.copied_items<=1);
    }
    panic!("test catalog retained incomplete original factory tickets");
}


/// 🧹️ Drives a test store's owned close to its terminal-empty witness.
pub fn close_test_store<P, Mutation>(store: &mut semio_framework_os_store_engine::lifted_store::ArtifactStore<P, Mutation>)
where
    P: Clone + ToValue + FromValue + ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Send + Sync + 'static,
    Mutation: Clone + ToValue + FromValue + semio_framework_os_spr::Mutation<P> + semio_framework_os_spr::ColdMutation<P> + OpBinary + OpText + Send + 'static,
{
    if store.close_owned_terminal_is_empty() { return; }
    if store.close_owned_phase_witness().contains("started=false") { drop(store.detach_backbone().expect("a closing test store releases its backbone handle")); }
    let grant = physical_test_close_grant();
    let mut last = RetainedCloneStep::Progress(Default::default());
    let mut stalled = 0;
    for _ in 0..1 << 24 {
        if store.close_owned_terminal_is_empty() { return; }
        let demand = store.close_owned_demands(grant.maximum_copy_bytes).expect("current independent Store close frontier");
        let turn = RetainedCloneGrant { maximum_copy_bytes: demand.copy_bytes.max(grant.maximum_copy_bytes), maximum_capacity_bytes: demand.capacity_bytes.max(grant.maximum_capacity_bytes), maximum_release_bytes: demand.release_bytes.max(grant.maximum_release_bytes), maximum_depth: demand.depth.max(grant.maximum_depth), ..grant };
        last = store.close_owned_step(turn).expect("test store closes under its exact owner grant");
        assert!(last.progress().fits(turn));
        stalled = if last.progress() == RetainedCloneProgress::default() { stalled + 1 } else { 0 };
        assert!(stalled < 4096 || std::thread::panicking(), "test store close stalled on a fully funded turn; frontier {demand:?} phase {} displaced {} outstanding reads {} returned reads {}", store.close_owned_phase_witness(), store.close_displaced_witness(), store.owners.outstanding_snapshot_read_count(), store.owners.returned_snapshot_read_count());
        if matches!(last, RetainedCloneStep::Complete(_)) { assert!(store.close_owned_terminal_is_empty()); return; }
    }
    panic!("test store did not reach its exact terminal-empty witness; last step {last:?}");
}


impl<P, Mutation> Drop for ArtifactStore<P, Mutation>
where
    P: Clone + ToValue + FromValue,
    Mutation: Clone + ToValue + FromValue + semio_framework_os_spr::Mutation<P> + semio_framework_os_spr::ColdMutation<P>,
{
    fn drop(&mut self) {
        if let Some(close) = self.1.take() {
            if !std::thread::panicking() {
                close(&mut self.0);
            }
        }
    }
}

impl<P, Mutation> SpaceMember for ArtifactStore<P, Mutation>
where
    P: Clone + ToValue + FromValue + ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + semio_framework_schema_composition::ArtifactCompositionFields + Send + Sync + 'static,
    Mutation: Clone + ToValue + FromValue + semio_framework_os_spr::Mutation<P> + semio_framework_os_spr::ColdMutation<P> + OpBinary + OpText + Send + 'static,
{
    fn local_actor_id(&self) -> &ActorId {
        SpaceMember::local_actor_id(&self.0)
    }
    async fn document_id(&self) -> &str {
        SpaceMember::document_id(&self.0).await
    }
    fn artifact_ref(&self) -> Option<semio_framework_artifact_reference::ArtifactRef> {
        SpaceMember::artifact_ref(&self.0)
    }
    fn owner_ref(&self) -> Option<OwnerRef> {
        SpaceMember::owner_ref(&self.0)
    }
    fn child_restore_projection(&self) -> Result<ChildRestoreProjection<'_>, ChildRestoreProjectionError> {
        SpaceMember::child_restore_projection(&self.0)
    }
    fn child_restore_projection_for_read<'read>(&self,read:&'read ErasedSnapshotRead)->Result<ChildRestoreProjection<'read>,ChildRestoreProjectionError>{SpaceMember::child_restore_projection_for_read(&self.0,read)}
    fn one_item_publication_identity(&self) -> (u64, [u8; 32]) {
        SpaceMember::one_item_publication_identity(&self.0)
    }
    fn one_item_wire_publication_supported(&self) -> bool {
        SpaceMember::one_item_wire_publication_supported(&self.0)
    }

    fn owned_publication_birth_bytes(&self, request: &MemberStoreOwnedBatchRequest) -> Result<usize, String> { SpaceMember::owned_publication_birth_bytes(&self.0, request) }
    fn begin_one_item_owned_publication(&self, request: &mut MemberStoreOwnedBatchRequest, grant: ArtifactStoreOneItemGrant) -> Result<Option<Box<dyn ErasedMemberStoreOneItemPublication>>, String> {
        SpaceMember::begin_one_item_owned_publication(&self.0, request, grant)
    }
    fn advance_one_item_publication(&mut self, publication: &mut dyn ErasedMemberStoreOneItemPublication, body:usize, grant: ArtifactStoreOneItemGrant, receiving:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> Result<ArtifactStoreOneItemAdvance, String> {
        SpaceMember::advance_one_item_publication(&mut self.0, publication, body, grant,receiving)
    }
    fn prepare_one_item_publication(&mut self, publication: &mut dyn ErasedMemberStoreOneItemPublication, body:usize, grant: ArtifactStoreOneItemGrant, receiving:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> Result<ArtifactStoreOneItemPreparationStep, String> {
        SpaceMember::prepare_one_item_publication(&mut self.0, publication, body, grant,receiving)
    }
    fn stage_one_item_publication(&mut self, publication: &mut dyn ErasedMemberStoreOneItemPublication, visibility: &Arc<semio_framework_os_vcs::ArtifactGroupVisibility>, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, String> {
        SpaceMember::stage_one_item_publication(&mut self.0, publication, visibility, grant)
    }
    fn adopt_one_item_publication(&mut self, publication: &mut dyn ErasedMemberStoreOneItemPublication, visibility: &Arc<semio_framework_os_vcs::ArtifactGroupVisibility>, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, String> {
        SpaceMember::adopt_one_item_publication(&mut self.0, publication, visibility, grant)
    }
    fn snapshot_read_erased_for_publication(&self, publication: &mut dyn ErasedMemberStoreOneItemPublication, visibility: &Arc<semio_framework_os_vcs::ArtifactGroupVisibility>, grant: ArtifactStoreOneItemGrant) -> Result<Option<(u64, [u8; 32], ErasedSnapshotRead, RetainedCloneProgress)>, ValueError> {
        SpaceMember::snapshot_read_erased_for_publication(&self.0, publication, visibility, grant)
    }
    fn return_prepared_snapshot_read_erased(&self, publication: &mut dyn ErasedMemberStoreOneItemPublication, visibility: &Arc<semio_framework_os_vcs::ArtifactGroupVisibility>, snapshot: ErasedSnapshotRead, grant: ArtifactStoreOneItemGrant) -> Result<RetainedCloneStep, (ValueError, ErasedSnapshotRead)> {
        SpaceMember::return_prepared_snapshot_read_erased(&self.0, publication, visibility, snapshot, grant)
    }
    fn abort_one_item_publication(&mut self, publication: &mut dyn ErasedMemberStoreOneItemPublication, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        SpaceMember::abort_one_item_publication(&mut self.0, publication, grant)
    }
    fn snapshot_read_erased_now(&self) -> Result<ErasedSnapshotRead, String> {
        SpaceMember::snapshot_read_erased_now(&self.0)
    }
    async fn snapshot_read_erased(&self) -> Result<ErasedSnapshotRead, String> {
        SpaceMember::snapshot_read_erased(&self.0).await
    }
    fn retire_snapshot_read_erased(&mut self, snapshot: &mut Option<ErasedSnapshotRead>, grant: RetainedCloneGrant) -> Result<(Option<Box<dyn ErasedSnapshotRetirement>>, RetainedCloneProgress), ValueError> {
        SpaceMember::retire_snapshot_read_erased(&mut self.0, snapshot, grant)
    }
    fn take_returned_snapshot_read_retirement(&mut self, grant: RetainedCloneGrant) -> Result<(Option<Box<dyn ErasedSnapshotRetirement>>, RetainedCloneProgress), ValueError> {
        SpaceMember::take_returned_snapshot_read_retirement(&mut self.0, grant)
    }
    fn snapshot_read_retirement_demand(&self) -> semio_framework_value::RetirementDemand {
        SpaceMember::snapshot_read_retirement_demand(&self.0)
    }
    fn returned_snapshot_read_retirement_demand(&self) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        SpaceMember::returned_snapshot_read_retirement_demand(&self.0)
    }
    fn snapshot_read_leases_terminal_is_empty(&self) -> bool {
        SpaceMember::snapshot_read_leases_terminal_is_empty(&self.0)
    }
    fn close_owned_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        SpaceMember::close_owned_step(&mut self.0, grant)
    }
    fn close_owned_terminal_is_empty(&self) -> bool {
        SpaceMember::close_owned_terminal_is_empty(&self.0)
    }
    fn close_owned_demands(&self, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        SpaceMember::close_owned_demands(&self.0, maximum_body_bytes)
    }
    fn content_revision_now(&self) -> [u8; 32] {
        SpaceMember::content_revision_now(&self.0)
    }
    async fn content_revision(&self) -> [u8; 32] {
        SpaceMember::content_revision(&self.0).await
    }
    async fn is_dirty(&self) -> bool {
        SpaceMember::is_dirty(&self.0).await
    }
    /// 🎯️ Overrides the trait default (`MergePolicy::default()`, always `Normal`) the same way
    /// the REAL `super::ArtifactStore`'s own `impl SpaceMember` block does (see that block's
    /// `merge_policy` for the full rationale) — without this override, `.merge_policy()` called
    /// on THIS wrapper (whether directly, or generically inside `CompositionCoordinator::
    /// dispatch_group`'s `M: SpaceMember` body) silently reads the trait's `Normal`-only default
    /// instead of whatever `set_merge_policy` set on the real inner store.
    async fn merge_policy(&self) -> semio_framework_os_spr::MergePolicy {
        SpaceMember::merge_policy(&self.0).await
    }

    async fn current_checkpoint_id(&self) -> Option<String> {
        SpaceMember::current_checkpoint_id(&self.0).await
    }
    async fn current_alternative_id(&self) -> Option<String> {
        SpaceMember::current_alternative_id(&self.0).await
    }


    async fn last_local_edit_timestamp(&self) -> Option<HybridLogicalTimestamp> {
        SpaceMember::last_local_edit_timestamp(&self.0).await
    }
    async fn last_undone_local_edit_timestamp(&self) -> Option<HybridLogicalTimestamp> {
        SpaceMember::last_undone_local_edit_timestamp(&self.0).await
    }


    async fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }






    async fn tail_group_id(&self) -> Option<String> {
        SpaceMember::tail_group_id(&self.0).await
    }
    async fn tail_edit_id(&self) -> Option<String> {
        SpaceMember::tail_edit_id(&self.0).await
    }
    async fn redo_tail(&self) -> Option<(String, Option<String>)> {
        SpaceMember::redo_tail(&self.0).await
    }






}

impl<P,Mutation> ReferenceSpaceMember for ArtifactStore<P,Mutation>
where P:Clone+ToValue+FromValue+ArtifactPack+semio_framework_os_store_text_io::ArtifactStoreSnapshot+semio_framework_schema_composition::ArtifactCompositionFields+Send+Sync+'static,Mutation:Clone+ToValue+FromValue+semio_framework_os_spr::Mutation<P>+semio_framework_os_spr::ColdMutation<P>+OpBinary+OpText+Send+'static,
{
    fn begin_one_item_wire_publication(&self, request: MemberStoreOneItemWireRequest) -> Result<Box<dyn ErasedMemberStoreOneItemPublication>, ArtifactStoreBatchAdmissionRejected<MemberStoreOneItemWire>> {
        ReferenceSpaceMember::begin_one_item_wire_publication(&self.0, request)
    }
    async fn commit_checkpoint(&mut self, message: String, authors: Vec<Author>, identity: &mut semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<String, VcsError> {
        ReferenceSpaceMember::commit_checkpoint(&mut self.0, message, authors, identity).await
    }
    async fn checkout(&mut self, checkpoint_id: &str, alternative_id: &str, identity: &mut semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), VcsError> {
        ReferenceSpaceMember::checkout(&mut self.0, checkpoint_id, alternative_id, identity).await
    }
    async fn create_alternative(&mut self, name: String, identity: &mut semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<String, VcsError> {
        ReferenceSpaceMember::create_alternative(&mut self.0, name, identity).await
    }
    async fn undo(&mut self, identity: &mut semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), VcsError> {
        ReferenceSpaceMember::undo(&mut self.0, identity).await
    }
    async fn redo(&mut self, identity: &mut semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), VcsError> {
        ReferenceSpaceMember::redo(&mut self.0, identity).await
    }
    async fn preview_wire(&self, ops: &[Vec<u8>]) -> Vec<semio_framework_os_spr::MutationMessage> {
        ReferenceSpaceMember::preview_wire(&self.0, ops).await
    }
    async fn dispatch_wire(&mut self, command: &[u8], identity: &mut semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<CommandReceipt, VcsError> {
        ReferenceSpaceMember::dispatch_wire(&mut self.0, command, identity).await
    }
    async fn dispatch_wire_with_policy(&mut self, command: &[u8], policy: semio_framework_os_spr::MergePolicy, identity: &mut semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<CommandReceipt, VcsError> {
        ReferenceSpaceMember::dispatch_wire_with_policy(&mut self.0, command, policy, identity).await
    }
    async fn event_log_payload(&self) -> Result<Vec<u8>, VcsError> {
        ReferenceSpaceMember::event_log_payload(&self.0).await
    }
    async fn announce_tail_edit_payload(&mut self) -> Result<Vec<u8>, VcsError> {
        ReferenceSpaceMember::announce_tail_edit_payload(&mut self.0).await
    }
    async fn ingest_remote_payload(&mut self, envelopes: &[u8]) -> Result<(), VcsError> {
        ReferenceSpaceMember::ingest_remote_payload(&mut self.0, envelopes).await
    }
    async fn stamp_tail_group_id(&mut self, group_id: &str) -> Result<(), VcsError> {
        ReferenceSpaceMember::stamp_tail_group_id(&mut self.0, group_id).await
    }
    async fn stamp_tail_origin(&mut self, origin: semio_framework_os_spr::MutationOrigin) -> Result<(), VcsError> {
        ReferenceSpaceMember::stamp_tail_origin(&mut self.0, origin).await
    }
    async fn set_owner(&mut self, owner: Option<OwnerRef>) {
        ReferenceSpaceMember::set_owner(&mut self.0, owner).await;
    }
    async fn document_pack_bytes(&self) -> Result<Vec<u8>, VcsError> {
        ReferenceSpaceMember::document_pack_bytes(&self.0).await
    }
    async fn envelope_pack_bytes(&self) -> Result<Vec<u8>, VcsError> {
        ReferenceSpaceMember::envelope_pack_bytes(&self.0).await
    }
    async fn pack_at_checkpoint(&self, checkpoint_id: &str) -> Result<Vec<u8>, VcsError> {
        ReferenceSpaceMember::pack_at_checkpoint(&self.0, checkpoint_id).await
    }
}

/// 🧪️ Exact factories for the three typed coordinator fixtures; empty genesis remains invalid.
macro_rules! fixture_member_factory {
    ($mutation:ty) => {
        impl MemberFactory for ArtifactStore<DemoSnapshot, $mutation> {
            const OPEN_DECLARATIONS: &'static [MemberOpenDeclaration] = &[MemberOpenDeclaration { kind: "s.stdio.mesh", standard: "1", subset: "*", schema: "demo/v1",history_limits:protocol::codec::HistoryReadLimits{file_bytes:67108864,frame_body_bytes:1048576,records:8192,maximum_fields:128,maximum_text_bytes:1048576,maximum_binary_bytes:1048576,maximum_json_bytes:1048576,maximum_json_allocation_bytes:8388608,maximum_json_items:65536,maximum_json_depth:64},maximum_history_allocation_bytes:8388608 }];
            type Open = UnsupportedMemberFactoryOpen<Self>;
            fn open_birth_demand(request: &MemberOpenRequest) -> Result<RetainedCloneBirthDemand, MemberOpenDiagnostic> { request.admitted_expected().map(|_| RetainedCloneBirthDemand { capacity_bytes: 0, depth: 0 }) }
            fn member_frame_bytes(&self) -> usize { 0 }
            fn begin_open(admission:&mut MemberOpenSourceAdmission,opened:&mut Option<Self::Open>,cx:&mut semio_framework_job::StepContext<'_>,parent:&semio_framework_job::OriginalCallerSourceParent)->Result<bool,semio_framework_job::OriginalTurnFailure>{
                UnsupportedMemberFactoryOpen::begin(admission,opened,cx,parent)
            }

            async fn create(id: &str, dialect: &semio_framework_artifact_reference::ArtifactDialect, initial_pack: &[u8], actor: ActorId) -> Result<Self, VcsError> {
                if dialect != &demo_child_dialect() {
                    return Err(VcsError::ValidationFailed("unregistered fixture member dialect".into()));
                }
                Ok(Self(create_member_store("demo/v1", id, dialect, initial_pack, actor, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer, &crate::DemoMemberStoreProducer::<$mutation>::new()).await?, Some(close_test_store::<DemoSnapshot, $mutation>)))
            }
            async fn open(expected: &semio_framework_artifact_reference::ArtifactRef, owner: Option<&OwnerRef>, envelope_pack: &[u8], actor: ActorId) -> Result<Self, VcsError> {
                if expected.dialect != demo_child_dialect() {
                    return Err(VcsError::ValidationFailed("unregistered fixture member dialect".into()));
                }
                Ok(Self(open_member_store("demo/v1", expected, owner, envelope_pack, actor, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer, &crate::DemoMemberStoreProducer::<$mutation>::new()).await?, Some(close_test_store::<DemoSnapshot, $mutation>)))
            }
        }

        impl MemberVisit for ArtifactStore<DemoSnapshot, $mutation> {
            fn visit_member<V: MemberStoreVisitor>(&self, visitor: V) -> V::Output {
                visitor.visit(&self.0)
            }
            async fn visit_member_mut<V: MemberStoreVisitorMut>(&mut self, visitor: V) -> V::Output {
                visitor.visit_mut(&mut self.0).await
            }
        }
    };
}

fixture_member_factory!(DemoMutation);

fixture_member_factory!(ValidatedMutation);

fixture_member_factory!(SeverityMutation);


/// 🫳️ The canonical wire an authoring catalog names for every fixture mutation, so authoring mints the operation identity from the original operation.
pub fn fixture_wire_source<M: ArtifactCanonicalJson>(mutation: &M) -> Option<ArtifactPreparedOperationSource<'_>> {
    Some(ArtifactPreparedOperationSource::CanonicalJson { header: b"fixture", body: mutation })
}

/// 🏭️ Authoring-only semantic authority of a fixture catalog: it names each original mutation's canonical wire and prepares no retained gesture.
#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct FixtureAuthoringFactory<M: 'static>(pub std::marker::PhantomData<fn() -> M>);

/// 🧬️ The catalog identity fixture prepares no semantic replay.
impl semio_framework_os_store_replay_operation::ArtifactReplayPreparationFactory<DemoSnapshot, DemoMutation> for FixtureAuthoringFactory<DemoMutation> {
 fn preparation_birth_demand(&self)->semio_framework_value::RetirementDemand{Default::default()}
 fn begin(&self,request:semio_framework_os_store_replay_operation::ArtifactReplayPreparationRequest<DemoSnapshot,DemoMutation>,_:semio_framework_value::retained_clone::RetainedCloneGrant)->Result<(Box<dyn semio_framework_os_store_replay_operation::ArtifactReplayPreparation<DemoSnapshot,DemoMutation>>,semio_framework_value::retained_clone::RetainedCloneProgress),(semio_framework_value::ValueError,semio_framework_os_store_replay_operation::ArtifactReplayPreparationRequest<DemoSnapshot,DemoMutation>)>{Err((semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"catalog identity fixture prepares no semantic replay"),request))}
}


impl<M: ArtifactCanonicalJson + ArtifactCanonicalJsonTree + semio_framework_value::retirement::RetireOwned + Send + Sync + 'static> ArtifactStoreOneItemPreparationFactory<DemoSnapshot, M> for FixtureAuthoringFactory<M> {
    fn operation_wire_source<'a>(&self, mutation: &'a M) -> Option<ArtifactPreparedOperationSource<'a>> {
        fixture_wire_source(mutation)
    }

    fn preflight(&self, _mutation: &M, _lane: HistoryLane) -> Result<ArtifactStoreOneItemFootprint, String> {
        Err("fixture authoring catalog prepares no retained gesture".into())
    }

    fn begin_batch_digest(&self, edit: &mut Option<Box<Edit<M>>>, grant: RetainedCloneGrant) -> Result<Option<(Box<dyn ArtifactStoreBatchDigest<M>>, RetainedCloneProgress)>, ValueError> {
        admit_artifact_batch_digest(edit, grant)
    }

    fn begin_demand(&self, _mutation: &M, _lane: HistoryLane) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, ValueError> {
        Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "fixture authoring catalog prepares no retained gesture"))
    }
    fn begin_copy_byte_demand(&self,input:&M,lane:HistoryLane)->Result<usize,semio_framework_value::ValueError>{Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "fixture authoring catalog prepares no retained gesture"))}


    fn begin(&self, request: ArtifactStoreOneItemPreparationRequest<DemoSnapshot, M, M>, _grant: ArtifactStoreOneItemGrant) -> Result<(Box<dyn ArtifactStoreOneItemPreparation<DemoSnapshot, M>>, RetainedCloneProgress), (ValueError, ArtifactStoreOneItemPreparationRequest<DemoSnapshot, M, M>)> {
        Err((ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "fixture authoring catalog prepares no retained gesture"), request))
    }
}

/// 🏭️ The authoring catalog of the demo document: it names each original mutation's canonical wire and originates the batch digest the store's batched publication seals, and prepares no retained gesture of its own (the publication's explicit domain factory does).
#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct DemoAuthoringFactory;

impl ArtifactStoreOneItemPreparationFactory<DemoSnapshot, DemoMutation> for DemoAuthoringFactory {
    fn operation_wire_source<'a>(&self, mutation: &'a DemoMutation) -> Option<ArtifactPreparedOperationSource<'a>> {
        fixture_wire_source(mutation)
    }

    fn preflight(&self, _mutation: &DemoMutation, _lane: HistoryLane) -> Result<ArtifactStoreOneItemFootprint, String> {
        Err("fixture authoring catalog prepares no retained gesture".into())
    }

    fn begin_batch_digest(&self, edit: &mut Option<Box<Edit<DemoMutation>>>, grant: RetainedCloneGrant) -> Result<Option<(Box<dyn ArtifactStoreBatchDigest<DemoMutation>>, RetainedCloneProgress)>, ValueError> {
        admit_artifact_batch_digest(edit, grant)
    }

    fn begin_demand(&self, _mutation: &DemoMutation, _lane: HistoryLane) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, ValueError> {
        Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "fixture authoring catalog prepares no retained gesture"))
    }
    fn begin_copy_byte_demand(&self,input:&DemoMutation,lane:HistoryLane)->Result<usize,semio_framework_value::ValueError>{Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "fixture authoring catalog prepares no retained gesture"))}


    fn begin(&self, request: ArtifactStoreOneItemPreparationRequest<DemoSnapshot, DemoMutation, DemoMutation>, _grant: ArtifactStoreOneItemGrant) -> Result<(Box<dyn ArtifactStoreOneItemPreparation<DemoSnapshot, DemoMutation>>, RetainedCloneProgress), (ValueError, ArtifactStoreOneItemPreparationRequest<DemoSnapshot, DemoMutation, DemoMutation>)> {
        Err((ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "fixture authoring catalog prepares no retained gesture"), request))
    }
}

/// 🌱️ Quotes the original catalog source tree: the three issuer Arcs, the inline disposer and the authoring factory Arc.
pub fn fixture_authoring_birth_demand<M>() -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, ValueError>
where M: ArtifactCanonicalJson + ArtifactCanonicalJsonTree + Mutation<DemoSnapshot> + semio_framework_os_spr::ColdMutation<DemoSnapshot> + semio_framework_value::retirement::RetireOwned + Clone + ToValue + FromValue + OpBinary + OpText + Send + Sync + 'static {
    let base = DocumentStoreOwners::<DemoSnapshot, M>::source_birth_bytes::<DemoSnapshotRetirementFactory, DemoInitialSnapshotRetirementFactory, DemoMutationRetirementFactory, ArtifactStoreCursorDisposer<DemoSnapshot, M>>()?;
    let capacity_bytes = base.checked_add(semio_framework_value::factory_arc_birth_bytes::<FixtureAuthoringFactory<M>>()).and_then(|bytes| bytes.checked_add(cold_replay_birth_bytes::<M>())).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "fixture authoring catalog extent overflow"))?;
    Ok(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes, depth: 1 })
}

/// 🗃️ Admits the demo member catalog together with its authoring-only semantic authority.
pub fn fixture_authoring_member_owners<M>(grant: RetainedCloneGrant) -> Result<(DocumentStoreOwners<DemoSnapshot, M>, RetainedCloneProgress), DocumentStoreOwnersAdmissionError<DemoSnapshot, M>>
where M: ArtifactCanonicalJson + ArtifactCanonicalJsonTree + Mutation<DemoSnapshot> + semio_framework_os_spr::ColdMutation<DemoSnapshot> + semio_framework_value::retirement::RetireOwned + Clone + ToValue + FromValue + OpBinary + OpText + Send + Sync + 'static {
    let preparation = semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: semio_framework_value::factory_arc_birth_bytes::<FixtureAuthoringFactory<M>>() + cold_replay_birth_bytes::<M>(), depth: 1 };
    DocumentStoreOwners::admit_source_constructor_with_semantic_preparations(grant, preparation, || (DemoSnapshotRetirementFactory, DemoInitialSnapshotRetirementFactory, DemoMutationRetirementFactory, ArtifactStoreCursorDisposer::<DemoSnapshot, M>::new(), Arc::new(FixtureAuthoringFactory::<M>(std::marker::PhantomData)) as Arc<dyn ArtifactStoreOneItemPreparationFactory<DemoSnapshot, M>>, crate::cold_demo_replay_preparation_factory::<M>()))
}

/// 🧮️ The original allocations of one cold native replay authority: the preparation factory and its three owned issuers.
fn cold_replay_birth_bytes<M: Clone + ToValue + FromValue + Mutation<DemoSnapshot> + semio_framework_os_spr::ColdMutation<DemoSnapshot> + semio_framework_value::retirement::RetireOwned + ArtifactCanonicalJsonTree + Send + Sync + 'static>() -> usize {
    semio_framework_value::factory_arc_birth_bytes::<semio_framework_os_store_snapshot_clone::RetainedClonePreparationFactory<DemoSnapshot, M, ColdRetainedCloneEdit<M>>>()
        + semio_framework_value::factory_arc_birth_bytes::<ColdRetainedCloneEdit<M>>()
        + semio_framework_value::factory_arc_birth_bytes::<DemoMutationRetirementFactory>()
        + semio_framework_value::factory_arc_birth_bytes::<DemoSnapshotRetirementFactory>()
}

/// 🗃️ Completes a demo catalog whose retirement issuers are the caller's own, adding the authoring-only publication authority and the cold native replay authority.
pub fn fixture_authoring_catalog<F1, F2, F3>(grant: RetainedCloneGrant, snapshots: F1, initial: F2, mutations: F3) -> DocumentStoreOwners<DemoSnapshot, DemoMutation>
where
    F1: SnapshotRetirementFactory<DemoSnapshot> + semio_framework_value::FactoryPayloadRetirement,
    F2: ArtifactOwnedValueRetirementFactory<DemoSnapshot> + semio_framework_value::FactoryPayloadRetirement,
    F3: ArtifactOwnedValueRetirementFactory<DemoMutation> + semio_framework_value::FactoryPayloadRetirement,
{
    let preparation = semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: semio_framework_value::factory_arc_birth_bytes::<DemoAuthoringFactory>(), depth: 1 };
    complete_test_catalog(complete_test_catalog(DocumentStoreOwners::admit_source_constructor_with_one_item_preparation(grant, preparation, || (snapshots, initial, mutations, ArtifactStoreCursorDisposer::<DemoSnapshot, DemoMutation>::new(), Arc::new(DemoAuthoringFactory) as Arc<dyn ArtifactStoreOneItemPreparationFactory<DemoSnapshot, DemoMutation>>))).with_replay_preparation(crate::cold_demo_replay_preparation_factory::<DemoMutation>(), physical_test_close_grant()))
}



pub struct DemoMemberStoreProducer<M>(pub ::std::marker::PhantomData<fn()->(M)>);

impl<M> Copy for DemoMemberStoreProducer<M> {}

impl<M> Clone for DemoMemberStoreProducer<M> {fn clone(&self)->Self{*self}}

impl<M> DemoMemberStoreProducer<M> {pub const fn new()->Self{Self(::std::marker::PhantomData)}}

impl MemberStoreProducer<DemoSnapshot,DemoMutation> for crate::DemoMemberStoreProducer<DemoMutation> {
    type SnapshotOpen = UnsupportedMemberSnapshotOpen<DemoSnapshot>;

    fn member_store_owners_birth_demand(&self) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, semio_framework_value::ValueError> {
        fixture_authoring_birth_demand::<DemoMutation>()
    }

    fn member_store_owners(&self,grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<(DocumentStoreOwners<DemoSnapshot, DemoMutation>, semio_framework_value::retained_clone::RetainedCloneProgress), semio_framework_os_store_engine_owners::DocumentStoreOwnersAdmissionError<DemoSnapshot, DemoMutation>> {
        fixture_authoring_member_owners::<DemoMutation>(grant)
    }
}


pub fn close_erased_member_publication(publication: &mut dyn ErasedMemberStoreOneItemPublication, grant: ArtifactStoreOneItemGrant) {
    publication.begin_close();
    let retained=grant.retained_grant();
    for _ in 0..65_536 {
        let(step,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||publication.close_step(retained).expect("one retained member close unit"));
        assert!(step.progress().fits(retained));assert_eq!((allocated,released),(step.progress().retained_capacity_bytes,step.progress().released_bytes));
        match step {
            RetainedCloneStep::Complete(_) => { assert!(publication.terminal_is_empty());return; }
            RetainedCloneStep::Progress(progress) => assert!(progress!=RetainedCloneProgress::default(),"admitted member close has no external wait; declared policy must advance its original owner"),
        }
    }
    panic!("member publication did not retire its exact owners");
}


pub fn retained_demo_member_owners() -> DocumentStoreOwners<DemoSnapshot, DemoMutation> {
    complete_test_catalog(complete_test_catalog(demo_base_store_owners().with_one_item_preparation(Arc::new(DemoOneItemPreparationFactory::admissible()),physical_test_close_grant())).with_one_item_wire_preparation(Arc::new(DemoMemberWirePreparationFactory::admissible()),physical_test_close_grant()))
}


#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct DemoMemberWirePreparationFactory {
    #[factory_child]
    pub typed: Arc<DemoOneItemPreparationFactory>,
}

impl DemoMemberWirePreparationFactory {
    pub fn admissible() -> Self { Self { typed: Arc::new(DemoOneItemPreparationFactory::admissible()) } }
}

pub struct DemoMemberWirePreparation {
    pub request: Option<ArtifactStoreOneItemPreparationRequest<DemoSnapshot, MemberStoreOneItemWire, DemoMutation>>,
    pub typed_request: Option<ArtifactStoreOneItemPreparationRequest<DemoSnapshot, DemoMutation, DemoMutation>>,
    pub wire: Option<MemberStoreOneItemWire>,
    pub inner: Option<Box<dyn ArtifactStoreOneItemPreparation<DemoSnapshot, DemoMutation>>>,
    pub typed: Option<Arc<DemoOneItemPreparationFactory>>,
    pub base: Option<SnapshotRead<DemoSnapshot>>,
    pub authority: Option<Arc<ArtifactStoreOneItemLiveAuthority>>,
    pub active: Option<Box<dyn ErasedSnapshotRetirement>>,
    pub factory: Option<semio_framework_value::FactoryAuthority>,
    pub offset: usize,
    pub magnitude: u32,
    pub negative: bool,
    pub state: u8,
    pub closing: bool,
}

impl MemberStoreOneItemWirePreparationFactory<DemoSnapshot, DemoMutation> for DemoMemberWirePreparationFactory {
    fn preflight(&self, wire: &MemberStoreOneItemWire, lane: HistoryLane) -> Result<ArtifactStoreOneItemFootprint, String> {
        if wire.schema != "demo.member.json-number" || wire.bytes.is_empty() || lane != HistoryLane::Document {
            return Err("demo retained member wire schema or metadata is not admitted".into());
        }
        Ok(ArtifactStoreOneItemFootprint { work_items: wire.bytes.len().saturating_add(3), retained_bytes: wire.bytes.len().saturating_add(1024) })
    }

    fn begin_demand(&self, _: &MemberStoreOneItemWire, _: HistoryLane) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, ValueError> {
        Ok(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: std::mem::size_of::<DemoMemberWirePreparation>(), depth: 1 })
    }
    fn begin_copy_byte_demand(&self,input:&MemberStoreOneItemWire,lane:HistoryLane)->Result<usize,semio_framework_value::ValueError>{self.begin_demand(input,lane)?; std::mem::size_of::<ArtifactStoreOneItemPreparationRequest<DemoSnapshot, MemberStoreOneItemWire, DemoMutation>>().checked_add(std::mem::size_of::<DemoMemberWirePreparation>()).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"original preparation header quote overflow"))}


    fn begin(&self, request: ArtifactStoreOneItemPreparationRequest<DemoSnapshot, MemberStoreOneItemWire, DemoMutation>, grant: ArtifactStoreOneItemGrant) -> Result<(Box<dyn ArtifactStoreOneItemPreparation<DemoSnapshot, DemoMutation>>, RetainedCloneProgress), (ValueError, ArtifactStoreOneItemPreparationRequest<DemoSnapshot, MemberStoreOneItemWire, DemoMutation>)> {
        let demand = match self.begin_demand(&request.mutation, request.lane) { Ok(demand) => demand, Err(error) => return Err((error, request)) };
        let progress = match demand.admit(grant.retained_grant()) { Ok(progress) => progress, Err(error) => return Err((error, request)) };
        Ok((Box::new(DemoMemberWirePreparation { request: Some(request), typed_request: None, wire: None, inner: None, typed: Some(Arc::clone(&self.typed)), base: None, authority: None, active: None, factory: None, offset: 0, magnitude: 0, negative: false, state: 0, closing: false }), progress))
    }
}

impl DemoMemberWirePreparation {
    pub fn parse_byte(&mut self, byte: u8) -> Result<(), ValueError> {
        let whitespace = matches!(byte, b' ' | b'\t' | b'\n' | b'\r');
        match self.state {
            0 if whitespace => {}
            0 if byte == b'-' => {
                self.negative = true;
                self.state = 1;
            }
            0 | 1 if byte.is_ascii_digit() => {
                self.magnitude = u32::from(byte - b'0');
                self.state = if byte == b'0' { 2 } else { 3 };
            }
            3 if byte.is_ascii_digit() => {
                self.magnitude = self.magnitude.checked_mul(10).and_then(|value| value.checked_add(u32::from(byte - b'0'))).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "member number overflows i32"))?;
                if self.magnitude > i32::MAX as u32 + u32::from(self.negative) {
                    return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "member number overflows i32"));
                }
            }
            2..=4 if whitespace => self.state = 4,
            _ => return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "member wire is not an exact JSON integer")),
        }
        Ok(())
    }


    pub fn wire_demand(wire: &MemberStoreOneItemWire) -> semio_framework_value::RetirementDemand {
        semio_framework_value::RetirementDemand { release_bytes: if wire.bytes.capacity() != 0 { wire.bytes.capacity() } else { wire.schema.capacity() }, depth: 1, ..Default::default() }
    }

    pub fn close_wire(wire: &mut MemberStoreOneItemWire, grant: RetainedCloneGrant) -> RetainedCloneProgress {
        let bytes = Self::wire_demand(wire).release_bytes;
        if grant.maximum_items == 0 || grant.maximum_release_bytes < bytes { return Default::default(); }
        if wire.bytes.capacity() != 0 { drop(std::mem::replace(&mut wire.bytes, Vec::new())); }
        else if wire.schema.capacity() != 0 { drop(std::mem::replace(&mut wire.schema, String::new())); }
        RetainedCloneProgress { copied_items: 1, released_bytes: bytes, ..Default::default() }
    }

    pub fn close_demand(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        use semio_framework_value::RetirementDemand;
        let mut demand = if let Some(inner) = self.inner.as_ref() {
            if inner.terminal_is_empty() { RetirementDemand { release_bytes: std::mem::size_of_val(inner.as_ref()), depth: 1, ..Default::default() } }
            else { RetirementDemand { copy_bytes: inner.next_close_copy_byte_demand()?, capacity_bytes: inner.next_close_capacity_byte_demand(body)?, release_bytes: inner.next_close_release_byte_demand()?, depth: inner.next_close_depth_demand()?.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"wire child depth overflow"))? } }
        } else if let Some(request) = self.typed_request.as_ref() {
            let birth = self.typed.as_ref().expect("typed request retains original issuer").begin_demand(&request.mutation, request.lane)?;
            RetirementDemand { capacity_bytes: birth.capacity_bytes, depth: birth.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"typed ingress depth overflow"))?, ..Default::default() }
        } else if let Some(wire) = self.wire.as_ref() { Self::wire_demand(wire) }
        else if let Some(request) = self.request.as_ref() {
            if request.mutation.bytes.capacity() != 0 || request.mutation.schema.capacity() != 0 { Self::wire_demand(&request.mutation) }
            else { RetirementDemand { copy_bytes: std::mem::size_of_val(request), depth: 1, ..Default::default() } }
        } else if self.base.is_some() { RetirementDemand { depth: 1, ..Default::default() } }
        else if let Some(active) = self.active.as_ref() {
            let mut demand = artifact_retirement_box_demands(active, body)?;
            demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"wire original owner depth overflow"))?;
            demand
        } else if let Some(authority) = self.authority.as_ref() { RetirementDemand { capacity_bytes: authority.retirement_birth_demand().capacity_bytes, depth: authority.retirement_birth_demand().depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"wire authority depth overflow"))?, ..Default::default() } }
        else if let Some(factory) = self.factory.as_ref() {
            let mut demand = factory.demands(body)?;
            demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"wire issuer depth overflow"))?;
            demand
        } else if self.typed.is_some() { RetirementDemand { copy_bytes: std::mem::size_of::<Arc<DemoOneItemPreparationFactory>>(), depth: 1, ..Default::default() } }
        else { RetirementDemand::default() };
        demand.depth = demand.depth.max(1);
        Ok(demand)
    }

    pub fn begin_typed(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<RetainedCloneProgress, ValueError> {
        let factory = self.typed.as_ref().expect("typed request retains original issuer");
        let original = self.typed_request.take().expect("typed ingress retains original request");
        match factory.begin(original, grant) {
            Ok((mut inner, progress)) => { if self.closing { inner.begin_close(); } self.inner = Some(inner); Ok(progress) }
            Err((error, original)) => { self.typed_request = Some(original); Err(error) }
        }
    }
}

impl ArtifactStoreOneItemPreparation<DemoSnapshot, DemoMutation> for DemoMemberWirePreparation {
    fn advance(&mut self, body:usize,grant: ArtifactStoreOneItemGrant,receiving:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> Result<ArtifactStoreOneItemPreparationStep, ValueError> {
        if !grant.permits_one() || self.closing { return Ok(ArtifactStoreOneItemPreparationStep::Blocked); }
        if let Some(inner) = self.inner.as_mut() {
            if grant.maximum_depth < 2 { return Ok(ArtifactStoreOneItemPreparationStep::Blocked); }
            return match inner.advance(body,ArtifactStoreOneItemGrant { maximum_depth: grant.maximum_depth - 1, ..grant },receiving)? {
                ArtifactStoreOneItemPreparationStep::Prepared(_, progress) => Ok(ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint(), progress)),
                ArtifactStoreOneItemPreparationStep::Progress(_, progress) => Ok(ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint(), progress)),
                ArtifactStoreOneItemPreparationStep::Blocked => Ok(ArtifactStoreOneItemPreparationStep::Blocked),
            };
        }
        if self.typed_request.is_some() {
            if grant.maximum_depth <= 1 { return Ok(ArtifactStoreOneItemPreparationStep::Blocked); }
            return match self.begin_typed(ArtifactStoreOneItemGrant { maximum_depth: grant.maximum_depth - 1, ..grant }) {
                Ok(progress) if progress.fits(grant.retained_grant()) => Ok(ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint(), progress)),
                Ok(progress) => { self.closing = true; Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "typed wire ingress exceeded supplied grant").with_retained_progress(progress)) },
                Err(error) => { self.closing = true; Err(error) },
            };
        }
        if self.request.as_ref().is_some_and(|request| self.offset < request.mutation.bytes.len()) {
            if grant.maximum_copy_bytes == 0 { return Ok(ArtifactStoreOneItemPreparationStep::Blocked); }
            let byte = self.request.as_ref().expect("wire retains original input").mutation.bytes[self.offset];
            let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: 1, ..Default::default() };
            if let Err(error) = self.parse_byte(byte) { self.closing = true; return Err(error.with_retained_progress(progress)); }
            self.offset += 1;
            return Ok(ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint(), progress));
        }
        if !matches!(self.state, 2..=4) { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "member wire ended without an integer")); }
        let copied = std::mem::size_of::<ArtifactStoreOneItemPreparationRequest<DemoSnapshot, MemberStoreOneItemWire, DemoMutation>>();
        if grant.maximum_copy_bytes < copied { return Ok(ArtifactStoreOneItemPreparationStep::Blocked); }
        let request = self.request.take().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "member wire lost its request"))?;
        let n = if self.negative { -(i64::from(self.magnitude)) as i32 } else { self.magnitude as i32 };
        self.wire = Some(request.mutation);
        self.typed_request = Some(ArtifactStoreOneItemPreparationRequest { operation: request.operation, generation: request.generation, base_revision: request.base_revision, lane: request.lane, authority: request.authority, base: request.base, mutation: DemoMutation::SetN(SetN { n }), mutation_retirement:request.mutation_retirement,snapshot_retirement:request.snapshot_retirement });
        Ok(ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint(), RetainedCloneProgress { copied_items: 1, copied_bytes: copied, ..Default::default() }))
    }

    fn checkpoint(&self) -> ArtifactStoreOneItemCheckpoint {
        let inner = self.inner.as_ref().map_or_else(ArtifactStoreOneItemCheckpoint::default, |inner| inner.checkpoint());
        ArtifactStoreOneItemCheckpoint { cursor: self.offset as u32 + inner.cursor, completed_items: self.offset as u32 + inner.completed_items, completed_bytes: self.offset as u64, digest: inner.digest }
    }
    fn prepared(&self) -> Option<&ArtifactStoreOneItemPrepared<DemoSnapshot, DemoMutation>> { self.inner.as_ref().and_then(|inner| inner.prepared()) }
    fn take_prepared(&mut self) -> Option<ArtifactStoreOneItemPrepared<DemoSnapshot, DemoMutation>> { self.inner.as_mut().and_then(|inner| inner.take_prepared()) }
    fn cancel(&mut self) { self.closing = true; if let Some(inner) = self.inner.as_mut() { inner.cancel(); } }
    fn begin_close(&mut self) { self.closing = true; if let Some(inner) = self.inner.as_mut() { inner.begin_close(); } }
    fn close_step(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<RetainedCloneStep, ValueError> {
        let grant = grant.retained_grant(); let empty = RetainedCloneProgress::default();
        if !self.closing || grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
        let demand = self.close_demand(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"wire close exceeds supplied depth")); }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
        if let Some(inner) = self.inner.as_mut() {
            if inner.terminal_is_empty() { let released_bytes = std::mem::size_of_val(inner.as_ref()); drop(self.inner.take()); return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes, ..empty })); }
            let step = inner.close_step(ArtifactStoreOneItemGrant { maximum_items: child.maximum_items, maximum_copy_bytes: child.maximum_copy_bytes, maximum_capacity_bytes: child.maximum_capacity_bytes, maximum_release_bytes: child.maximum_release_bytes, maximum_depth: child.maximum_depth })?;
            semio_framework_value::retained_clone::admit_retained_clone_close(child,step,inner.terminal_is_empty(),"captured typed wire preparation")?;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if self.typed_request.is_some() {
            let progress = self.begin_typed(ArtifactStoreOneItemGrant { maximum_items: child.maximum_items, maximum_copy_bytes: child.maximum_copy_bytes, maximum_capacity_bytes: child.maximum_capacity_bytes, maximum_release_bytes: child.maximum_release_bytes, maximum_depth: child.maximum_depth })?;
            return Ok(RetainedCloneStep::Progress(progress));
        }
        if let Some(wire) = self.wire.as_mut() {
            if wire.bytes.capacity() != 0 || wire.schema.capacity() != 0 { return Ok(RetainedCloneStep::Progress(Self::close_wire(wire,grant))); }
            self.wire = None; return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }));
        }
        if let Some(request) = self.request.as_mut() {
            if request.mutation.bytes.capacity() != 0 || request.mutation.schema.capacity() != 0 { return Ok(RetainedCloneStep::Progress(Self::close_wire(&mut request.mutation,grant))); }
            let request = self.request.take().unwrap(); self.base = Some(request.base); self.authority = Some(request.authority);
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty }));
        }
        if let Some(original) = self.base.take() {
            return match original.try_return_to_registry_witness() {
                Ok(_) => Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty })),
                Err(original) => { self.base = Some(original); Ok(RetainedCloneStep::Progress(empty)) }
            };
        }
        if self.active.is_some() { let step = artifact_retirement_box_close_step(&mut self.active,child)?; return Ok(RetainedCloneStep::Progress(step.progress())); }
        if let Some(original) = self.authority.take() {
            return match original.retire(child) {
                Ok((owner,progress)) => { self.active = Some(owner); Ok(RetainedCloneStep::Progress(progress)) },
                Err((error,original)) => { self.authority = Some(original); Err(error) }
            };
        }
        if let Some(factory) = self.factory.as_mut() {
            let step = factory.step(child)?;
            semio_framework_value::retained_clone::admit_retained_clone_close(child,step,factory.terminal_is_empty(),"captured wire issuer")?;
            if factory.terminal_is_empty() { self.factory = None; }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(original) = self.typed.take() {
            let original: Arc<dyn semio_framework_value::FactoryRetirement> = original;
            self.factory = Some(semio_framework_value::FactoryAuthority::new(original));
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty }));
        }
        Ok(RetainedCloneStep::Complete(empty))
    }
    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demand(0)?.copy_bytes) }
    fn next_close_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { Ok(self.close_demand(body)?.capacity_bytes) }
    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demand(0)?.release_bytes) }
    fn next_close_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demand(0)?.depth) }
    fn terminal_is_empty(&self) -> bool { self.closing && self.request.is_none() && self.typed_request.is_none() && self.wire.is_none() && self.inner.is_none() && self.base.is_none() && self.authority.is_none() && self.active.is_none() && self.typed.is_none() && self.factory.is_none() }
}


pub fn close_durable_publication(publication: &mut ArtifactStoreBatchPublication<DemoSnapshot, DemoMutation>) {
    let grant=physical_test_close_grant();
    for _ in 0..4_096 {
        let(step,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||publication.close_step(grant).expect("declared durable publication close"));
        assert!(step.progress().fits(grant));assert_eq!((allocated,released),(step.progress().retained_capacity_bytes,step.progress().released_bytes));
        if matches!(step,RetainedCloneStep::Complete(_)) { assert!(publication.terminal_is_empty());return; }
    }
    panic!("durable publication did not reach terminal empty");
}

pub fn close_demo_artifact_store(store: &mut ArtifactStore<DemoSnapshot, DemoMutation>) { close_test_store(store); }


/// 🔁️ Settles every physical replay owner before exposing a report to the fixture.
pub fn drive_test_report_replay<P, M>(replay: &mut EditReplay<P, M>, edits: &impl ReplayEdits<M>)
where P: Clone, M: semio_framework_os_spr::Mutation<P> + semio_framework_os_spr::ColdMutation<P> + OpBinary,
{
    for _ in 0..10_000 {
        if matches!(replay.step(edits, &mut || false).expect("bounded report replay advances"), ReplayStep::Finished(_)) {
            assert!(replay.is_finished());
            assert_eq!(replay.progress().done, replay.progress().total);
            return;
        }
    }
    panic!("report replay did not reach its exact terminal witness");
}


pub fn demo_closable_store_owners() -> DocumentStoreOwners<DemoSnapshot, DemoMutation> {
    fixture_authoring_catalog(physical_test_close_grant(), DemoSnapshotRetirementFactory, DemoInitialSnapshotRetirementFactory, DemoMutationRetirementFactory)
}

/// 🗃️ The demo catalog before any semantic preparation authority is installed, for laws that install their own.
pub fn demo_base_store_owners() -> DocumentStoreOwners<DemoSnapshot, DemoMutation> {
    complete_test_catalog(DocumentStoreOwners::admit_source_constructor(physical_test_close_grant(), || (DemoSnapshotRetirementFactory, DemoInitialSnapshotRetirementFactory, DemoMutationRetirementFactory, ArtifactStoreCursorDisposer::<DemoSnapshot, DemoMutation>::new())))
}


impl MemberStoreProducer<DemoSnapshot,SeverityMutation> for crate::DemoMemberStoreProducer<SeverityMutation> {
    type SnapshotOpen = UnsupportedMemberSnapshotOpen<DemoSnapshot>;

    fn member_store_owners_birth_demand(&self) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, semio_framework_value::ValueError> {
        fixture_authoring_birth_demand::<SeverityMutation>()
    }

    fn member_store_owners(&self,grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<(DocumentStoreOwners<DemoSnapshot, SeverityMutation>, semio_framework_value::retained_clone::RetainedCloneProgress), semio_framework_os_store_engine_owners::DocumentStoreOwnersAdmissionError<DemoSnapshot, SeverityMutation>> {
        fixture_authoring_member_owners::<SeverityMutation>(grant)
    }
}


/// 🛂️ The composition fixture's negative value is a fatal outcome, so the dry-run
/// rejects it before group dispatch can alter either member's history.

impl MemberStoreProducer<DemoSnapshot,ValidatedMutation> for crate::DemoMemberStoreProducer<ValidatedMutation> {
    type SnapshotOpen = UnsupportedMemberSnapshotOpen<DemoSnapshot>;

    fn member_store_owners_birth_demand(&self) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, semio_framework_value::ValueError> {
        fixture_authoring_birth_demand::<ValidatedMutation>()
    }

    fn member_store_owners(&self,grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<(DocumentStoreOwners<DemoSnapshot, ValidatedMutation>, semio_framework_value::retained_clone::RetainedCloneProgress), semio_framework_os_store_engine_owners::DocumentStoreOwnersAdmissionError<DemoSnapshot, ValidatedMutation>> {
        fixture_authoring_member_owners::<ValidatedMutation>(grant)
    }
}

impl MemberStoreProducer<DemoSnapshot,TimestampedMutation> for crate::DemoMemberStoreProducer<TimestampedMutation> {
    type SnapshotOpen = UnsupportedMemberSnapshotOpen<DemoSnapshot>;

    fn member_store_owners_birth_demand(&self) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, semio_framework_value::ValueError> {
        fixture_authoring_birth_demand::<TimestampedMutation>()
    }

    fn member_store_owners(&self,grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<(DocumentStoreOwners<DemoSnapshot, TimestampedMutation>, semio_framework_value::retained_clone::RetainedCloneProgress), semio_framework_os_store_engine_owners::DocumentStoreOwnersAdmissionError<DemoSnapshot, TimestampedMutation>> {
        fixture_authoring_member_owners::<TimestampedMutation>(grant)
    }
}
