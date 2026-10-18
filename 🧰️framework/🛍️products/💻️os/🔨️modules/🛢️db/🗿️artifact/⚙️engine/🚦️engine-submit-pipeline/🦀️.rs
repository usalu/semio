//! 🚦️ Engine submit pipeline: planning, committing, history transition refusal and `submit`. See [`super`].

use super::*;

//#region 🔖️Ids
//#region 🔖️Engine
impl ArtifactEngine {
    /// 🧭️ Plans `envelope` against this engine and the batch so far without applying anything: `None` when it is
    /// already applied (the per-envelope half of the dedupe law), else its decoded entries — values encoded as the state
    /// stores them — with the regions it touches and the conflicts it meets. Refuses an envelope whose dependency is
    /// neither applied, nor earlier in `batch_ids`, nor folded by a durable group decision.
    pub(super) async fn plan_one(&self, envelope: &protocol::MutationEnvelope, batch_ids: &HashSet<String>, written: &mut HashMap<String, protocol::MutationId>) -> Result<Option<PlannedEntries>, DbError> {
        if self.applied.contains_key(&envelope.mutation_id.0) {
            return Ok(None);
        }
        for dependency in &envelope.dependencies {
            if !self.applied.contains_key(&dependency.0) && !batch_ids.contains(&dependency.0) && !self.names_durable_group_operation(&dependency.0) {
                return Err(DbError::InvalidArgument(format!("operation {} depends on unseen operation {}", envelope.mutation_id.0, dependency.0)));
            }
        }
        let entries = diff_entries(&envelope.diff).await?;
        check_len(entries.len() as u64, self.config.limits.max_batch_commands as u64, "db_artifact::diff_entries")?;
        let (touched, conflicts) = self.state.plan_entries(&envelope.mutation_id, &envelope.dependencies, &entries, written)?;
        let entries = entries.into_iter().map(|(path, value)| (path, value.map(|value| store::pack_rt::encode_wire_value(&value)))).collect();
        Ok(Some(PlannedEntries { entries, touched, conflicts }))
    }

    /// ✍️ Applies one planned envelope to the document state and records it as applied.
    pub(super) async fn commit_one(&mut self, envelope: &protocol::MutationEnvelope, entries: Vec<(String, Option<Vec<u8>>)>) -> Result<(), DbError> {
        self.state.apply_entries(&envelope.mutation_id, entries).await?;
        self.applied.insert(envelope.mutation_id.0.clone(), envelope.clone());
        *self.actor_seq.entry(envelope.actor.0.clone()).or_insert(0) += 1;
        Ok(())
    }

    /// 🪪️ The event log's twin of `protocol::fold_history`'s admission, so the log never holds a history transition every
    /// replica refuses: refused before the WAL, whatever the merge policy, with one typed message.
    /// - A payload that does not decode: [`MALFORMED_HISTORY_TRANSITION_CODE`].
    /// - An undo belongs to its author: a `Revert` or `Reinstate` naming an operation this document's log (or an envelope of
    ///   the same batch) records under another actor: [`FOREIGN_HISTORY_TRANSITION_CODE`]. An operation the log does not
    ///   know belongs to anyone (the replicas' fold says the same).
    /// - A `Supersede` has no ownership rule: editing history is non-destructive and reversible, and write permission is the
    ///   security gate's. It must depend on exactly its targets ([`MALFORMED_HISTORY_TRANSITION_CODE`]), and every target
    ///   must be committed, `earlier` in its batch, or folded by a durable group decision ([`UNKNOWN_SUPERSEDE_TARGET_CODE`]).
    ///
    /// 📋️ `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/📋️design.md` §2, §9.1.
    fn history_transition_refusal(&self, envelope: &protocol::MutationEnvelope, batch: &[protocol::MutationEnvelope], earlier: &HashSet<String>) -> Option<protocol::MutationMessage> {
        let transition = match protocol::history_transition_from_envelope(envelope) {
            Ok(Some(transition)) => transition,
            Ok(None) => return None,
            Err(_) => return Some(protocol::MutationMessage::error(MALFORMED_HISTORY_TRANSITION_CODE, MALFORMED_HISTORY_TRANSITION_MESSAGE)),
        };
        match transition {
            protocol::HistoryTransition::Revert { mutation_ids } | protocol::HistoryTransition::Reinstate { mutation_ids } => {
                let author = |id: &protocol::MutationId| self.applied.get(&id.0).map(|applied| &applied.actor).or_else(|| batch.iter().find(|earlier| &earlier.mutation_id == id).map(|earlier| &earlier.actor));
                mutation_ids.iter().any(|id| author(id).is_some_and(|actor| actor != &envelope.actor)).then(|| protocol::MutationMessage::error(FOREIGN_HISTORY_TRANSITION_CODE, FOREIGN_HISTORY_TRANSITION_MESSAGE))
            }
            protocol::HistoryTransition::Supersede(supersede) => {
                let targets = supersede.targets();
                if envelope.dependencies != targets {
                    return Some(protocol::MutationMessage::error(MALFORMED_HISTORY_TRANSITION_CODE, MALFORMED_HISTORY_TRANSITION_MESSAGE));
                }
                targets
                    .into_iter()
                    .find(|target| !self.applied.contains_key(&target.0) && !earlier.contains(&target.0) && !self.names_durable_group_operation(&target.0))
                    .map(|target| protocol::MutationMessage::error(UNKNOWN_SUPERSEDE_TARGET_CODE, UNKNOWN_SUPERSEDE_TARGET_MESSAGE).at([target.0]))
            }
            protocol::HistoryTransition::Commit(_) | protocol::HistoryTransition::Branch { .. } | protocol::HistoryTransition::Checkout { .. } | protocol::HistoryTransition::Repin { .. } => None,
        }
    }

