//! 🔄️ Reprojection turns and base watch of the instance time-travel turns: the reprojection step, its cancel and resume, the change and dirty marks and the store base watch. See [`super`].

use super::*;
use semio_framework::Fault;

impl<'ctx, A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + 'static, X: InstanceExtension> TimeTravelCtx<'ctx, A, M, X> {
    /// ⏭️ One budget of the waiting reprojection (design §16.6, gap N17), a remote change or this replica's own history
    /// step: progress refreshes the history at most every [`TIME_TRAVEL_PROGRESS_REFRESH_MS`]; the adoption swaps the
    /// document in, delivers `BaseMoved` and refreshes every body; a refused adoption (the store drops the change) keeps its
    /// code on the row — a local step whose replay would leave errors reads [`HISTORY_STEP_BLOCKED_CODE`] — until the next
    /// dispatch dismisses it ([`Self::dismiss_refused_reprojection`]). Nothing left waiting lifts a pause (audit W2A-8): the
    /// next remote change is driven again.
    pub(super) async fn step_reprojection_turn(&mut self, deadline_us: u64, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<(), Fault> {
        let generation = self.stores.store.generation();
        let local = self.stores.store.local_step_pending();
        let stepped = self.stores.store.step_reprojection(Some(deadline_us)).await;
        if local && !self.stores.store.local_step_pending() {
            if let Some(action) = self.time_travel.deferred_history_row.take().filter(|_| stepped.is_ok()) {
                self.tools.record_command(&mut *self.history, &action, ActionKind::History, None, None, Vec::new(), None);
            }
        }
        match stepped {
            Ok(Some(_)) => {
                self.time_travel.reprojection_fault = None;
                let now_ms = semio_framework_job::default_now_ms().unwrap_or(0);
                if now_ms.saturating_sub(self.time_travel.reprojection_refreshed_ms) >= TIME_TRAVEL_PROGRESS_REFRESH_MS {
                    self.time_travel.reprojection_refreshed_ms = now_ms;
                    self.note_time_travel_changed(false, false);
                }
            }
            Ok(None) => {
                self.time_travel.reprojection_fault = None;
                self.time_travel.reprojection_paused = false;
                if self.stores.store.generation() != generation {
                    self.history.cache = None;
                    self.as_dispatch().deliver_base_moved(identity, cx).await?;
                }
                self.note_time_travel_changed(true, true);
            }
            Err(error) => {
                let code = match local && matches!(error, vcs::VcsError::Rejected { .. }) {
                    true => HISTORY_STEP_BLOCKED_CODE.to_string(),
                    false => error.into_fault().code.0,
                };
                self.time_travel.reprojection_fault = Some((code, if local { HistoryReprojectionKind::Step } else { HistoryReprojectionKind::Remote }));
                self.history.cache = None;
                self.note_time_travel_changed(true, true);
            }
        }
        Ok(())
    }

    /// ⏹️ `historyEditCancelReplay` while no session is open: a whole-document load still running is cancelled (the previous
    /// document stays, zero trace); a local history step that still replays is dropped with zero trace
    /// (`discard_local_step`); a waiting remote change's running replay is dropped (the store keeps the change) and driver
    /// turns leave it until `historyEditRerun`.
    pub(super) fn cancel_reprojection_turns(&mut self) -> TimeTravelActionOutcome {
        if let Some(active) = self.ingest.live_document_load().and_then(|operation| self.ingest.document_archive_loads.get_mut(operation)) {
            active.request_terminal(ActiveDocumentArchiveLoadState::Cancelled);
            self.note_time_travel_changed(true, true);
            return TimeTravelActionOutcome::Applied(TimeTravelStage::Inactive);
        }
        if self.stores.store.discard_local_step() {
            self.time_travel.reprojection_fault = None;
            self.time_travel.deferred_history_row = None;
            self.note_time_travel_changed(true, true);
            return TimeTravelActionOutcome::Applied(TimeTravelStage::Inactive);
        }
        if self.time_travel.reprojection_paused || self.stores.store.reprojection_progress().is_none() {
            return TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::Session(TimeTravelRefusal::Illegal));
        }
        self.stores.store.cancel_reprojection();
        self.time_travel.reprojection_paused = true;
        self.note_time_travel_changed(false, false);
        TimeTravelActionOutcome::Applied(TimeTravelStage::Inactive)
    }

    /// ▶️ `historyEditRerun` while no session is open: drives the paused remote replay again from its start.
    pub(super) fn resume_reprojection_turns(&mut self) -> TimeTravelActionOutcome {
        if !self.time_travel.reprojection_paused {
            return TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::Session(TimeTravelRefusal::Illegal));
        }
        self.time_travel.reprojection_paused = false;
        self.note_time_travel_changed(false, false);
        TimeTravelActionOutcome::Applied(TimeTravelStage::Inactive)
    }

    /// 🧾️ A session change: a fresh history cursor, so the next patch is always delivered, carrying every row when
    /// their overlay moved (`rows`); the body refreshes (every window once when the rendered `document` swapped).
    pub(super) fn note_time_travel_changed(&mut self, document: bool, rows: bool) {
        self.history.log_generation += 1;
        if rows {
            self.history.history_dirty_sequences.extend(self.history.command_log.iter().map(|entry| entry.seq));
        }
        self.time_travel.ui_dirty = true;
        self.time_travel.document_dirty |= document;
        self.time_travel.patch_due = true;
    }

    /// 📮️ Hands the owed scope to the typed UI outbox once it has room; until then the flags stay set.
    pub(super) fn flush_time_travel_ui_dirty(&mut self) {
        if !self.time_travel.is_ui_dirty() || self.tools.typed_ui_outbox.len() != 0 {
            return;
        }
        if self.tools.typed_ui_outbox.push(self.time_travel.dirty_scope()).is_ok() {
            self.time_travel.ui_dirty = false;
            self.time_travel.document_dirty = false;
        }
    }

    /// 👀️ Store watch (design §4, driver contract): every change of the content revision of the store the session edits
    /// (never of its local generation alone: a document port attaching or detaching is no base move) reaches the
    /// session as `BaseMoved` with the re-resolved target positions — the open editor's too, so the rows downstream of
    /// the edited mutation stay the pending ones when a remote edit sorts before it; a target that vanished (undone
    /// remotely, document replaced, the edited member gone) exits the session, since nothing it drafted is addressable
    /// anymore. Answers whether the base moved.
    pub(super) async fn watch_time_travel_base(&mut self, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<bool, Fault> {
        let base = self.shared().time_travel_base();
        if base == Some(self.time_travel.session.base) {
            return Ok(false);
        }
        let mut effects = Vec::new();
        match (base, self.shared().time_travel_positions()) {
            (Some(base), Some(positions)) => effects.extend(self.time_travel.session.apply(TimeTravelEvent::BaseMoved { base, positions }).unwrap_or_default()),
            (base, _) => {
                let base = base.unwrap_or(self.time_travel.session.base);
                effects.extend(self.time_travel.session.apply(TimeTravelEvent::Exit).unwrap_or_default());
                effects.extend(self.time_travel.session.apply(TimeTravelEvent::BaseMoved { base, positions: Vec::new() }).unwrap_or_default());
            }
        }
        if let (Some(editor), Some(pending)) = (self.time_travel.editor.as_mut(), self.time_travel.session.pending.as_ref()) {
            editor.position = pending.target.position;
        }
        if self.time_travel.is_active() || !effects.is_empty() {
            self.note_time_travel_changed(true, true);
        }
        let mut dialogs = Vec::new();
        self.perform_time_travel_effects(effects, None, &mut dialogs, identity).await?;
        Ok(true)
    }

}
