//! ⏳️ Deferred reprojection and supersede reports: the deferred remote and local replays, the reprojection drive, the pending adoption, the replay reports and the merge report absorption. See [`super`].

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
    //#region 🔖️DeferredReprojection
    /// 🐢️ Steps the Report replay a remote history change needs — a supersession, undo or redo another replica authored —
    /// one `budget` per turn ([`ReplayTurnBudget`]: a wall deadline first, an operation cap second;
    /// [`Self::step_reprojection`], also every [`Self::tick`]) instead of inside the ingest that admitted it, so a long
    /// downstream replay never freezes this replica. Until the change is adopted the replica shows the history before it,
    /// and the adoption is exactly the one an ingest without a budget makes. `None` replays inside the ingest.
    pub fn defer_remote_replays(&mut self, budget: Option<ReplayTurnBudget>) {
        self.replay_budget = budget.filter(|budget| budget.operations > 0 && budget.wall_us > 0);
    }

    /// 🐌️ Steps the Report replay a local history step needs — an interior undo or redo, a checkout or alternative switch
    /// away from the applied tail, a supersession (finalize) authored without a finished replay (design §16.6) — one
    /// `budget` per turn ([`ReplayTurnBudget`]) instead of inside its dispatch, which replays the first turn and answers. Until
    /// the step is adopted ([`Self::step_reprojection`], also every [`Self::tick`]) the replica shows the history before it,
    /// nothing of the step is recorded or announced, every further history step is refused
    /// ([`VcsError::HistoryReplaying`]) and [`Self::discard_local_step`] drops it with zero trace; edits and remote changes
    /// meanwhile restart its replay. The adoption is exactly the one an undeferred dispatch makes, a supersession whose
    /// report blocks finalizing included (refused with `Rejected { policy: Normal }`). Steps that need no replay adopt
    /// inside their dispatch. `None` replays inside the dispatch.
    pub fn defer_local_replays(&mut self, budget: Option<ReplayTurnBudget>) {
        self.local_replay_budget = budget.filter(|budget| budget.operations > 0 && budget.wall_us > 0);
    }

    /// ⏳️ Whether a local history step still replays ([`Self::defer_local_replays`]).
    pub fn local_step_pending(&self) -> bool {
        self.pending_reprojection.as_ref().is_some_and(|pending| pending.local.is_some())
    }

    /// 🗑️ Drops the local history step that still replays ([`Self::defer_local_replays`]): nothing of it was recorded,
    /// announced or shown, so the store is exactly as before its dispatch; a waiting remote change keeps waiting and
    /// replays again from its start. Answers whether a step was waiting.
    pub fn discard_local_step(&mut self, store_owners: &mut StoreOwners<P, Mutation>) -> bool {
        let Some(mut pending) = self.pending_reprojection.take() else {
            return false;
        };
        let discarded = pending.local.take().is_some();
        if discarded {
            let _ = store_owners.retire_pending_replay(pending.replay.take());
        }
        self.pending_reprojection = (!pending.transitions.is_empty() || pending.local.is_some()).then_some(pending);
        discarded
    }

    /// 📶️ The progress of the deferred reprojection — a remote change, a local step or both (`done` 0 until its replay
    /// starts, and again once an edit or another change made its replay restart), `None` when nothing waits.
    pub fn reprojection_progress(&self, store_cursor: &StoreCursor<P, Mutation>) -> Option<ReplayProgress> {
        self.pending_reprojection.as_ref().map(|pending| pending.replay.as_ref().filter(|(revision, _)| *revision == store_cursor.content_revision).map_or_else(ReplayProgress::default, |(_, replay)| replay.progress()))
    }

    /// 📈️ Reports cumulative semantic work from the revision-fenced reprojection owner.
    pub fn reprojection_operation_preparation_progress(&self, store_cursor: &StoreCursor<P, Mutation>) -> Option<ArtifactReplayPreparationProgress> {
        self.pending_reprojection.as_ref().map(|pending| pending.replay.as_ref().filter(|(revision, _)| *revision == store_cursor.content_revision).map_or_else(ArtifactReplayPreparationProgress::default, |(_, replay)| replay.operation_preparation_progress()))
    }

    /// ✋️ Drops the running replay of the deferred reprojection: the store is untouched, the change stays admitted (a
    /// local step too — [`Self::discard_local_step`] drops it) and the next step replays it from the start. Answers whether a
    /// replay was running.
    pub fn cancel_reprojection(&mut self, store_owners: &mut StoreOwners<P, Mutation>) -> bool {
        let Some(replay) = self.pending_reprojection.as_mut().and_then(|pending| pending.replay.take()) else {
            return false;
        };
        let _ = store_owners.retire_pending_replay(Some(replay));
        true
    }

    /// ⏭️ Advances the deferred reprojection by one turn — until `deadline_us` on the budget's clock when the runtime names
    /// its turn deadline, else one [`ReplayTurnBudget`] — and adopts it once its replay finished, announcing a local step's
    /// transitions. Answers its progress while it still waits, `None` once nothing waits.
    pub async fn step_reprojection(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_wire: &mut StoreWire<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>, store_live: &mut StoreLive<P, Mutation>, deadline_us: Option<u64>) -> Result<Option<ReplayProgress>, VcsError> {
        match self.advance_reprojection(store_log, store_owners, store_wire, store_cursor, store_live, deadline_us).await? {
            Some(ReprojectionAdvance::Pending(progress)) => Ok(Some(progress)),
            Some(ReprojectionAdvance::Adopted(_)) => {
                self.bump(store_log, store_owners, store_cursor, store_live)?;
                store_cursor.last_projection_cause = Some(ArtifactProjectionCause::Replay);
                Ok(None)
            }
            None => Ok(None),
        }
    }

    /// 🛗️ Takes a local history step whose Report replay must not run inside its dispatch ([`Self::defer_local_replays`]):
    /// joins it to the waiting change (with any remote transitions already waiting), advances the clock past its
    /// transitions, steps one budget and adopts everything once the replay finished — here or in a later turn. Answers the
    /// step back when it is adopted synchronously instead: local replays are not deferred, or a plain step needs no replay.
    pub async fn defer_local_step(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_wire: &mut StoreWire<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>, store_live: &mut StoreLive<P, Mutation>, step: PendingLocalStep, clock: HybridLogicalTimestamp) -> Result<Option<PendingLocalStep>, VcsError> {
        if self.local_replay_budget.is_none() {
            return Ok(Some(step));
        }
        if !step.finalizing {
            let fold = store_log.prospective_fold_at(&step.envelopes, &store_log.viewer_head(&step.head))?;
            if fold.supersessions == *store_live.supersessions && store_log.applied_edit_ids.starts_with(&fold.applied) {
                return Ok(Some(step));
            }
        }
        let mut pending = self.pending_reprojection.take().unwrap_or_else(PendingReprojection::empty);
        if pending.local.is_some() {
            self.pending_reprojection = Some(pending);
            return Err(VcsError::HistoryReplaying);
        }
        store_owners.retire_pending_replay(pending.replay.take())?;
        pending.local = Some(step);
        self.pending_reprojection = Some(pending);
        store_log.clock = clock;
        match self.advance_reprojection(store_log, store_owners, store_wire, store_cursor, store_live, None).await? {
            Some(ReprojectionAdvance::Adopted(_)) => {
                self.bump(store_log, store_owners, store_cursor, store_live)?;
                store_cursor.last_projection_cause = Some(ArtifactProjectionCause::Replay);
                Ok(None)
            }
            Some(ReprojectionAdvance::Pending(_)) | None => Ok(None),
        }
    }

    /// 🧮️ One turn of the deferred reprojection (until `deadline_us`, else one [`ReplayTurnBudget`]): (re)starts its Report
    /// replay when the history it started from changed, steps it, and adopts the change with the finished replay; a change
    /// that needs no replay is adopted at once. A refused adoption drops the change — a refused local step only, its remote
    /// transitions wait again — and answers the refusal.
    pub(super) async fn advance_reprojection(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_wire: &mut StoreWire<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>, store_live: &mut StoreLive<P, Mutation>, deadline_us: Option<u64>) -> Result<Option<ReprojectionAdvance>, VcsError> {
        let Some(mut pending) = self.pending_reprojection.take() else {
            return Ok(None);
        };
        if pending.replay.as_ref().is_none_or(|(revision, _)| *revision != store_cursor.content_revision) {
            let _ = store_owners.retire_pending_replay(pending.replay.take());
            let started = store_log.pending_fold(&pending).and_then(|fold| {
                if fold.supersessions == *store_live.supersessions && store_log.applied_edit_ids.starts_with(&fold.applied) {
                    return Ok(None);
                }
                let (from, horizon) = store_live.replay_window(store_log, &fold.applied, &fold.supersessions)?;
                store_live.report_replay(store_log, store_cursor, &fold.applied, from, fold.supersessions.clone(), horizon, StoreLive::<P, Mutation>::prefix_stride(fold.applied.len())).map(Some)
            });
            match started {
                Ok(Some(replay)) => pending.replay = Some((store_cursor.content_revision, replay)),
                Ok(None) => return self.adopt_pending(store_log, store_owners, store_wire, store_cursor, store_live, pending, None).await.map(|replayed| Some(ReprojectionAdvance::Adopted(replayed))),
                Err(error) => {
                    self.refuse_local_step(store_owners, pending);
                    return Err(error);
                }
            }
        }
        let budget = pending.local.as_ref().and(self.local_replay_budget).or(self.replay_budget).unwrap_or(ReplayTurnBudget::operations(usize::MAX));
        let deadline = budget.turn_deadline(deadline_us);
        let (_, replay) = pending.replay.as_mut().expect("a started deferred replay");
        let step = replay.step_operations(&store_log.envelope.vcs.edits, budget.operations, &mut || budget.turn_ends(0, deadline));
        match step {
            Ok(ReplayStep::Pending(progress)) => {
                self.pending_reprojection = Some(pending);
                Ok(Some(ReprojectionAdvance::Pending(progress)))
            }
            Ok(ReplayStep::Finished(_)) => {
                let (_, mut replay) = pending.replay.take().expect("a finished deferred replay");
                match replay.finish() {
                    Ok(result) => self.adopt_pending(store_log, store_owners, store_wire, store_cursor, store_live, pending, Some(result)).await.map(|replayed| Some(ReprojectionAdvance::Adopted(replayed))),
                    Err(error) => {
                        let _ = store_owners.retire_raw_replay(replay);
                        self.refuse_local_step(store_owners, pending);
                        Err(error)
                    }
                }
            }
            Err(error) => {
                self.refuse_local_step(store_owners, pending);
                Err(error)
            }
        }
    }

    /// ⛔️ Drops a refused waiting change: its local step is refused with nothing of it recorded, and its remote transitions
    /// wait again (their replay restarts) — or, refused themselves, are dropped.
    fn refuse_local_step(&mut self, store_owners: &mut StoreOwners<P, Mutation>, mut pending: PendingReprojection<P, Mutation>) {
        let _ = store_owners.retire_pending_replay(pending.replay.take());
        if pending.local.take().is_some() && !pending.transitions.is_empty() {
            self.pending_reprojection = Some(pending);
        }
    }

    /// 🧶️ Adopts a waiting change with its finished replay (`None`: it needs none): its remote transitions alone through
    /// [`Self::adopt_remote_transitions`]; with a local step, both are recorded in the log, this replica moves to the step's
    /// head, the projection follows and the step's transitions are announced. A finalizing step whose report blocks is
    /// refused (`Rejected { policy: Normal }`, every replay message) with nothing of it recorded while its remote transitions
    /// wait again. Atomic: a refused adoption keeps the log and the head.
    async fn adopt_pending(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_wire: &mut StoreWire<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>, store_live: &mut StoreLive<P, Mutation>, pending: PendingReprojection<P, Mutation>, replayed: Option<EditReplayResult<P, Mutation>>) -> Result<Vec<semio_framework_os_spr::EditMessages>, VcsError> where P: Clone, Mutation: ColdMutation<P> {
        let PendingReprojection { transitions, local, replay } = pending;
        store_owners.retire_pending_replay(replay)?;
        let Some(local) = local else {
            return store_live.adopt_remote_transitions(store_log, store_owners, store_cursor, transitions, replayed).await;
        };
        let report = match replayed.as_ref().filter(|_| local.finalizing).map(|result| store_live.replay_report(store_log, store_wire, store_cursor, result)).transpose() {
            Ok(report) => report,
            Err(error) => {
                if let Some(replayed) = replayed {
                    let _ = store_owners.retire_replay_result(replayed);
                }
                self.refuse_local_step(store_owners, PendingReprojection { transitions, local: Some(local), replay: None });
                return Err(error);
            }
        };
        if let Some(report) = report.as_ref().filter(|report| report.blocks_finalize()) {
            if let Some(replayed) = replayed {
                let _ = store_owners.retire_replay_result(replayed);
            }
            let messages = report.outcomes.iter().flat_map(|outcome| outcome.messages.iter().cloned()).collect();
            self.refuse_local_step(store_owners, PendingReprojection { transitions, local: Some(local), replay: None });
            return Err(VcsError::Rejected { policy: semio_framework_os_spr::MergePolicy::Normal, messages });
        }
        let PendingLocalStep { envelopes, head, finalizing } = local;
        let saved = store_log.viewer_head_now();
        let remote: Vec<MutationId> = transitions.iter().map(|envelope| envelope.mutation_id.clone()).collect();
        let authored: Vec<MutationId> = envelopes.iter().map(|envelope| envelope.mutation_id.clone()).collect();
        for envelope in transitions.into_iter().chain(envelopes.iter().cloned()) {
            store_live.insert_transition(store_log, envelope);
        }
        (store_log.envelope.active_alternative_id, store_log.envelope.viewer_checkpoint_id) = head.clone();
        let projected = match replayed {
            Some(result) => store_live.reproject_replayed(store_log, store_owners, store_cursor, result),
            None => store_live.reproject(store_log, store_owners, store_cursor).await,
        };
        let replayed = match projected {
            Ok(replayed) => replayed,
            Err(error) => {
                let waiting: Vec<semio_framework_os_spr::MutationEnvelope> = store_log.envelope.transitions.extract_if(.., |known| remote.contains(&known.mutation_id)).collect();
                store_log.envelope.transitions.retain(|known| !authored.contains(&known.mutation_id));
                (store_log.envelope.active_alternative_id, store_log.envelope.viewer_checkpoint_id) = saved;
                self.refuse_local_step(store_owners, PendingReprojection { transitions: waiting, local: Some(PendingLocalStep { envelopes, head, finalizing }), replay: None });
                return Err(error);
            }
        };
        for envelope in &envelopes {
            store_log.dag.seed_applied(envelope.mutation_id.clone()).map_err(|error| VcsError::ValidationFailed(error.to_string()))?;
        }
        if let Some(report) = report {
            self.record_replay_report(&report);
        }
        self.pending_report.author_outbound(envelopes);
        self.flush_outbound(store_owners, store_wire).await?;
        Ok(replayed)
    }

    //#endregion 🔖️DeferredReprojection
    //#region 🔖️Supersede
    /// 📮️ Hands a supersession's replay messages to the dispatch receipt, grouped by edit.
    pub fn record_replay_report(&mut self, report: &protocol::ReplayReport) {
        let mut grouped: Vec<semio_framework_os_spr::EditMessages> = Vec::new();
        for outcome in report.outcomes.iter().filter(|outcome| !outcome.messages.is_empty()) {
            match grouped.last_mut() {
                Some(entry) if entry.edit_id == outcome.edit_id => entry.messages.extend(outcome.messages.iter().cloned()),
                _ => grouped.push(semio_framework_os_spr::EditMessages { edit_id: outcome.edit_id.clone(), messages: outcome.messages.clone() }),
            }
        }
        self.pending_report.record_replay(grouped, report.worst);
    }

    //#endregion 🔖️Supersede
    /// 📨️ Copies a `MergeReport`'s messages/worst level into `pending_report` for `dispatch` to
    /// hand back as the resulting `CommandReceipt` — shared by the `IngestRemote`/`ResolveConflict`
    /// arms above.
    pub fn absorb_merge_report(&mut self, report: &semio_framework_os_spr::MergeReport) {
        self.pending_report.declare_edit_ids_unless_declared(|| if report.accepted { report.replayed.iter().map(|edit_messages| edit_messages.edit_id.clone()).collect() } else { Vec::new() });
        self.pending_report.record_replay(report.replayed.clone(), report.worst);
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
