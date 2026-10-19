//! 📒️ Driver and ledger of the tool run ledger: the tick receipt, the driver, the ledger with its trace window and the ledger operations over the selected run. See [`super`].

use super::*;
use semio_framework::kernel::UiDirtyScope;
use std::collections::BTreeSet;
use semio_framework_os_kernel::os_store::ArtifactStore;
use semio_framework::Fault;
use semio_framework_plugin_decl_views::child_view::ChildContentView;

//#region 🔖️Ledger
/// 📬️ What one applied tick changed.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ToolRunTickReceipt {
    pub stale: bool,
    pub appended: u32,
    pub retracted: u32,
    pub evicted: u32,
    pub capped: bool,
}

/// 🧭️ Driver watch state and turn budget (§3.3): settings publications the store generations cannot see.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ToolRunDriver {
    pub turn_wall_us: u64,
    pub window_config_publications: u64,
}

impl Default for ToolRunDriver {
    fn default() -> Self {
        Self { turn_wall_us: TOOL_RUN_TURN_WALL_US, window_config_publications: 0 }
    }
}

/// 🗄️ The tool runs of one document instance for its local actor (§2.2 invariant 5): at most one non-terminal
/// mutating run, and concurrent read-only runs, one per tool and window ([`ToolRunLane`]); plus the owners still
/// retiring.
///
/// 🎯️ The ledger operates on one *selected* run. Outside its own calls the selection is the primary run — the
/// non-terminal mutating run, else the most recent one — so the single-run accessors (`state`, `slot`, `view`, …)
/// answer for it; [`ToolRunLedger::select_run`] addresses another.
pub struct ToolRunLedger<A: ArtifactApp> {
    pub next_run: u64,
    pub driver_cursor: usize,
    pub entries: Vec<Box<ToolRunEntry<A>>>,
    pub selected: Option<usize>,
    pub driver: ToolRunDriver,
    pub retired_jobs: Vec<ToolRunJobSlot<A::Config>>,
    pub retired_publications: Vec<store::os_store::one_item_publication::ArtifactStoreBatchPublication<A::Snapshot, A::Mutation>>,
    /// ♻️ Snapshot aliases owed to the store's alias retirement, and the one being retired now.
    pub retired_snapshots: Vec<Arc<A::Snapshot>>,
    pub snapshot_retirement: Option<Box<dyn store::ErasedSnapshotRetirement>>,
    pending_snapshot_alias: Option<Arc<A::Snapshot>>,
    pending_discarded: Option<A::Mutation>,
    pub discarded: Vec<A::Mutation>,
    /// 🧩️ Member runs that left their slot, whose typed owners and composed reads still retire against their member store.
    pub retired_members: Vec<ToolRunMemberRun>,
    pub closing: bool,
    pub trace_windows: BTreeMap<String, ToolRunTraceWindow>,
    pub ui_dirty: bool,
    pub document_dirty: bool,
    pub original_receipt: Option<(RetainedCloneGrant,RetainedCloneProgress)>,
}

/// 🪟️ One scene window the trace lane was delivered to: its body key and what its renderer last echoed.
pub(crate) struct ToolRunTraceWindow {
    pub body_key: String,
    pub echoed: Option<ToolRunTraceCursor>,
    pub answered: bool,
    stalls: u8,
}

impl<A: ArtifactApp> Default for ToolRunLedger<A> {
    fn default() -> Self {
        Self {
            next_run: 1,
            driver_cursor: 0,
            entries: Vec::new(),
            selected: None,
            driver: ToolRunDriver::default(),
            retired_jobs: Vec::new(),
            retired_publications: Vec::new(),
            retired_snapshots: Vec::new(),
            snapshot_retirement: None,
            pending_snapshot_alias: None,
            pending_discarded: None,
            discarded: Vec::new(),
            retired_members: Vec::new(),
            closing: false,
            trace_windows: BTreeMap::new(),
            ui_dirty: false,
            document_dirty: false,
            original_receipt: None,
        }
    }
}

