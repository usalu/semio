//! 🗂️ Snapshot, catalogue and kind infos of the flow host: replacing and applying host snapshots, the history store reset, the host catalogue and the neuron kind infos. See [`super`].

use super::*;

impl FlowHost {
    /// 📥️ Loads fixture content while keeping catalogue, operator metadata, eval bridge, and the live camera. A load is a history
    /// reset: no edit survives it.
    pub fn replace_host_snapshot(&mut self, host_snapshot: FlowHostSnapshot) {
        self.apply_host_snapshot(host_snapshot, true, false);
    }

    /// 📥️ Scene resync: reloads fixture layout/content without discarding eval baseline or cached outputs. A scene that merely
    /// echoes the live content leaves the history alone; a scene whose content differs is a load and resets the history, because
    /// a content change may only enter the history as concrete leaves.
    pub fn resync_host_snapshot_from_scene(&mut self, host_snapshot: FlowHostSnapshot) {
        self.apply_host_snapshot(host_snapshot, false, true);
    }

    fn apply_host_snapshot(&mut self, mut host_snapshot: FlowHostSnapshot, force_reset: bool, preserve_eval: bool) {
        self.evaluation_revision=self.evaluation_revision.wrapping_add(1);
        self.interaction_revision = self.interaction_revision.wrapping_add(1);
        dedupe_host_snapshot_widgets(&mut host_snapshot);
        let load = force_reset || self.host_snapshot.widgets != host_snapshot.widgets || self.host_snapshot.synapses != host_snapshot.synapses || self.host_snapshot.layout != host_snapshot.layout;
        // 🎥️ Camera is ephemeral view state (same as undo/redo) — never snap the live pan/zoom when a
        // scene resync reloads fixture content (hover, eval tick, remote operations, …).
        let camera = self.host_snapshot.camera.clone();
        host_snapshot.camera = camera;
        std::mem::replace(&mut self.host_snapshot, host_snapshot).retire_cold();
        if !preserve_eval {
            self.displace_eval_state().expect("cold fixture load requires settled original baseline custody");
            self.last_eval_json.clear();
        }
        self.pan_anchor = None;
        self.ghost_node = None;
        self.rebuild_dag();
        self.refresh_interaction_projection();
        if load {
            self.reset_history_store();
            self.pending_change = false;
            self.gesture_active = false;
            self.edited_note = None;
            self.discard_pending_leaves();
        }
    }

    /// 🧹️ Restarts the history at the live content: the store is reset to it (when one exists) and the armed baseline is
    /// retired. Used by every load and by a refused history row, since the store can no longer represent the live content.
    pub(in super::super) fn reset_history_store(&mut self) {
        if let Some(store) = self.history_store.as_mut() {
            let producer=self.history_store_factory.as_ref().expect("existing history Store retains its original producer");
            let envelope=(producer.envelope)(self.host_snapshot.clone()).expect("original child history genesis reset");
            ::semio_framework_async::poll::resolve_ready(store.reset(envelope)).expect("failed to reset flow history store");
        }
        if let Some(stale) = self.pending_history_baseline.take() {
            stale.retire_cold();
        }
    }

    pub fn parse_host_snapshot_json(json: &str) -> Result<FlowHostSnapshot, FlowCoreError> {
        Ok(semio_framework_pack_json::from_json_str(json, semio_framework_pack_json::JsonMemberPolicy::Reject)?)
    }

    pub fn host_snapshot_json(&self) -> Result<String, FlowCoreError> {
        Ok(semio_framework_pack_json::to_json_string(&self.host_snapshot))
    }

    pub fn document(&self) -> FlowArtifact {
        self.host_snapshot.to_artifact()
    }

    pub fn catalogue_json(&self) -> Result<String, FlowCoreError> {
        let sections = merge_catalogue_sections(&self.host_catalogue_json)?;
        Ok(semio_framework_pack_json::to_json_string(&sections))
    }

    pub fn set_host_catalogue_json(&mut self, json: &str) {
        self.host_catalogue_json = json.to_string();
    }

