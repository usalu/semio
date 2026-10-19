//! 📣️ Replay invalidation, announcements and transactions: the projection invalidation, the pending reports, the suffix replay, the announcement and admission of transitions, the append and the open transaction. See [`super`].

use super::*;
use semio_framework_os_spr::ColdMutation;

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
    /// 🔄️ Invalidates projections after a verified replay that did not otherwise change history.
    pub fn invalidate_after_replay(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_wire: &mut StoreWire<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>, store_live: &mut StoreLive<P, Mutation>) -> Result<ArtifactProjectionInvalidation, VcsError> {
        self.invalidate_projections(store_log, store_owners, store_wire, store_cursor, store_live, ArtifactProjectionCause::Replay)
    }

    /// 🛂️ Invalidates projections after a policy change outside semantic event history.
    pub fn invalidate_after_policy_change(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_wire: &mut StoreWire<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>, store_live: &mut StoreLive<P, Mutation>) -> Result<ArtifactProjectionInvalidation, VcsError> {
        self.invalidate_projections(store_log, store_owners, store_wire, store_cursor, store_live, ArtifactProjectionCause::PolicyChange)
    }

    /// 🔗️ Invalidates projections after an external resource changed beneath the artifact.
    pub fn invalidate_after_external_resource_change(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_wire: &mut StoreWire<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>, store_live: &mut StoreLive<P, Mutation>) -> Result<ArtifactProjectionInvalidation, VcsError> {
        self.invalidate_projections(store_log, store_owners, store_wire, store_cursor, store_live, ArtifactProjectionCause::ExternalResourceChange)
    }

    pub fn invalidate_projections(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_wire: &mut StoreWire<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>, store_live: &mut StoreLive<P, Mutation>, cause: ArtifactProjectionCause) -> Result<ArtifactProjectionInvalidation, VcsError> {
        store_wire.ensure_durable_group_idle()?;
        self.bump(store_log, store_owners, store_cursor, store_live)?;
        store_cursor.last_projection_cause = Some(cause);
        Ok(ArtifactProjectionInvalidation { cause, stamp: store_cursor.projection_stamp(store_log, store_wire) })
    }

    fn pending_report_owner_slots(value: &PendingCommandReport) -> usize {
        usize::from(value.holds_owner_slot())
    }

    fn push_pending_report_replacement_reserved(&mut self, store_owners: &mut StoreOwners<P, Mutation>, next: PendingCommandReport) {
        let previous = std::mem::replace(&mut *self.pending_report, next);
        if Self::pending_report_owner_slots(&previous) != 0 {
            store_owners.displaced_retirements.push_reserved(Box::new(ArtifactStorePendingReportRetirement::new(previous)));
        }
    }

    pub fn replace_pending_report_retained(&mut self, store_owners: &mut StoreOwners<P, Mutation>, next: PendingCommandReport) -> Result<(), VcsError> {
        store_owners.displaced_retirements.reserve(Self::pending_report_owner_slots(&self.pending_report))?;
        self.push_pending_report_replacement_reserved(store_owners, next);
        Ok(())
    }

    /// 🔀️ Folds `order[k..]` from `base`, recomputing each op's `diff` against the shifted state (so
    /// its inverse is rebased and its messages collected) and deciding accept-vs-quarantine PER EDIT
    /// rather than atomically over the whole suffix (`26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-
    /// CLASS-CONFLICTS` §C6 step 7-8, H1 determinism fix). An edit already committed by an
    /// earlier `ingest_remote` call, once a later-arriving earlier-HLC envelope forces
    /// `order[k..]` to include it again, gets exactly the accept/quarantine verdict it would have
    /// gotten had every envelope in `order[k..]` arrived in true HLC order in the first place —
    /// so the merge converges to the same `applied_edit_ids`/state/conflicts regardless of which
    /// of two separate `ingest_remote` calls happened first. A quarantined edit's forward ops and
    /// `MutationMeta` are never touched (they stay exactly as recorded); only its diff/inverse/
    /// messages were freshly recomputed above, and it is simply left out of `committed_ids` — the
    /// same "diffs/inverses/messages get recomputed, forwards ops never rewritten" contract an
    /// `Undo` already relies on (id drops out of `applied_edit_ids`, the `Edit` record stays put).
    /// The loop itself is the [`EditReplay`] stepper in `Merge(policy)` mode, driven to completion here; every operation
    /// reads its effective input under `supersessions`.
    pub fn replay_suffix_partitioned(
        base: Arc<P>,
        order: &[String],
        k: usize,
        edits: &impl ReplayEdits<Mutation>,
        schema: &str,
        supersessions: &EffectiveSupersessions,
        policy: semio_framework_os_spr::MergePolicy,
    ) -> Result<EditReplayResult<P, Mutation>, VcsError> {
        EditReplay::new(ReplayMode::Merge(policy), base, order.to_vec(), k, schema, supersessions.clone(), edits)?.run(edits)
    }

    //#region 📬️OneItemPublication
    /// 📤️ Announces the ONE edit an outbound batched publication just made visible as ONE
    /// `BackboneMessage::Mutations` batch. `false` when the publication is not an outbound one awaiting
    /// its announcement; the ACK becomes admissible only after this returns `true`.
    pub async fn flush_published_apply_batch(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_wire: &mut StoreWire<P, Mutation>, publication: &mut ArtifactStoreBatchPublication<P, Mutation>) -> Result<bool, VcsError> {
        
        if !publication.original_registry.as_ref().is_some_and(|registry|registry.ptr_eq(&self.snapshot_read_leases)){return Err(VcsError::InverseRefused(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"batched announcement requires its original Store issuer")));}
