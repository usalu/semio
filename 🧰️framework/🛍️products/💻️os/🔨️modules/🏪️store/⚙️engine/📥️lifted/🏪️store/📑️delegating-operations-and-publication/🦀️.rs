//! 📑️ Delegating operations and publication: the store operations that delegate to the live, replay, cursor and log components. See [`super`].

use super::*;
use semio_framework_os_spr::ColdMutation;

impl<P, Mutation> ArtifactStore<P, Mutation>
where
    P: semio_framework_os_spr::StoreState,
    Mutation: semio_framework_os_spr::StoreMutationFor<P>,
{
    /// 🪞️ Delegates to [`StoreLive::mutation_ops`].
    #[inline] pub fn mutation_ops(&self) -> Result<Vec<AppliedMutation<'_, Mutation>>, VcsError> { self.live.mutation_ops(&self.log, &self.wire, &self.cursor) }

    /// 🪞️ Delegates to [`StoreCursor::mutation_position`].
    #[inline] pub fn mutation_position(&self, target: &MutationId) -> Result<(usize, usize), VcsError> { self.cursor.mutation_position(&self.log, &self.wire, target) }

    /// 🪞️ Delegates to [`StoreLive::applied_mutation`].
    #[inline] pub fn applied_mutation(&self, target: &MutationId) -> Result<AppliedMutation<'_, Mutation>, VcsError> { self.live.applied_mutation(&self.log, &self.wire, &self.cursor, target) }

    /// 🪞️ Delegates to [`StoreLive::applied_edit_mutations`].
    #[inline] pub fn applied_edit_mutations(&self, position: usize) -> Result<Vec<AppliedMutation<'_, Mutation>>, VcsError> { self.live.applied_edit_mutations(&self.log, &self.wire, &self.cursor, position) }

    /// 🪞️ Delegates to [`StoreLive::mutation_outcomes`].
    #[inline] pub fn mutation_outcomes(&self) -> Result<Vec<protocol::MutationReplayOutcome>, VcsError> { self.live.mutation_outcomes(&self.log, &self.wire, &self.cursor) }

    /// 🪞️ Delegates to [`StoreLive::applied_edit_outcomes`].
    #[inline] pub fn applied_edit_outcomes(&self, position: usize) -> Result<Vec<protocol::MutationReplayOutcome>, VcsError> { self.live.applied_edit_outcomes(&self.log, &self.wire, &self.cursor, position) }

    /// 🪞️ Delegates to [`StoreCursor::applied_edit_position`].
    #[inline] pub fn applied_edit_position(&self, edit_id: &str) -> Option<usize> { self.cursor.applied_edit_position(&self.log, &self.wire, edit_id) }

    /// 🪞️ Delegates to [`StoreLive::unit_id`].
    #[inline] pub fn unit_id(&self, mutation_id: &MutationId) -> Result<Option<String>, VcsError> { self.live.unit_id(&self.log, &self.wire, &self.cursor, mutation_id) }

    /// 🪞️ Delegates to [`StoreCursor::unit_operations`].
    #[inline] pub fn unit_operations(&self, mutation_id: &MutationId) -> Result<Vec<MutationId>, VcsError> { self.cursor.unit_operations(&self.log, &self.wire, mutation_id) }

    /// 🪞️ Delegates to [`StoreLive::state_before`].
    #[inline] pub fn state_before(&mut self, mutation_id: &MutationId, drafts: &protocol::HistoryInputDrafts) -> Result<Arc<P>, VcsError> where P: Clone, Mutation: ColdMutation<P> { self.live.state_before(&mut self.log, &mut self.owners, &mut self.wire, &mut self.cursor, mutation_id, drafts) }

    /// 🪞️ Delegates to [`StoreLive::begin_report_replay`].
    #[inline] pub fn begin_report_replay(&self, drafts: &protocol::HistoryInputDrafts, from: Option<&MutationId>) -> Result<EditReplay<P, Mutation>, VcsError> { self.live.begin_report_replay(&self.log, &self.wire, &self.cursor, drafts, from) }

    /// 🪞️ Delegates to [`StoreLive::refuse_blocking_history`].
    #[inline] pub fn refuse_blocking_history(&self) -> Result<(), VcsError> { self.live.refuse_blocking_history(&self.log) }

    /// 🪞️ Delegates to [`StoreLive::replay_report`].
    #[inline] pub fn replay_report(&self, result: &EditReplayResult<P, Mutation>) -> Result<protocol::ReplayReport, VcsError> { self.live.replay_report(&self.log, &self.wire, &self.cursor, result) }

    /// 🪞️ Delegates to [`StoreCursor::admit_apply_batch`].
    #[inline] pub fn admit_apply_batch( &self, operation:semio_framework_job::OperationId, expected_generation:u64, expected_revision:[u8;32], actor:&mut Option<semio_framework_value::retirement::controlled::ControlledRetirement<semio_framework_value::SharedUtf8>>, mutations:&mut Vec<Mutation>, lane:HistoryLane, factory:Option<&Arc<dyn ArtifactStoreOneItemPreparationFactory<P,Mutation>>>, transaction:&mut Option<protocol::TransactionRef>, verb:Option<&str>, group:Option<&str>, outbound:bool, grant:RetainedCloneGrant, )->Result<Option<(ArtifactStoreBatchPublication<P,Mutation>,RetainedCloneProgress)>,ValueError> where P:Sync,Mutation:Send { self.cursor.admit_apply_batch(&self.log, &self.owners, &self.wire, operation, expected_generation, expected_revision, actor, mutations, lane, factory, transaction, verb, group, outbound, grant) }

    /// 🪞️ Delegates to [`StoreCursor::begin_apply_batch`].
    #[inline] pub fn begin_apply_batch( &self, operation: semio_framework_job::OperationId, expected_generation: u64, expected_revision: [u8; 32], actor: semio_framework_value::SharedUtf8, mutations: Vec<Mutation>, lane: HistoryLane, factory: Option<&Arc<dyn ArtifactStoreOneItemPreparationFactory<P, Mutation>>>, transaction: Option<protocol::TransactionRef>, ) -> Result<ArtifactStoreBatchPublication<P, Mutation>, ArtifactStoreBatchAdmissionRejected<Mutation>> where P: Sync, Mutation: Send, { self.cursor.begin_apply_batch(&self.log, &self.owners, &self.wire, operation, expected_generation, expected_revision, actor, mutations, lane, factory, transaction) }

    /// 🪞️ Delegates to [`StoreCursor::begin_outbound_apply_batch`].
    #[allow(clippy::too_many_arguments)] #[inline] pub fn begin_outbound_apply_batch( &self, operation: semio_framework_job::OperationId, expected_generation: u64, expected_revision: [u8; 32], actor: semio_framework_value::SharedUtf8, mutations: Vec<Mutation>, factory: Option<&Arc<dyn ArtifactStoreOneItemPreparationFactory<P, Mutation>>>, transaction: Option<protocol::TransactionRef>, ) -> Result<ArtifactStoreBatchPublication<P, Mutation>, ArtifactStoreBatchAdmissionRejected<Mutation>> where P: Sync, Mutation: Send, { self.cursor.begin_outbound_apply_batch(&self.log, &self.owners, &self.wire, operation, expected_generation, expected_revision, actor, mutations, factory, transaction) }

    /// 📤️ Borrows the original queued causal bytes through the Store's actual live ownership.
    pub fn original_outbound_message(&self)->Option<&dyn semio_framework_value::ByteReadSource>{self.wire.original_outbox.front().map(|bytes|bytes as &dyn semio_framework_value::ByteReadSource)}

    /// 🚇️ Transfers retained intent to the actual attached actor queue without encoding or gathering bytes.
    pub fn advance_original_outbound(&mut self,grant:RetainedCloneGrant)->Result<Option<(bool,RetainedCloneProgress)>,ValueError>{
        if self.wire.original_outbox.front().is_none(){if self.wire.original_outbox.terminal_is_empty(){return Ok(None)}let step=self.wire.original_outbox.recycle_step(grant)?;let p=step.progress();return Ok((p!=Default::default()).then_some((false,p)))}
        match self.wire.backbone.as_ref(){Some(Backbones::Channel(channel))=>channel.send_original(&mut self.wire.original_outbox,grant),_=>Ok(None)}
    }

    /// 🛑️ Returns private original causal suffixes through their issuing Store before generic batch teardown.
    pub fn prepare_apply_batch_close(&mut self,publication:&mut ArtifactStoreBatchPublication<P,Mutation>,grant:RetainedCloneGrant)->Result<Option<RetainedCloneProgress>,ValueError>{
        let Some(stage)=publication.stage.as_mut()else{return Ok(None)};
        if !publication.original_registry.as_ref().is_some_and(|registry|registry.ptr_eq(&self.owners.snapshot_read_leases)){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"batch abort requires its original Store issuer"))}
        let admits=semio_framework_value::retained_clone::retained_clone_grant_admits;
        if let Some(owner)=stage.seed_retiring.as_mut(){let d=if owner.terminal_is_empty(){semio_framework_value::RetirementDemand{copy_bytes:size_of::<Option<semio_framework_value::retirement::controlled::ControlledRetirement<String>>>(),depth:1,..Default::default()}}else{owner.next_demand(grant.maximum_copy_bytes)?};if !admits(grant,d){return Ok(Some(Default::default()))}if !owner.terminal_is_empty(){return owner.step(grant).map(|step|Some(step.progress()))}stage.seed_retiring=None;return Ok(Some(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}
        if stage.seed_pending.is_some(){let d=semio_framework_value::RetirementDemand{copy_bytes:size_of::<Option<protocol::MutationId>>()+size_of::<semio_framework_value::retirement::controlled::ControlledRetirement<String>>(),depth:1,..Default::default()};if !admits(grant,d){return Ok(Some(Default::default()))}let id=stage.seed_pending.take().unwrap();stage.seed_retiring=Some(semio_framework_value::retirement::controlled::ControlledRetirement::new(id.0).map_err(|(error,_)|error)?);return Ok(Some(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}
        if let Some(owner)=stage.seed_owner.as_ref(){let d=self.log.dag.original_seed_abort_demands(owner)?;if !admits(grant,d){return Ok(Some(Default::default()))}let Some((id,p))=self.log.dag.abort_original_seed_step(&mut stage.seed_owner,grant)?else{return Ok(Some(Default::default()))};stage.seed_pending=id.map(protocol::MutationId);return Ok(Some(p))}
        if let Some(metadata)=stage.commit_metadata.as_mut(){
            use semio_framework_os_store_read::{ArtifactStoreStringVectorRetirement,ArtifactStoreRevisionAccumulatorRetirement,ArtifactStoreStringRetirement};
            use semio_framework_value::retirement::frame::admit_retirement_frame;
            let error=|message|ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,message);
            if metadata.history.is_some(){let d=semio_framework_value::RetirementDemand{copy_bytes:size_of::<Option<semio_framework_os_store_ledger::ArtifactStoreHistoryCommitReservation>>(),depth:1,..Default::default()};if !admits(grant,d){return Ok(Some(Default::default()))}let original=metadata.history.take().unwrap();if let Err(history)=self.log.envelope.vcs.edits.cancel_reservation(original.history){metadata.history=Some(semio_framework_os_store_ledger::ArtifactStoreHistoryCommitReservation{history,rejected_owner:original.rejected_owner});return Err(error("original metadata abort lost its exact history reservation"))}self.owners.displaced_retirements.release_owner_slots(original.rejected_owner).map_err(|_|error("original metadata abort lost its rejected owner reservation"))?;return Ok(Some(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}
            if let Some(reservation)=metadata.displaced.as_mut(){
                macro_rules! retire_stack{($field:ident)=>{if metadata.$field.capacity()!=0{let d=semio_framework_value::RetirementDemand{capacity_bytes:size_of::<ArtifactStoreStringVectorRetirement>(),depth:1,..Default::default()};if !admits(grant,d){return Ok(Some(Default::default()))}let original=std::mem::replace(&mut metadata.$field,semio_framework_os_vcs::HistoryPageStack::empty());match admit_retirement_frame(original,grant,ArtifactStoreStringVectorRetirement::new){Ok((owner,p))=>{self.owners.displaced_retirements.push_owner_reserved(reservation,owner).unwrap_or_else(|_|panic!("original metadata retains its exact reserved disposer slot"));return Ok(Some(p))},Err((reason,original))=>{metadata.$field=original;return Err(reason)}}}}}
                retire_stack!(applied);retire_stack!(cursor);
                macro_rules! retire_optional{($field:ident,$kind:ty,$construct:path)=>{if metadata.$field.is_some(){let d=semio_framework_value::RetirementDemand{capacity_bytes:size_of::<$kind>(),depth:1,..Default::default()};if !admits(grant,d){return Ok(Some(Default::default()))}let original=metadata.$field.take().unwrap();match admit_retirement_frame(original,grant,$construct){Ok((owner,p))=>{self.owners.displaced_retirements.push_owner_reserved(reservation,owner).unwrap_or_else(|_|panic!("original metadata retains its exact reserved disposer slot"));return Ok(Some(p))},Err((reason,original))=>{metadata.$field=Some(original);return Err(reason)}}}}}
                retire_optional!(cursor_redo,ArtifactStoreStringVectorRetirement,ArtifactStoreStringVectorRetirement::new);retire_optional!(log_redo,ArtifactStoreStringVectorRetirement,ArtifactStoreStringVectorRetirement::new);
                retire_optional!(revision,ArtifactStoreRevisionAccumulatorRetirement,ArtifactStoreRevisionAccumulatorRetirement::new);retire_optional!(checkpoint,ArtifactStoreStringRetirement,ArtifactStoreStringRetirement::new);
                let d=semio_framework_value::RetirementDemand{copy_bytes:size_of::<Option<semio_framework_os_store_read::ArtifactStoreDisplacedOwnerReservation>>(),depth:1,..Default::default()};if !admits(grant,d){return Ok(Some(Default::default()))}self.owners.displaced_retirements.release_owner_slots(metadata.displaced.take().unwrap()).map_err(|_|error("original metadata abort lost its reserved disposer slots"))?;return Ok(Some(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))
            }
            let d=semio_framework_value::RetirementDemand{copy_bytes:size_of::<Option<semio_framework_os_store_one_item_batch::lifted_member_group::BatchCommitMetadata>>(),depth:1,..Default::default()};if !admits(grant,d){return Ok(Some(Default::default()))}stage.commit_metadata=None;return Ok(Some(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))
        }
        Ok(None)
    }

    /// 🪞️ Delegates to [`StoreReplay::flush_published_apply_batch`].
    #[inline] pub async fn flush_published_apply_batch(&mut self, publication: &mut ArtifactStoreBatchPublication<P, Mutation>) -> Result<bool, VcsError> { self.replay.flush_published_apply_batch(&mut self.log, &mut self.owners, &mut self.wire, publication).await }

    /// 🪞️ Delegates to [`StoreCursor::admit_member_apply_batch`].
    #[inline] pub fn admit_member_apply_batch( &self, operation:semio_framework_job::OperationId, expected_generation:u64, expected_revision:[u8;32], actor:&mut Option<semio_framework_value::retirement::controlled::ControlledRetirement<semio_framework_value::SharedUtf8>>, mutations:&mut Vec<Mutation>, transaction:&mut Option<protocol::TransactionRef>, grant:RetainedCloneGrant, )->Result<Option<(ArtifactStoreBatchPublication<P,Mutation>,RetainedCloneProgress)>,ValueError> where P:Sync,Mutation:Send { self.cursor.admit_member_apply_batch(&self.log, &self.owners, &self.wire, operation, expected_generation, expected_revision, actor, mutations, transaction, grant) }

    /// 🪞️ Delegates to [`StoreCursor::begin_member_apply_batch`].
    #[inline] pub fn begin_member_apply_batch( &self, operation: semio_framework_job::OperationId, expected_generation: u64, expected_revision: [u8; 32], actor: semio_framework_value::SharedUtf8, mutations: Vec<Mutation>, ) -> Result<ArtifactStoreBatchPublication<P, Mutation>, ArtifactStoreBatchAdmissionRejected<Mutation>> where P: Sync, Mutation: Send, { self.cursor.begin_member_apply_batch(&self.log, &self.owners, &self.wire, operation, expected_generation, expected_revision, actor, mutations) }

    /// 🪞️ Delegates to [`StoreCursor::begin_apply_batch_owned`].
    #[allow(clippy::too_many_arguments)] #[inline] pub fn begin_apply_batch_owned<A>( &self, operation: semio_framework_job::OperationId, expected_generation: u64, expected_revision: [u8; 32], actor: semio_framework_value::SharedUtf8, lane: HistoryLane, group_id: Option<String>, source: ArtifactStoreBatchSourceOf<P, Mutation, A>, outbound: bool, transaction: Option<protocol::TransactionRef>, ) -> Result<ArtifactStoreBatchPublication<P, Mutation>, ArtifactStoreBatchAdmissionRejected<A::Input>> where P: Sync + Send, A: ArtifactStoreBatchItemAuthority<P, Mutation> + 'static, Mutation: 'static, P: 'static, { self.cursor.begin_apply_batch_owned::<A>(&self.log, &self.owners,&self.wire, operation, expected_generation, expected_revision, actor, lane, group_id, source, outbound, transaction) }

    /// 🪞️ Delegates to [`StoreLive::batch_item_base`].
    #[inline] fn batch_item_base(&self, staged_root: Option<&Arc<P>>) -> Result<SnapshotRead<P>, VcsError> where P: Sync, { self.live.batch_item_base(&self.owners, &self.wire, &self.cursor, staged_root) }

    /// 🪞️ Delegates to [`StoreLog::batch_amend_target_ref`].
    #[inline] fn batch_amend_target_ref(&self, transaction: Option<&protocol::TransactionRef>) -> Option<&str> { self.log.batch_amend_target_ref(transaction) }

    /// 🪞️ Delegates to [`StoreLog::batch_amend_target`].
    #[inline] fn batch_amend_target(&self, transaction: Option<&protocol::TransactionRef>) -> Option<String> { self.log.batch_amend_target(transaction) }

    /// 🪞️ Delegates to [`StoreLive::advance_prepared_foreign_steps`].
    #[inline] pub fn advance_prepared_foreign_steps( &self, publication:&ArtifactStoreBatchPublication<P,Mutation>, output:&mut protocol::ForeignStepsPreparation, grant:RetainedCloneGrant, )->Result<RetainedCloneStep,ValueError> { self.live.advance_prepared_foreign_steps(&self.owners, &self.cursor, publication, output, grant) }

    /// 🪞️ Delegates to [`StoreLive::advance_prepared_operation_wire`].
    #[inline] pub fn advance_prepared_operation_wire( &self, publication:&ArtifactStoreBatchPublication<P,Mutation>, cursor:&mut ArtifactPreparedOperationCursor,output:&mut[u8],grant:RetainedCloneGrant, )->Result<ArtifactPreparedOperationProgress,ArtifactPreparedOperationError> { self.live.advance_prepared_operation_wire(&self.owners, &self.cursor, publication, cursor, output, grant) }

    /// 🪞️ Delegates to [`StoreLog::cancel_apply_batch`].
    #[inline] pub fn cancel_apply_batch(&mut self, publication: &mut ArtifactStoreBatchPublication<P, Mutation>) -> bool { self.log.cancel_apply_batch(publication) }

    /// 🪞️ Delegates to [`StoreOwners::fold_batch_item`].
    #[inline] fn fold_batch_item(&mut self, publication: &mut ArtifactStoreBatchPublication<P, Mutation>, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemAdvance, VcsError> { self.owners.fold_batch_item(&mut self.log, publication, grant) }

    /// 🪞️ Delegates to [`StoreOwners::empty_batch_stage`].
    #[inline] pub fn empty_batch_stage(authority: &Arc<ArtifactStoreOneItemLiveAuthority>) -> ArtifactStoreBatchStage<P, Mutation> { StoreOwners::<P, Mutation>::empty_batch_stage(authority) }

    /// 🪞️ Delegates to [`StoreOwners::empty_batch_edit`].
    #[inline] pub fn empty_batch_edit(authority: &ArtifactStoreOneItemLiveAuthority) -> Edit<Mutation> { StoreOwners::<P, Mutation>::empty_batch_edit(authority) }

    /// 🪞️ Delegates to [`StoreLog::edit_mutation_ids`].
    #[inline] pub fn edit_mutation_ids(&self, edit_id: &str) -> Result<Vec<MutationId>, VcsError> { self.log.edit_mutation_ids(edit_id) }

    /// 🪞️ Delegates to [`StoreLog::checkpoint_origin`].
    #[inline] pub fn checkpoint_origin(&self, checkpoint_id: &str) -> Option<MutationId> { self.log.checkpoint_origin(checkpoint_id) }

    /// 🪞️ Delegates to [`StoreLog::seed_known_events`].
    #[inline] pub(super) fn seed_known_events(&mut self) -> Result<(), VcsError> { self.log.seed_known_events() }

    /// 🪞️ Delegates to [`StoreReplay::admit_remote_transitions`].
    #[inline] async fn admit_remote_transitions(&mut self, transitions: Vec<semio_framework_os_spr::MutationEnvelope>) -> Result<Vec<semio_framework_os_spr::EditMessages>, VcsError> where P: Clone, Mutation: ColdMutation<P> { self.replay.admit_remote_transitions(&mut self.log, &mut self.owners, &mut self.wire, &mut self.cursor, &mut self.live, transitions).await }

    /// 🪞️ Delegates to [`StoreLog::admit_history_shape`].
    #[inline] pub fn admit_history_shape(&self, envelopes: &[semio_framework_os_spr::MutationEnvelope]) -> Result<(), VcsError> { self.log.admit_history_shape(envelopes) }

    /// 🪞️ Delegates to [`StoreReplay::retract_transitions`].
    #[inline] pub(super) async fn retract_transitions(&mut self, refused: &[String]) -> Result<Vec<String>, VcsError> where P: Clone, Mutation: semio_framework_os_spr::ColdMutation<P> { self.replay.retract_transitions(&mut self.log, &mut self.owners, &mut self.cursor, &mut self.live, refused).await }

    /// 🪞️ Delegates to [`StoreLive::insert_transition`].
    #[inline] pub fn insert_transition(&mut self, envelope: semio_framework_os_spr::MutationEnvelope) { self.live.insert_transition(&mut self.log, envelope) }

    /// 🪞️ Delegates to [`StoreLog::transition_envelope`].
    #[inline] pub fn transition_envelope(&self, transition: &semio_framework_os_spr::HistoryTransition, dependencies: Vec<MutationId>, target: Vec<String>, timestamp: HybridLogicalTimestamp) -> semio_framework_os_spr::MutationEnvelope { self.log.transition_envelope(transition, dependencies, target, timestamp) }

    /// 🪞️ Delegates to [`StoreLive::reproject`].
    #[inline] pub(super) async fn reproject(&mut self) -> Result<Vec<semio_framework_os_spr::EditMessages>, VcsError>  where P: Clone, Mutation: ColdMutation<P> { self.live.reproject(&mut self.log, &mut self.owners, &mut self.cursor).await }

    /// 🪞️ Delegates to [`StoreReplay::tail_step`].
    #[inline] pub(super) fn tail_step(&self, envelope: &semio_framework_os_spr::MutationEnvelope) -> Option<TailStep> { self.replay.tail_step(&self.log, &self.cursor, &self.live, envelope) }

    /// 🪞️ Delegates to [`StoreLive::live_prefix_state`].
    #[inline] pub fn live_prefix_state(&self, length: usize) -> Result<(Arc<P>, Vec<(usize, Option<[u8; 32]>, Arc<P>)>), VcsError> where P: Clone, Mutation: semio_framework_os_spr::ColdMutation<P> { self.live.live_prefix_state(&self.log, &self.cursor, length) }

    /// 🪞️ Delegates to [`StoreLive::reproject_replayed`].
    #[inline] pub fn reproject_replayed(&mut self, result: EditReplayResult<P, Mutation>) -> Result<Vec<semio_framework_os_spr::EditMessages>, VcsError> where P: Clone, Mutation: ColdMutation<P> { self.live.reproject_replayed(&mut self.log, &mut self.owners, &mut self.cursor, result) }

    /// 🪞️ Delegates to [`StoreOwners::retire_replay_result`].
    #[inline] pub fn retire_replay_result(&mut self, mut result: EditReplayResult<P, Mutation>) -> Result<(), VcsError> { self.owners.retire_replay_result(result) }

    /// 🪞️ Delegates to [`StoreOwners::retire_pending_replay`].
    #[inline] pub fn retire_pending_replay(&mut self, replay: Option<([u8; 32], EditReplay<P, Mutation>)>) -> Result<(), VcsError> { self.owners.retire_pending_replay(replay) }

    /// 🪞️ Delegates to [`StoreLive::replace_supersessions_retained`].
    #[inline] pub fn replace_supersessions_retained(&mut self, next: EffectiveSupersessions) -> Result<(), VcsError> { self.live.replace_supersessions_retained(&mut self.owners, next) }

    /// 🪞️ Delegates to [`StoreLive::admit_prefix_snapshots`].
    #[inline] pub fn admit_prefix_snapshots(&mut self, snapshots: Vec<(usize, Option<[u8; 32]>, Arc<P>)>) -> Result<(), VcsError> { self.live.admit_prefix_snapshots(&mut self.log, &mut self.owners, snapshots) }

    /// 🪞️ Delegates to [`StoreLive::clear_prefix_ring`].
    #[inline] pub(super) fn clear_prefix_ring(&mut self) -> Result<(), VcsError> { self.live.clear_prefix_ring(&mut self.owners) }

    /// 🪞️ Delegates to [`StoreReplay::append_transaction`].
    #[inline] pub(super) async fn append_transaction(&mut self, mutations: Vec<Mutation>, transaction: protocol::TransactionRef, identity: &mut EntityIdentityAuthority<'_>) -> Result<(), VcsError> where P: Clone, Mutation: semio_framework_os_spr::ColdMutation<P> { self.replay.append_transaction(&mut self.log, &mut self.owners, &mut self.cursor, &mut self.live, mutations, transaction, identity).await }

    /// 🪞️ Delegates to [`StoreReplay::continue_pending_replay_across_append`].
    #[inline] pub fn continue_pending_replay_across_append(&mut self, before: [u8; 32], edit_id: &str, appended: usize) { self.replay.continue_pending_replay_across_append(&mut self.cursor, before, edit_id, appended) }

    /// 🪞️ Delegates to [`StoreReplay::commit_open_transaction`].
    #[inline] pub fn commit_open_transaction(&mut self, transaction_id: &str) -> Result<(), VcsError> { self.replay.commit_open_transaction(&mut self.log, &mut self.owners, &mut self.cursor, &mut self.live, transaction_id) }

    /// 🪞️ Delegates to [`StoreReplay::abort_open_transaction`].
    #[inline] pub fn abort_open_transaction(&mut self, transaction_id: &str) -> Result<(), VcsError> where P: Clone, Mutation: semio_framework_os_spr::ColdMutation<P> { self.replay.abort_open_transaction(&mut self.log, &mut self.owners, &mut self.cursor, &mut self.live, transaction_id) }

    /// 🪞️ Delegates to [`StoreReplay::flush_announcements`].
    #[inline] pub async fn flush_announcements(&mut self) -> Result<(), VcsError> { self.replay.flush_announcements(&mut self.owners, &mut self.wire).await }

    /// 🪞️ Delegates to [`StoreLive::lift_open_transaction`].
    #[inline] pub(super) async fn lift_open_transaction(&mut self) -> Result<Option<(OpenToolTransaction, Edit<Mutation>)>, VcsError> where P: Clone, Mutation: semio_framework_os_spr::ColdMutation<P> { self.live.lift_open_transaction(&mut self.log, &mut self.owners, &mut self.cursor).await }

    /// 🪞️ Delegates to [`StoreReplay::restore_open_transaction`].
    #[inline] pub(super) async fn restore_open_transaction(&mut self, open: OpenToolTransaction, mut edit: Edit<Mutation>) -> Result<(), VcsError> where P: Clone, Mutation: semio_framework_os_spr::ColdMutation<P> { self.replay.restore_open_transaction(&mut self.log, &mut self.owners, &mut self.cursor, &mut self.live, open, edit).await }

    /// 🪞️ Delegates to [`StoreReplay::defer_remote_replays`].
    #[inline] pub fn defer_remote_replays(&mut self, budget: Option<ReplayTurnBudget>) { self.replay.defer_remote_replays(budget) }

    /// 🪞️ Delegates to [`StoreReplay::defer_local_replays`].
    #[inline] pub fn defer_local_replays(&mut self, budget: Option<ReplayTurnBudget>) { self.replay.defer_local_replays(budget) }



    /// 🪞️ Delegates to [`StoreReplay::discard_local_step`].
    #[inline] pub fn discard_local_step(&mut self) -> bool { self.replay.discard_local_step(&mut self.owners) }

    /// 🪞️ Delegates to [`StoreReplay::reprojection_progress`].
    #[inline] pub fn reprojection_progress(&self) -> Option<ReplayProgress> { self.replay.reprojection_progress(&self.cursor) }

    /// 🪞️ Delegates to [`StoreReplay::reprojection_operation_preparation_progress`].
    #[inline] pub fn reprojection_operation_preparation_progress(&self) -> Option<ArtifactReplayPreparationProgress> { self.replay.reprojection_operation_preparation_progress(&self.cursor) }

    /// 🪞️ Delegates to [`StoreReplay::cancel_reprojection`].
    #[inline] pub fn cancel_reprojection(&mut self) -> bool { self.replay.cancel_reprojection(&mut self.owners) }

    /// 🪞️ Delegates to [`StoreReplay::step_reprojection`].
    #[inline] pub async fn step_reprojection(&mut self, deadline_us: Option<u64>) -> Result<Option<ReplayProgress>, VcsError> where P: Clone, Mutation: ColdMutation<P> { self.replay.step_reprojection(&mut self.log, &mut self.owners, &mut self.wire, &mut self.cursor, &mut self.live, deadline_us).await }

    /// 🪞️ Delegates to [`StoreReplay::defer_local_step`].
    #[inline] pub async fn defer_local_step(&mut self, step: PendingLocalStep, clock: HybridLogicalTimestamp) -> Result<Option<PendingLocalStep>, VcsError> where P: Clone, Mutation: ColdMutation<P> { self.replay.defer_local_step(&mut self.log, &mut self.owners, &mut self.wire, &mut self.cursor, &mut self.live, step, clock).await }

    /// 🪞️ Delegates to [`StoreLog::author_head_alternative`].
    #[inline] pub(super) async fn author_head_alternative(&self, name: String, clock: &mut HybridLogicalTimestamp, envelopes: &mut Vec<semio_framework_os_spr::MutationEnvelope>, identity: &mut EntityIdentityAuthority<'_>) -> Result<String, VcsError> { self.log.author_head_alternative(name, clock, envelopes, identity).await }

    /// 🪞️ Delegates to [`StoreLog::prospective_fold`].
    #[inline] pub(super) fn prospective_fold(&self, candidates: &[semio_framework_os_spr::MutationEnvelope]) -> Result<semio_framework_os_spr::HistoryFold, VcsError> { self.log.prospective_fold(candidates) }

    /// 🪞️ Delegates to [`StoreLive::dry_run`].
    #[inline] pub(super) fn dry_run(&self, fold: &semio_framework_os_spr::HistoryFold) -> Result<EditReplayResult<P, Mutation>, VcsError> { self.live.dry_run(&self.log, &self.cursor, fold) }

    /// 🪞️ Delegates to [`StoreLive::supersede_inputs`].
    #[inline] pub fn supersede_inputs(&self, inputs: Vec<SupersedeInput<Mutation>>) -> Result<(Vec<protocol::SupersededInput>, Vec<String>), VcsError> { self.live.supersede_inputs(&self.log, &self.wire, &self.cursor, inputs) }

    /// 🪞️ Delegates to [`StoreLive::draft_inputs`].
    #[inline] pub(super) fn draft_inputs(&self, drafts: &protocol::HistoryInputDrafts) -> Result<(Vec<protocol::SupersededInput>, Vec<String>), VcsError> { self.live.draft_inputs(&self.log, &self.wire, &self.cursor, drafts) }

    /// 🪞️ Delegates to [`StoreReplay::record_replay_report`].
    #[inline] pub fn record_replay_report(&mut self, report: &protocol::ReplayReport) { self.replay.record_replay_report(report) }

    /// 🪞️ Delegates to [`StoreReplay::absorb_merge_report`].
    #[inline] pub(super) fn absorb_merge_report(&mut self, report: &semio_framework_os_spr::MergeReport) { self.replay.absorb_merge_report(report) }

    /// 🪞️ Delegates to [`StoreLog::edit_lane`].
    #[inline] pub fn edit_lane(&self, edit_id: &str) -> HistoryLane { self.log.edit_lane(edit_id) }

    /// 🪞️ Delegates to [`StoreLog::redo_position`].
    #[inline] pub(super) fn redo_position(&self, lane: impl Fn(HistoryLane) -> bool) -> Option<usize> { self.log.redo_position(lane) }

    /// 🪞️ Delegates to [`StoreReplay::switch_local_alternative`].
    #[inline] pub(super) async fn switch_local_alternative(&mut self, alternative_id: String) -> Result<(), VcsError> where P: Clone, Mutation: ColdMutation<P> { self.replay.switch_local_alternative(&mut self.log, &mut self.owners, &mut self.wire, &mut self.cursor, &mut self.live, alternative_id).await }

    /// 🪞️ Delegates to [`StoreReplay::checkout_local_checkpoint`].
    #[inline] pub(super) async fn checkout_local_checkpoint(&mut self, checkpoint_id: String) -> Result<(), VcsError> where P: Clone, Mutation: ColdMutation<P> { self.replay.checkout_local_checkpoint(&mut self.log, &mut self.owners, &mut self.wire, &mut self.cursor, &mut self.live, checkpoint_id).await }

    /// 🪞️ Delegates to [`StoreLog::viewer_head_now`].
    #[inline] pub fn viewer_head_now(&self) -> (Option<String>, Option<String>) { self.log.viewer_head_now() }

    /// 🪞️ Delegates to [`StoreLog::pending_checkpoint_transition`].
    #[inline] pub async fn pending_checkpoint_transition(&self, message: Option<String>, authors: Vec<Author>, identity: &mut EntityIdentityAuthority<'_>) -> Result<(semio_framework_os_spr::HistoryTransition, Vec<MutationId>, String), VcsError> { self.log.pending_checkpoint_transition(message, authors, identity).await }

    /// 🪞️ Delegates to [`StoreReplay::apply_command`].
    #[inline] pub(super) async fn apply_command(&mut self, mutations: Vec<Mutation>, lane: HistoryLane, transaction: Option<protocol::TransactionRef>, identity: &mut EntityIdentityAuthority<'_>) -> Result<(), VcsError> where P: Clone, Mutation: semio_framework_os_spr::ColdMutation<P> { self.replay.apply_command(&mut self.log, &mut self.owners, &mut self.cursor, &mut self.live, mutations, lane, transaction, identity).await }

    /// 🪞️ Delegates to [`StoreOwners::replace_edit_messages`].
    #[inline] pub fn replace_edit_messages(&mut self, edit_id: &str, messages: Vec<semio_framework_os_spr::MutationMessage>) -> Result<(), VcsError> { self.owners.replace_edit_messages(&mut self.log, edit_id, messages) }

    /// 🪞️ Delegates to [`StoreOwners::authored_operation_bytes`].
    #[inline] pub fn authored_operation_bytes(&mut self,mutations:&[Mutation],identity:&mut EntityIdentityAuthority<'_>)->Result<Vec<Vec<u8>>,VcsError> { self.owners.authored_operation_bytes(&mut self.log, mutations, identity) }

    /// 🪞️ Delegates to [`StoreOwners::authored_edit_operation_bytes`].
    #[inline] pub fn authored_edit_operation_bytes(&mut self,identity:&mut EntityIdentityAuthority<'_>)->Result<Option<(Vec<Vec<u8>>,Vec<Vec<u8>>)>,VcsError> { self.owners.authored_edit_operation_bytes(&mut self.log, identity) }

    /// 🪞️ Delegates to [`StoreLog::snapshot_pack`].
    #[inline] pub async fn snapshot_pack(&self) -> Result<ArtifactPackFiles, VcsError> where P: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot, { self.log.snapshot_pack().await }

    /// 🪞️ Delegates to [`StoreLive::snapshot_json`].
    #[inline] pub fn snapshot_json(&self) -> Result<String, VcsError> where P: Clone { self.live.snapshot_json(&self.wire) }

    /// 🪞️ Delegates to [`StoreLog::envelope_json`].
    #[inline] pub fn envelope_json(&self) -> Result<String, VcsError> where P: ToValue, Mutation: ToValue, { self.log.envelope_json() }

    /// 🪞️ Delegates to [`StoreLog::event_log`].
    #[inline] pub fn event_log(&self) -> Result<Vec<semio_framework_os_spr::MutationEnvelope>, VcsError> { self.log.event_log() }

    /// 🪞️ Delegates to [`StoreLog::event_log_payload`].
    #[inline] pub fn event_log_payload(&self) -> Result<Vec<u8>, VcsError> { self.log.event_log_payload() }

    /// 🪞️ Delegates to [`StoreLog::announce_tail_edit_payload`].
    #[inline] pub fn announce_tail_edit_payload(&mut self) -> Result<Vec<u8>, VcsError> { self.log.announce_tail_edit_payload() }

    /// 🪞️ Delegates to [`StoreWire::send_member_mutations`].
    #[inline] pub async fn send_member_mutations(&mut self, owner: &str, slot: &str, child_id: &str, envelopes: Vec<u8>) -> Result<(), VcsError> { self.wire.send_member_mutations(&mut self.owners, owner, slot, child_id, envelopes).await }

    /// 🪞️ Delegates to [`StoreWire::take_member_inbound`].
    #[inline] pub fn take_member_inbound(&mut self) -> Vec<(String, String, String, Vec<u8>)> { self.wire.take_member_inbound() }

    /// 🪞️ Delegates to [`StoreWire::announce_history`].
    #[inline] pub(super) async fn announce_history(&mut self) -> Result<(), VcsError> { self.wire.announce_history(&mut self.log, &mut self.owners).await }

    /// 🪞️ Delegates to [`StoreReplay::detach_backbone`].
    #[inline] pub fn detach_backbone(&mut self) -> Result<Option<Backbones>, VcsError> { self.replay.detach_backbone(&mut self.log, &mut self.owners, &mut self.wire, &mut self.cursor, &mut self.live) }

    /// 🪞️ Delegates to [`StoreLog::backbone_ref`].
    #[inline] pub fn backbone_ref(&self) -> Option<&ArtifactBackboneRef> { self.log.backbone_ref() }

    /// 🪞️ Delegates to [`StoreOwners::refuse_with_candidate_dag`].
    #[inline] fn refuse_with_candidate_dag<T>(&mut self, candidate_dag: semio_framework_os_spr::MutationDag, error: VcsError) -> Result<T, VcsError> { self.owners.refuse_with_candidate_dag::<T>(candidate_dag, error) }

    /// 🪞️ Delegates to [`StoreLog::assert_equivalent_remote_envelope`].
    #[inline] fn assert_equivalent_remote_envelope(&self, existing: &Edit<Mutation>, incoming: &semio_framework_os_spr::MutationEnvelope) -> Result<(), VcsError> { self.log.assert_equivalent_remote_envelope(existing, incoming) }

    /// 🪞️ Delegates to [`StoreLog::assert_equivalent_remote_mutation`].
    #[inline] fn assert_equivalent_remote_mutation(&self, incoming: &semio_framework_os_spr::MutationEnvelope) -> Result<(), VcsError> { self.log.assert_equivalent_remote_mutation(incoming) }

    /// 🪞️ Delegates to [`StoreReplay::resolve_conflict`].
    #[inline] pub async fn resolve_conflict(&mut self, conflict_id: &str, resolution: semio_framework_os_spr::ConflictResolution) -> Result<semio_framework_os_spr::MergeReport, VcsError> where P: Clone + Sync, Mutation: ColdMutation<P>, { self.replay.resolve_conflict(&mut self.log, &mut self.owners, &mut self.wire, &mut self.cursor, &mut self.live, conflict_id, resolution).await }

    /// 🪞️ Delegates to [`StoreLog::verify_genesis`].
    #[inline] pub(super) fn verify_genesis(&self, pack: &[u8]) -> Result<(), VcsError> { self.log.verify_genesis(pack) }

    /// 🪞️ Delegates to [`StoreReplay::flush_outbound`].
    #[inline] pub async fn flush_outbound(&mut self) -> Result<(), VcsError> { self.replay.flush_outbound(&mut self.owners, &mut self.wire).await }

    /// 🪞️ Delegates to [`StoreLog::edit_is_local`].
    #[inline] pub fn edit_is_local(&self, edit_id: &str) -> bool { self.log.edit_is_local(edit_id) }

    /// 🪞️ Delegates to [`StoreCursor::restamp_applied_tail`].
    #[inline] pub fn restamp_applied_tail(&mut self) { self.cursor.restamp_applied_tail(&mut self.log) }

    /// 🪞️ Delegates to [`StoreCursor::push_applied_edit_id`].
    #[inline] pub fn push_applied_edit_id(&mut self, edit_id: String) { self.cursor.push_applied_edit_id(&mut self.log, edit_id) }

    /// 🪞️ Delegates to [`StoreLive::pop_applied_edit_id`].
    #[inline] pub(super) fn pop_applied_edit_id(&mut self) -> Option<String> { self.live.pop_applied_edit_id(&mut self.log, &mut self.cursor) }

    /// 🪞️ Delegates to [`StoreCursor::push_redo_edit_id`].
    #[inline] pub(super) fn push_redo_edit_id(&mut self, edit_id: String) { self.cursor.push_redo_edit_id(&mut self.log, edit_id) }

    /// 🪞️ Delegates to [`StoreCursor::pop_redo_edit_id`].
    #[inline] pub(super) fn pop_redo_edit_id(&mut self) -> Option<String> { self.cursor.pop_redo_edit_id(&mut self.log) }

    /// 🪞️ Delegates to [`StoreReplay::bump`].
    #[inline] pub fn bump(&mut self) -> Result<(), VcsError> { self.replay.bump(&mut self.log, &mut self.owners, &mut self.cursor, &mut self.live) }

    /// 🪞️ Delegates to [`StoreReplay::assert_mirrors_are_live`].
    #[doc(hidden)] #[inline] pub fn assert_mirrors_are_live(&self) { self.replay.assert_mirrors_are_live(&self.log, &self.cursor, &self.live) }
}

impl<P, Mutation> ArtifactStore<P, Mutation>
where
    P: semio_framework_os_spr::StoreState,
    Mutation: semio_framework_os_spr::StoreMutationFor<P>,
{
    /// 🪞️ Delegates to [`StoreReplay::local_step_pending`].
    #[inline] pub fn local_step_pending(&self) -> bool { self.replay.local_step_pending() }
}
