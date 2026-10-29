//! 🪞️ Delegating accessors and reads: the store accessors that delegate to the wire, cursor, log, owners and live components. See [`super`].

use super::*;
use semio_framework_os_spr::ColdMutation;

impl<P, Mutation> ArtifactStore<P, Mutation>
where
    P: semio_framework_os_spr::StoreState,
    Mutation: semio_framework_os_spr::StoreMutationFor<P>,
{


    /// 🪞️ Delegates to [`StoreWire::artifact_revision`].
    #[inline] pub fn artifact_revision(&self) -> ArtifactRevision { self.wire.artifact_revision(&self.log) }

    /// 🪞️ Delegates to [`StoreCursor::generation_now`].
    #[inline] pub fn generation_now(&self) -> u64 { self.cursor.generation_now(&self.wire) }

    /// 🪞️ Delegates to [`StoreLog::clock_now`].
    #[inline] pub fn clock_now(&self) -> HybridLogicalTimestamp { self.log.clock_now() }


    /// 🪞️ Delegates to [`StoreCursor::content_revision_now`].
    #[inline] pub fn content_revision_now(&self) -> [u8; 32] { self.cursor.content_revision_now(&self.wire) }

    /// 🪞️ Delegates to [`StoreCursor::projection_stamp`].
    #[inline] pub fn projection_stamp(&self) -> ArtifactProjectionStamp { self.cursor.projection_stamp(&self.log, &self.wire) }

    /// 🪞️ Delegates to [`StoreLive::projection_event`].
    

    /// 🪞️ Delegates to [`StoreCursor::last_projection_invalidation`].
    #[inline] pub fn last_projection_invalidation(&self) -> Option<ArtifactProjectionInvalidation> { self.cursor.last_projection_invalidation(&self.log, &self.wire) }

    /// 🪞️ Delegates to [`StoreReplay::invalidate_after_replay`].
    #[inline] pub fn invalidate_after_replay(&mut self) -> Result<ArtifactProjectionInvalidation, VcsError> { self.replay.invalidate_after_replay(&mut self.log, &mut self.owners, &mut self.wire, &mut self.cursor, &mut self.live) }

    /// 🪞️ Delegates to [`StoreReplay::invalidate_after_policy_change`].
    #[inline] pub fn invalidate_after_policy_change(&mut self) -> Result<ArtifactProjectionInvalidation, VcsError> { self.replay.invalidate_after_policy_change(&mut self.log, &mut self.owners, &mut self.wire, &mut self.cursor, &mut self.live) }

    /// 🪞️ Delegates to [`StoreReplay::invalidate_after_external_resource_change`].
    #[inline] pub fn invalidate_after_external_resource_change(&mut self) -> Result<ArtifactProjectionInvalidation, VcsError> { self.replay.invalidate_after_external_resource_change(&mut self.log, &mut self.owners, &mut self.wire, &mut self.cursor, &mut self.live) }

    /// 🪞️ Delegates to [`StoreReplay::invalidate_projections`].
    #[inline] pub fn invalidate_projections(&mut self, cause: ArtifactProjectionCause) -> Result<ArtifactProjectionInvalidation, VcsError> { self.replay.invalidate_projections(&mut self.log, &mut self.owners, &mut self.wire, &mut self.cursor, &mut self.live, cause) }

    /// 🪞️ Delegates to [`StoreCursor::accept_projection_result`].
    #[inline] pub fn accept_projection_result<Output, Diff>(&self, result: ArtifactProjectionResult<Output, Diff>) -> Result<AcceptedArtifactProjection<Output, Diff>, StaleArtifactProjection> { self.cursor.accept_projection_result::<Output, Diff>(&self.log, &self.wire, result) }

    /// 🪞️ Delegates to [`StoreLog::envelope`].
    #[inline] pub fn envelope(&self) -> &ArtifactEnvelope<P, Mutation> { self.log.envelope() }

    /// 🪞️ Delegates to [`StoreLog::envelope_view`].
    #[inline] pub fn envelope_view(&self) -> ArtifactEnvelopeView<'_, P, Mutation> { self.log.envelope_view() }

    /// 🪞️ Delegates to [`StoreWire::applied_edit_ids`].
    #[inline] pub fn applied_edit_ids(&self) -> &semio_framework_os_vcs::HistoryPageStack<String> { self.wire.applied_edit_ids(&self.log) }

    /// 🪞️ Delegates to [`StoreWire::redo_edit_ids`].
    #[inline] pub fn redo_edit_ids(&self) -> &semio_framework_os_vcs::HistoryPageStack<String> { self.wire.redo_edit_ids(&self.log) }

    /// 🪞️ Delegates to [`StoreLog::current_checkpoint_id`].
    #[inline] pub fn current_checkpoint_id(&self) -> Option<&str> { self.log.current_checkpoint_id() }


    /// 🪞️ Delegates to [`StoreLog::authoring_verb`].
    #[inline] pub fn authoring_verb(&self) -> Option<&str> { self.log.authoring_verb() }

    /// 🪞️ Delegates to [`StoreLog::set_authoring_verb`].
    #[inline] pub fn set_authoring_verb(&mut self, verb: Option<String>) { self.log.set_authoring_verb(verb) }

    /// 🪞️ Delegates to [`StoreLog::open_transaction`].
    #[inline] pub fn open_transaction(&self) -> Option<&OpenToolTransaction> { self.log.open_transaction() }

    /// 🪞️ Delegates to [`StoreLog::edit_mutations`].
    #[inline] pub fn edit_mutations(&self) -> Option<(&[Mutation], &semio_framework_value_persistent::list::PagedList<Mutation, {usize::MAX}>, &[MutationMeta])> { self.log.edit_mutations() }

    /// 🪞️ Delegates to [`StoreLog::history_columns`].
    #[inline] pub async fn history_columns(&self) -> Vec<HistoryColumn> { self.log.history_columns().await }

    /// 🪞️ Delegates to [`StoreLive::snapshot`].
    #[inline] pub fn snapshot(&self) -> Result<P, VcsError> where P: Clone { self.live.snapshot(&self.wire) }

    /// 🪞️ Delegates to [`StoreLive::snapshot_ref`].
    #[inline] pub fn snapshot_ref(&self) -> &P { self.live.snapshot_ref(&self.wire) }

    /// 🪞️ Delegates to [`StoreLive::derived_snapshot_head`].
    #[inline] pub fn derived_snapshot_head(&self) -> ArtifactDerivedSnapshot<P> { self.live.derived_snapshot_head(&self.owners, &self.wire, &self.cursor) }

    /// 🪞️ Delegates to [`StoreCursor::snapshot_read_derived`].
    #[inline] pub fn snapshot_read_derived(&self, source: &ArtifactDerivedSnapshot<P>) -> Result<ErasedSnapshotRead, VcsError> where P: Sync, { self.cursor.snapshot_read_derived(&self.owners, &self.wire, source) }

    /// 🪞️ Delegates to [`StoreOwners::retained_derived_read`].
    

    /// 🪞️ Delegates to [`StoreCursor::derive_provisional_snapshot`].
    

    /// 🪞️ Delegates to [`StoreLive::derived_snapshot_before`].
    #[inline] pub fn derived_snapshot_before(&mut self, target: &MutationId, drafts: &protocol::HistoryInputDrafts) -> Result<ArtifactDerivedSnapshot<P>, VcsError>  where P: Clone, Mutation: ColdMutation<P> { self.live.derived_snapshot_before(&mut self.log, &mut self.owners, &mut self.wire, &mut self.cursor, target, drafts) }

    /// 🪞️ Delegates to [`StoreCursor::derive_history_snapshot`].
    

    /// 🪞️ Delegates to [`StoreCursor::begin_derived_history_preview`].
    

    /// 🪞️ Delegates to [`StoreOwners::prepare_history_read_retirement`].
    #[inline] pub fn prepare_history_read_retirement(&self,grant:RetainedCloneGrant)->Result<Option<(PreparedHistoryReadRetirement<P,Mutation>,RetainedCloneProgress)>,ValueError> where P:Sync { self.owners.prepare_history_read_retirement(grant) }

    /// 🪞️ Delegates to [`StoreOwners::retire_derived_history_preview`].
    

    /// 🪞️ Delegates to [`StoreOwners::retire_derived_report_replay`].
    

    /// 🪞️ Delegates to [`StoreOwners::retire_finished_history_replay`].
    #[inline] pub fn retire_finished_history_replay(&self, owner: &mut Option<EditReplayResult<P, Mutation>>,grant:RetainedCloneGrant) -> Result<Option<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress)>, ValueError> where P: Sync { self.owners.retire_finished_history_replay(owner, grant) }

    /// 🪞️ Delegates to [`StoreLive::step_derived_history_preview`].
    

    /// 🪞️ Delegates to [`StoreCursor::finish_derived_history_preview`].
    #[inline] pub fn finish_derived_history_preview(&self, owner: &mut Option<ArtifactDerivedHistoryPreview<P, Mutation>>) -> Result<ArtifactDerivedSnapshot<P>, VcsError> { self.cursor.finish_derived_history_preview(&self.owners, &self.wire, owner) }

    /// 🪞️ Delegates to [`StoreCursor::begin_derived_report_replay`].
    

    /// 🪞️ Delegates to [`StoreLive::step_derived_report_replay`].
    


    #[inline] pub fn finish_derived_report_replay(&self, owner: &mut Option<ArtifactDerivedReplay<P, Mutation>>) -> Result<(EditReplayResult<P, Mutation>, Option<ArtifactDerivedSnapshot<P>>), VcsError> { self.cursor.finish_derived_report_replay(&self.owners, &self.wire, owner) }

    /// 🪞️ Delegates to [`StoreLive::snapshot_read`].
    #[inline] pub fn snapshot_read(&self) -> Result<SnapshotRead<P>, VcsError> where P: Sync, { self.live.snapshot_read(&self.owners, &self.wire, &self.cursor) }
    /// 🎟️ Preserves the exact original read issuer and native refusal without constructing an owned error message.
    #[inline] pub fn snapshot_read_original(&self) -> Result<SnapshotRead<P>, semio_framework_value::ValueError> where P: Sync, { self.live.snapshot_read_original(&self.owners,&self.wire,&self.cursor) }


    /// 🪞️ Delegates to [`StoreLive::snapshot_owner`].
    #[inline] pub fn snapshot_owner(&self) -> Arc<P> { self.live.snapshot_owner(&self.wire) }

    /// 🪞️ Delegates to [`StoreLive::snapshot_root`].
    #[inline] pub fn snapshot_root(&self) -> Arc<P> { self.live.snapshot_root(&self.wire) }

    /// 🪞️ Delegates to [`StoreOwners::owned_snapshot_retirement_factory`].
    #[inline] pub fn owned_snapshot_retirement_factory(&self)->Result<&Arc<dyn ArtifactOwnedValueRetirementFactory<P>>,ValueError> { self.owners.owned_snapshot_retirement_factory() }

    /// 🪞️ Delegates to [`StoreOwners::owned_mutation_retirement_factory`].
    #[inline] pub fn owned_mutation_retirement_factory(&self)->Result<&Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,ValueError> { self.owners.owned_mutation_retirement_factory() }

    /// 🪞️ Delegates to [`StoreOwners::owned_mutation_retirement_birth_bytes`].
    #[inline] pub fn owned_mutation_retirement_birth_bytes(&self)->Result<usize,ValueError> { self.owners.owned_mutation_retirement_birth_bytes() }

    /// 🪞️ Delegates to [`StoreLog::history_read_retirement_birth_bytes`].
    #[inline] pub fn history_read_retirement_birth_bytes()->usize { StoreLog::<P,Mutation>::history_read_retirement_birth_bytes() }

    /// 🪞️ Delegates to [`StoreLog::snapshot_alias_retirement_birth_bytes`].
    #[inline] pub fn snapshot_alias_retirement_birth_bytes()->usize { StoreLog::<P,Mutation>::snapshot_alias_retirement_birth_bytes() }

    /// 🪞️ Delegates to [`StoreOwners::retire_owned_mutation`].
    #[inline] pub fn retire_owned_mutation(&self,owner:&mut Option<Mutation>,grant:RetainedCloneGrant)->Result<Option<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress)>,ValueError> { self.owners.retire_owned_mutation(owner, grant) }

    /// 🪞️ Delegates to [`StoreOwners::install_document_store_owners_exact`].
    #[inline] pub fn install_document_store_owners_exact(&mut self, mut owners: DocumentStoreOwners<P, Mutation>) -> Result<(), (ValueError, DocumentStoreOwners<P, Mutation>)> { self.owners.install_document_store_owners_exact(&mut self.log, owners) }

    /// 🪞️ Delegates to [`StoreLog::install_operation_wire`].
    #[inline] pub fn install_operation_wire(&mut self, wire: ArtifactOperationWire<Mutation>) { self.log.install_operation_wire(wire) }

    /// 🪞️ Delegates to [`StoreLog::operation_wire`].
    #[inline] pub fn operation_wire(&self) -> Option<ArtifactOperationWire<Mutation>> { self.log.operation_wire() }

    /// 🪞️ Delegates to [`StoreOwners::owned_disposer_installed`].
    #[inline] pub fn owned_disposer_installed(&self) -> bool { self.owners.owned_disposer_installed() }

    /// 🪞️ Delegates to [`StoreLive::replace_current_retained`].
    #[inline] pub fn replace_current_retained(&mut self, next: Arc<P>) -> Result<(), VcsError> { self.live.replace_current_retained(&mut self.log, &mut self.owners, next) }

    /// 🪞️ Delegates to [`StoreLive::prepare_document_root_commit`].
    #[inline] pub(super) fn prepare_document_root_commit(&mut self, additional_displaced_owners: usize) -> Result<ArtifactStoreDocumentRootCommitAuthority<P, Mutation>, VcsError> { self.live.prepare_document_root_commit(&mut self.owners, additional_displaced_owners) }

    /// 🪞️ Delegates to [`StoreLive::commit_document_roots_retained`].
    #[inline] pub(super) fn commit_document_roots_retained( &mut self, next_envelope: ArtifactEnvelope<P, Mutation>, next_current: Arc<P>, next_dag: semio_framework_os_spr::MutationDag, next_tail: Option<(String, Arc<P>)>, authority: ArtifactStoreDocumentRootCommitAuthority<P, Mutation>, ) { self.live.commit_document_roots_retained(&mut self.log, &mut self.owners, next_envelope, next_current, next_dag, next_tail, authority) }

    /// 🪞️ Delegates to [`StoreCursor::push_revision_replacement_reserved`].
    #[inline] pub fn push_revision_replacement_reserved(&mut self, next: CursorRevisionAccumulator) { self.cursor.push_revision_replacement_reserved(&mut self.owners, next) }

    /// 🪞️ Delegates to [`StoreLive::replace_applied_edit_ids_retained`].
    #[inline] pub fn replace_applied_edit_ids_retained(&mut self, next: Vec<String>) -> Result<(), VcsError> { self.live.replace_applied_edit_ids_retained(&mut self.log, &mut self.owners, &mut self.cursor, next) }

    /// 🪞️ Delegates to [`StoreLive::replace_redo_edit_ids_retained`].
    #[inline] pub fn replace_redo_edit_ids_retained(&mut self, next: Vec<String>) -> Result<(), VcsError> { self.live.replace_redo_edit_ids_retained(&mut self.log, &mut self.owners, &mut self.cursor, next) }

    /// 🪞️ Delegates to [`StoreReplay::replace_pending_report_retained`].
    #[inline] pub(super) fn replace_pending_report_retained(&mut self, next: PendingCommandReport) -> Result<(), VcsError> { self.replay.replace_pending_report_retained(&mut self.owners, next) }

    /// 🪞️ Delegates to [`StoreWire::replace_backbone_retained`].
    #[inline] pub fn replace_backbone_retained(&mut self, next: Option<Backbones>) -> Result<(), VcsError> { self.wire.replace_backbone_retained(&mut self.owners, next) }

    /// 🪞️ Delegates to [`StoreOwners::replace_causal_dag_retained`].
    #[inline] fn replace_causal_dag_retained(&mut self, next: semio_framework_os_spr::MutationDag) -> Result<(), VcsError> { self.owners.replace_causal_dag_retained(&mut self.log, next) }

    /// 🪞️ Delegates to [`StoreOwners::reserve_edit_history_slot`].
    #[inline] pub fn reserve_edit_history_slot(&mut self) -> Result<ArtifactStoreHistoryCommitReservation, VcsError> { self.owners.reserve_edit_history_slot(&mut self.log) }

    /// 🪞️ Delegates to [`StoreLive::insert_reserved_edit_history`].
    #[inline] pub fn insert_reserved_edit_history(&mut self, mut reservation: ArtifactStoreHistoryCommitReservation, edit: Edit<Mutation>) -> Result<ArtifactHistoryKey, VcsError> { self.live.insert_reserved_edit_history(&mut self.log, &mut self.owners, reservation, edit) }

    /// 🪞️ Delegates to [`StoreLive::replace_tail_undo_cache_retained`].
    #[inline] pub fn replace_tail_undo_cache_retained(&mut self, next: Option<(String, Arc<P>)>) -> Result<(), VcsError> { self.live.replace_tail_undo_cache_retained(&mut self.log, &mut self.owners, next) }

    /// 🪞️ Delegates to [`StoreOwners::maintenance_retirements_step`].
    #[inline] pub fn maintenance_retirements_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> { self.owners.maintenance_retirements_step(grant) }

    /// 🪞️ Delegates to [`StoreOwners::handoff_batch_publication_close`].
    #[inline] pub fn handoff_batch_publication_close(&mut self, publication: &mut Option<ArtifactStoreBatchPublication<P, Mutation>>) -> Result<bool, VcsError> where P: Sync, { self.owners.handoff_batch_publication_close(publication) }

    /// 🪞️ Delegates to [`StoreOwners::maintenance_retirements_terminal_is_empty`].
    #[inline] pub fn maintenance_retirements_terminal_is_empty(&self) -> bool { self.owners.maintenance_retirements_terminal_is_empty() }

    /// 🪞️ Delegates to [`StoreOwners::maintenance_retirements_demands`].
    #[inline] pub fn maintenance_retirements_demands(&self, copy: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> { self.owners.maintenance_retirements_demands(copy) }

    /// 🪞️ Delegates to [`StoreOwners::maintenance_retirements_under_pressure`].
    #[inline] pub fn maintenance_retirements_under_pressure(&self) -> bool { self.owners.maintenance_retirements_under_pressure() }

    /// 🪞️ Delegates to [`StoreOwners::close_owned_phase_witness`].
    #[inline] pub fn close_owned_phase_witness(&self) -> String { self.owners.close_owned_phase_witness() }

    /// 🪞️ Delegates to [`StoreWire::close_displaced_witness`].
    #[inline] pub fn close_displaced_witness(&self) -> String { self.wire.close_displaced_witness(&self.owners) }

    /// 🪞️ Delegates to [`StoreOwners::close_one_factory_alias`].
    #[inline] pub(super) fn close_one_factory_alias(&mut self) { self.owners.close_one_factory_alias() }

    /// 🪞️ Delegates to [`StoreOwners::retire_snapshot_alias`].
    #[inline] pub fn retire_snapshot_alias(&self,alias:&mut Option<Arc<P>>,grant:RetainedCloneGrant)->Result<Option<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress)>,ValueError> where P:Sync { self.owners.retire_snapshot_alias(alias, grant) }

    /// 🪞️ Delegates to [`StoreOwners::snapshot_read_retirement_demand`].
    #[inline] pub fn snapshot_read_retirement_demand(&self) -> semio_framework_value::RetirementDemand { self.owners.snapshot_read_retirement_demand() }

    /// 🪞️ Delegates to [`StoreOwners::returned_snapshot_read_retirement_demand`].
    #[inline] pub fn returned_snapshot_read_retirement_demand(&self) -> Result<semio_framework_value::RetirementDemand, ValueError> where P: Sync, { self.owners.returned_snapshot_read_retirement_demand() }

    /// 🪞️ Delegates to [`StoreOwners::take_returned_snapshot_read_retirement`].
    #[inline] pub fn take_returned_snapshot_read_retirement(&mut self, grant: RetainedCloneGrant) -> Result<(Option<Box<dyn ErasedSnapshotRetirement>>, RetainedCloneProgress), ValueError> where P: Sync, { self.owners.take_returned_snapshot_read_retirement(grant) }

    /// 🪞️ Delegates to [`StoreLive::close_cursor_demands`].
    #[inline] pub fn close_cursor_demands(&self, phase: ArtifactStoreCursorDisposerPhase, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> { self.live.close_cursor_demands(&self.log, &self.owners, &self.wire, phase, body) }

    /// 🪞️ Delegates to [`StoreWire::close_take_backbone_retirement`].
    #[inline] pub fn close_take_backbone_retirement(&mut self, grant: RetainedCloneGrant) -> Result<(Option<Box<dyn ErasedSnapshotRetirement>>, RetainedCloneProgress), ValueError> { self.wire.close_take_backbone_retirement(grant) }

    /// 🪞️ Delegates to [`StoreOwners::snapshot_read_leases_terminal_is_empty`].
    #[inline] pub fn snapshot_read_leases_terminal_is_empty(&self) -> bool { self.owners.snapshot_read_leases_terminal_is_empty() }

    /// 🪞️ Delegates to [`StoreOwners::returned_snapshot_read_count`].
    #[inline] pub fn returned_snapshot_read_count(&self) -> usize { self.owners.returned_snapshot_read_count() }

    /// 🪞️ Delegates to [`StoreOwners::outstanding_snapshot_read_count`].
    #[inline] pub fn outstanding_snapshot_read_count(&self) -> usize { self.owners.outstanding_snapshot_read_count() }

    /// 🪞️ Delegates to [`StoreLive::fold_current`].
    #[inline] pub async fn fold_current(&self) -> Result<P, VcsError> where P: Clone, Mutation: semio_framework_os_spr::ColdMutation<P> { self.live.fold_current(&self.log).await }

    /// 🪞️ Delegates to [`StoreReplay::replay_suffix_partitioned`].
    #[inline] pub fn replay_suffix_partitioned( base: Arc<P>, order: &[String], k: usize, edits: &impl ReplayEdits<Mutation>, schema: &str, supersessions: &EffectiveSupersessions, policy: semio_framework_os_spr::MergePolicy, ) -> Result<EditReplayResult<P, Mutation>, VcsError> { StoreReplay::<P, Mutation>::replay_suffix_partitioned(base, order, k, edits, schema, supersessions, policy) }

    /// 🪞️ Delegates to [`StoreLog::ensure_open_conflict_capacity`].
    #[inline] fn ensure_open_conflict_capacity(&self, additional_open: usize) -> Result<(), VcsError> { self.log.ensure_open_conflict_capacity(additional_open) }

    /// 🪞️ Delegates to [`StoreOwners::prune_resolved_conflicts`].
    #[inline] pub fn prune_resolved_conflicts(&mut self) -> Result<(), VcsError> { self.owners.prune_resolved_conflicts(&mut self.log) }

    /// 🪞️ Delegates to [`StoreLog::conflicts`].
    #[inline] pub fn conflicts(&self) -> &[semio_framework_os_spr::Conflict] { self.log.conflicts() }

    /// 🪞️ Delegates to [`StoreLog::open_conflicts`].
    #[inline] pub fn open_conflicts(&self) -> impl Iterator<Item = &semio_framework_os_spr::Conflict> { self.log.open_conflicts() }

    /// 🪞️ Delegates to [`StoreLog::merge_policy`].
    #[inline] pub fn merge_policy(&self) -> semio_framework_os_spr::MergePolicy { self.log.merge_policy() }

    /// 🪞️ Delegates to [`StoreLog::set_merge_policy`].
    #[inline] pub fn set_merge_policy(&mut self, policy: semio_framework_os_spr::MergePolicy) { self.log.set_merge_policy(policy) }

    /// 🪞️ Delegates to [`StoreLog::messages_for_edit`].
    #[inline] pub fn messages_for_edit(&self, edit_id: &str) -> &[semio_framework_os_spr::MutationMessage] { self.log.messages_for_edit(edit_id) }

    /// 🪞️ Delegates to [`StoreLog::trunk_alternative_id`].
    #[inline] pub fn trunk_alternative_id(&self) -> String { self.log.trunk_alternative_id() }

    /// 🪞️ Delegates to [`StoreLog::active_line_id`].
    #[inline] pub fn active_line_id(&self) -> String { self.log.active_line_id() }

    /// 🪞️ Delegates to [`StoreLive::supersessions`].
    #[inline] pub fn supersessions(&self) -> &EffectiveSupersessions { self.live.supersessions() }

    /// 🪞️ Delegates to [`StoreLog::replay_edits`].
    #[inline] pub fn replay_edits(&self) -> &ArtifactHistoryLedger<Edit<Mutation>> { self.log.replay_edits() }

}

