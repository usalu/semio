//! ↩️ Supersede, undo, redo and backbone pump: the supersede commands, the replays, the authored supersessions, the undo and redo lanes, the text and binary dispatch, the backbone attachment and the pump. See [`super`].

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
impl<P, Mutation> ArtifactStore<P, Mutation>
where
    P: Clone + ToValue + FromValue + Send + Sync + 'static,
    Mutation: Clone + ToValue + FromValue + super::Mutation<P> + semio_framework_os_spr::ColdMutation<P> + OpBinary + OpText + Send + 'static,
{
    //#region 🔖️Supersede
    /// ✏️ `Supersede`: supersedes typed inputs in every alternative or within one existing alternative
    /// ([`Self::author_supersession`]).
    pub(super) async fn supersede_command(&mut self, scope: Option<String>, inputs: Vec<SupersedeInput<Mutation>>, identity: &mut EntityIdentityAuthority<'_>) -> Result<(), VcsError> {
        let trunk = self.trunk_alternative_id();
        let scope = match scope {
            None => SupersedeScope::Document,
            Some(alternative) if alternative == trunk || self.log.envelope.vcs.alternatives.iter().any(|known| known.id == alternative) => SupersedeScope::Alternative(alternative),
            Some(alternative) => {
                retire_supersede_inputs::<P, Mutation>(inputs);
                return Err(VcsError::UnknownAlternative(alternative));
            }
        };
        let (inputs, target) = self.supersede_inputs(inputs)?;
        self.author_supersession(scope, inputs, target, None, identity).await
    }

    /// 🌿️ `CreateAlternativeWithSupersede`: the pending-edit commit, a `Branch` at the head and the supersession scoped to
    /// the new alternative, authored as one batch ([`Self::author_supersession`]).
    pub(super) async fn create_alternative_with_supersede(&mut self, name: String, inputs: Vec<SupersedeInput<Mutation>>, identity: &mut EntityIdentityAuthority<'_>) -> Result<(), VcsError> {
        let (inputs, target) = self.supersede_inputs(inputs)?;
        self.author_supersession(SupersedeScope::NewAlternative(name), inputs, target, None, identity).await
    }

    /// 🔚️ Finalizes a time-travel session without replaying again: `finished` is the completed Report replay of the
    /// session's drafts ([`Self::begin_report_replay`]). Refused with `Stale` when this store's content moved since the replay
    /// began (its content revision differs; the session replays again — a port rebinding moves no event and keeps the replay) and with `Rejected { policy: Normal }` when its report blocks finalizing; otherwise the
    /// drafts are authored as an overwrite or a new alternative and installed atomically with the replay's projection,
    /// rebased inverses and messages in one outbound batch — the same install the `Supersede` command takes.
    pub async fn commit_finished_replay(&mut self, finished: EditReplayResult<P, Mutation>, finalization: HistoryFinalization, identity: &mut EntityIdentityAuthority<'_>) -> Result<CommandReceipt, VcsError>
    where
        P: Sync,
    {
        let mut slot = Some(finished);
        let outcome = self.commit_slotted_replay(&mut slot, finalization, identity).await;
        if let Some(unused) = slot {
            let _ = self.retire_replay_result(unused);
        }
        outcome
    }

    /// 🔚️ [`Self::commit_finished_replay`] over a slot that keeps the replay with the caller until the supersession takes it.
    async fn commit_slotted_replay(&mut self, slot: &mut Option<EditReplayResult<P, Mutation>>, finalization: HistoryFinalization, identity: &mut EntityIdentityAuthority<'_>) -> Result<CommandReceipt, VcsError>
    where
        P: Sync,
    {
        self.ensure_durable_group_idle()?;
        if let Some(open) = self.log.envelope.open_transaction.as_ref() {
            return Err(VcsError::TransactionOpen { transaction_id: open.transaction.id.clone() });
        }
        if self.local_step_pending() {
            return Err(VcsError::HistoryReplaying);
        }
        let finished = slot.as_ref().expect("a committing replay stays in its slot until its supersession takes it");
        if finished.revision != self.cursor.content_revision {
            return Err(VcsError::Stale { expected_generation: finished.generation, generation: self.cursor.generation });
        }
        if finished.drafts.is_empty() {
            return Err(VcsError::ValidationFailed("a finished replay without drafts supersedes nothing".into()));
        }
        let report = self.replay_report(finished)?;
        if report.blocks_finalize() {
            return Err(VcsError::Rejected { policy: semio_framework_os_spr::MergePolicy::Normal, messages: report.outcomes.into_iter().flat_map(|outcome| outcome.messages).collect() });
        }
        let (inputs, target) = self.draft_inputs(&finished.drafts)?;
        let scope = match finalization {
            HistoryFinalization::Overwrite => SupersedeScope::Document,
            HistoryFinalization::Scope { alternative_id } if alternative_id == self.trunk_alternative_id() || self.log.envelope.vcs.alternatives.iter().any(|known| known.id == alternative_id) => SupersedeScope::Alternative(alternative_id),
            HistoryFinalization::Scope { alternative_id } => return Err(VcsError::UnknownAlternative(alternative_id)),
            HistoryFinalization::Alternative { name } => SupersedeScope::NewAlternative(name),
        };
        self.replace_pending_report_retained(PendingCommandReport::default())?;
        self.author_supersession(scope, inputs, target, slot.take(), identity).await?;
        self.cursor.last_projection_cause = Some(ArtifactProjectionCause::Replay);
        self.flush_outbound().await?;
        let (messages, worst) = self.replay.pending_report.take_messages_and_worst();
        Ok(CommandReceipt { edit_ids: Vec::new(), generation: self.generation(), messages, worst })
    }

    /// ✍️ The one authoring path of every supersession: builds its transitions (`Document`/`Alternative`: one `Supersede`;
    /// `NewAlternative`: the pending-edit commit, a `Branch` at the head and the scoped `Supersede`), takes `replayed` when it
    /// is the Report replay of exactly the resulting log and dry-runs it otherwise (under [`Self::defer_local_replays`] the
    /// dry run is a local step stepped across turns), refuses a report that blocks finalizing (`Error`/`Fatal`, the hub
    /// check-in strictness) with `Rejected { policy: Normal }` and every replay message, and installs everything atomically
    /// without replaying again.
    async fn author_supersession(&mut self, scope: SupersedeScope, inputs: Vec<protocol::SupersededInput>, target: Vec<String>, replayed: Option<EditReplayResult<P, Mutation>>, identity: &mut EntityIdentityAuthority<'_>) -> Result<(), VcsError> {
        let mut slot = replayed;
        let outcome = self.author_slotted_supersession(scope, inputs, target, &mut slot, identity).await;
        if let Some(unused) = slot {
            let _ = self.retire_replay_result(unused);
        }
        outcome
    }

    /// ✍️ [`Self::author_supersession`] over a slot that keeps `replayed` with the caller until the resulting log takes it.
    async fn author_slotted_supersession(&mut self, scope: SupersedeScope, inputs: Vec<protocol::SupersededInput>, target: Vec<String>, replayed: &mut Option<EditReplayResult<P, Mutation>>, identity: &mut EntityIdentityAuthority<'_>) -> Result<(), VcsError> {
        let mut clock = self.log.clock;
        let mut envelopes = Vec::with_capacity(3);
        let saved_line = self.log.envelope.active_alternative_id.clone();
        let saved_checkpoint = self.log.envelope.viewer_checkpoint_id.clone();
        let mut viewing_new = false;
        let scope = match scope {
            SupersedeScope::Document => None,
            SupersedeScope::Alternative(alternative_id) => Some(alternative_id),
            SupersedeScope::NewAlternative(name) => match self.author_head_alternative(name, &mut clock, &mut envelopes, identity).await {
                Ok(alternative_id) => {
                    self.log.envelope.active_alternative_id = Some(alternative_id.clone());
                    self.log.envelope.viewer_checkpoint_id = None;
                    viewing_new = true;
                    Some(alternative_id)
                }
                Err(error) => return Err(error),
            },
        };
        let restore_head = |store: &mut Self| {
            if viewing_new {
                store.log.envelope.active_alternative_id = saved_line.clone();
                store.log.envelope.viewer_checkpoint_id = saved_checkpoint.clone();
            }
        };
        let supersede = protocol::TransitionSupersede { scope, inputs };
        if let Err(error) = supersede.validate() {
            restore_head(self);
            return Err(VcsError::ValidationFailed(error.to_string()));
        }
        let dependencies = supersede.targets();
        clock.tick(now_ms());
        envelopes.push(self.transition_envelope(&semio_framework_os_spr::HistoryTransition::Supersede(supersede), dependencies, target, clock));
        let fold = match self.prospective_fold(&envelopes) {
            Ok(fold) => fold,
            Err(error) => {
                restore_head(self);
                return Err(error);
            }
        };
        let result = match replayed.take() {
            Some(result) if result.order == fold.applied && same_effective_inputs(&result.supersessions, &fold.supersessions) => result,
            other if self.replay.local_replay_budget.is_some() => {
                *replayed = other;
                drop(fold);
                let head = self.viewer_head_now();
                restore_head(self);
                return self.defer_local_step(PendingLocalStep { envelopes, head, finalizing: true }, clock).await.map(|_| ());
            }
            other => {
                *replayed = other;
                match self.dry_run(&fold) {
                    Ok(result) => result,
                    Err(error) => {
                        restore_head(self);
                        return Err(error);
                    }
                }
            }
        };
        let report = match self.replay_report(&result) {
            Ok(report) => report,
            Err(error) => {
                let _ = self.retire_replay_result(result);
                restore_head(self);
                return Err(error);
            }
        };
        if report.blocks_finalize() {
            let _ = self.retire_replay_result(result);
            restore_head(self);
            return Err(VcsError::Rejected { policy: semio_framework_os_spr::MergePolicy::Normal, messages: report.outcomes.iter().flat_map(|outcome| outcome.messages.iter().cloned()).collect() });
        }
        if let Err(error) = self.install_transitions(envelopes, clock, Some(result)).await {
            restore_head(self);
            return Err(error);
        }
        self.record_replay_report(&report);
        Ok(())
    }

    //#endregion 🔖️Supersede
    //#region 🔖️HistoryLane
    /// ⏪️ Shared body of `Undo`/`UndoWithPolicy`: picks the edit the policy names and reverts it.
    pub(super) async fn undo_with_policy(&mut self, policy: UndoPolicy, semantic_command: Option<Box<ArtifactCommand<Mutation>>>, identity: &mut EntityIdentityAuthority<'_>) -> Result<(), VcsError>
    where
        P:  Clone + Sync,
     Mutation: ColdMutation<P> ,
    {
        match policy {
            UndoPolicy::ExactBaseOnly => {
                let position = self.log.applied_edit_ids.iter().rposition(|id| self.edit_lane(id) == HistoryLane::Document).ok_or(VcsError::NothingToUndo)?;
                self.undo_lane_position(position).await
            }
            UndoPolicy::TransformAgainstConcurrent => {
                let position = self.log.applied_edit_ids.iter().rposition(|id| self.edit_is_local(id) && self.edit_lane(id) == HistoryLane::Document).ok_or(VcsError::NothingToUndo)?;
                self.undo_lane_position(position).await
            }
            UndoPolicy::SemanticUndo | UndoPolicy::CompensatingAction => {
                let command = semantic_command.ok_or_else(|| VcsError::Backbone("semantic undo requires compensating command".into()))?;
                Box::pin(self.dispatch_inner(*command, identity)).await
            }
        }
    }

    /// ⏪️ Shared tail of every undo path: the applied edit at `position` must be this actor's
    /// own; it is withdrawn by a `Revert` transition and the projection follows the event log.
    pub(super) async fn undo_lane_position(&mut self, position: usize) -> Result<(), VcsError> {
        let edit_id = self.log.applied_edit_ids[position].clone();
        if !self.edit_is_local(&edit_id) {
            return Err(VcsError::ForeignEdit(edit_id));
        }
        let mutation_ids = self.edit_mutation_ids(&edit_id)?;
        self.commit_transition(semio_framework_os_spr::HistoryTransition::Revert { mutation_ids: mutation_ids.clone() }, mutation_ids).await
    }

    /// ⏩️ Shared tail of every redo path: the redo entry at `position` is restored by a
    /// `Reinstate` transition, landing back at its own HLC position in the applied order.
    pub(super) async fn redo_lane_position(&mut self, position: usize) -> Result<(), VcsError> {
        let edit_id = self.log.redo_edit_ids[position].clone();
        let mutation_ids = self.edit_mutation_ids(&edit_id)?;
        self.commit_transition(semio_framework_os_spr::HistoryTransition::Reinstate { mutation_ids: mutation_ids.clone() }, mutation_ids).await
    }

    /// 🚩️ Commits every applied-but-uncommitted edit as one change on a new checkpoint.
    pub(super) async fn commit_pending_checkpoint(&mut self, message: Option<String>, authors: Vec<Author>, identity: &mut EntityIdentityAuthority<'_>) -> Result<(), VcsError> {
        let (transition, dependencies, _) = self.pending_checkpoint_transition(message, authors, identity).await?;
        self.commit_transition(transition, dependencies).await
    }

    //#endregion 🔖️HistoryLane
    /// 🕹️ Parses `command_text` via [`parse_command`] and dispatches it — the op-line
    /// textual entry point (op-efficient one-line-per-structural-field commands, indented op
    /// lines for `Apply`/`AppendTransaction`).
    pub async fn dispatch_text(&mut self, command_text: &str, identity: &mut EntityIdentityAuthority<'_>) -> Result<CommandReceipt, VcsError>
    where
        Mutation: OpText,
        P:  Clone + Sync,
     Mutation: ColdMutation<P> ,
    {
        let command = parse_command::<P, Mutation>(command_text).await.map_err(|error| VcsError::Deserialize(error.to_string()))?;
        self.dispatch(command, identity).await
    }

    /// 🕹️ Decodes `command_bytes` via [`decode_command`] and dispatches it — the binary
    /// entry point used for both communication (backbone/semio_hub) and storage (`.spr`).
    pub async fn dispatch_binary(&mut self, command_bytes: &[u8], identity: &mut EntityIdentityAuthority<'_>) -> Result<CommandReceipt, VcsError>
    where
        Mutation: OpBinary,
        P:  Clone + Sync,
     Mutation: ColdMutation<P> ,
    {
        let command = ArtifactCommand::<Mutation>::decode_command::<P>(command_bytes).map_err(|error| VcsError::Deserialize(error.to_string()))?;
        self.dispatch(command, identity).await
    }

    /// 🔗️ Attaches a backbone channel: ingests whatever events the other end already queued,
    /// then announces this replica's genesis and its complete event log. The other end folds the
    /// events it lacks and ignores the ones it holds — no snapshot ever crosses the channel.
    pub async fn attach_backbone(&mut self, backbone: Backbones) -> Result<(), VcsError> {
        self.ensure_durable_group_idle()?;
        self.log.envelope.backbone = Some(backbone.descriptor().await);
        self.replace_backbone_retained(Some(backbone))?;
        self.seed_known_events()?;
        self.pump().await?;
        self.announce_history().await?;
        self.bump()?;
        Ok(())
    }

    /// 📥️ Folds an encoded envelope payload a composing parent routed to THIS member's lane.
    /// Inbound twin of {@link announce_tail_edit_payload}; each envelope goes through the same
    /// {@link ingest_remote} gate a parent-lane message does, so causal order and conflict
    /// quarantining are identical on both lanes.
    pub async fn ingest_remote_payload(&mut self, envelopes: &[u8]) -> Result<Vec<semio_framework_os_spr::MergeReport>, VcsError> where P: Clone, Mutation: ColdMutation<P> {
        let envelopes = semio_framework_os_spr::decode_envelopes(envelopes).map_err(|error| VcsError::Deserialize(error.to_string()))?;
        let mut reports = Vec::with_capacity(envelopes.len());
        for envelope in envelopes {
            reports.push(self.ingest_remote(envelope).await?);
        }
        Ok(reports)
    }

    pub async fn attach_hot_backbone(&mut self, backbone: Backbones) -> Result<(), VcsError> {
        self.ensure_durable_group_idle()?;
        self.log.envelope.backbone = Some(backbone.descriptor().await);
        self.replace_backbone_retained(Some(backbone))?;
        self.pump().await?;
        self.bump()?;
        Ok(())
    }

    /// 🔗️ Resolves a backbone URI and attaches it. Only available inside the wasm sandbox,
    /// where every scheme forwards to the host over the injected {@link BackboneChannelPort} (a pure
    /// queue). On native targets, callers attach an explicit `Backbones` value via
    /// {@link attach_backbone} — the `framework/sync` actor layer owns all IO-performing endpoints.
    #[cfg(target_arch = "wasm32")]
    pub async fn attach_backbone_uri(&mut self, uri: &str) -> Result<(), VcsError> {
        self.attach_backbone(resolve_backbone(uri).await?).await
    }

    /// 📡️ Drains inbound backbone messages into the edit timeline. Safe to call anytime;
    /// `dispatch` already calls this before every command.
    pub async fn tick(&mut self) -> Result<bool, VcsError> {
        self.ensure_durable_group_idle()?;
        let pumped = self.pump().await?;
        let pending = self.replay.pending_reprojection.is_some();
        let progress = self.step_reprojection(None).await?;
        Ok(pumped || (pending && progress.is_none()))
    }

    pub async fn tick_backbone_reports(&mut self) -> Result<Vec<semio_framework_os_spr::MergeReport>, VcsError> {
        self.ensure_durable_group_idle()?;
        self.pump_with_reports().await.map(|(_, reports)| reports)
    }

    /// 🛟️ The sole public remote write gate ([`Self::ingest_remote_merge`]). While a tool transaction is open its
    /// unannounced edit is lifted off the applied tail for the merge and folded back onto the merged head after it, so a
    /// remote edit never reorders, quarantines or degrades it and it stays the tail its commit announces.
    pub async fn ingest_remote(&mut self, envelope: semio_framework_os_spr::MutationEnvelope) -> Result<semio_framework_os_spr::MergeReport, VcsError> where P: Clone, Mutation: ColdMutation<P> {
        let Some((open, edit)) = self.lift_open_transaction().await? else {
            return self.ingest_remote_merge(envelope).await;
        };
        let report = self.ingest_remote_merge(envelope).await;
        self.restore_open_transaction(open, edit).await?;
        report
    }

    /// 🪞️ Delegates to [`semio_framework_os_store_engine_bundle::ingest_remote_merge`]: feeds one remote envelope through the causal DAG and merges it into the edit timeline.
    #[inline]
    async fn ingest_remote_merge(&mut self, envelope: semio_framework_os_spr::MutationEnvelope) -> Result<semio_framework_os_spr::MergeReport, VcsError> where P: Clone, Mutation: ColdMutation<P> {
        semio_framework_os_store_engine_bundle::ingest_remote_merge(&mut self.parts(), envelope).await
    }

    /// 📥️ Pumps every queued inbound message from the attached backbone into the timeline.
    pub async fn pump(&mut self) -> Result<bool, VcsError> {
        self.pump_with_reports().await.map(|(changed, _)| changed)
    }

    /// 🪆️ A member lane names a document this store does not own; only the composing app can
    /// resolve `(slot, child_id)` to a live member, so the message waits for its owner.
    ///
    /// A store never consumes acks (they flow store→actor); drain and ignore any that echo back.
    async fn pump_with_reports(&mut self) -> Result<(bool, Vec<semio_framework_os_spr::MergeReport>), VcsError> where P: Clone, Mutation: ColdMutation<P> {
        let Some(mut backbone) = self.wire.backbone.take() else {
            return Ok((false, Vec::new()));
        };
        let received = backbone.receive().await;
        self.replace_backbone_retained(Some(backbone))?;
        let messages = received?;
        if messages.is_empty() {
            return Ok((false, Vec::new()));
        }
        let mut acked_op_ids: Vec<String> = Vec::new();
        let mut reports = Vec::new();
        for message in messages {
            match message {
                BackboneMessage::Genesis { pack } => self.verify_genesis(&pack)?,
                BackboneMessage::Mutations { envelopes } => {
                    let envelopes = semio_framework_os_spr::decode_envelopes(&envelopes).map_err(|error| VcsError::Deserialize(error.to_string()))?;
                    let op_ids: Vec<String> = envelopes.iter().map(|envelope| envelope.mutation_id.0.clone()).collect();
                    for envelope in envelopes {
                        reports.push(self.ingest_remote(envelope).await?);
                    }
                    acked_op_ids.extend(op_ids);
                }
                member @ BackboneMessage::Member { .. } => self.wire.member_inbox.push_back(member),
                BackboneMessage::Ack { .. } => {}
                BackboneMessage::Retract { mutation_ids } => {
                    self.retract_transitions(&mutation_ids).await?;
                }
            }
        }
        if !acked_op_ids.is_empty() {
            if let Some(mut backbone) = self.wire.backbone.take() {
                let result = backbone.send(BackboneMessage::Ack { op_ids: acked_op_ids }).await;
                self.replace_backbone_retained(Some(backbone))?;
                result?;
            }
        }
        Ok((true, reports))
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