    /// 👁️ Whether `dependency` names an operation a committed durable group decision folded: the
    /// decision's edit itself, or `<edit>#<position>`.
    fn names_durable_group_operation(&self, dependency: &str) -> bool {
        self.durable_group_edit_ids.contains(dependency) || dependency.rsplit_once('#').is_some_and(|(edit, position)| !position.is_empty() && position.bytes().all(|byte| byte.is_ascii_digit()) && self.durable_group_edit_ids.contains(edit))
    }

    /// 🎯️ Keeps `envelope`'s declared target while its touch is in the recent commit window, so a later
    /// concurrent write can be graded against it; targets that left the window are dropped with it.
    pub(super) fn remember_target(&mut self, envelope: &protocol::MutationEnvelope) {
        if self.recent_targets.len() >= MAX_RECENT_TOUCHES {
            let window: HashSet<&str> = self.recent_touches.iter().map(|touch| touch.command_id.0.as_str()).collect();
            self.recent_targets.retain(|command, _| window.contains(command.as_str()));
        }
        self.recent_targets.insert(envelope.mutation_id.0.clone(), envelope.target.clone());
    }

    /// 🚦️ The full command pipeline: admit → dedupe → base-resolve/deps → authz → validate →
    /// conflict → execute → WAL append → durability → publish → project →
    /// preview-reconcile → live-query notify → receipt.
    // 🔒️ `batch` is taken by value deliberately, not just because the current body happens not to
    // move it: submitting a batch is the one place in this API where the caller's copy is
    // logically spent (it becomes part of durable history), matching `ArtifactMessage::Submit`'s
    // owned payload on the actor-mailbox side one level up. Keeping ownership here leaves room for
    // a future zero-copy WAL append that moves `batch.envelopes` directly into the write path.
    #[allow(clippy::needless_pass_by_value)]
    pub async fn submit(&mut self, batch: CommandBatch, options: SubmitOptions, now_ms: u64) -> Result<CommandReceipt, DbError> {
        // admit
        check_len(batch.envelopes.len() as u64, self.config.limits.max_batch_commands as u64, "db_artifact::batch_commands")?;
        for envelope in &batch.envelopes {
            if envelope.document_id != self.protocol_document {
                return Err(DbError::InvalidArgument(format!("envelope targets document {:?} but this actor owns {:?}", envelope.document_id, self.protocol_document)));
            }
        }
        let command_id = batch.envelopes.last().expect("CommandBatch::new guarantees at least one envelope").mutation_id.clone();

        for envelope in &batch.envelopes {
            if let Some(applied) = self.applied.get(&envelope.mutation_id.0) {
                if applied.diff != envelope.diff || applied.inverse != envelope.inverse || applied.dependencies != envelope.dependencies {
                    return Err(DbError::Conflict(format!("mutation id {} is already committed with different content", envelope.mutation_id.0)));
                }
            }
        }

        // dedupe (whole-batch, keyed by the batch's designated command_id)
        if let Some(cached) = self.applied_receipts.get(&command_id.0) {
            return Ok(cached.clone());
        }

        // index backlog bound: write what earlier commits queued before this one adds more; refuse
        // transiently while index storage fails, before the WAL holds anything of this batch
        if self.index_backlog.pending() > INDEX_BACKLOG_ENTRIES_MAX {
            if let Err(error) = self.maintain_index().await {
                return Err(DbError::Unavailable(format!("index backlog of {} entries cannot be written: {error}", self.index_backlog.pending())));
            }
        }

        let mut batch_ids: HashSet<String> = HashSet::new();
        let mut written: HashMap<String, protocol::MutationId> = HashMap::new();
        let mut planned: Vec<(&protocol::MutationEnvelope, PlannedEntries)> = Vec::new();
        let mut commands: Vec<Vec<u8>> = Vec::new();
        for envelope in &batch.envelopes {
            if batch_ids.contains(&envelope.mutation_id.0) {
                return Err(DbError::Conflict(format!("mutation id {} appears twice in one batch", envelope.mutation_id.0)));
            }
            if self.applied.contains_key(&envelope.mutation_id.0) {
                batch_ids.insert(envelope.mutation_id.0.clone());
                continue;
            }

            // authz: the `db_security::SecurityGate`, keyed by a permissive principal synthesized from the
            // envelope's own actor (see `ArtifactEngineConfig::security`'s doc). An already-committed
            // envelope (content-checked above) skipped it: it is an idempotent resend.
            let principal = db_security::Principal::new(envelope.actor.clone(), db_security::TenantId::from("default"), vec!["member".to_string()]);
            self.config.security.admit_commands(&principal, &db_security::TenantId::from("default"), &envelope.document_id, &envelope.diff.schema.0, &[(&envelope.actor, &envelope.mutation_id)], now_ms).await?;
            if let Some(refusal) = self.history_transition_refusal(envelope, &batch.envelopes, &batch_ids) {
                return Err(DbError::Rejected { policy: options.policy, worst: semio_framework_diagnostic::Severity::Error, messages: vec![refusal] });
            }

            // 🎯️ W5: `WalRecord::Command`'s bytes are `protocol::encode_envelope`'s binary record now
            // (M-C's "storage AND communication both binary") — `db_sync::replay_sync_state` reads
            // these same WAL records via `decode_command_envelope` (the same binary codec) to serve
            // semio_hub bootstrap/catch-up, so this crate's own write-and-read-back convention below must
            // agree with it byte-for-byte.
            let mut envelope_bytes = Vec::new();
            protocol::encode_envelope(envelope, &mut envelope_bytes);
            check_len(envelope_bytes.len() as u64, self.config.limits.max_command_bytes, "db_artifact::envelope_bytes")?;

            let plan = self.plan_one(envelope, &batch_ids, &mut written).await?;
            batch_ids.insert(envelope.mutation_id.0.clone());
            if let Some(plan) = plan {
                planned.push((envelope, plan));
                commands.push(envelope_bytes);
            }
        }

        if planned.is_empty() {
            // Every envelope in this (re-)submitted batch was already durable individually — a
            // full no-op commit, per-envelope half of the dedupe law (see `plan_one`'s doc).
            let receipt = CommandReceipt { command_id, frontier: self.frontier.clone(), durability: options.durability, conflicts: Vec::new(), state_hash: Some(self.state.content_hash().await?), messages: Vec::new() };
            self.remember_receipt(&receipt);
            return Ok(receipt);
        }

        // outcome step (contract §C9): union this batch's own `db_conflict::ConflictDetector`
        // findings (probed against recent commit history) into graded `protocol::MutationMessage`s,
        // then let `options.policy` decide before touching `self.recent_touches`/`self.outbox`/the
        // WAL at all — a rejected batch must leave every one of those untouched.
        let batch_touches: Vec<db_conflict::CommandTouch> = planned.iter().map(|(envelope, plan)| command_touch(envelope, &plan.touched)).collect();
        let mut messages: Vec<protocol::MutationMessage> = Vec::new();
        for (index, (envelope, _)) in planned.iter().enumerate() {
            for unseen in unseen_concurrent_writes(&self.recent_touches, &batch_touches[..index], &batch_touches[index], envelope) {
                let unseen_target = self.recent_targets.get(&unseen.command_id.0).map(Vec::as_slice).or_else(|| planned[..index].iter().find(|(earlier, _)| earlier.mutation_id == unseen.command_id).map(|(earlier, _)| earlier.target.as_slice())).unwrap_or(&[]);
                messages.extend(grade_concurrent_write(&batch_touches[index], &envelope.target, unseen, unseen_target).await);
            }
        }
        if let Some(worst) = protocol::worst_level(&messages) {
            if options.policy.rejects(worst) {
                return Err(DbError::Rejected { policy: options.policy, worst, messages });
            }
        }

        let head_seq = self.frontier.head_seq.checked_add(planned.len() as u64).ok_or(DbError::LimitExceeded("db_artifact::head_seq"))?;
        let mut records = db_wal_log::WalRecordBatch::new();
        let _ = records.push(db_wal_log::WalRecord::Frontier(Frontier { document: self.document.clone(), head_seq, commit_seq: self.frontier.commit_seq + 1, chain_hash: [0; 32], epoch: self.frontier.epoch }));
        self.wal.preflight_submit(&commands, &records)?;
        let _ = records.close_step()?;
        let wal_facet = self.storage.wal().await;
        self.wal.begin_write(&wal_facet).await?;
        drop(wal_facet);

        let mut touched_all = db_state::TouchedSet::new();
        let mut conflicts_all: Vec<ConflictRecord> = Vec::new();
        let mut applied_actor_seqs: Vec<u64> = Vec::with_capacity(planned.len());
        let mut newly_applied: Vec<(&protocol::MutationEnvelope, db_state::TouchedSet)> = Vec::with_capacity(planned.len());
        for (envelope, plan) in planned {
            self.commit_one(envelope, plan.entries).await?;
            applied_actor_seqs.push(*self.actor_seq.get(&envelope.actor.0).unwrap_or(&0));
            for region in &plan.touched.regions {
                touched_all.record(region.clone());
            }
            conflicts_all.extend(plan.conflicts);
            newly_applied.push((envelope, plan.touched));
        }

        // Bookkeeping for `preview_conflicts`'s real, additive `db_conflict::ConflictDetector`
        // integration (see its own doc) — `submit`'s own returned `ConflictRecord`s stay this
        // crate's original path-granular last-writer detection above (see `🔖️Conflict`'s doc). Only
        // reached once the outcome step above has accepted the batch.
        for (envelope, _) in &newly_applied {
            self.remember_target(envelope);
        }
        for touch in batch_touches {
            remember_recent_touch(&mut self.recent_touches, touch);
        }

        // publish: compute + WAL-append the new frontier in the same transaction as its commands
        let new_frontier = Frontier { document: self.document.clone(), head_seq, commit_seq: self.frontier.commit_seq + 1, chain_hash: self.state.content_hash().await?.0, epoch: self.frontier.epoch };
        let mut records = db_wal_log::WalRecordBatch::new();
        let _ = records.push(db_wal_log::WalRecord::Frontier(new_frontier.clone()));

        // WAL append + durability (ArtifactWal::submit wraps `records` in its own TxBegin/TxCommit)
        let wal_facet = self.storage.wal().await;
        let appended = self.wal.submit(&wal_facet, &commands, &records, options.durability, now_ms).await?;
        drop(wal_facet);
        self.config.security.record_committed(&newly_applied.iter().map(|(envelope, _)| (&envelope.actor, &envelope.mutation_id)).collect::<Vec<_>>(), now_ms);
        let _ = records.close_step()?;
        drop(records);
        self.frontier = new_frontier.clone();

        // publish: index entries join the backlog; the runner writes its full runs after the receipt
        let base_seq = self.frontier.head_seq - newly_applied.len() as u64;
        let queued = ArtifactIndexWatermarks::default();
        for (offset, ((envelope, _), actor_seq)) in newly_applied.iter().zip(&applied_actor_seqs).enumerate() {
            let seq = base_seq + offset as u64 + 1;
            let location = db_index::RecordLocation { segment: appended.segment_index, offset: seq, len: 1 };
            self.index_backlog.queue_command(&queued, seq, location, to_core_actor_id(&envelope.actor).await, *actor_seq);
        }
        self.index_backlog.queue_frontier(&queued, &new_frontier);

        // project: run every registered projection over each newly-applied envelope
        let projection_classes = (self.config.projections)();
        if !projection_classes.is_empty() {
            let index_facet = self.storage.index().await;
            let engine = db_projection::ProjectionEngine::new(&index_facet, self.document.clone(), projection_classes).await?;
            for (offset, (envelope, touched)) in newly_applied.iter().enumerate() {
                engine.apply_envelope(base_seq + offset as u64 + 1, envelope, touched).await?;
            }
        }

        // preview-reconcile
        self.previews.reconcile_with(&db_preview::LandedCommand { frontier: new_frontier.clone(), touched: touched_all }, &db_preview::DbConflictOracle::default());
        self.head_edit_id = Some(command_id.clone());

        // live-query notify
        // 🩹️ R13: was `let _ = self.refresh_live_queries();` — silently dropped the future, so
        // this notify step never actually ran. `.await`ed now; the `Vec<(u64, QueryDiff)>` result
        // is genuinely unneeded here (callers observe diffs via their own subscription poll), so
        // `let _ =` on the awaited, resolved value is the correct fire-and-forget shape.
        let _ = self.refresh_live_queries().await;

        // receipt
        let receipt = CommandReceipt { command_id, frontier: new_frontier, durability: options.durability, conflicts: conflicts_all, state_hash: Some(self.state.content_hash().await?), messages };
        self.remember_receipt(&receipt);
        Ok(receipt)
    }

}
//#endregion 🔖️Engine
//#endregion 🔖️Ids