impl<P, Mutation> ArtifactStore<P, Mutation>
where
    P: semio_framework_os_spr::StoreState,
    Mutation: semio_framework_os_spr::StoreMutationFor<P>,
{
    /// 🪞️ Delegates to [`StoreWire::ensure_durable_group_idle`].
    #[inline] pub fn ensure_durable_group_idle(&self) -> Result<(), VcsError> { self.wire.ensure_durable_group_idle() }

    /// 🪞️ Delegates to [`StoreCursor::generation`].
    #[inline] pub fn generation(&self) -> u64 { self.cursor.generation(&self.wire) }

    /// 🪞️ Delegates to [`StoreCursor::content_revision`].
    #[inline] pub fn content_revision(&self) -> [u8; 32] { self.cursor.content_revision(&self.wire) }

    /// 🪞️ Delegates to [`StoreLog::local_actor_id`].
    #[inline] pub fn local_actor_id(&self) -> &ActorId { self.log.local_actor_id() }
}

impl<P, Mutation> ArtifactStore<P, Mutation>
where
    P: semio_framework_os_spr::StoreState,
    Mutation: semio_framework_os_spr::StoreMutationFor<P>,
{
    /// 🪞️ Delegates to [`StoreCursor::finish_derived_report_replay`].
    /// 🎁️ Moves the completed original result while preserving all typed source and recipient identity.
    #[inline] pub fn take_derived_report_result(&self,owner:&mut Option<ArtifactDerivedReplay<P,Mutation>>)->Result<EditReplayResult<P,Mutation>,VcsError>{self.cursor.take_derived_report_result(&self.owners,&self.wire,owner)}
}
