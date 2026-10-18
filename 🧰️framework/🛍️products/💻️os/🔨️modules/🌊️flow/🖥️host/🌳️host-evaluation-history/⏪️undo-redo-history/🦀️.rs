//! ⏪️ Undo and redo history of the flow host: the history store, the change leaves and journal, the edit recording, gestures and the undo and redo steps. See [`super`].

use super::*;

// #region History
impl FlowHost {
    /// 🧾️ Lazily seeds the undo/redo store from `baseline`.
    ///
    /// ⚠️ `baseline` is CONSUMED only on the first call — the store is seeded once, and every later
    /// `flush_pending_change` hands in a fresh `FlowHostSnapshot` clone this function does not need. A
    /// `FlowHostSnapshot` owns the fail-closed `OrderedMap<WidgetLayout>` root, so that surplus clone is
    /// RETIRED through the artifact's own bounded frontier instead of dropped; the bare drop aborted
    /// the pool worker on the second discrete edit of any session
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️unit-suite-3d-2026-09-09.md` §5).
    pub fn history_store_from_baseline(&mut self, baseline: FlowHostSnapshot) -> Option<&mut FlowStore> {
        if self.history_store.is_some() {
            baseline.retire_cold();
            return self.history_store.as_mut();
        }
        let Some(producer)=self.history_store_factory.as_ref() else{baseline.retire_cold();return None};
        let store=(producer.construct)(baseline).ok()?;
        self.history_store = Some(store);
        self.history_store.as_mut()
    }

    /// 🧾️ Appends the concrete leaves a gesture site just emitted to the edit in progress.
    pub fn note_leaves(&mut self, leaves: impl IntoIterator<Item = FlowMutation>) {
        self.pending_leaves.extend(leaves);
    }

    /// 🗑️ Retires the leaves of an edit that is not recorded; a widget leaf owns fail-closed roots that refuse a bare drop.
    pub(in super::super) fn discard_pending_leaves(&mut self) {
        std::mem::take(&mut self.pending_leaves).into_iter().for_each(retire_flow_mutation);
    }

    /// 🩹 The `change-widget` leaf carrying `widget_id`'s current content, when the widget exists.
    pub(in super::super) fn changed_widget_leaf(&self, widget_id: &str) -> Option<FlowMutation> {
        self.host_snapshot.widgets.iter().find(|widget| widget_id_for(widget) == widget_id).map(|widget| FlowMutation::ChangeWidget(ChangeWidget { id: widget_id.to_string(), widget: widget.clone() }))
    }

    /// 📐️ The `change-layout` leaf assigning each of `ids` its current layout; an id without a layout is skipped.
    pub(in super::super) fn layout_leaf<'a>(&self, ids: impl IntoIterator<Item = &'a str>) -> Option<FlowMutation> {
        let entries: Vec<FlowLayoutEntry> = ids.into_iter().filter_map(|id| self.host_snapshot.layout.get(id).map(|layout| FlowLayoutEntry { id: id.to_string(), layout: Some(layout.clone()) })).collect();
        (!entries.is_empty()).then(|| FlowMutation::ChangeLayout(ChangeLayout { entries }))
    }

    /// 🔗️ The leaves of the wire, slider and move edits the open pointer gesture journalled in the dag (the gesture's own
    /// narration, read without draining); a port insert is recorded where `add_*_port` runs, and a connect that displaced
    /// the wire already feeding its target removes it first.
    fn gesture_journal_leaves(&self, baseline: &FlowHostSnapshot) -> Vec<FlowMutation> {
        let edits = self.dag.graph.graph_edits();
        self.journal_leaves(edits.get(self.journal_mark..).unwrap_or(edits), baseline)
    }

    /// 🔗️ The leaves one gesture's journal `rows` stand for, given the `baseline` before the gesture and the live content after it.
    pub fn journal_leaves(&self, rows: &[dag::DagGraphEdit], baseline: &FlowHostSnapshot) -> Vec<FlowMutation> {
        let mut leaves = Vec::new();
        for edit in rows {
            match edit {
                dag::DagGraphEdit::Connect { source_node_id, source_port_id, target_node_id, target_port_id } => {
                    let displaced = baseline.synapses.iter().filter(|synapse| synapse.to == *target_node_id && synapse.to_port == *target_port_id && self.host_snapshot.synapses.iter().all(|now| now.id != synapse.id));
                    leaves.extend(displaced.map(|synapse| FlowMutation::RemoveSynapse(RemoveSynapse { id: synapse.id.clone() })));
                    let created = self.host_snapshot.synapses.iter().enumerate().find(|(_, synapse)| synapse.from == *source_node_id && synapse.from_port == *source_port_id && synapse.to == *target_node_id && synapse.to_port == *target_port_id);
                    leaves.extend(created.map(|(index, synapse)| FlowMutation::AddSynapse(AddSynapse { index: u32::try_from(index).unwrap_or(u32::MAX), synapse: synapse.clone() })));
                }
                dag::DagGraphEdit::Disconnect { synapse_id } => {
                    if baseline.synapses.iter().any(|synapse| synapse.id == *synapse_id) {
                        leaves.push(FlowMutation::RemoveSynapse(RemoveSynapse { id: synapse_id.clone() }));
                    }
                }
                dag::DagGraphEdit::Move { node_ids, .. } => leaves.extend(self.layout_leaf(node_ids.iter().map(String::as_str))),
                dag::DagGraphEdit::SetSlider { node_id, .. } => leaves.extend(self.changed_widget_leaf(node_id)),
                dag::DagGraphEdit::InsertPort { .. } => {}
            }
        }
        leaves
    }

    /// 🧾️ Records the leaves emitted since the edit was armed as ONE store transaction (seeding the store from `baseline` on
    /// first use), retiring them cold when there is nothing to record; an edit that emitted no leaf leaves no history row.
    /// A refused transaction is never ignored: the history restarts at the live content (no partial row survives) and the
    /// refusal is kept as [`FlowCoreError::HistoryRefused`] for [`FlowHost::take_history_fault`].
    pub fn record_history_edit(&mut self, baseline: FlowHostSnapshot) {
        let operations = std::mem::take(&mut self.pending_leaves);
        #[cfg(test)]
        {
            if !operations.is_empty() {
                self.recorded.push(operations.clone());
            }
        }
        let recorded = !operations.is_empty();
        let refusal = match self.history_store_from_baseline(baseline) {
            Some(store) if recorded => history_identity!(|identity| ::semio_framework_async::poll::resolve_ready(store.dispatch(ArtifactCommand::Apply { mutations: operations, transaction: None }, &mut identity))).err().map(|error| format!("{error:?}")),
            _ => {
                operations.into_iter().for_each(retire_flow_mutation);
                None
            }
        };
        if let Some(reason) = refusal {
            self.reset_history_store();
            self.history_fault = Some(FlowCoreError::HistoryRefused(reason));
        }
    }

    /// 🚦️ Closes a discrete edit: records its leaves now unless a gesture is still coalescing them.
    pub(super) fn finish_change(&mut self) {
        if !self.gesture_active {
            self.flush_pending_change();
        }
    }

    /// 🚦️ [`Self::finish_change`] that hands a refused history row back to the gesture that caused it.
    pub fn checked_change(&mut self) -> Result<(), FlowCoreError> {
        self.finish_change();
        self.history_outcome()
    }

    /// 🧯️ Hands over the refusal of the last history row, if any; the gestures that cannot return it (slider and text setters,
    /// pointer release, note commit) leave it here for their caller to collect.
    pub fn take_history_fault(&mut self) -> Option<FlowCoreError> {
        self.history_fault.take()
    }

    /// 🧯️ [`Self::take_history_fault`] as a `Result`.
    pub fn history_outcome(&mut self) -> Result<(), FlowCoreError> {
        self.take_history_fault().map_or(Ok(()), Err)
    }

    /// 🧾️ Flushes an armed-but-not-yet-recorded discrete mutation into `history_store` as the concrete flow leaves its gesture site emitted — the standard `semio_framework_os_kernel::os_store::ArtifactStore`/`Mutation`/
    /// `MutationDiff` mechanism (see `🔖️Mutations`) driving undo/redo here instead of the old
    /// hand-rolled `Vec<FlowHostSnapshot>` snapshot stack. An armed edit that emitted no leaf leaves no history row.
    fn flush_pending_change(&mut self) {
        if self.pending_change {
            self.pending_change = false;
            let baseline = self.pending_history_baseline.take().unwrap_or_else(|| self.host_snapshot.clone());
            self.record_history_edit(baseline);
        }
    }

    /// ↩️ Arms a checkpoint for the mutation about to happen, unless a gesture (`begin_gesture`) is
    /// currently coalescing several mutations into one.
    pub fn begin_change(&mut self) {
        self.evaluation_revision=self.evaluation_revision.wrapping_add(1);
        if !self.gesture_active {
            self.flush_pending_change();
            self.arm_history_baseline();
            self.pending_change = true;
        }
    }

    /// 🧾️ Arms a fresh undo baseline, RETIRING the one it replaces.
    ///
    /// 🩸️ A baseline is a `FlowHostSnapshot`, which owns the fail-closed `OrderedMap<WidgetLayout>` root,
    /// and overwriting the field simply DROPPED the old one: `panicked at 🗂️ordered/🦀️.rs:81:
    /// ordered-map root must be explicitly retired before drop`, aborting the whole pool worker.
    /// Reached on 6118 by the FOURTH middle-button pan of one session — a gesture whose release never
    /// committed leaves its baseline armed, and the next gesture's `begin_gesture` overwrote it
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-node-graph-gestures-2026-09-13.md` §4).
    /// The same hazard sat on `begin_change` and on the history reset inside `apply_fixture`.
    fn arm_history_baseline(&mut self) {
        if let Some(stale) = self.pending_history_baseline.take() {
            stale.retire_cold();
        }
        self.pending_history_baseline = Some(self.host_snapshot.clone());
    }

    /// 🖐️ Starts a coalescing gesture (drag, inline note edit): flushes anything already armed first,
    /// then suppresses further `begin_change` checkpoints until `commit_gesture_history`.
    pub fn begin_gesture(&mut self) {
        self.flush_pending_change();
        self.arm_history_baseline();
        self.journal_mark = self.dag.graph.graph_edits().len();
        self.gesture_active = true;
    }

    /// 🧾️ Closes a coalescing gesture, recording ONE invertible edit when the gesture actually
    /// changed content.
    ///
    /// 🩸️ A gesture that changed nothing — every plain CLICK on the graph — still holds a
    /// `FlowHostSnapshot` baseline, and a `FlowHostSnapshot` owns the fail-closed `OrderedMap<WidgetLayout>`
    /// root: letting it fall out of scope aborted the whole pool worker with `ordered-map root must
    /// be explicitly retired before drop` on the FIRST click. It was unreachable from wgpu only
    /// because no pointer ever reached the graph; the retention fix reaches it on press one (ticket
    /// 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-node-graph-surface-retention-2026-09-13.md`).
    /// A no-op baseline is therefore RETIRED, exactly as `history_store_from_baseline` retires the
    /// surplus clone it does not need.
    pub fn commit_gesture_history(&mut self) {
        if self.gesture_active {
            self.gesture_active = false;
            let mut baseline = self.pending_history_baseline.take().unwrap_or_else(|| self.host_snapshot.clone());
            if let Err(refusal) = self.journal_gesture_moves(&baseline) {
                self.discard_pending_leaves();
                baseline.camera = self.host_snapshot.camera.clone();
                std::mem::replace(&mut self.host_snapshot, baseline).retire_cold();
                self.rebuild_dag();
                self.dag.graph.carry_journal_refusal(refusal);
                return;
            }
            if let Some(id) = self.edited_note.take() {
                let edited = self.changed_widget_leaf(&id).filter(|_| baseline.widgets.iter().find(|widget| widget_id_for(widget) == id).is_some_and(|before| self.host_snapshot.widgets.iter().find(|widget| widget_id_for(widget) == id) != Some(before)));
                self.note_leaves(edited);
            }
            let journalled = self.gesture_journal_leaves(&baseline);
            self.note_leaves(journalled);
            if self.pending_leaves.is_empty() {
                baseline.retire_cold();
                return;
            }
            self.record_history_edit(baseline);
        }
    }

