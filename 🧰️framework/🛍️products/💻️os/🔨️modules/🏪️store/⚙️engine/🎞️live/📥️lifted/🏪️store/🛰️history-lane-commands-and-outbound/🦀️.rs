//! 🛰️ History lane commands and outbound: the local alternatives and checkpoints, the command application, the edit messages, the conflict resolution, the quarantined events and the outbound flushes. See [`super`].

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
impl<P, Mutation> StoreReplay<P, Mutation>
where
    P: Clone + ToValue + FromValue + Send + Sync + 'static,
    Mutation: Clone + ToValue + FromValue + super::Mutation<P> + OpBinary + OpText + Send + 'static,
{
    //#region 🔖️HistoryLane
    /// 👁 Moves this replica onto `alternative_id` at its tip. Registration stays the shared log;
    /// no checkout event is authored, so every other replica keeps its own head.
    pub async fn switch_local_alternative(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_wire: &mut StoreWire<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>, store_live: &mut StoreLive<P, Mutation>, alternative_id: String) -> Result<(), VcsError> {
        let trunk = store_log.trunk_alternative_id();
        if alternative_id != trunk {
            let alternative = store_log.envelope.vcs.alternatives.iter().find(|alternative| alternative.id == alternative_id).ok_or_else(|| VcsError::UnknownAlternative(alternative_id.clone()))?;
            let checkpoint_id = alternative.checkpoint_ids.last().ok_or(VcsError::NoCheckpoint)?.clone();
            if !store_log.envelope.vcs.checkpoints.iter().any(|checkpoint| checkpoint.id == checkpoint_id) {
                return Err(VcsError::NoCheckpoint);
            }
            self.move_head(store_log, store_owners, store_wire, store_cursor, store_live, (Some(alternative_id), None)).await
        } else {
            self.move_head(store_log, store_owners, store_wire, store_cursor, store_live, (None, None)).await
        }
    }

    /// 👁 Looks at `checkpoint_id` on this replica only. The line stays the current one when that
    /// checkpoint is on it; otherwise it is the first registered alternative whose chain contains it.
    pub async fn checkout_local_checkpoint(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_wire: &mut StoreWire<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>, store_live: &mut StoreLive<P, Mutation>, checkpoint_id: String) -> Result<(), VcsError> {
        if !store_log.envelope.vcs.checkpoints.iter().any(|checkpoint| checkpoint.id == checkpoint_id) {
            return Err(VcsError::UnknownChange(checkpoint_id));
        }
        let current = store_log.active_line_id();
        let line = if store_log.envelope.vcs.alternatives.iter().any(|alternative| alternative.id == current && alternative.checkpoint_ids.iter().any(|id| id == &checkpoint_id)) {
            current
        } else {
            store_log.envelope.vcs.alternatives.iter().find(|alternative| alternative.checkpoint_ids.iter().any(|id| id == &checkpoint_id)).map(|alternative| alternative.id.clone()).unwrap_or(current)
        };
        self.move_head(store_log, store_owners, store_wire, store_cursor, store_live, (store_log.branched_alternative(line), Some(checkpoint_id))).await
    }

    /// 🚶️ Moves this replica's head to `(line, checkpoint)` and materializes the projection; a refused move keeps the head it
    /// had. Under [`Self::defer_local_replays`] a move whose replay is longer than one budget waits for later turns.
    async fn move_head(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_wire: &mut StoreWire<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>, store_live: &mut StoreLive<P, Mutation>, head: (Option<String>, Option<String>)) -> Result<(), VcsError> {
        let Some(PendingLocalStep { head, .. }) = self.defer_local_step(store_log, store_owners, store_wire, store_cursor, store_live, PendingLocalStep { envelopes: Vec::new(), head, finalizing: false }, store_log.clock).await? else {
            return Ok(());
        };
        let saved = store_log.viewer_head_now();
        (store_log.envelope.active_alternative_id, store_log.envelope.viewer_checkpoint_id) = head;
        if let Err(error) = store_live.reproject(store_log, store_owners, store_cursor).await {
            (store_log.envelope.active_alternative_id, store_log.envelope.viewer_checkpoint_id) = saved;
            return Err(error);
        }
        self.bump(store_log, store_owners, store_cursor, store_live)
    }

    /// 🛤️ Shared body of `Apply`/`ApplyInLane`: identical edit-recording logic to the
    /// pre-lane `Apply` arm, plus tagging the fresh edit into `envelope.lanes` when `lane` isn't
    /// the default `Document` (kept sparse — see `ArtifactEnvelope.lanes`'s doc).
    ///
    /// ⚡️ `current` is always up to date (maintained by every mutating command below), so this
    /// is an O(1) clone instead of a full replay — see the `current` field doc.
    pub async fn apply_command(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>, store_live: &mut StoreLive<P, Mutation>, mutations: Vec<Mutation>, lane: HistoryLane, transaction: Option<protocol::TransactionRef>, identity: &mut EntityIdentityAuthority<'_>) -> Result<(), VcsError> {
        if mutations.is_empty() {
            return Err(VcsError::EmptyApply);
        }
        if let Some(open) = store_log.envelope.open_transaction.as_ref() {
            let transaction_id = open.transaction.id.clone();
            retire_operations::<P, Mutation>(mutations);
            return Err(VcsError::TransactionOpen { transaction_id });
        }
        let started_at = now_iso();
        let pre_snapshot = Arc::clone(&*store_live.current);
        let (forwards, inverse, mutation_meta, post, messages, unit_flags) = store_owners.replay_mutations(store_log, &pre_snapshot, mutations, transaction.as_ref(), identity).await?;
        let actor = edit_actor_from_meta(&mutation_meta).await;

        store_log.edit_sequence += 1;
        let forwards_fingerprint = identity.encode(|control|semio_framework_pack_json::to_json_string_controlled(&*forwards,control))?.into_bytes();
        let reservation = store_owners.reserve_edit_history_slot(store_log)?;
        let mut edit = ScratchOwner::new(
            Edit {
                id: identity.encode(|control|mint_edit_id(store_log.clock.actor, store_log.edit_sequence, &forwards_fingerprint, control))?,
                actor,
                forwards: forwards.adopt(),
                inverse: inverse.adopt().into(),
                mutation_meta,
                verb: store_log.authoring_verb.clone(),
                line: store_log.envelope.active_alternative_id.clone(),
                sequence_number: store_log.edit_sequence,
                started_at,
                finished_at: Some(now_iso()),
            },
            |edit| retire_scratch_edits::<P, Mutation>([edit]),
        );
        stamp_primary_operation_identity(&mut *edit);
        let edit_id = edit.id.clone();
        let operations = store_log.operation_envelopes(&*edit)?;
        let tail = self.folded_tail(store_log, store_live, &*edit);
        let actor = edit.actor.clone();
        store_live.insert_reserved_edit_history(store_log, store_owners, reservation, edit.adopt())?;
        if !lane.is_document().await {
            store_log.envelope.lanes.insert(edit_id.clone(), lane);
        }
        self.record_edit_messages(store_log, store_owners, &edit_id, messages)?;
        store_live.replace_tail_undo_cache_retained(store_log, store_owners, Some((edit_id.clone(), pre_snapshot)))?;
        let digest = CursorRevisionAccumulator::hash_record(b"edit-id", &[edit_id.as_bytes()]);
        store_cursor.push_applied_edit_id(store_log, edit_id);
        store_live.replace_current_retained(store_log, store_owners, Arc::new(post.adopt()))?;
        self.announce_operations(store_log, operations)?;
        match tail {
            Some(key) => store_live.adopt_folded_tail(store_log, store_owners, store_cursor, actor.as_deref(), key)?,
            None => drop(store_live.reproject(store_log, store_owners, store_cursor).await?),
        }
        store_cursor.revision_accumulator.unit_flags.extend(unit_flags.into_iter().map(|(index, flag)| ((digest, index), flag)));
        self.bump(store_log, store_owners, store_cursor, store_live)?;
        Ok(())
    }

    //#endregion 🔖️HistoryLane
    /// 📨️ Records `edit_id`'s replay messages into the durable ledger and `pending_report`, shared
    /// by `apply_command` and the transaction appends — every local dispatch's own `DispatchReport`-shaped tail.
    pub(super) fn record_edit_messages(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, edit_id: &str, messages: Vec<semio_framework_os_spr::MutationMessage>) -> Result<(), VcsError> {
        if messages.is_empty() {
            return Ok(());
        }
        if edit_id.len() > ARTIFACT_EDIT_MESSAGE_ID_BYTES {
            return Err(VcsError::ValidationFailed("artifact edit-message id exceeds its fixed byte authority".into()));
        }
        let mut candidate = if let Some(established) = store_log.envelope.edit_messages.get_by_id(edit_id) {
            let mut combined = established.messages.clone();
            combined.extend(messages.clone());
            semio_framework_os_spr::EditMessages { edit_id: edit_id.to_string(), messages: combined }
        } else {
            semio_framework_os_spr::EditMessages { edit_id: edit_id.to_string(), messages: messages.clone() }
        };
        bound_edit_messages(edit_id, &mut candidate.messages);
        store_owners.displaced_retirements.reserve(1)?;
        if let Some(established) = store_log.envelope.edit_messages.get_mut_by_id(edit_id) {
            let previous = std::mem::replace(established, candidate);
            store_owners.displaced_retirements.push_reserved(Box::new(ArtifactStoreMessageLedgerRetirement::new(previous.edit_id, previous.messages)));
        } else if let Err(rejected) = store_log.envelope.edit_messages.admit(candidate) {
            store_owners.displaced_retirements.push_reserved(Box::new(ArtifactStoreMessageLedgerRetirement::new(rejected.edit_id, rejected.messages)));
            return Err(VcsError::ValidationFailed("artifact edit-message fixed ledger rejected its exact candidate owner".into()));
        }
        self.pending_report.record_edit_messages(edit_id.to_string(), messages);
        Ok(())
    }

    /// ✂️ Detaches the backbone; the WIP graph stays in memory, simply unsynchronized.
    ///
    /// 🛡️ {@link bump} runs FIRST so a refusal leaves the store exactly as it was. Clearing
    /// the persisted descriptor before the bump meant a store whose displaced-retirement
    /// destination was full (or whose generation was exhausted) reported a refusal while having
    /// already forgotten which backbone it was attached to — the descriptor was gone but the live
    /// transport, its queued payload and its generation were all still in place.
    pub fn detach_backbone(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_wire: &mut StoreWire<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>, store_live: &mut StoreLive<P, Mutation>) -> Result<Option<Backbones>, VcsError> {
        store_wire.ensure_durable_group_idle()?;
        self.bump(store_log, store_owners, store_cursor, store_live)?;
        store_log.envelope.backbone = None;
        Ok(store_wire.backbone.take())
    }

    /// ⚖️ Resolves an `Open` `crate::os_spr::Conflict` by id (`26/08/16/MUTATION-OUTCOMES-
    /// MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` §C6). `Quarantined`+`Accept` admits the conflict's
    /// withheld events into the log and re-folds it, validated under `LaissezFaire` (a `Fatal`
    /// message still rejects — `LaissezFaire` never waives that), landing the same state
    /// `ingest_remote` would have produced under `LaissezFaire` in the first place, and marks the
    /// conflict `Accepted` WITHOUT raising a second one. `Quarantined`+`Discard` seeds the quarantined ids into
    /// the causal dag as already-seen — never applied, never relayed — and marks the conflict
    /// `Discarded`. `Degraded`+`Accept` only flips the status: the batch is already durable history,
    /// resolving never rewrites it. `Degraded`+`Discard` is refused outright — shared history is
    /// never rewritten.
    pub async fn resolve_conflict(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_wire: &mut StoreWire<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>, store_live: &mut StoreLive<P, Mutation>, conflict_id: &str, resolution: semio_framework_os_spr::ConflictResolution) -> Result<semio_framework_os_spr::MergeReport, VcsError>
    where
        P: Sync,
    {
        store_wire.ensure_durable_group_idle()?;
        let index = store_log.envelope.conflicts.iter().position(|conflict| conflict.id.0 == conflict_id && conflict.status == semio_framework_os_spr::ConflictStatus::Open).ok_or_else(|| VcsError::UnknownConflict(conflict_id.to_string()))?;
        let conflict = store_log.envelope.conflicts[index].clone();
        match (conflict.kind.clone(), resolution) {
            (semio_framework_os_spr::ConflictKind::Quarantined { envelopes, .. }, semio_framework_os_spr::ConflictResolution::Accept) => self.accept_quarantined_events(store_log, store_owners, store_cursor, store_live, index, conflict.id.clone(), envelopes).await,
            (semio_framework_os_spr::ConflictKind::Quarantined { envelopes, .. }, semio_framework_os_spr::ConflictResolution::Discard) => {
                for envelope in &envelopes {
                    store_log.dag.seed_applied(envelope.mutation_id.clone()).map_err(|error| VcsError::ValidationFailed(error.to_string()))?;
                }
                store_log.envelope.conflicts[index].status = semio_framework_os_spr::ConflictStatus::Discarded;
                self.pending_report.declare_edit_ids(Vec::new());
                self.bump(store_log, store_owners, store_cursor, store_live)?;
                store_cursor.last_projection_cause = Some(ArtifactProjectionCause::RemoteIngest);
                store_owners.prune_resolved_conflicts(store_log)?;
                Ok(semio_framework_os_spr::MergeReport { policy: store_log.merge_policy, accepted: false, insertion_index: 0, replayed: Vec::new(), worst: None, conflict: Some(conflict.id.clone()) })
            }
            (semio_framework_os_spr::ConflictKind::Degraded { .. }, semio_framework_os_spr::ConflictResolution::Accept) => {
                store_log.envelope.conflicts[index].status = semio_framework_os_spr::ConflictStatus::Accepted;
                self.pending_report.declare_edit_ids(Vec::new());
                self.bump(store_log, store_owners, store_cursor, store_live)?;
                store_cursor.last_projection_cause = Some(ArtifactProjectionCause::RemoteIngest);
                store_owners.prune_resolved_conflicts(store_log)?;
                Ok(semio_framework_os_spr::MergeReport { policy: store_log.merge_policy, accepted: true, insertion_index: 0, replayed: Vec::new(), worst: None, conflict: Some(conflict.id.clone()) })
            }
            (semio_framework_os_spr::ConflictKind::Degraded { .. }, semio_framework_os_spr::ConflictResolution::Discard) => Err(VcsError::ValidationFailed("a Degraded conflict's batch is already durable history and can never be discarded".to_string())),
        }
    }

    /// ✅️ Admits a quarantined conflict's withheld events: the log is re-folded as if the
    /// conflict were accepted and the resulting order is replayed from genesis under `LaissezFaire`.
    /// A `Fatal` outcome reports the replay and changes nothing; otherwise the withheld edits join the
    /// log, the conflict turns `Accepted`, and the projection follows the fold.
    async fn accept_quarantined_events(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>, store_live: &mut StoreLive<P, Mutation>, index: usize, conflict_id: semio_framework_os_spr::ConflictId, envelopes: Vec<semio_framework_os_spr::MutationEnvelope>) -> Result<semio_framework_os_spr::MergeReport, VcsError> {
        let mut admitted: Vec<Edit<Mutation>> = Vec::with_capacity(envelopes.len());
        for envelope in &envelopes {
            let mut edit = edit_from_operation_envelope::<Mutation>(envelope).await?;
            edit.actor = Some(envelope.actor.0.clone());
            stamp_edit_semantics::<P, Mutation>(&mut edit, &store_log.envelope.schema);
            if !store_log.envelope.vcs.edits.iter().any(|known| known.id == edit.id) && !admitted.iter().any(|known| known.id == edit.id) {
                admitted.push(edit);
            } else {
                retire_scratch_edits::<P, Mutation>([edit]);
            }
        }
        let mut conflicts = store_log.envelope.conflicts.clone();
        conflicts[index].status = semio_framework_os_spr::ConflictStatus::Accepted;
        let mut edits: Vec<&Edit<Mutation>> = store_log.envelope.vcs.edits.iter().collect();
        edits.extend(admitted.iter());
        let head = envelope_viewer_head(&store_log.envelope);
        let fold = match fold_event_log::<P, Mutation>(&store_log.envelope.id, &edits, &store_log.envelope.transitions, &conflicts, &head) {
            Ok(fold) => fold,
            Err(error) => {
                retire_scratch_edits::<P, Mutation>(admitted);
                return Err(error);
            }
        };
        let edits_by_id: HashMap<String, Edit<Mutation>> = edits.iter().map(|edit| (edit.id.clone(), (*edit).clone())).collect();
        let replay = Self::replay_suffix_partitioned(store_log.envelope.vcs.genesis.facts().share_snapshot(), &fold.applied, 0, &edits_by_id, &store_log.envelope.schema, &fold.supersessions, semio_framework_os_spr::MergePolicy::LaissezFaire);
        retire_scratch_edits::<P, Mutation>(edits_by_id.into_values());
        let (state, quarantined_ids, mut rebased_inverse, replayed) = match replay {
            Ok(mut replay) => {
                let taken = (replay.take_state(), replay.take_quarantined(), replay.take_rebased_undo_rows().into_iter().collect::<HashMap<String, semio_framework_value::list::PagedList<Mutation, {usize::MAX}>>>(), replay.take_replayed());
                let _ = store_owners.retire_replay_result(replay);
                taken
            }
            Err(error) => {
                retire_scratch_edits::<P, Mutation>(admitted);
                return Err(error);
            }
        };
        if let Some(state) = state {
            retire_shared_projection::<P, Mutation>(state);
        }
        let admitted_replay: Vec<semio_framework_os_spr::EditMessages> = replayed.into_iter().filter(|entry| admitted.iter().any(|edit| edit.id == entry.edit_id)).collect();
        let worst = admitted_replay.iter().flat_map(|entry| entry.messages.iter()).map(|message| message.level).max();
        let report = semio_framework_os_spr::MergeReport { policy: semio_framework_os_spr::MergePolicy::LaissezFaire, accepted: quarantined_ids.is_empty(), insertion_index: 0, replayed: admitted_replay, worst, conflict: Some(conflict_id) };
        if !report.accepted {
            retire_scratch_edits::<P, Mutation>(admitted);
            for (_, inverse) in rebased_inverse {
                retire_scratch_operations::<P, Mutation>(inverse);
            }
            self.pending_report.declare_edit_ids(Vec::new());
            return Ok(report);
        }
        let admitted_ids: Vec<String> = admitted.iter().map(|edit| edit.id.clone()).collect();
        for mut edit in admitted {
            if let Some(inverse) = rebased_inverse.remove(&edit.id) {
                retire_scratch_operations::<P, Mutation>(std::mem::replace(&mut edit.inverse, inverse));
            }
            let reservation = store_owners.reserve_edit_history_slot(store_log)?;
            store_live.insert_reserved_edit_history(store_log, store_owners, reservation, edit)?;
        }
        for (_, inverse) in rebased_inverse {
            retire_scratch_operations::<P, Mutation>(inverse);
        }
        for entry in &report.replayed {
            store_owners.replace_edit_messages(store_log, &entry.edit_id, entry.messages.clone())?;
        }
        for envelope in &envelopes {
            match store_log.dag.seed_applied(envelope.mutation_id.clone()) {
                Ok(()) => {}
                Err(rejected) if rejected.error == semio_framework_os_spr::MutationDagError::Duplicate => {}
                Err(rejected) => return Err(VcsError::ValidationFailed(rejected.error.to_string())),
            }
        }
        store_log.envelope.conflicts[index].status = semio_framework_os_spr::ConflictStatus::Accepted;
        store_live.reproject(store_log, store_owners, store_cursor).await?;
        self.pending_report.declare_edit_ids(admitted_ids);
        self.bump(store_log, store_owners, store_cursor, store_live)?;
        store_cursor.last_projection_cause = Some(ArtifactProjectionCause::RemoteIngest);
        store_owners.prune_resolved_conflicts(store_log)?;
        Ok(report)
    }

    /// 📤️ Announces every event the finished command authored — edit operations and history
    /// transitions alike — as one `BackboneMessage::Mutations` batch.
    pub async fn flush_outbound(&mut self, store_owners: &mut StoreOwners<P, Mutation>, store_wire: &mut StoreWire<P, Mutation>) -> Result<(), VcsError> {
        if !self.pending_report.has_outbound() {
            return Ok(());
        }
        let Some(mut backbone) = store_wire.backbone.take() else {
            return Ok(());
        };
        let mut result = Ok(());
        for batch in document_backbone_batches(self.pending_report.take_outbound()) {
            result = backbone.send(BackboneMessage::Mutations { envelopes: semio_framework_os_spr::encode_envelopes(&batch) }).await;
            if result.is_err() {
                break;
            }
        }
        store_wire.replace_backbone_retained(store_owners, Some(backbone))?;
        result
    }

    /// 📤️ Announces exactly the `items` operations ONE outbound batched publication appended at
    /// position `from` of the tail applied edit, through the command outbound queue that
    /// `flush_outbound` drains. A closing transaction append extends the open edit whose earlier ticks were
    /// already announced; announcing the whole edit re-sent every earlier, already Accepted operation and the
    /// hub refused the batch as a replay (ticket 26/09/23 C10 09:4x: `[edit-A#0, edit-B#0]` on the
    /// second keystroke of a hub writer).
    pub(super) async fn flush_apply_outbound(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_wire: &mut StoreWire<P, Mutation>, from: usize, items: usize) -> Result<(), VcsError> {
        if store_wire.backbone.is_none() {
            return Ok(());
        }
        let tail = store_log.applied_edit_ids.last().ok_or_else(|| VcsError::ValidationFailed("outbound batched publication has no applied tail edit".into()))?;
        let edit = store_log.envelope.vcs.edits.find_near(|edit| edit.id == *tail).ok_or_else(|| VcsError::UnknownEdit(tail.clone()))?;
        if edit.forwards.len() != from.saturating_add(items) {
            return Err(VcsError::ValidationFailed("outbound batched publication no longer names the tail edit it appended to".into()));
        }
        let operations = store_log.operation_envelopes_since(edit, from)?;
        self.announce_operations(store_log, operations)?;
        self.flush_outbound(store_owners, store_wire).await
    }

    /// 🧭️ Whether the live cursor is the proven fold of the log as it stands ([`Self::fold_frontier`]) with no history step
    /// waiting.
    pub(super) fn fold_is_live(&self, store_live: &StoreLive<P, Mutation>) -> bool {
        store_live.fold_frontier.is_some() && self.pending_reprojection.is_none()
    }

    pub fn bump(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>, store_live: &mut StoreLive<P, Mutation>) -> Result<(), VcsError> {
        let next_generation = store_cursor.generation.checked_add(1).ok_or_else(|| VcsError::ValidationFailed("artifact store generation counter is exhausted".into()))?;
        store_owners.displaced_retirements.reserve(3)?;
        store_cursor.generation = next_generation;
        let dirt = std::mem::replace(&mut store_cursor.mirror_dirt, StackMirrorDirt::CLEAN);
        store_owners.sync_cursor(store_log, dirt);
        let regrown = store_cursor.revision_accumulator.reconcile::<P, Mutation>(&store_log.applied_edit_ids, &store_log.redo_edit_ids, &store_log.envelope.vcs.edits, &store_live.supersessions, dirt, store_cursor.revision_dirty_from.take());
        store_cursor.content_revision = store_cursor.revision_accumulator.revision(store_log.current_checkpoint_id.as_deref());
        assert!(store_owners.snapshot_read_leases.publish_authority(store_cursor.generation, store_cursor.content_revision), "single-owner store bump must advance its snapshot commit authority");
        #[cfg(test)]
        self.assert_mirrors_are_live(store_log, store_cursor, store_live);
        if let Some(position) = regrown {
            store_live.prefix_ring_dirty_from = store_live.prefix_ring_dirty_from.min(position);
        }
        store_live.prune_prefix_ring(store_owners)?;
        store_owners.relieve_displaced_pressure()
    }

    /// 🔬️ Test-build oracle of every incremental mirror on short histories ([`MIRROR_ORACLE_HISTORY`]): the persisted cursor
    /// equals the live stacks, the content revision equals a revision rebuilt from scratch, and a claimed fold frontier is
    /// the fold of the log.
    #[doc(hidden)]
    pub fn assert_mirrors_are_live(&self, store_log: &StoreLog<P, Mutation>, store_cursor: &StoreCursor<P, Mutation>, store_live: &StoreLive<P, Mutation>) {
        let length = store_log.applied_edit_ids.len();
        if length > MIRROR_ORACLE_HISTORY || (length > MIRROR_ORACLE_EVERY_HISTORY && length % MIRROR_ORACLE_STRIDE != 0) {
            return;
        }
        let cursor = store_log.envelope.cursor.as_ref().expect("a bumped store holds its cursor");
        assert!(cursor.applied_edit_ids == *store_log.applied_edit_ids && cursor.redo_edit_ids == *store_log.redo_edit_ids && cursor.checkpoint_id == *store_log.current_checkpoint_id, "the persisted cursor mirrors the live stacks");
        let mut rebuilt = CursorRevisionAccumulator { identity_digest: store_cursor.revision_accumulator.identity_digest, applied: semio_framework_os_vcs::HistoryPageStack::new(), redo: semio_framework_os_vcs::HistoryPageStack::new(), applied_tail_chains: None, mutation_positions: protocol::HistoryFoldIndex::new(), indexed_edits: protocol::HistoryFoldIndex::new(), unit_flags: protocol::HistoryFoldIndex::new() };
        rebuilt.reconcile::<P, Mutation>(&store_log.applied_edit_ids, &store_log.redo_edit_ids, &store_log.envelope.vcs.edits, &store_live.supersessions, StackMirrorDirt::ALL, None);
        for (stack, live, scratch) in [("applied", &store_cursor.revision_accumulator.applied, &rebuilt.applied), ("redo", &store_cursor.revision_accumulator.redo, &rebuilt.redo)] {
            let stale = live.iter().zip(scratch.iter()).position(|(live, scratch)| live.id_digest != scratch.id_digest || live.edit_digest != scratch.edit_digest);
            assert!(stale.is_none() && live.len() == scratch.len(), "the incremental {stack} revision records equal records rebuilt from scratch: first stale record {stale:?}, {} live vs {} rebuilt", live.len(), scratch.len());
        }
        assert_eq!(rebuilt.revision(store_log.current_checkpoint_id.as_deref()), store_cursor.content_revision, "the incremental content revision equals one rebuilt from scratch");
        if self.fold_is_live(store_live) {
            let fold = fold_envelope_history(&store_log.envelope).expect("a proven fold refolds");
            assert!(
                fold.applied == *store_log.applied_edit_ids && fold.redo == *store_log.redo_edit_ids && fold.checkpoint == *store_log.current_checkpoint_id && fold.alternative == store_log.envelope.active_alternative_id && fold.supersessions == *store_live.supersessions,
                "a claimed fold frontier is the fold of the log"
            );
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
