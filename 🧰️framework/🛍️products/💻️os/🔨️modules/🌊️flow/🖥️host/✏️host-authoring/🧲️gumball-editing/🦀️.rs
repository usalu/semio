//! 🧲️ Gumball editing of the flow host: inserting a widget between two others, making space and setting the neuron parameters. See [`super`].

use super::*;

// #region GumballEditing
impl FlowHost {
    /// 🔀️ Splices `mid_id` between `anchor_id` and its downstream consumers on `anchor_out_port`.
    pub fn insert_between(&mut self, anchor_id: &str, anchor_out_port: &str, mid_id: &str, mid_in_port: &str, mid_out_port: &str) -> Result<(), FlowCoreError> {
        self.begin_change();
        if !self.host_snapshot.widgets.iter().any(|widget| widget_id_for(widget) == anchor_id) {
            return Err(FlowCoreError::UnknownWidget(anchor_id.to_string()));
        }
        if !self.host_snapshot.widgets.iter().any(|widget| widget_id_for(widget) == mid_id) {
            return Err(FlowCoreError::UnknownWidget(mid_id.to_string()));
        }
        if anchor_id == mid_id {
            return Err(FlowCoreError::SelfInsertion);
        }
        if !widget_has_output(anchor_id, &self.host_snapshot.widgets, &self.host_snapshot.synapses, &self.kind_infos) {
            return Err(FlowCoreError::NoOutputPort(anchor_id.to_string()));
        }
        if !widget_has_input(mid_id, &self.host_snapshot.widgets, &self.host_snapshot.synapses, &self.kind_infos) {
            return Err(FlowCoreError::NoInputPort(mid_id.to_string()));
        }
        if !widget_has_output(mid_id, &self.host_snapshot.widgets, &self.host_snapshot.synapses, &self.kind_infos) {
            return Err(FlowCoreError::NoOutputPort(mid_id.to_string()));
        }
        let existing: Vec<(String, String)> = self.host_snapshot.synapses.iter().map(|synapse| (synapse.from.clone(), synapse.to.clone())).collect();
        if would_create_cycle(&existing, anchor_id, mid_id) {
            return Err(FlowCoreError::CycleWouldBeCreated);
        }
        let mid_has_input = self.host_snapshot.synapses.iter().any(|synapse| synapse.to == mid_id);
        let mut rerouted = Vec::new();
        if !mid_has_input {
            for synapse in &mut self.host_snapshot.synapses {
                if synapse.from == anchor_id && synapse.from_port == anchor_out_port {
                    synapse.from = mid_id.to_string();
                    synapse.from_port = mid_out_port.to_string();
                    rerouted.push(synapse.clone());
                }
            }
        }
        self.note_leaves(rerouted.into_iter().map(|synapse| FlowMutation::ChangeSynapse(ChangeSynapse { id: synapse.id.clone(), synapse })));
        if self.host_snapshot.synapses.iter().any(|synapse| synapse.from == anchor_id && synapse.from_port == anchor_out_port && synapse.to == mid_id && synapse.to_port == mid_in_port) {
            self.rebuild_dag();
            self.checked_change()?;
            return Ok(());
        }
        let synapse_id = self.next_synapse_id();
        let synapse = SynapseSpec { id: synapse_id, from: anchor_id.to_string(), to: mid_id.to_string(), from_port: anchor_out_port.to_string(), to_port: mid_in_port.to_string() };
        let added = FlowMutation::AddSynapse(AddSynapse { index: u32::try_from(self.host_snapshot.synapses.len()).unwrap_or(u32::MAX), synapse: synapse.clone() });
        self.host_snapshot.synapses.push(synapse);
        self.note_leaves([added]);
        self.rebuild_dag();
        self.checked_change()?;
        Ok(())
    }

    /// ↔ Shifts widgets to the right of `anchor_id` to open layout space for inserted nodes.
    pub fn make_space(&mut self, anchor_id: &str, dx: f64, dy: f64) -> Result<(), FlowCoreError> {
        self.begin_change();
        let anchor_x = self.host_snapshot.layout.get(anchor_id).map(|layout| layout.x).ok_or_else(|| FlowCoreError::UnknownWidgetLayout(anchor_id.to_string()))?;
        let previous = std::mem::take(&mut self.host_snapshot.layout);
        let mut shifted = Vec::new();
        for (widget_id, layout) in &previous {
            let mut layout = layout.clone();
            if layout.x > anchor_x {
                layout.x += dx;
                layout.y += dy;
                shifted.push(widget_id.clone());
            }
            let _ = self.dag.graph.set_widget_position(widget_id, layout.x, layout.y);
            self.host_snapshot.layout.insert(widget_id.clone(), layout);
        }
        let mut retirement = semio_framework_artifact_flow_flow::retained::FlowRetirement::default();
        retirement.push_cold(semio_framework_artifact_flow_flow::retained::FlowOwner::Layouts(previous));
        retirement.retire_cold();
        let moved = self.layout_leaf(shifted.iter().map(String::as_str));
        self.note_leaves(moved);
        self.checked_change()?;
        Ok(())
    }

    /// 🧬️ Merges JSON params into a neuron widget for compact transform values.
    pub fn set_neuron_params(&mut self, widget_id: &str, params_json: &str) -> Result<(), FlowCoreError> {
        self.begin_change();
        let patch: Dictionary = semio_framework_pack_json::from_json_str(params_json, semio_framework_pack_json::JsonMemberPolicy::Reject)?;
        let merged = match self.host_snapshot.widgets.iter_mut().find(|widget| widget_id_for(widget) == widget_id) {
            Some(Widget::Neuron { params, .. }) => Ok(std::mem::replace(params, params.merge(&patch))),
            Some(_) => Err(FlowCoreError::NotNeuron(widget_id.to_string())),
            None => Err(FlowCoreError::UnknownWidget(widget_id.to_string())),
        };
        self.displace_dictionary(patch);
        self.displace_dictionary(merged?);
        let changed = self.changed_widget_leaf(widget_id);
        self.note_leaves(changed);
        self.sync_dag_display_from_widgets();
        self.checked_change()?;
        Ok(())
    }
}
// #endregion GumballEditing