    /// 📦️ Installs the palette sections from a content-addressed payload — see
    /// [`resolve_flow_shared_payload`]. Several parts are concatenated section by section.
    pub fn set_host_catalogue_payload(&mut self, payload: &str) -> Result<(), FlowCoreError> {
        let parts = resolve_flow_shared_payload_parts(payload)?;
        if let [single] = parts.as_slice() {
            self.set_host_catalogue_json(single);
            return Ok(());
        }
        let mut sections: Vec<CatalogueSection> = Vec::new();
        for part in &parts {
            if part.trim().is_empty() {
                continue;
            }
            sections.extend(semio_framework_pack_json::from_json_str::<Vec<CatalogueSection>>(part, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_default());
        }
        self.set_host_catalogue_json(&semio_framework_pack_json::to_json_string(&sections));
        Ok(())
    }

    /// 🔎️ The operator kind ids this host indexes, sorted — the public read-back a law composes a
    /// table against without reaching into `kind_infos`.
    pub fn neuron_kind_ids(&self) -> Vec<String> {
        let mut ids: Vec<String> = self.kind_infos.keys().cloned().collect();
        ids.sort();
        ids
    }

    /// 📦️ Installs the operator kind infos from a content-addressed payload — see
    /// [`resolve_flow_shared_payload`].
    ///
    /// The table is composed of its parts in order (the app-static catalogue, then whatever records
    /// the scene itself derived), so a later id wins over an earlier one exactly as it did when the
    /// host concatenated the two sources into one body itself.
    pub fn set_neuron_kind_infos_payload(&mut self, payload: &str) -> Result<(), FlowCoreError> {
        let parts = resolve_flow_shared_payload_parts(payload)?;
        let mut infos: HistoryFoldIndex<String, OperatorInfo> = HistoryFoldIndex::new();
        for part in &parts {
            if part.trim().is_empty() {
                continue;
            }
            for info in semio_framework_pack_json::from_json_str::<Vec<OperatorInfo>>(part, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_default() {
                infos.insert(info.id.clone(), info);
            }
        }
        self.install_kind_infos(Arc::new(infos));
        Ok(())
    }

    /// 🧹️ Installs a new operator catalogue and retires the one it displaces — but ONLY when this
    /// host was its last owner. The catalogue is shared by every live host
    /// ([`FlowHost::kind_infos`]), and its `ChannelSpec::default` values are `Value`s that fail
    /// closed on a bare drop, so a plain assignment aborted the process the moment a host swapped
    /// a uniquely-owned catalogue out (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    fn install_kind_infos(&mut self, kind_infos: Arc<HistoryFoldIndex<String, OperatorInfo>>) {
        if let Some(displaced) = Arc::into_inner(std::mem::replace(&mut self.kind_infos, kind_infos)) {
            displaced.retire_cold();
        }
        self.rebuild_dag();
    }

    pub fn set_neuron_kind_infos_json(&mut self, json: &str) {
        self.install_kind_infos(Arc::new(if json.trim().is_empty() { HistoryFoldIndex::new() } else { semio_framework_pack_json::from_json_str::<Vec<OperatorInfo>>(json, semio_framework_pack_json::JsonMemberPolicy::Reject).map(|items| items.into_iter().map(|info| (info.id.clone(), info)).collect()).unwrap_or_default() }));
    }

    /// 🧠️ Same as `set_neuron_kind_infos_json` but over the already-built id-keyed map — the ONE
    /// in-process path, because the JSON form of this catalogue is ~108 kB and an evaluation tick
    /// that serialized and re-parsed it spent 11-26 ms per tick doing nothing else
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub fn set_neuron_kind_info_map(&mut self, infos: Arc<HistoryFoldIndex<String, OperatorInfo>>) {
        self.install_kind_infos(infos);
    }

    /// 🧠️ Same as `set_neuron_kind_infos_json` but over the typed `NodeGraphScene.operators` records.
    pub fn set_neuron_kind_infos(&mut self, infos: &[semio_framework_ui_declaration::ui::NodeGraphOperatorRecord]) {
        self.install_kind_infos(Arc::new(infos.iter().map(|record| (record.id.clone(), node_graph_operator_record_to_operator_info(record))).collect()));
    }

    pub fn evaluate(&mut self) -> Result<String, FlowCoreError> {
        self.evaluate_internal();
        Ok(self.last_eval_json.clone())
    }
}
