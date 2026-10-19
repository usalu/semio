//! 🚦️ Cursor admission, dispatch and apply batch: the cursor close admission, the structural owner checks, the dispatch of commands, the apply batch, the transition installs and the inner dispatch. See [`super`].

use super::*;

//#region 🔖️EphemeralLanes
//#region 🔖️Backbone
//#region 🔖️Backbone
//#region 🔖️Authority
//#region 🔖️EphemeralLanes
//#region 🔖️Text
//#region 🔖️Pack
//#region 🔖️CodecRegistry
//#region 🔖️TextFormat
//#region 🔖️History
//#region 🔖️ArtifactStore
//#region 🔖️ArtifactStore
impl<P, Mutation> ArtifactStore<P, Mutation>
where
    P: Clone + ToValue + FromValue + Send + Sync + 'static,
    Mutation: Clone + ToValue + FromValue + super::Mutation<P> + OpBinary + OpText + Send + 'static,
{
    pub fn close_cursor_admit(&mut self, phase: ArtifactStoreCursorDisposerPhase, grant: RetainedCloneGrant) -> Result<(Option<Box<dyn ErasedSnapshotRetirement>>, RetainedCloneProgress), ValueError> {
        let structural = RetainedCloneProgress { copied_items: 1, ..Default::default() };
        match phase {
            ArtifactStoreCursorDisposerPhase::RuntimeOwners(0) => {
                let original = std::mem::replace(&mut *self.log.applied_edit_ids, semio_framework_os_vcs::HistoryPageStack::empty());
                match admit_artifact_retirement(original, grant, ArtifactStoreStringVectorRetirement::new) {
                    Ok((owner, receipt)) => Ok((Some(owner), receipt)),
                    Err((error, original)) => { *self.log.applied_edit_ids = original; Err(error) }
                }
            }
            ArtifactStoreCursorDisposerPhase::RuntimeOwners(1) => {
                let original = std::mem::replace(&mut *self.log.redo_edit_ids, semio_framework_os_vcs::HistoryPageStack::empty());
                match admit_artifact_retirement(original, grant, ArtifactStoreStringVectorRetirement::new) {
                    Ok((owner, receipt)) => Ok((Some(owner), receipt)),
                    Err((error, original)) => { *self.log.redo_edit_ids = original; Err(error) }
                }
            }
            ArtifactStoreCursorDisposerPhase::RuntimeOwners(2) => {
                let original = std::mem::replace(&mut *self.cursor.revision_accumulator, CursorRevisionAccumulator { identity_digest: [0; 32], applied: semio_framework_os_vcs::HistoryPageStack::empty(), redo: semio_framework_os_vcs::HistoryPageStack::empty(), applied_tail_chains: None, mutation_positions: protocol::HistoryFoldIndex::new(), indexed_edits: protocol::HistoryFoldIndex::new(), unit_flags: protocol::HistoryFoldIndex::new() });
                match admit_artifact_retirement(original, grant, ArtifactStoreRevisionAccumulatorRetirement::new) {
                    Ok((owner, receipt)) => Ok((Some(owner), receipt)),
                    Err((error, original)) => { *self.cursor.revision_accumulator = original; Err(error) }
                }
            }
            ArtifactStoreCursorDisposerPhase::RuntimeOwners(6) => {
                let original = std::mem::replace(&mut *self.live.supersessions, EffectiveSupersessions::default());
                match admit_artifact_retirement(original, grant, ArtifactStoreSupersessionRetirement::new) {
                    Ok((owner, receipt)) => Ok((Some(owner), receipt)),
                    Err((error, original)) => { *self.live.supersessions = original; Err(error) }
                }
            }
            ArtifactStoreCursorDisposerPhase::RuntimeOwners(7) => {
                let original = std::mem::replace(&mut *self.replay.pending_report, PendingCommandReport::default());
                match admit_artifact_retirement(original, grant, ArtifactStorePendingReportRetirement::new) {
                    Ok((owner, receipt)) => Ok((Some(owner), receipt)),
                    Err((error, original)) => { *self.replay.pending_report = original; Err(error) }
                }
            }
            ArtifactStoreCursorDisposerPhase::RuntimeOwners(3) => {
                let Some(original) = self.log.current_checkpoint_id.take() else { return Ok((None, structural)); };
                match admit_artifact_retirement(original, grant, ArtifactStoreStringRetirement::new) {
                    Ok((owner, receipt)) => Ok((Some(owner), receipt)),
                    Err((error, original)) => { *self.log.current_checkpoint_id = Some(original); Err(error) }
                }
            }
            ArtifactStoreCursorDisposerPhase::RuntimeOwners(4) => {
                let original = std::mem::take(&mut self.log.local_actor_id.0);
                match admit_artifact_retirement(original, grant, |original|semio_framework_value::retirement::controlled::ControlledRetirement::new(original).unwrap_or_else(|_|unreachable!("shared UTF8 declares controlled retirement"))) {
                    Ok((owner, receipt)) => Ok((Some(owner), receipt)),
                    Err((error, original)) => { self.log.local_actor_id.0 = original; Err(error) }
                }
            }
            ArtifactStoreCursorDisposerPhase::RuntimeOwners(5) => {
                let Some((edit_id, _)) = self.live.tail_undo_cache.as_mut() else { return Ok((None, structural)); };
                let original = std::mem::take(edit_id);
                match admit_artifact_retirement(original, grant, ArtifactStoreStringRetirement::new) {
                    Ok((owner, receipt)) => Ok((Some(owner), receipt)),
                    Err((error, original)) => { *edit_id = original; Err(error) }
                }
            }
            ArtifactStoreCursorDisposerPhase::PrefixSnapshots => {
                if self.live.prefix_ring.is_empty() { return Ok((None, structural)); }
                let factory = self.owners.snapshot_retirement_factory.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "prefix close requires installed factory"))?;
                let entry = self.live.prefix_ring.pop().expect("observed original prefix owner");
                if Arc::strong_count(&entry.snapshot) > 1 { drop(entry); return Ok((None, structural)); }
                match factory.retire(entry.snapshot, grant) {
                    Ok((owner, receipt)) => Ok((Some(owner), receipt)),
                    Err((error, snapshot)) => { self.live.prefix_ring.push(PrefixSnapshot { snapshot, ..entry }); Err(error) }
                }
            }
            ArtifactStoreCursorDisposerPhase::TailSnapshot => {
                if self.live.tail_undo_cache.as_ref().is_some_and(|(id, _)| id.capacity() != 0) { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "tail snapshot still retains original edit-id backing")); }
                if self.live.tail_undo_cache.is_none() { return Ok((None, structural)); }
                let factory = self.owners.snapshot_retirement_factory.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "tail close requires installed factory"))?;
                let (id, snapshot) = self.live.tail_undo_cache.take().expect("observed original tail owner");
                if Arc::strong_count(&snapshot) > 1 { drop((id, snapshot)); return Ok((None, structural)); }
                match factory.retire(snapshot, grant) {
                    Ok((owner, receipt)) => Ok((Some(owner), receipt)),
                    Err((error, snapshot)) => { *self.live.tail_undo_cache = Some((id, snapshot)); Err(error) }
                }
            }
            ArtifactStoreCursorDisposerPhase::CurrentSnapshot => {
                if self.live.current_detached { return Ok((None, structural)); }
                if self.live.tail_undo_cache.is_some() || !self.snapshot_read_leases_terminal_is_empty() { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "current snapshot still holds tail or read owners")); }
                let genesis = std::ptr::eq(self.live.current.as_ref(), self.log.envelope.vcs.genesis.facts().snapshot());
                let result = if genesis {
                    let factory = self.owners.initial_snapshot_retirement_factory.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "genesis alias requires exact initial factory"))?.clone();
                    let original = unsafe { std::mem::ManuallyDrop::take(&mut self.live.current) };
                    ReturnedSnapshotReadRetirement::admit(original, factory, grant).map_err(|(error, original, alias)| { drop(alias); (error, original) })
                } else {
                    let factory = self.owners.snapshot_retirement_factory.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "current snapshot requires exact factory"))?;
                    let original = unsafe { std::mem::ManuallyDrop::take(&mut self.live.current) };
                    factory.retire(original, grant)
                };
                match result {
                    Ok((owner, receipt)) => { self.live.current_detached = true; Ok((Some(owner), receipt)) }
                    Err((error, original)) => { self.live.current = std::mem::ManuallyDrop::new(original); Err(error) }
                }
            }
            ArtifactStoreCursorDisposerPhase::Backbone => self.close_take_backbone_retirement(grant),
            ArtifactStoreCursorDisposerPhase::Causal => {
                let original = std::mem::replace(&mut *self.log.dag, semio_framework_os_spr::MutationDag::empty());
                match admit_artifact_retirement(original, grant, ArtifactStoreMutationDagRetirement::new) {
                    Ok((owner, receipt)) => Ok((Some(owner), receipt)),
                    Err((error, original)) => { *self.log.dag = original; Err(error) }
                }
            }
            ArtifactStoreCursorDisposerPhase::Envelope => {
                if self.log.envelope_detached { return Ok((None, structural)); }
                if !self.live.current_detached { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "whole envelope requires detached current owner")); }
                let snapshots = self.owners.initial_snapshot_retirement_factory.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "whole envelope requires initial factory"))?.clone();
                let mutations = self.owners.mutation_retirement_factory.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "whole envelope requires mutation factory"))?.clone();
                let original = unsafe { std::mem::ManuallyDrop::take(&mut self.log.envelope) };
                match retire_document_envelope(original, snapshots, mutations, grant) {
                    Ok((owner, receipt)) => { self.log.envelope_detached = true; Ok((Some(owner), receipt)) }
                    Err((error, original, snapshots, mutations)) => { self.log.envelope = std::mem::ManuallyDrop::new(original); drop((snapshots, mutations)); Err(error) }
                }
            }
            _ => Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "installed close addressed a non-admission phase")),
        }
    }

    pub(super) fn close_structural_owners_terminal_is_empty(&self) -> bool {
        let envelope_shell_is_empty = self.log.envelope_detached
            || (self.log.envelope.conflicts.is_empty()
                && self.log.envelope.transitions.is_empty()
                && self.log.envelope.vcs.edits.is_empty()
                && self.log.envelope.vcs.changes.is_empty()
                && self.log.envelope.vcs.checkpoints.is_empty()
                && self.log.envelope.vcs.alternatives.is_empty()
                && self.log.envelope.edit_messages.is_empty()
                && self.log.envelope.lanes.is_empty());
        let runtime_shell_is_empty = self.wire.backbone.is_none()
            && self.wire.member_inbox.capacity() == 0
            && self.replay.pending_report.is_idle()
            && self.live.supersessions.is_empty()
            && self.live.prefix_ring.is_empty();
        envelope_shell_is_empty && runtime_shell_is_empty && self.wire.durable_group_root.is_none() && self.log.dag.terminal_is_empty() && self.owners.displaced_retirements.terminal_is_empty()
    }

    pub fn owned_roots_terminal_is_empty(&self) -> bool {
        self.live.current_detached
            && self.log.envelope_detached
            && self.wire.durable_group_root.is_none()
            && self.live.tail_undo_cache.is_none()
            && self.live.prefix_ring.is_empty()
            && self.snapshot_read_leases_terminal_is_empty()
            && self.close_structural_owners_terminal_is_empty()
    }

    pub async fn fold_history<I, S>(envelope: &ArtifactEnvelope<P, Mutation>, applied_edit_ids: I, supersessions: &EffectiveSupersessions) -> Result<P, VcsError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        fold_history(envelope, applied_edit_ids, supersessions).await
    }

    /// 🪞️ Delegates to [`semio_framework_os_store_engine_bundle::edits_for_ids`]: the strict id-to-edit resolution of the merge.
    #[inline]
    pub fn edits_for_ids(ids: &[String], edits: &HashMap<String, Edit<Mutation>>) -> Result<Vec<Edit<Mutation>>, VcsError> {
        semio_framework_os_store_engine_bundle::edits_for_ids(ids, edits)
    }

    /// 🌀️ The unawaited future borrows `command`, which is then moved into `dispatch_inner`
    /// below — awaited immediately instead of deferred to avoid a move-while-borrowed (E0505).
    ///
    /// Undo/redo shrink `applied_edit_ids`; Apply/AppendTransaction append past `before`; a merge that
    /// rebased mid-history overrides this tail-diff guess via `PendingCommandReport::declare_edit_ids`.
    pub async fn dispatch(&mut self, command: ArtifactCommand<Mutation>, identity: &mut EntityIdentityAuthority<'_>) -> Result<CommandReceipt, VcsError>
    where
        P:  Clone + Sync,
     Mutation: ColdMutation<P> ,
    {
        self.ensure_durable_group_idle()?;
        self.pump().await?;
        let projection_cause = command.projection_cause().await;
        let before = self.log.applied_edit_ids.len();
        self.replace_pending_report_retained(PendingCommandReport::default())?;
        self.dispatch_inner(command, identity).await?;
        self.cursor.last_projection_cause = projection_cause;
        self.flush_outbound().await?;
        let (edit_ids, messages, worst) = self.replay.pending_report.take_receipt(|| if self.log.applied_edit_ids.len() >= before { self.log.applied_edit_ids.cloned_from(before) } else { Vec::new() });
        Ok(CommandReceipt { edit_ids, generation: self.generation(), messages, worst })
    }

    pub async fn dispatch_apply_exact(&mut self, mutations: Vec<Mutation>, identity: &mut EntityIdentityAuthority<'_>) -> Result<CommandReceipt, VcsError>
    where
        P:  Clone + Sync,
     Mutation: ColdMutation<P> ,
    {
        self.ensure_durable_group_idle()?;
        self.pump().await?;
        let before = self.log.applied_edit_ids.len();
        self.replace_pending_report_retained(PendingCommandReport::default())?;
        self.apply_command(mutations, HistoryLane::Document, None, identity).await?;
        self.cursor.last_projection_cause = Some(ArtifactProjectionCause::Apply);
        self.flush_outbound().await?;
        let (edit_ids, messages, worst) = self.replay.pending_report.take_receipt(|| self.log.applied_edit_ids.cloned_from(before));
        Ok(CommandReceipt { edit_ids, generation: self.generation(), messages, worst })
    }

}

impl<P, Mutation> ArtifactStore<P, Mutation>
where
    P: Clone + ToValue + FromValue + Send + Sync + 'static,
    Mutation: Clone + ToValue + FromValue + super::Mutation<P> + OpBinary + OpText + Send + 'static,
{
    //#region 📬️OneItemPublication
    /// 🪞️ Delegates to [`semio_framework_os_store_engine_bundle::advance_apply_batch`]: advances at most one preparation, fold, cursor, preflight or atomic move-publication unit of the batched publication.
    #[inline]
    pub fn advance_apply_batch(&mut self, publication: &mut ArtifactStoreBatchPublication<P, Mutation>,  body:usize, grant: ArtifactStoreOneItemGrant, receiving:&mut semio_framework_value::NativeDecodeControl<'_>) -> Result<ArtifactStoreOneItemAdvance, VcsError>
    where
        P: Sync,
    {
        semio_framework_os_store_engine_bundle::advance_apply_batch(&mut self.parts(), publication, body, grant, receiving)
    }


    //#endregion 📬️OneItemPublication
    pub async fn apply_one(&mut self, expected_generation: u64, mutation: Mutation, lane: HistoryLane, identity: &mut EntityIdentityAuthority<'_>) -> Result<(CommandReceipt, LaneItemReceipt), VcsError>
    where
        P: Sync,
         Mutation: ColdMutation<P>,
{
        self.ensure_durable_group_idle()?;
        self.pump().await?;
        if self.cursor.generation != expected_generation {
            return Err(VcsError::ValidationFailed("one-item publication generation is stale".to_string()));
        }
        let generation_before = self.cursor.generation;
        self.replace_pending_report_retained(PendingCommandReport::default())?;
        let before = self.log.applied_edit_ids.len();
        self.apply_command(vec![mutation], lane, None, identity).await?;
        self.cursor.last_projection_cause = Some(ArtifactProjectionCause::Apply);
        self.flush_outbound().await?;
        let (edit_ids, messages, worst) = self.replay.pending_report.take_receipt(|| self.log.applied_edit_ids.cloned_from(before));
        let receipt = CommandReceipt { edit_ids, generation: self.cursor.generation, messages, worst };
        Ok((receipt, LaneItemReceipt { generation_before, generation_after: self.cursor.generation }))
    }

    //#region 🔖️EventSourcedHistory
    /// 🖋️ Authors one local history transition: stamps it on this replica's clock, records it
    /// in the event log, and materializes the projection through the same fold remote transitions
    /// take. The caller's dispatch announces it outbound with every other new event.
    pub(super) async fn commit_transition(&mut self, transition: semio_framework_os_spr::HistoryTransition, dependencies: Vec<MutationId>) -> Result<(), VcsError> {
        let mut timestamp = self.log.clock;
        timestamp.tick(now_ms());
        let envelope = self.transition_envelope(&transition, dependencies, Vec::new(), timestamp);
        self.install_transitions(vec![envelope], timestamp, None).await
    }

    /// 🏗️ Records locally authored transition envelopes and materializes the projection once — adopting `replayed` when it
    /// is the Report replay of exactly the resulting log, replaying otherwise (under [`Self::defer_local_replays`] a replay
    /// longer than one budget waits for later turns). All of them leave in this dispatch's outbound batch, or none is
    /// recorded.
    pub(super) async fn install_transitions(&mut self, envelopes: Vec<semio_framework_os_spr::MutationEnvelope>, clock: HybridLogicalTimestamp, replayed: Option<EditReplayResult<P, Mutation>>) -> Result<(), VcsError> where P: Clone, Mutation: ColdMutation<P> {
        if let Err(error) = self.admit_history_shape(&envelopes) {
            if let Some(replayed) = replayed {
                let _ = self.retire_replay_result(replayed);
            }
            return Err(error);
        }
        if let (None, [envelope]) = (&replayed, envelopes.as_slice()) {
            if let Some(step) = self.tail_step(envelope) {
                let envelope = envelopes.into_iter().next().expect("one tail-step envelope");
                return self.adopt_tail_step(step, envelope, clock);
            }
        }
        let envelopes = match replayed {
            Some(_) => envelopes,
            None => match self.defer_local_step(PendingLocalStep { envelopes, head: self.viewer_head_now(), finalizing: false }, clock).await? {
                Some(step) => step.envelopes,
                None => return Ok(()),
            },
        };
        let identities: Vec<MutationId> = envelopes.iter().map(|envelope| envelope.mutation_id.clone()).collect();
        for envelope in &envelopes {
            self.insert_transition(envelope.clone());
        }
        let projected = match replayed {
            Some(result) => self.reproject_replayed(result),
            None => self.reproject().await,
        };
        if let Err(error) = projected {
            self.log.envelope.transitions.retain(|known| !identities.contains(&known.mutation_id));
            return Err(error);
        }
        for identity in identities {
            self.log.dag.seed_applied(identity).map_err(|error| VcsError::ValidationFailed(error.to_string()))?;
        }
        self.log.clock = clock;
        self.replay.pending_report.author_outbound(envelopes);
        self.bump()
    }

    /// ⚡️ Adopts a tail step ([`Self::tail_step`]) without folding the log: projects through the tail snapshot or the nearest
    /// retained prefix folded forward (undo), or the edit's effective forwards folded onto the head (redo), records `envelope`,
    /// moves the edit between the stacks and keeps the prefixes it folded through. The edit's inverse and messages stand — its
    /// base is the projection it was last replayed on — and the announcement leaves in this dispatch's outbound batch.
    fn adopt_tail_step(&mut self, step: TailStep, envelope: semio_framework_os_spr::MutationEnvelope, clock: HybridLogicalTimestamp) -> Result<(), VcsError> where P: Clone, Mutation: ColdMutation<P> {
        let key = envelope.timestamp.cmp_key();
        let (projection, tail, recorded) = match step {
            TailStep::Undo => {
                let (base, recorded) = self.live_prefix_state(self.log.applied_edit_ids.len() - 1)?;
                (base, None, recorded)
            }
            TailStep::Redo => {
                let edit_id = self.log.redo_edit_ids.last().expect("a redo tail step names the redo top").clone();
                let edit = self.log.envelope.vcs.edits.find_near(|edit| edit.id == edit_id).ok_or_else(|| VcsError::UnknownEdit(edit_id.clone()))?;
                let pre = Arc::clone(&*self.live.current);
                let mut next = Arc::clone(&pre);
                fold_effective_edit(&mut next, edit, &self.log.envelope.schema, &self.live.supersessions, usize::MAX);
                (next, Some((edit_id, pre)), Vec::new())
            }
        };
        if let Err(error) = self.owners.displaced_retirements.reserve(REPROJECTION_OWNER_SLOTS + 2 * recorded.len()) {
            retire_shared_projection::<P, Mutation>(projection);
            if let Some((_, snapshot)) = tail {
                retire_shared_projection::<P, Mutation>(snapshot);
            }
            for (_, _, snapshot) in recorded {
                retire_shared_projection::<P, Mutation>(snapshot);
            }
            return Err(error);
        }
        match step {
            TailStep::Undo => {
                self.replace_current_retained(projection)?;
                self.replace_tail_undo_cache_retained(tail)?;
                let edit_id = self.pop_applied_edit_id().expect("an undo tail step pops the applied tail");
                self.push_redo_edit_id(edit_id);
            }
            TailStep::Redo => {
                self.replace_tail_undo_cache_retained(tail)?;
                self.replace_current_retained(projection)?;
                let edit_id = self.pop_redo_edit_id().expect("a redo tail step pops the redo top");
                self.push_applied_edit_id(edit_id);
            }
        }
        self.admit_prefix_snapshots(recorded)?;
        self.insert_transition(envelope.clone());
        self.live.fold_frontier = Some(key);
        self.log.dag.seed_applied(envelope.mutation_id.clone()).map_err(|error| VcsError::ValidationFailed(error.to_string()))?;
        self.log.clock = clock;
        self.replay.pending_report.author_outbound(std::iter::once(envelope));
        self.bump()?;
        self.live.tail_step_generation = Some(self.cursor.generation);
        Ok(())
    }

    //#endregion 🔖️EventSourcedHistory
    /// 🛤️ Same edit-recording path as `Apply`, tagging the fresh edit into `lane` instead of
    /// the implicit `Document` default — see `HistoryLane`'s doc.
    pub(super) async fn dispatch_inner(&mut self, command: ArtifactCommand<Mutation>, identity: &mut EntityIdentityAuthority<'_>) -> Result<(), VcsError>
    where
        P:  Clone + Sync,
     Mutation: ColdMutation<P> ,
    {
        let shape = self.log.envelope.history_shape;
        if let Some(kind) = command.history_transition_kinds().iter().copied().find(|kind| !shape.admits(*kind)) {
            if let ArtifactCommand::Supersede { inputs, .. } | ArtifactCommand::CreateAlternativeWithSupersede { inputs, .. } = command {
                retire_supersede_inputs::<P, Mutation>(inputs);
            }
            return Err(VcsError::HistoryShape { shape, kind });
        }
        if let Some(open) = self.log.envelope.open_transaction.as_ref() {
            let admitted = match &command {
                ArtifactCommand::AppendTransaction { transaction, .. } => transaction.id == open.transaction.id,
                ArtifactCommand::CommitTransaction { .. } | ArtifactCommand::AbortTransaction { .. } | ArtifactCommand::IngestRemote { .. } | ArtifactCommand::SetMergePolicy { .. } => true,
                _ => false,
            };
            if !admitted {
                let transaction_id = open.transaction.id.clone();
                retire_command::<P, Mutation>(command);
                return Err(VcsError::TransactionOpen { transaction_id });
            }
        }
        if command.moves_history() && self.local_step_pending() {
            retire_command::<P, Mutation>(command);
            return Err(VcsError::HistoryReplaying);
        }
        match command {
            ArtifactCommand::Undo => self.undo_with_policy(UndoPolicy::TransformAgainstConcurrent, None, identity).await,
            ArtifactCommand::UndoWithPolicy { policy, semantic_command } => self.undo_with_policy(policy, semantic_command, identity).await,
            ArtifactCommand::Redo => {
                let position = self.redo_position(|lane| lane == HistoryLane::Document).ok_or(VcsError::NothingToRedo)?;
                self.redo_lane_position(position).await
            }
            ArtifactCommand::UndoInLane { lane } => {
                let position = self.log.applied_edit_ids.iter().rposition(|id| self.edit_is_local(id) && self.edit_lane(id) == lane).ok_or(VcsError::NothingToUndo)?;
                self.undo_lane_position(position).await
            }
            ArtifactCommand::RedoInLane { lane } => {
                let position = self.redo_position(|edit_lane| edit_lane == lane).ok_or(VcsError::NothingToRedo)?;
                self.redo_lane_position(position).await
            }
            ArtifactCommand::CommitCheckpoint { message, authors } => self.commit_pending_checkpoint(message, authors, identity).await,
            ArtifactCommand::CreateAlternative { name } => {
                if self.log.envelope.vcs.checkpoints.is_empty() {
                    if self.log.applied_edit_ids.is_empty() {
                        return Err(VcsError::NoCheckpoint);
                    }
                    self.commit_pending_checkpoint(None, Vec::new(), identity).await?;
                }
                let checkpoint_id = (*self.log.current_checkpoint_id).clone().or_else(|| self.log.envelope.vcs.checkpoints.last().map(|checkpoint| checkpoint.id.clone())).ok_or(VcsError::NoCheckpoint)?;
                let alternative_id = identity.encode(|control|mint_alternative_id(&name, std::slice::from_ref(&checkpoint_id), control))?;
                if self.log.envelope.vcs.alternatives.iter().any(|alternative| alternative.id == alternative_id) {
                    return Err(VcsError::ValidationFailed(format!("alternative {alternative_id} already exists")));
                }
                let dependencies = self.checkpoint_origin(&checkpoint_id).into_iter().collect();
                self.commit_transition(semio_framework_os_spr::HistoryTransition::Branch { alternative_id: alternative_id.clone(), name, checkpoint_id }, dependencies).await?;
                self.log.envelope.active_alternative_id = Some(alternative_id);
                self.log.envelope.viewer_checkpoint_id = None;
                self.reproject().await?;
                self.bump()
            }
            ArtifactCommand::SwitchAlternative { alternative_id } => self.switch_local_alternative(alternative_id).await,
            ArtifactCommand::CheckoutCheckpoint { checkpoint_id } => self.checkout_local_checkpoint(checkpoint_id).await,
            ArtifactCommand::Apply { mutations, transaction } => self.apply_command(mutations, HistoryLane::Document, transaction, identity).await,
            ArtifactCommand::ApplyInLane { mutations, lane, transaction } => self.apply_command(mutations, lane, transaction, identity).await,
            ArtifactCommand::IngestRemote { envelope } => {
                let report = self.ingest_remote(envelope).await?;
                self.absorb_merge_report(&report);
                Ok(())
            }
            ArtifactCommand::PruneDrafts => Err(VcsError::ValidationFailed("draft pruning is not implemented by ArtifactStore; no draft history was removed".to_string())),
            ArtifactCommand::SetMergePolicy { policy } => {
                self.log.merge_policy = policy;
                Ok(())
            }
            ArtifactCommand::ResolveConflict { conflict_id, resolution } => {
                let report = self.resolve_conflict(&conflict_id, resolution).await?;
                self.absorb_merge_report(&report);
                Ok(())
            }
            ArtifactCommand::Supersede { scope, inputs } => self.supersede_command(scope, inputs, identity).await,
            ArtifactCommand::CreateAlternativeWithSupersede { name, inputs } => self.create_alternative_with_supersede(name, inputs, identity).await,
            ArtifactCommand::AppendTransaction { mutations, transaction } => self.append_transaction(mutations, transaction, identity).await,
            ArtifactCommand::CommitTransaction { transaction_id } => self.commit_open_transaction(&transaction_id),
            ArtifactCommand::AbortTransaction { transaction_id } => self.abort_open_transaction(&transaction_id),
        }
    }

}
//#endregion 🔖️ArtifactStore
//#endregion 🔖️ArtifactStore
//#endregion 🔖️History
//#endregion 🔖️TextFormat
//#endregion 🔖️CodecRegistry
//#endregion 🔖️Pack
//#endregion 🔖️Text
//#endregion 🔖️EphemeralLanes
//#endregion 🔖️Authority
//#endregion 🔖️Backbone
//#endregion 🔖️Backbone
//#endregion 🔖️EphemeralLanes