impl<A: ArtifactApp> ToolRunLedger<A> {
    /// 🎟️ A retained original receipt stops the next publication callback until transport collects it.
    pub fn has_original_receipt(&self)->bool{self.original_receipt.is_some()}
    /// 🧾️ Transfers the actual caller grant and actual currencies without inventing another operation.
    pub fn take_original_receipt(&mut self)->Option<(RetainedCloneGrant,RetainedCloneProgress)>{self.original_receipt.take()}
    pub fn retain_original_receipt(&mut self,grant:RetainedCloneGrant,progress:RetainedCloneProgress)->Result<(),Fault>{
        if progress==RetainedCloneProgress::default(){return Ok(());}
        if self.original_receipt.is_some()||!progress.fits(grant)||grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"ToolRun original receipt has no unchanged granted transport slot").with_retained_progress(progress).into_fault());}
        self.original_receipt=Some((grant,progress));Ok(())
    }
    /// 👑️ The primary run: the non-terminal mutating run, else the most recent run.
    fn primary_index(&self) -> Option<usize> {
        self.entries.iter().position(|entry| entry.definition.mutating && !entry.slot.state.is_terminal()).or_else(|| self.entries.iter().enumerate().max_by_key(|(_, entry)| entry.slot.run).map(|(index, _)| index))
    }

    /// 👑️ Selects the primary run.
    pub fn select_primary(&mut self) {
        self.selected = self.primary_index();
    }

    /// 🎯️ Selects run `run`; `false` (and no selection) when this instance holds no such run.
    pub fn select_run(&mut self, run: u64) -> bool {
        self.selected = self.entries.iter().position(|entry| entry.slot.run == run);
        self.selected.is_some()
    }

    /// 🗂️ Every run this instance holds, oldest first.
    pub fn views(&self) -> Vec<ToolRunView> {
        self.entries.iter().map(|entry| entry.view()).collect()
    }

    /// 🪟️ The run a window renders: the most recent read-only run started from `window_id`, else the primary run.
    pub fn view_for(&self, window_id: Option<&str>) -> Option<ToolRunView> {
        self.window_index(window_id).or_else(|| self.primary_index()).map(|index| self.entries[index].view())
    }

    pub(crate) fn poll_view_for(&self,window_id:Option<&str>)->Option<semio_framework_plugin_decl_views::tool_run_contract::ToolRunPollView<'_>>{self.window_index(window_id).or_else(||self.primary_index()).map(|index|{let entry=&self.entries[index];semio_framework_plugin_decl_views::tool_run_contract::ToolRunPollView{tool_id:&entry.tool_id,identity:entry.identity,state:entry.slot.state}})}

    fn window_index(&self, window_id: Option<&str>) -> Option<usize> {
        let window_id = window_id?;
        self.entries.iter().enumerate().filter(|(_, entry)| !entry.definition.mutating && entry.window.as_ref().is_some_and(|(id, _)| id == window_id)).max_by_key(|(_, entry)| entry.slot.run).map(|(index, _)| index)
    }

    pub fn state(&self) -> Option<ToolRunState> {
        selected_entry!(self).map(|entry| entry.slot.state)
    }

    pub fn slot(&self) -> Option<ToolRunSlot> {
        selected_entry!(self).map(|entry| entry.slot)
    }

    pub fn identity(&self) -> Option<ToolRunIdentity> {
        selected_entry!(self).map(|entry| entry.identity)
    }

    pub fn tool_id(&self) -> Option<&str> {
        selected_entry!(self).map(|entry| entry.tool_id.as_str())
    }

    /// 📮️ The port every job of the selected run shares with its app instance.
    pub fn port(&self) -> Option<&ToolRunJobPort> {
        selected_entry!(self).map(|entry| &entry.port)
    }

    pub fn provisional(&self) -> &[A::Mutation] {
        selected_entry!(self).map_or(&[], |entry| entry.provisional.as_slice())
    }

    pub fn provisional_entities(&self) -> Option<&BTreeSet<u64>> {
        selected_entry!(self).map(|entry| entry.entities.as_ref())
    }

    pub fn steps(&self) -> Option<&ToolRunStepRing> {
        selected_entry!(self).map(|entry| &entry.steps)
    }

    pub fn trace(&self) -> Option<&ToolRunTraceStore> {
        selected_entry!(self).map(|entry| &entry.trace)
    }

    pub fn conflicts(&self) -> u32 {
        selected_entry!(self).map_or(0, |entry| entry.conflicts)
    }

    pub fn is_refolding(&self) -> bool {
        selected_entry!(self).is_some_and(|entry| entry.refold.is_some())
    }

    /// 📸️ The current run's last `StepOutcome::CheckpointReady` state, the one a rebuilt job continues from.
    pub fn checkpoint(&self) -> Option<&[u8]> {
        selected_entry!(self)?.checkpoint.as_deref()
    }

    /// 🔑️ The trace key allocator every job of the current run shares.
    pub fn trace_keys(&self) -> Option<&ToolRunTraceKeys> {
        selected_entry!(self).map(|entry| &entry.trace_keys)
    }

    /// 🏷️ `(provisional length at the end of the appending tick, entity)` per provisional entity, in append order.
    pub fn entity_marks(&self) -> &[(u32, u64)] {
        selected_entry!(self).map_or(&[], |entry| entry.entity_marks.as_slice())
    }

    /// 🪟️ `(window instance id, window kind id)` the current run was started from.
    pub fn window(&self) -> Option<(&str, &str)> {
        selected_entry!(self)?.window.as_ref().map(|(window_id, window_kind_id)| (window_id.as_str(), window_kind_id.as_str()))
    }

    /// 📊️ Progress snapshot derived from the ledger (§2.3); the step ring is the framework's own.
    pub fn progress(&self) -> Option<semio_framework_tool_run::ToolRunProgress> {
        selected_entry!(self).map(ToolRunEntry::progress)
    }

    /// 🪞️ The document a renderer reads: the mutating run's overlay while it holds provisional state, else `committed`.
    /// Read-only runs author no provisional edit, so their overlays never compose.
    pub fn overlay_or<'a>(&'a self, committed: &'a Arc<A::Snapshot>) -> &'a Arc<A::Snapshot> {
        match self.entries.iter().find(|entry| entry.definition.mutating && !entry.slot.state.is_terminal()) {
            Some(entry) if matches!(entry.slot.state, ToolRunState::Starting | ToolRunState::Running | ToolRunState::Paused | ToolRunState::Complete | ToolRunState::Finalizing) && !Arc::ptr_eq(&entry.overlay, &entry.base) => &entry.overlay,
            _ => committed,
        }
    }

    /// 🧩️ The composed children a renderer reads (§20.15): the mutating member run's composed read while it holds provisional
    /// state, else `live`. The parent document is untouched by a member run, so [`Self::overlay_or`] answers `committed`.
    pub fn children_or<'a>(&'a self, live: &'a ChildContentView) -> &'a ChildContentView {
        match self.entries.iter().find(|entry| entry.definition.mutating && !entry.slot.state.is_terminal()) {
            Some(entry) if matches!(entry.slot.state, ToolRunState::Starting | ToolRunState::Running | ToolRunState::Paused | ToolRunState::Complete | ToolRunState::Finalizing) => entry.member.as_ref().and_then(|member| member.children.as_ref()).unwrap_or(live),
            _ => live,
        }
    }

    /// 🧩️ The `(slot, child id)` of the member the selected run edits, `None` for a run on the document's own store.
    pub fn member(&self) -> Option<(&str, &str)> {
        selected_entry!(self)?.member.as_ref().map(|member| (member.slot.as_str(), member.child_id.as_str()))
    }

    /// 🧩️ The selected member run's provisional ops (that member's `OpBinary` ops); empty for a document run.
    pub fn member_ops(&self) -> &[Vec<u8>] {
        selected_entry!(self).and_then(|entry| entry.member.as_ref()).map_or(&[], |member| member.ops.as_slice())
    }

    pub fn view(&self) -> Option<ToolRunView> {
        selected_entry!(self).map(ToolRunEntry::view)
    }

    /// ⏪️ Whether a non-terminal mutating run holds provisional state on this instance: history editing cannot open then.
    pub fn holds_mutating_run(&self) -> bool {
        self.entries.iter().any(|entry| entry.definition.mutating && !entry.slot.state.is_terminal())
    }

    /// 🧊️ `freeze` rebase policy: local artifact emits fail with `toolRun.busy` while a freezing run is non-terminal.
    pub fn freezes_local_emits(&self) -> bool {
        self.entries.iter().any(|entry| entry.definition.rebase == ToolRunRebasePolicy::Freeze && !entry.slot.state.is_terminal())
    }

    /// 🏃️ Whether a driver turn has work: any run's stepping job, refold, close, finalize or port effects, or cold
    /// retirement. A job waiting on its port is not work.
    pub fn has_pending_work(&self) -> bool {
        !self.retired_jobs.is_empty()
            || !self.retired_publications.is_empty()
            || !self.retired_snapshots.is_empty()
            || self.snapshot_retirement.is_some()
            || self.pending_snapshot_alias.is_some()
            || self.pending_discarded.is_some()
            || !self.discarded.is_empty()
            || !self.retired_members.is_empty()
            || self.entries.iter().any(|entry| entry.has_pending_work())
    }

    /// 🧬️ Applies one decoded tick to the current run (the driver's per-tick O(k) overlay append);
    /// ticks of another run or generation are stale no-ops.
    pub fn apply_tick(&mut self, tick: ToolRunTick) -> Result<ToolRunTickReceipt, Fault> {
        let Some(entry) = selected_entry_mut!(self) else { return Ok(ToolRunTickReceipt { stale: true, ..ToolRunTickReceipt::default() }) };
        entry.apply_tick(tick, &mut self.discarded)
    }

    /// 🪟️ A window-config publication happened; the driver treats it as a settings change.
    pub fn note_window_config_published(&mut self) {
        self.driver.window_config_publications = self.driver.window_config_publications.wrapping_add(1);
    }

    /// 👥️ Ephemeral-shared presence summary (§3.4), `completed ≤ total` by construction.
    pub fn presence(&self) -> Option<protocol::PresenceToolRun> {
        let entry = &self.entries[self.primary_index()?];
        Some(protocol::PresenceToolRun {
            tool_id: entry.tool_id.clone(),
            state: protocol::PresenceToolRunState::from_wire_name(entry.slot.state.as_str())?,
            stage: entry.stage,
            completed: entry.total.map_or(entry.completed, |total| entry.completed.min(total)),
            total: entry.total,
        })
    }

    /// 📼️ Base64url (unpadded) `ToolRunTraceDelta` after `cursor`, for the scene lane `toolRunTrace` (§3.2).
    ///
    /// - A cursor of another run or generation, or none, gets `clear` plus the resend of the live log.
    /// - A caught-up cursor gets `None`, so an idle run costs nothing.
    /// - Without a run (dismissed or retired), a renderer still holding pages (`page > 0`) gets one empty
    ///   `clear` delta; `page == 0` already means an empty layer.
    pub fn trace_delta(&self, cursor: Option<ToolRunTraceCursor>, byte_budget: usize) -> Option<String> {
        let index = cursor.and_then(|cursor| self.entries.iter().position(|entry| entry.slot.run == cursor.run)).or_else(|| self.primary_index());
        let delta = match index.map(|index| &self.entries[index]) {
            Some(entry) => entry.trace.delta_after(cursor, byte_budget),
            None => {
                let cursor = cursor.filter(|cursor| cursor.page > 0)?;
                ToolRunTraceDelta { identity: ToolRunIdentity { id: ToolRunId { app_instance_id: 0, run: cursor.run }, generation: cursor.generation, base_revision: [0; 32] }, clear: true, next: 0, pages: Vec::new() }
            }
        };
        tool_run_trace_lane_text(&delta)
    }

    /// 🪟️ The lane text scene window `window_id` (body `body_key`) receives for its echoed `cursor`, and the
    /// bookkeeping that keeps a delivery backlog moving: pages left after the byte budget mark the ledger
    /// UI-dirty until the renderer's cursor stands still for `TOOL_RUN_TRACE_STALL_REFRESHES` refreshes.
    pub fn trace_lane(&mut self, window_id: &str, body_key: &str, cursor: Option<ToolRunTraceCursor>, byte_budget: usize) -> Option<String> {
        let Some(entry) = self.window_index(Some(window_id)).or_else(|| self.primary_index()).map(|index| &self.entries[index]) else {
            self.trace_windows.remove(window_id);
            return self.trace_delta(cursor, byte_budget);
        };
        let delta = entry.trace.delta_after(cursor, byte_budget);
        let backlog = delta.next < entry.trace.next_page();
        let window = self.trace_windows.entry(window_id.to_string()).or_insert_with(|| ToolRunTraceWindow { body_key: body_key.to_string(), echoed: None, answered: false, stalls: 0 });
        if window.body_key != body_key {
            window.body_key = body_key.to_string();
        }
        window.stalls = if window.answered && window.echoed == cursor { window.stalls.saturating_add(1) } else { 0 };
        window.echoed = cursor;
        window.answered = true;
        if backlog && window.stalls < TOOL_RUN_TRACE_STALL_REFRESHES {
            self.ui_dirty = true;
        }
        tool_run_trace_lane_text(&delta)
    }

    /// 🧹️ Forgets scene windows that closed or stopped hosting a scene surface.
    pub fn retain_trace_windows(&mut self, live: impl Fn(&str) -> bool) {
        self.trace_windows.retain(|window_id, _| live(window_id));
    }

    /// 🎯️ The minimal UI scope a run change dirties: the ToolRun panel body, the body of every scene window its trace
    /// lane rides in and the body of every window kind the run declares it renders in (`ToolRunDefinition::windows`) —
    /// never unrelated windows or panels.
    pub fn dirty_scope(&self) -> UiDirtyScope {
        let mut window_bodies: Vec<String> = self.trace_windows.values().map(|window| window.body_key.clone()).chain(self.entries.iter().flat_map(|entry| entry.window_bodies.iter().cloned())).collect();
        window_bodies.sort_unstable();
        window_bodies.dedup();
        UiDirtyScope::Partial { window_bodies, panel_bodies: vec![FRAMEWORK_TOOL_RUN_BODY_KEY.to_string()], utilities: false, tools: false, engagements: false, measures: false, labels: false }
    }

    /// 🚩️ Whether a run change still owes the host a dirty scope.
    pub fn is_ui_dirty(&self) -> bool {
        self.ui_dirty || self.document_dirty
    }

    pub fn apply_event(&mut self, event: ToolRunEvent) -> Result<ToolRunEffect, ToolRunRejection> {
        let transition = ToolRunMachine::apply(self.slot(), event)?;
        if transition.slot.is_none()&&selected_entry!(self).and_then(|entry|entry.finalize.as_ref()).is_some_and(|finalize|!finalize.ingress.terminal_is_empty()||finalize.publication.is_some()){return Err(ToolRunRejection::Busy);}
        match (self.selected, transition.slot) {
            (Some(index), Some(slot)) => {
                let entry = &mut self.entries[index];
                let generation_changed = entry.slot.generation != slot.generation;
                entry.slot = slot;
                if generation_changed {
                    entry.rebind();
                }
            }
            (Some(index), None) => {
                let entry = self.entries.remove(index);
                self.retire_entry(*entry);
                self.select_primary();
            }
            (None, _) => {}
        }
        Ok(transition.effect)
    }

    pub fn retire_entry(&mut self, entry: ToolRunEntry<A>) {
        let ToolRunEntry { job, provisional, finalize, base, overlay, refold, mut displaced, member, .. } = entry;
        self.retired_members.extend(member);
        self.retired_snapshots.push(base);
        self.retired_snapshots.push(overlay);
        if let Some(running) = refold.and_then(|refold| refold.running) {
            self.retired_snapshots.push(Arc::new(running));
        }
        self.retired_snapshots.append(&mut displaced);
        self.retire_owners(job, provisional, finalize);
    }

    pub fn retire_owners(&mut self, job: Option<ToolRunJobSlot<A::Config>>, mut provisional: Vec<A::Mutation>, finalize: Option<ToolRunFinalize<A>>) {
        assert!(finalize.as_ref().is_none_or(|finalize|finalize.ingress.terminal_is_empty()),"original ToolRun ingress remains in its entry until granted closure");
        if let Some(mut job) = job {
            job.begin_close();
            self.retired_jobs.push(job);
        }
        self.discarded.append(&mut provisional);
        if let Some(publication) = finalize.and_then(|finalize| finalize.publication) {
            self.retired_publications.push(publication);
        }
    }

    pub fn close_current_job(&mut self) {
        let Some(entry) = selected_entry_mut!(self) else { return };
        entry.settle_refold();
        if let Some(mut job) = entry.job.take() {
            job.begin_close();
            self.retired_jobs.push(job);
        }
    }

    /// 🎯️ Hands the current job the entry's identity through `retarget`; a job that is not retargetable, or
    /// declines, is closed so the driver rebuilds it from its checkpoint.
    pub fn retarget_current_job(&mut self, retarget: impl FnOnce(&mut dyn ToolRunRetargetableJob<A::Config>, ToolRunIdentity) -> bool) {
        let Some(entry) = selected_entry_mut!(self) else { return };
        let identity = entry.identity;
        let Some(slot) = entry.job.as_mut() else { return };
        if slot.retargetable().is_some_and(|job| retarget(job, identity)) {
            slot.generation = semio_framework_job::Generation(u64::from(identity.generation));
        } else {
            self.close_current_job();
        }
    }

    /// 🧹️ Throws the run's provisional work away, payload included: an abort, a fault, a rebase-restart
    /// or a reconfigure-restart invalidates the run's whole intermediate result, so the windows that
    /// read `ToolRunView::payload` must stop showing it.
    pub fn discard_provisional(&mut self) {
        self.release_provisional(false);
    }

    /// 🧹️ Releases the provisional ops a COMPLETED finalize has already published, keeping the run's
    /// `payload`. The payload is not provisional work — it is the run's own result (§2.3: "the latest
    /// intermediate result of the run … the run's windows read it back through
    /// `ToolRunView::payload`"), and a `mutating: false` run publishes nothing else at all, so clearing
    /// it on finalize is what made a finished read-only run's windows revert to their pre-run render the
    /// instant it reached `Finalized`. It is cleared on a genuine discard and when the entry retires.
    pub fn release_provisional(&mut self, keep_payload: bool) {
        let Some(entry) = selected_entry_mut!(self) else { return };
        self.discarded.append(&mut entry.provisional);
        entry.entity_marks.clear();
        entry.entities = Arc::new(BTreeSet::new());
        if let Some(running) = entry.refold.take().and_then(|refold| refold.running) {
            entry.displaced.push(Arc::new(running));
        }
        let overlay = Arc::clone(&entry.base);
        entry.replace_overlay(overlay);
        entry.checkpoint = None;
        if let Some(member) = entry.member.as_mut() {
            member.release();
        }
        if !keep_payload {
            entry.payload = None;
        }
    }

    /// 🧹️ Advances the owners that outlived their slot by one bounded unit; `None` when nothing is retiring.
    pub fn retirement_demands(&self,store:&ArtifactStore<A::Snapshot,A::Mutation>,body:usize)->Result<RetirementDemand,ValueError>{
        if self.closing&&!self.entries.is_empty(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original tool entries retain job cancellation and native metadata until granted entry retirement is available"));}
        if let Some(job)=self.retired_jobs.last(){
            if job.terminal_is_empty(){return Ok(RetirementDemand{copy_bytes:size_of::<ToolRunJobSlot<A::Config>>(),release_bytes:job.scaffold_release_bytes(),depth:1,..Default::default()});}
            return job.next_demands(body);
        }
        if let Some(publication)=self.retired_publications.last(){
            if publication.terminal_is_empty(){return Ok(RetirementDemand{copy_bytes:size_of::<store::os_store::one_item_publication::ArtifactStoreBatchPublication<A::Snapshot,A::Mutation>>(),depth:1,..Default::default()});}
            let mut demand=publication.retirement_demands(body)?;demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"tool publication close depth overflow"))?;return Ok(demand);
        }
        if self.snapshot_retirement.is_some(){return tool_erased_retirement_demand(&self.snapshot_retirement,body);}
        if self.pending_snapshot_alias.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<Arc<A::Snapshot>>>(),capacity_bytes:ArtifactStore::<A::Snapshot,A::Mutation>::snapshot_alias_retirement_birth_bytes(),depth:2,..Default::default()});}
        if self.pending_discarded.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<A::Mutation>>(),capacity_bytes:store.owned_mutation_retirement_birth_bytes()?,depth:2,..Default::default()});}
        if !self.retired_snapshots.is_empty()||self.entries.iter().any(|entry|!entry.displaced.is_empty()){return Ok(RetirementDemand{copy_bytes:2*size_of::<Option<Arc<A::Snapshot>>>(),depth:1,..Default::default()});}
        if !self.discarded.is_empty(){return Ok(RetirementDemand{copy_bytes:2*size_of::<Option<A::Mutation>>(),depth:1,..Default::default()});}
        if let Some((key,window))=self.trace_windows.iter().next(){return Ok(RetirementDemand{copy_bytes:size_of::<(String,ToolRunTraceWindow)>(),release_bytes:key.capacity().saturating_add(window.body_key.capacity()),depth:1,..Default::default()});}
        macro_rules! ledger_quote {($field:ident)=>{if let Some(demand)=ledger_empty_backing_demand(&self.$field)?{return Ok(demand);}};}
        ledger_quote!(entries);ledger_quote!(retired_jobs);ledger_quote!(retired_publications);ledger_quote!(retired_snapshots);ledger_quote!(discarded);ledger_quote!(retired_members);
        if !self.retired_members.is_empty()||!self.trace_windows.is_empty(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original tool member and trace metadata retain their granted retirement authority"));}
        Ok(Default::default())
    }
    pub fn retire_step(&mut self,store:&mut ArtifactStore<A::Snapshot,A::Mutation>,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<RetainedCloneStep>,semio_framework_job::OriginalTurnFailure>{
        let grant=cx.retained_grant();if cx.should_yield(){return Ok(Some(RetainedCloneStep::Progress(Default::default())))}
        if self.has_original_receipt(){return Ok(Some(RetainedCloneStep::Progress(Default::default())));}
        if grant.maximum_items==0{return Ok(Some(RetainedCloneStep::Progress(Default::default())));}
        let demand=self.retirement_demands(store,grant.maximum_copy_bytes)?;if demand==RetirementDemand::default(){return Ok(None)}
        if !tool_grant_funds(grant,demand){return Ok(Some(RetainedCloneStep::Progress(Default::default())));}
        if let Some(job)=self.retired_jobs.last_mut(){
            if job.terminal_is_empty(){let released_bytes=job.scaffold_release_bytes();let step=semio_framework_job::close_original_retained_step(cx,demand,|_|{drop(self.retired_jobs.pop());Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes,..Default::default()}))})?;return Ok(Some(RetainedCloneStep::Progress(step.progress())));}
            return job.close_step(cx).map(Some);
        }
        if self.terminal_is_empty(){return Ok(None)}
        let step=semio_framework_job::close_original_retained_step(cx,demand,|grant|{
        if let Some(publication)=self.retired_publications.last_mut(){
            if publication.terminal_is_empty(){drop(self.retired_publications.pop());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));}
            publication.begin_close();let child=RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant};let step=publication.close_step(child)?;
            let step=admit_retained_clone_close(child,step,publication.terminal_is_empty(),"tool publication original")?;return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if self.snapshot_retirement.is_some(){return close_tool_erased_retirement(&mut self.snapshot_retirement,grant);}
        if self.pending_snapshot_alias.is_some(){let result=store.retire_snapshot_alias(&mut self.pending_snapshot_alias,grant)?;return Ok(RetainedCloneStep::Progress(match result{Some((owner,progress))=>{self.snapshot_retirement=Some(owner);progress},None=>Default::default()}));}
        if self.pending_discarded.is_some(){let result=store.retire_owned_mutation(&mut self.pending_discarded,grant)?;return Ok(RetainedCloneStep::Progress(match result{Some((owner,progress))=>{self.snapshot_retirement=Some(owner);progress},None=>Default::default()}));}
        let alias=self.retired_snapshots.pop().or_else(||self.entries.iter_mut().find(|entry|!entry.displaced.is_empty()).and_then(|entry|entry.displaced.pop()));
        if let Some(alias)=alias{self.pending_snapshot_alias=Some(alias);return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));}
        if let Some(operation)=self.discarded.pop(){self.pending_discarded=Some(operation);return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));}
        if let Some((key,window))=self.trace_windows.pop_first(){let released_bytes=key.capacity().saturating_add(window.body_key.capacity());drop((key,window));return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes,..Default::default()}));}
        macro_rules! ledger_release {($field:ident)=>{if let Some(step)=close_ledger_backing(&mut self.$field,grant)?{return Ok(step);}};}
        ledger_release!(entries);ledger_release!(retired_jobs);ledger_release!(retired_publications);ledger_release!(retired_snapshots);ledger_release!(discarded);ledger_release!(retired_members);
        if !self.retired_members.is_empty()||!self.trace_windows.is_empty(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original tool metadata retains its granted retirement authority"));}
        Ok(RetainedCloneStep::Complete(Default::default()))
        })?;Ok(Some(RetainedCloneStep::Progress(step.progress())))
    }

    /// 🚪️ Document close, instance retirement or reload: same as abort, ephemeral state vanishes (§2.2).
    pub fn begin_close(&mut self) {
        if self.closing {
            return;
        }
        self.closing = true;
        self.selected = None;
    }

    /// 🧹️ One bounded close unit of every retiring owner.
    pub fn close_step(&mut self,store:&mut ArtifactStore<A::Snapshot,A::Mutation>,cx:&mut semio_framework_job::StepContext<'_>)->Result<RetainedCloneStep,semio_framework_job::OriginalTurnFailure>{
        if !self.closing{return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"tool run close requires granted entry cancellation before cleanup").into());}
        Ok(self.retire_step(store,cx)?.unwrap_or(RetainedCloneStep::Complete(Default::default())))
    }

    pub fn terminal_is_empty(&self) -> bool {
        ledger_backing_is_empty(&self.entries)&&ledger_backing_is_empty(&self.retired_jobs)&&ledger_backing_is_empty(&self.retired_publications)&&ledger_backing_is_empty(&self.retired_snapshots)&&ledger_backing_is_empty(&self.discarded)&&ledger_backing_is_empty(&self.retired_members)&&self.snapshot_retirement.is_none()&&self.pending_snapshot_alias.is_none()&&self.pending_discarded.is_none()&&self.trace_windows.is_empty()&&!self.has_original_receipt()
    }
}
//#endregion 🔖️Ledger