if publication.phase != ArtifactStoreOneItemPublicationPhase::AwaitingAck || !publication.outbound {
            return Ok(false);
        }
        publication.outbound = false;
        self.flush_apply_outbound(store_log, store_owners, store_wire, publication.announce_from, publication.announce_items).await?;
        Ok(true)
    }

    //#endregion 📬️OneItemPublication
    //#region 🔖️EventSourcedHistory
    /// 📤️ Marks locally authored `operations` applied in the causal graph, stamps each with the
    /// foreign operation its author had observed (advisory, never an ordering constraint) and queues
    /// them for this command's outbound announcement. Announcement happens when the command authors
    /// them, so an edit made during a connection shortage names what its author saw THEN.
    pub(super) fn announce_operations(&mut self, store_log: &mut StoreLog<P, Mutation>, mut operations: Vec<semio_framework_os_spr::MutationEnvelope>) -> Result<(), VcsError> {
        for operation in &mut operations {
            if operation.observed.is_none() {
                operation.observed = store_log.observed_foreign_operation(&operation.actor.0);
            }
        }
        for operation in &operations {
            match store_log.dag.seed_applied(operation.mutation_id.clone()) {
                Ok(()) => {}
                Err(rejected) if rejected.error == semio_framework_os_spr::MutationDagError::Duplicate => {}
                Err(rejected) => return Err(VcsError::ValidationFailed(rejected.error.to_string())),
            }
        }
        self.pending_report.author_outbound(operations);
        Ok(())
    }

    /// 📥️ Admits remote history transitions the causal graph released: known ones must repeat
    /// their established payload, new ones join the log, and the projection follows the fold.
    pub async fn admit_remote_transitions(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_wire: &mut StoreWire<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>, store_live: &mut StoreLive<P, Mutation>, transitions: Vec<semio_framework_os_spr::MutationEnvelope>) -> Result<Vec<semio_framework_os_spr::EditMessages>, VcsError> {
        let mut fresh: Vec<semio_framework_os_spr::MutationEnvelope> = Vec::new();
        for envelope in transitions {
            store_log.admit_history_shape(std::slice::from_ref(&envelope))?;
            let pending = self.pending_reprojection.as_ref().map_or(&[][..], |pending| pending.transitions.as_slice());
            if let Some(known) = store_log.envelope.transitions.iter().chain(pending).chain(fresh.iter()).find(|known| known.mutation_id == envelope.mutation_id) {
                if !StoreLog::<P, Mutation>::same_operation_identity_and_payload(known, &envelope) {
                    return Err(VcsError::ValidationFailed(format!("remote transition {} conflicts with its established payload", envelope.mutation_id.0)));
                }
                continue;
            }
            store_log.clock.merge(&envelope.timestamp);
            fresh.push(envelope);
        }
        if self.pending_reprojection.is_none() && (self.replay_budget.is_none() || fresh.is_empty()) {
            return store_live.adopt_remote_transitions(store_log, store_owners, store_cursor, fresh, None).await;
        }
        let mut pending = self.pending_reprojection.take().unwrap_or_else(PendingReprojection::empty);
        let known = pending.transitions.len();
        pending.transitions.extend(fresh);
        if let Err(error) = store_log.pending_fold(&pending) {
            pending.transitions.truncate(known);
            self.pending_reprojection = (!pending.transitions.is_empty() || pending.local.is_some()).then_some(pending);
            return Err(error);
        }
        store_owners.retire_pending_replay(pending.replay.take())?;
        self.pending_reprojection = Some(pending);
        match self.advance_reprojection(store_log, store_owners, store_wire, store_cursor, store_live, None).await? {
            Some(ReprojectionAdvance::Adopted(replayed)) => Ok(replayed),
            Some(ReprojectionAdvance::Pending(_)) | None => Ok(Vec::new()),
        }
    }

    /// 🔙️ `BackboneMessage::Retract`: removes from the event log every transition this replica authored that `refused`
    /// names, with every transition of this replica causally depending on one ([`transition_retraction_closure`]), and
    /// folds the projection again — transactional: a log that would not fold keeps every transition. A name this log does
    /// not hold, or one another replica authored, retracts nothing: only this replica's own speculative head can be
    /// refused to it. Answers the retracted ids.
    pub async fn retract_transitions(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>, store_live: &mut StoreLive<P, Mutation>, refused: &[String]) -> Result<Vec<String>, VcsError> where P: Clone, Mutation: ColdMutation<P> {
        let actor = store_log.transition_actor();
        let own: Vec<&semio_framework_os_spr::MutationEnvelope> = store_log.envelope.transitions.iter().filter(|transition| transition.actor == actor).collect();
        let retracted = transition_retraction_closure(&own, |transition| transition.mutation_id.0.as_str(), |transition| transition.dependencies.iter().map(|dependency| dependency.0.as_str()).collect(), refused);
        if retracted.is_empty() {
            return Ok(retracted);
        }
        let removed: Vec<semio_framework_os_spr::MutationEnvelope> = store_log.envelope.transitions.iter().filter(|transition| retracted.contains(&transition.mutation_id.0)).cloned().collect();
        store_log.envelope.transitions.retain(|transition| !retracted.contains(&transition.mutation_id.0));
        if let Err(error) = store_live.reproject(store_log, store_owners, store_cursor).await {
            for envelope in removed {
                store_live.insert_transition(store_log, envelope);
            }
            return Err(error);
        }
        store_cursor.last_projection_cause = Some(ArtifactProjectionCause::RemoteIngest);
        self.bump(store_log, store_owners, store_cursor, store_live)?;
        Ok(retracted)
    }

    /// 🪝️ `edit`'s `(hlc)` key when appending it — a fresh local edit tagged to this replica's line — makes the log's fold exactly
    /// the live cursor with `edit` applied last ([`crate::os_spr::fold_history_for`]): the live cursor is the proven fold of the
    /// log ([`Self::fold_frontier`]), `edit` sorts after every event, no history step waits and the head is its line's tip, where
    /// every uncommitted edit of the line is visible.
    pub(super) fn folded_tail(&self, store_log: &StoreLog<P, Mutation>, store_live: &StoreLive<P, Mutation>, edit: &Edit<Mutation>) -> Option<(u64, u64, u64)> {
        let key = edit_timestamp(edit).cmp_key();
        let tail = store_live.fold_frontier.is_some_and(|frontier| key > frontier) && self.pending_reprojection.is_none() && store_log.envelope.viewer_checkpoint_id.is_none() && edit.line == store_log.envelope.active_alternative_id;
        tail.then_some(key)
    }

    /// ⚡️ The tail step `envelope` — one locally authored `Revert` or `Reinstate` — makes when it changes nothing but the
    /// applied tail: the live cursor is the proven fold of the log ([`Self::fold_is_live`]) at this replica's line tip, the
    /// transition sorts after every event, and it names exactly the operations of the applied tail edit (`Undo`) or of the redo
    /// top a tail step just moved there with nothing happening since (`Redo`), authored by that edit's own actor. The fold then
    /// moves exactly that edit between the stacks (`crate::os_spr::fold_history_for`): a `Revert` pushes it onto the redo
    /// stack, a `Reinstate` takes it back at its own HLC position, which is the tail (design §20.14, D22).
    pub fn tail_step(&self, store_log: &StoreLog<P, Mutation>, store_cursor: &StoreCursor<P, Mutation>, store_live: &StoreLive<P, Mutation>, envelope: &semio_framework_os_spr::MutationEnvelope) -> Option<TailStep> {
        let frontier = store_live.fold_frontier.filter(|_| self.fold_is_live(store_live) && !store_live.current_detached && store_log.envelope.viewer_checkpoint_id.is_none() && store_log.envelope.open_transaction.is_none())?;
        if envelope.timestamp.cmp_key() <= frontier {
            return None;
        }
        let (step, mutation_ids, edit_id) = match semio_framework_os_spr::history_transition_from_envelope(envelope).ok()?? {
            semio_framework_os_spr::HistoryTransition::Revert { mutation_ids } if store_log.redo_edit_ids.admits_one() => (TailStep::Undo, mutation_ids, store_log.applied_edit_ids.last()?),
            semio_framework_os_spr::HistoryTransition::Reinstate { mutation_ids } if store_live.tail_step_generation == Some(store_cursor.generation) && store_log.applied_edit_ids.admits_one() => (TailStep::Redo, mutation_ids, store_log.redo_edit_ids.last()?),
            _ => return None,
        };
        let edit = store_log.envelope.vcs.edits.find_near(|edit| edit.id == *edit_id)?;
        let owned: HashSet<MutationId> = semio_framework_os_spr::mutation_ids_for_edit::<P, Mutation>(edit).into_iter().collect();
        let exact = mutation_ids.len() == owned.len() && mutation_ids.iter().all(|mutation_id| owned.contains(mutation_id));
        (exact && edit.actor.as_deref().is_none_or(|actor| actor == envelope.actor.0.as_str())).then_some(step)
    }

    //#endregion 🔖️EventSourcedHistory
    //#region 🔖️ToolTransactions
    /// 🪡️ `AppendTransaction`: the first append opens one edit stamped with the ref — no description — and
    /// every later one folds its operations onto the projection and appends them to that edit, every operation stamped with
    /// the ref. The open edit is always the applied tail ([`Self::ingest_remote`] lifts it over remote edits), and neither
    /// the shared log nor a persisted form holds it before its commit, so an abort leaves no trace anywhere.
    pub async fn append_transaction(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>, store_live: &mut StoreLive<P, Mutation>, mutations: Vec<Mutation>, transaction: protocol::TransactionRef, identity: &mut EntityIdentityAuthority<'_>) -> Result<(), VcsError> where P: Clone, Mutation: ColdMutation<P> {
        if mutations.is_empty() {
            return Err(VcsError::EmptyApply);
        }
        let Some(open) = store_log.envelope.open_transaction.clone() else {
            return self.open_transaction_edit(store_log, store_owners, store_cursor, store_live, mutations, transaction, identity).await;
        };
        let before = store_cursor.content_revision;
        let pre_snapshot = Arc::clone(&*store_live.current);
        let (forwards, inverse, mutation_meta, post, mut messages, unit_flags) = store_owners.replay_mutations(store_log, &pre_snapshot, mutations, Some(&transaction), identity).await?;
        let edit = store_log.envelope.vcs.edits.iter_mut().find(|edit| edit.id == open.edit_id).ok_or_else(|| VcsError::UnknownEdit(open.edit_id.clone()))?;
        let offset = edit.forwards.len() as u32;
        for message in messages.iter_mut() {
            message.op_index = message.op_index.map(|index| index + offset);
        }
        let appended = forwards.len();
        edit.forwards.extend(forwards.adopt());
        edit.inverse.extend(inverse.adopt());
        edit.mutation_meta.extend(mutation_meta);
        self.record_edit_messages(store_log, store_owners, &open.edit_id, messages)?;
        store_live.replace_current_retained(store_log, store_owners, Arc::new(post.adopt()))?;
        if !self.fold_is_live(store_live) {
            store_live.reproject(store_log, store_owners, store_cursor).await?;
        }
        let digest = CursorRevisionAccumulator::hash_record(b"edit-id", &[open.edit_id.as_bytes()]);
        store_cursor.revision_accumulator.unit_flags.extend(unit_flags.into_iter().map(|(index, flag)| ((digest, offset as usize + index), flag)));
        self.bump(store_log, store_owners, store_cursor, store_live)?;
        self.continue_pending_replay_across_append(store_cursor, before, &open.edit_id, appended);
        Ok(())
    }

    /// 🧵️ A tick appended to the open transaction's edit — the applied tail — changes nothing a waiting replay already
    /// replayed and nothing it reads by id before the tail: the waiting replay started against the revision `before` the tick
    /// keeps running against the grown edit instead of restarting, so a tool streaming every turn never starves a waiting
    /// history step (audit W1G-5). A replay started against any other revision restarts as before.
    pub fn continue_pending_replay_across_append(&mut self, store_cursor: &mut StoreCursor<P, Mutation>, before: [u8; 32], edit_id: &str, appended: usize) {
        let revision = store_cursor.content_revision;
        if let Some((started, replay)) = self.pending_reprojection.as_mut().and_then(|pending| pending.replay.as_mut()).filter(|(started, _)| *started == before) {
            *started = revision;
            replay.absorb_tail_growth(edit_id, appended);
        }
    }

    /// 🪁️ The first append of a tool transaction: one fresh edit at the applied tail holding its operations, open and
    /// unannounced.
    async fn open_transaction_edit(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>, store_live: &mut StoreLive<P, Mutation>, mutations: Vec<Mutation>, transaction: protocol::TransactionRef, identity: &mut EntityIdentityAuthority<'_>) -> Result<(), VcsError> where P: Clone, Mutation: ColdMutation<P> {
        let started_at = now_iso();
        let pre_snapshot = Arc::clone(&*store_live.current);
        let (forwards, inverse, mutation_meta, post, messages, unit_flags) = store_owners.replay_mutations(store_log, &pre_snapshot, mutations, Some(&transaction), identity).await?;
        let actor = edit_actor_from_meta(&mutation_meta).await;

        store_log.edit_sequence += 1;
        let forwards_fingerprint = identity.encode(|control|semio_framework_pack_json::to_json_string_controlled(&*forwards,control))?.into_bytes();
        let reservation = store_owners.reserve_edit_history_slot(store_log)?;
        let edit_id = identity.encode(|control|mint_edit_id(store_log.clock.actor, store_log.edit_sequence, &forwards_fingerprint, control))?;
        let edit = Edit {
            id: edit_id.clone(),
            actor,
            forwards: forwards.adopt(),
            inverse: inverse.adopt().into(),
            mutation_meta,
            verb: store_log.authoring_verb.clone(),
            line: store_log.envelope.active_alternative_id.clone(),
            sequence_number: store_log.edit_sequence,
            started_at,
            finished_at: None,
        };
        let tail = self.folded_tail(store_log, store_live, &edit);
        let actor = edit.actor.clone();
        store_live.insert_reserved_edit_history(store_log, store_owners, reservation, edit)?;
        self.record_edit_messages(store_log, store_owners, &edit_id, messages)?;
        store_live.replace_tail_undo_cache_retained(store_log, store_owners, Some((edit_id.clone(), pre_snapshot)))?;
        store_cursor.push_applied_edit_id(store_log, edit_id.clone());
        store_live.replace_current_retained(store_log, store_owners, Arc::new(post.adopt()))?;
        let digest = CursorRevisionAccumulator::hash_record(b"edit-id", &[edit_id.as_bytes()]);
        store_cursor.revision_accumulator.unit_flags.extend(unit_flags.into_iter().map(|(index, flag)| ((digest, index), flag)));
        store_log.envelope.open_transaction = Some(OpenToolTransaction { transaction, edit_id });
        match tail {
            Some(key) => store_live.adopt_folded_tail(store_log, store_owners, store_cursor, actor.as_deref(), key)?,
            None => drop(store_live.reproject(store_log, store_owners, store_cursor).await?),
        }
        self.bump(store_log, store_owners, store_cursor, store_live)
    }

    /// 🎗️ `CommitTransaction`: the open edit closes and every operation it holds is queued for announcement in one batch —
    /// exactly the edit one `Apply` of the same operations under the same ref would have recorded. Never suspends, so a
    /// runtime's one-page publisher closes a streamed transaction in place ([`Self::flush_announcements`] sends it).
    pub fn commit_open_transaction(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>, store_live: &mut StoreLive<P, Mutation>, transaction_id: &str) -> Result<(), VcsError> {
        let open = store_log.open_transaction_named(transaction_id)?;
        let edit = store_log.envelope.vcs.edits.iter_mut().find(|edit| edit.id == open.edit_id).ok_or_else(|| VcsError::UnknownEdit(open.edit_id.clone()))?;
        edit.finished_at = Some(now_iso());
        stamp_primary_operation_identity(edit);
        let edit = store_log.envelope.vcs.edits.find_near(|edit| edit.id == open.edit_id).ok_or_else(|| VcsError::UnknownEdit(open.edit_id.clone()))?;
        let operations = store_log.operation_envelopes(edit)?;
        store_log.envelope.open_transaction = None;
        self.announce_operations(store_log, operations)?;
        self.bump(store_log, store_owners, store_cursor, store_live)
    }

    /// 🪃️ `AbortTransaction`: the open edit leaves the ledger, its messages and its operations retire, the edit sequence it
    /// took is returned, and the projection folds back to the history without it — the projection and revision of before
    /// its first append. Never suspends.
    pub fn abort_open_transaction(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>, store_live: &mut StoreLive<P, Mutation>, transaction_id: &str) -> Result<(), VcsError> where P: Clone, Mutation: ColdMutation<P> {
        let open = store_log.open_transaction_named(transaction_id)?;
        let removed = store_log.envelope.vcs.edits.extract_if(|edit| edit.id == open.edit_id).map_err(|fault| VcsError::ValidationFailed(format!("the open transaction edit cannot leave its ledger: {fault:?}")))?;
        if removed.iter().any(|edit| edit.sequence_number == store_log.edit_sequence) {
            store_log.edit_sequence -= 1;
        }
        retire_scratch_edits::<P, Mutation>(removed);
        store_owners.replace_edit_messages(store_log, &open.edit_id, Vec::new())?;
        store_log.envelope.open_transaction = None;
        store_live.reproject_now(store_log, store_owners, store_cursor)?;
        self.bump(store_log, store_owners, store_cursor, store_live)
    }

    /// 📤️ Sends every announcement a non-suspending history step queued ([`Self::commit_open_transaction`]) as one
    /// `BackboneMessage::Mutations` batch; nothing queued sends nothing.
    pub async fn flush_announcements(&mut self, store_owners: &mut StoreOwners<P, Mutation>, store_wire: &mut StoreWire<P, Mutation>) -> Result<(), VcsError> {
        self.flush_outbound(store_owners, store_wire).await
    }

    /// 🛞️ Folds a lifted tool transaction's edit back as the applied tail of the merged history: its operations take fresh
    /// clock stamps after everything this replica has seen and its next edit sequence, and the Report replay recomputes
    /// their inverses and messages against the merged head (keep-and-record: nothing is quarantined or refused). When the
    /// ledger can no longer hold it the transaction is aborted — zero trace — and the refusal is answered.
    pub async fn restore_open_transaction(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>, store_live: &mut StoreLive<P, Mutation>, open: OpenToolTransaction, mut edit: Edit<Mutation>) -> Result<(), VcsError> where P: Clone, Mutation: ColdMutation<P> {
        for meta in edit.mutation_meta.iter_mut() {
            store_log.clock.tick(now_ms());
            meta.timestamp = store_log.clock;
        }
        store_log.edit_sequence += 1;
        edit.sequence_number = store_log.edit_sequence;
        let reservation = match store_owners.reserve_edit_history_slot(store_log) {
            Ok(reservation) => reservation,
            Err(error) => {
                store_log.edit_sequence -= 1;
                retire_scratch_edits::<P, Mutation>([edit]);
                return Err(error);
            }
        };
        store_live.insert_reserved_edit_history(store_log, store_owners, reservation, edit)?;
        store_log.envelope.open_transaction = Some(open);
        store_live.reproject(store_log, store_owners, store_cursor).await?;
        self.bump(store_log, store_owners, store_cursor, store_live)
    }

    //#endregion 🔖️ToolTransactions
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