    /// ✋️ Narrates a released gesture that moved nodes as the node-graph gesture record of design §13.3 — the moved widgets
    /// (in widget order) and their ONE offset from the gesture's baseline, one record per distinct offset (a grid snap can
    /// land off-grid starts on different offsets) — so the guest commits the drag as relative leaves, exactly the record
    /// the wgpu bounded path writes (`DagHost::plan_graph_edits`). Wires, inline sliders and port inserts are journalled
    /// where they happen; all move rows are admitted together or the caller restores the baseline.
    fn journal_gesture_moves(&mut self, baseline: &FlowHostSnapshot) -> Result<(), dag::DagJournalRefusal> {
        let displacements = self.host_snapshot.widgets.iter().filter_map(|widget| {
            let id = widget_id_for(widget);
            let (landed, start) = (self.host_snapshot.layout.get(id)?, baseline.layout.get(id)?);
            Some((id.to_string(), landed.x - start.x, landed.y - start.y))
        });
        let gesture_id = dag::dag_drag_gesture_id(self.interaction_revision);
        self.dag.graph.journal_moves(&gesture_id, displacements)
    }

    /// ↩️ Restores the previous fixture content snapshot, keeping the current camera.
    pub fn undo(&mut self) -> bool {
        self.flush_pending_change();
        let camera = self.host_snapshot.camera.clone();
        let Some(store) = self.history_store.as_mut() else {
            return false;
        };
        if history_identity!(|identity| ::semio_framework_async::poll::resolve_ready(store.dispatch(ArtifactCommand::Undo, &mut identity))).is_err() {
            return false;
        }
        let Ok(mut restored) = store.snapshot() else {
            return false;
        };
        restored.camera = camera;
        std::mem::replace(&mut self.host_snapshot, restored).retire_cold();
        self.rebuild_dag();
        true
    }

    /// ↪️ Re-applies a fixture content snapshot undone earlier, keeping the current camera.
    pub fn redo(&mut self) -> bool {
        let camera = self.host_snapshot.camera.clone();
        let Some(store) = self.history_store.as_mut() else {
            return false;
        };
        if history_identity!(|identity| ::semio_framework_async::poll::resolve_ready(store.dispatch(ArtifactCommand::Redo, &mut identity))).is_err() {
            return false;
        }
        let Ok(mut restored) = store.snapshot() else {
            return false;
        };
        restored.camera = camera;
        std::mem::replace(&mut self.host_snapshot, restored).retire_cold();
        self.rebuild_dag();
        true
    }

    /// ↩️ Whether a content undo step is available.
    pub fn can_undo(&self) -> bool {
        !self.pending_leaves.is_empty() || self.history_store.as_ref().is_some_and(|store| !store.applied_edit_ids().is_empty())
    }

    /// ↪️ Whether a content redo step is available.
    pub fn can_redo(&self) -> bool {
        self.history_store.as_ref().is_some_and(|store| !store.redo_edit_ids().is_empty())
    }
}
// #endregion History
