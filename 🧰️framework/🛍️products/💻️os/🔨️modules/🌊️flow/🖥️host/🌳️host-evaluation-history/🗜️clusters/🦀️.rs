//! 🗜️ Clusters of the flow host: collapsing a selection into a cluster and exploding a cluster. See [`super`].

use super::*;

impl FlowHost {
    /// 🧩️ Collapses the selected widgets into one cluster neuron.
    pub fn collapse_selection(&mut self, selected_ids: &[String]) -> Result<String, FlowCoreError> {
        if selected_ids.len() < 2 {
            return Err(FlowCoreError::CollapseNeedsTwoWidgets);
        }
        let selected: BTreeSet<String> = selected_ids.iter().cloned().collect();
        if !selected.iter().all(|id| self.host_snapshot.widgets.iter().any(|widget| widget_id_for(widget) == id)) {
            return Err(FlowCoreError::CollapseUnknownWidgets);
        }
        if selected.iter().any(|id| self.host_snapshot.widgets.iter().any(|widget| widget_id_for(widget) == id && matches!(widget, Widget::Cluster { .. }))) {
            return Err(FlowCoreError::CollapseContainsClusters);
        }
        self.begin_change();
        let severed: Vec<String> = self.host_snapshot.synapses.iter().filter(|synapse| selected.contains(&synapse.from) || selected.contains(&synapse.to)).map(|synapse| synapse.id.clone()).collect();
        let unplaced: Vec<FlowLayoutEntry> = selected.iter().filter(|id| self.host_snapshot.layout.contains_key(id.as_str())).map(|id| FlowLayoutEntry { id: id.clone(), layout: None }).collect();
        let mut crossing_external = Vec::new();
        for synapse in &self.host_snapshot.synapses {
            let from_selected = selected.contains(&synapse.from);
            let to_selected = selected.contains(&synapse.to);
            if (from_selected || to_selected) && !(from_selected && to_selected) {
                crossing_external.push(synapse.clone());
            }
        }
        let boundary_variables = boundary_variable_widget_ids(&selected, &crossing_external, &self.host_snapshot.widgets);
        let mut inner_neurons = Vec::new();
        let mut inner_layout = BTreeMap::new();
        for widget in &self.host_snapshot.widgets {
            let id = widget_id_for(widget).to_string();
            if !selected.contains(&id) {
                continue;
            }
            if boundary_variables.contains(&id) {
                continue;
            }
            if let Some(neuron) = widget_to_inner_neuron(widget) {
                inner_neurons.push(neuron);
            }
            if let Some(layout) = self.host_snapshot.layout.get(&id) {
                inner_layout.insert(id, layout.clone());
            }
        }
        let mut inner_synapses = Vec::new();
        let mut retained_external = Vec::new();
        for synapse in &self.host_snapshot.synapses {
            let from_selected = selected.contains(&synapse.from);
            let to_selected = selected.contains(&synapse.to);
            if from_selected && to_selected {
                if boundary_variables.contains(&synapse.from) || boundary_variables.contains(&synapse.to) {
                    continue;
                }
                inner_synapses.push(Synapse { id: synapse.id.clone(), from: synapse.from.clone(), to: synapse.to.clone(), from_port: synapse.from_port.clone(), to_port: synapse.to_port.clone() });
            } else if from_selected || to_selected {
            } else {
                retained_external.push(synapse.clone());
            }
        }
        let mut used_channels = BTreeSet::new();
        let mut input_serial = 0usize;
        let mut output_serial = 0usize;
        let mut boundary_index = 0usize;
        let mut cluster_external = Vec::new();
        let channels=self.current_channels.clone().or_else(||self.previous_channels.clone());
        let empty_outputs=HistoryFoldIndex::new();
        let outputs=channels.as_deref().map_or(&empty_outputs,|channels|&channels.outputs);
        let kind_infos = self.kind_infos.clone();
        let widgets = self.host_snapshot.widgets.clone();
        let synapses_snapshot = self.host_snapshot.synapses.clone();
        for synapse in crossing_external {
            let from_selected = selected.contains(&synapse.from);
            let to_selected = selected.contains(&synapse.to);
            if to_selected && !from_selected {
                let inner_target = if boundary_variables.contains(&synapse.to) {
                    self.host_snapshot.synapses.iter().find(|entry| entry.from == synapse.to && selected.contains(&entry.to)).map_or_else(|| (synapse.to.clone(), synapse.to_port.clone()), |entry| (entry.to.clone(), entry.to_port.clone()))
                } else {
                    (synapse.to.clone(), synapse.to_port.clone())
                };
                let (channel, schema) = if let Some((name, schema)) = variable_widget_meta(&widgets, &synapse.to) {
                    let schema = if schema.is_empty() { infer_port_schema(&outputs, &kind_infos, &widgets, &synapses_snapshot, &synapse.from, &synapse.from_port) } else { schema };
                    (name, schema)
                } else {
                    let channel = unique_generated_boundary_name("input", &mut input_serial, &used_channels);
                    let schema = infer_port_schema(&outputs, &kind_infos, &widgets, &synapses_snapshot, &synapse.from, &synapse.from_port);
                    (channel, schema)
                };
                used_channels.insert(channel.clone());
                boundary_index += 1;
                let boundary_id = format!("__in_{boundary_index}");
                inner_neurons.push(Neuron::with_kind(&boundary_id, INPUT_KIND, contract_boundary_params(&channel, &schema)));
                inner_synapses.push(Synapse { id: format!("{boundary_id}_link"), from: boundary_id, to: inner_target.0, from_port: String::new(), to_port: inner_target.1 });
                cluster_external.push(SynapseSpec { id: synapse.id.clone(), from: synapse.from.clone(), to: String::new(), from_port: synapse.from_port.clone(), to_port: channel });
            } else if from_selected && !to_selected {
                let inner_source = if boundary_variables.contains(&synapse.from) {
                    self.host_snapshot.synapses.iter().find(|entry| entry.to == synapse.from && selected.contains(&entry.from)).map_or_else(|| (synapse.from.clone(), synapse.from_port.clone()), |entry| (entry.from.clone(), entry.from_port.clone()))
                } else {
                    (synapse.from.clone(), synapse.from_port.clone())
                };
                let (channel, schema) = if let Some((name, schema)) = variable_widget_meta(&widgets, &synapse.from) {
                    let schema = if schema.is_empty() { infer_port_schema(&outputs, &kind_infos, &widgets, &synapses_snapshot, &inner_source.0, &inner_source.1) } else { schema };
                    (name, schema)
                } else {
                    let channel = unique_generated_boundary_name("output", &mut output_serial, &used_channels);
                    let schema = infer_port_schema(&outputs, &kind_infos, &widgets, &synapses_snapshot, &inner_source.0, &inner_source.1);
                    (channel, schema)
                };
                used_channels.insert(channel.clone());
                boundary_index += 1;
                let boundary_id = format!("__out_{boundary_index}");
                inner_neurons.push(Neuron::with_kind(&boundary_id, OUTPUT_KIND, contract_boundary_params(&channel, &schema)));
                inner_synapses.push(Synapse { id: format!("{boundary_id}_link"), from: inner_source.0, to: boundary_id, from_port: inner_source.1, to_port: String::new() });
                cluster_external.push(SynapseSpec { id: synapse.id.clone(), from: String::new(), to: synapse.to.clone(), from_port: channel, to_port: synapse.to_port.clone() });
            }
        }
        let (sum_x, sum_y, layout_count) = selected.iter().filter_map(|id| self.host_snapshot.layout.get(id)).fold((0.0, 0.0, 0usize), |(sx, sy, count), layout| (sx + layout.x, sy + layout.y, count + 1));
        let count = layout_count.max(1) as f64;
        let cluster_x = sum_x / count;
        let cluster_y = sum_y / count;
        self.next_widget_serial += 1;
        let cluster_id = format!("cluster_{}", self.next_widget_serial);
        let inner_tree = Tree { neurons: inner_neurons, synapses: inner_synapses };
        let cluster = Widget::Cluster {
            id: cluster_id.clone(),
            name: "Cluster".into(),
            tree: inner_tree,
            flow: FlowGui { camera: CameraJson { x: 0.0, y: 0.0, zoom: 1.0 }, nodes: inner_layout.into_iter().map(|(id, layout)| (id, FlowNodeGui { layout, chrome: NodeChrome::Plain { preview: true } })).collect(), previews: vec![] },
        };
        self.host_snapshot.widgets.retain(|widget| !selected.contains(widget_id_for(widget)));
        let clustered = FlowMutation::AddWidget(AddWidget { index: u32::try_from(self.host_snapshot.widgets.len()).unwrap_or(u32::MAX), widget: cluster.clone() });
        self.host_snapshot.widgets.push(cluster);
        for id in &selected {
            self.host_snapshot.layout.remove(id);
        }
        self.host_snapshot.layout.insert(cluster_id.clone(), WidgetLayout { x: cluster_x, y: cluster_y });
        let retained_count = retained_external.len();
        self.host_snapshot.synapses = retained_external;
        for synapse in cluster_external {
            if synapse.to.is_empty() {
                self.host_snapshot.synapses.push(SynapseSpec { id: synapse.id, from: synapse.from, to: cluster_id.clone(), from_port: synapse.from_port, to_port: synapse.to_port });
            } else {
                self.host_snapshot.synapses.push(SynapseSpec { id: synapse.id, from: cluster_id.clone(), to: synapse.to, from_port: synapse.from_port, to_port: synapse.to_port });
            }
        }
        let unplaced = (!unplaced.is_empty()).then(|| FlowMutation::ChangeLayout(ChangeLayout { entries: unplaced }));
        let rewired: Vec<FlowMutation> = self.host_snapshot.synapses[retained_count..].iter().enumerate().map(|(offset, synapse)| FlowMutation::AddSynapse(AddSynapse { index: u32::try_from(retained_count + offset).unwrap_or(u32::MAX), synapse: synapse.clone() })).collect();
        let placed = self.layout_leaf([cluster_id.as_str()]);
        self.note_leaves(
            severed.into_iter().map(|id| FlowMutation::RemoveSynapse(RemoveSynapse { id })).chain(unplaced).chain(selected.iter().map(|id| FlowMutation::RemoveWidget(RemoveWidget { id: id.clone() }))).chain([clustered]).chain(placed).chain(rewired),
        );
        self.rebuild_dag();
        self.checked_change()?;
        Ok(cluster_id)
    }

    /// 💥️ Explodes a cluster back into its inner widgets.
    pub fn explode_cluster(&mut self, cluster_id: &str) -> Result<(), FlowCoreError> {
        let cluster_index = self.host_snapshot.widgets.iter().position(|widget| matches!(widget, Widget::Cluster { id, .. } if id == cluster_id)).ok_or_else(|| FlowCoreError::UnknownCluster(cluster_id.to_string()))?;
        let exploded = self.host_snapshot.widgets[cluster_index].clone();
        let Widget::Cluster { tree, flow, .. } = &exploded else {
            exploded.retire_cold();
            return Err(FlowCoreError::WidgetNotCluster(cluster_id.to_string()));
        };
        let cluster_layout = self.host_snapshot.layout.get(cluster_id).cloned().unwrap_or(WidgetLayout { x: 0.0, y: 0.0 });
        self.begin_change();
        let severed: Vec<String> = self.host_snapshot.synapses.iter().filter(|synapse| synapse.from == cluster_id || synapse.to == cluster_id).map(|synapse| synapse.id.clone()).collect();
        let had_layout = self.host_snapshot.layout.contains_key(cluster_id);
        let mut boundary_channels: HashMap<String, (String, String)> = HashMap::new();
        for neuron in &tree.neurons {
            if neuron.kind == INPUT_KIND {
                let channel = neuron.params.get("channel").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()).unwrap_or(neuron.id.as_str()).to_string();
                boundary_channels.insert(channel.clone(), (format!("{cluster_id}/{}", neuron.id), channel));
            } else if neuron.kind == OUTPUT_KIND {
                let channel = neuron.params.get("channel").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()).unwrap_or(neuron.id.as_str()).to_string();
                boundary_channels.insert(channel.clone(), (format!("{cluster_id}/{}", neuron.id), channel));
            }
        }
        let mut restored_widgets = Vec::new();
        for neuron in &tree.neurons {
            let namespaced_id = format!("{cluster_id}/{}", neuron.id);
            if neuron.kind == INPUT_KIND || neuron.kind == OUTPUT_KIND {
                let widget = neuron_to_exploded_widget(neuron);
                let widget = match widget {
                    Widget::Variable { name, schema, .. } => Widget::Variable { id: namespaced_id.clone(), name, schema },
                    other => other,
                };
                let layout = flow.nodes.get(&neuron.id).map_or(WidgetLayout { x: 0.0, y: 0.0 }, |node| node.layout.clone());
                self.host_snapshot.layout.insert(namespaced_id.clone(), WidgetLayout { x: cluster_layout.x + layout.x, y: cluster_layout.y + layout.y });
                restored_widgets.push((namespaced_id, neuron.id.clone(), widget));
                continue;
            }
            let mut widget = neuron_to_exploded_widget(neuron);
            match &mut widget {
                Widget::Neuron { id, .. } | Widget::InputSlider { id, .. } | Widget::InputNote { id, .. } | Widget::InputImage { id, .. } | Widget::Variable { id, .. } => *id = namespaced_id.clone(),
                _ => {}
            }
            let layout = flow.nodes.get(&neuron.id).map_or(WidgetLayout { x: 0.0, y: 0.0 }, |node| node.layout.clone());
            self.host_snapshot.layout.insert(namespaced_id.clone(), WidgetLayout { x: cluster_layout.x + layout.x, y: cluster_layout.y + layout.y });
            restored_widgets.push((namespaced_id, neuron.id.clone(), widget));
        }
        let id_map: HashMap<String, String> = restored_widgets.iter().map(|(namespaced, original, _)| (original.clone(), namespaced.clone())).collect();
        self.host_snapshot.widgets.remove(cluster_index).retire_cold();
        self.host_snapshot.layout.remove(cluster_id);
        let mut restored_ids = Vec::with_capacity(restored_widgets.len());
        let mut restored_leaves = Vec::with_capacity(restored_widgets.len());
        for (namespaced_id, _, widget) in restored_widgets {
            restored_leaves.push(FlowMutation::AddWidget(AddWidget { index: u32::try_from(self.host_snapshot.widgets.len()).unwrap_or(u32::MAX), widget: widget.clone() }));
            restored_ids.push(namespaced_id);
            self.host_snapshot.widgets.push(widget);
        }
        let mut next_synapses = Vec::new();
        let mut rewired = Vec::new();
        for synapse in &self.host_snapshot.synapses {
            if synapse.to == cluster_id {
                if let Some((variable_id, variable_port)) = boundary_channels.get(&synapse.to_port) {
                    rewired.push(next_synapses.len());
                    next_synapses.push(SynapseSpec { id: synapse.id.clone(), from: synapse.from.clone(), to: variable_id.clone(), from_port: synapse.from_port.clone(), to_port: variable_port.clone() });
                    continue;
                }
            } else if synapse.from == cluster_id {
                if let Some((variable_id, variable_port)) = boundary_channels.get(&synapse.from_port) {
                    rewired.push(next_synapses.len());
                    next_synapses.push(SynapseSpec { id: synapse.id.clone(), from: variable_id.clone(), to: synapse.to.clone(), from_port: variable_port.clone(), to_port: synapse.to_port.clone() });
                    continue;
                }
            } else {
                next_synapses.push(synapse.clone());
            }
        }
        for synapse in &tree.synapses {
            let Some(from) = id_map.get(&synapse.from) else { continue };
            let Some(to) = id_map.get(&synapse.to) else { continue };
            let from_port = tree
                .neurons
                .iter()
                .find(|neuron| neuron.id == synapse.from && neuron.kind == INPUT_KIND)
                .and_then(|neuron| neuron.params.get("channel").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str())).map_or_else(|| synapse.from_port.clone(), str::to_string);
            let to_port = tree
                .neurons
                .iter()
                .find(|neuron| neuron.id == synapse.to && neuron.kind == OUTPUT_KIND)
                .and_then(|neuron| neuron.params.get("channel").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str())).map_or_else(|| synapse.to_port.clone(), str::to_string);
            let id = self.next_synapse_id();
            rewired.push(next_synapses.len());
            next_synapses.push(SynapseSpec { id, from: from.clone(), to: to.clone(), from_port, to_port });
        }
        let reconnected: Vec<FlowMutation> = rewired.into_iter().map(|index| FlowMutation::AddSynapse(AddSynapse { index: u32::try_from(index).unwrap_or(u32::MAX), synapse: next_synapses[index].clone() })).collect();
        self.host_snapshot.synapses = next_synapses;
        exploded.retire_cold();
        let unplaced = had_layout.then(|| FlowMutation::ChangeLayout(ChangeLayout { entries: vec![FlowLayoutEntry { id: cluster_id.to_string(), layout: None }] }));
        let placed = self.layout_leaf(restored_ids.iter().map(String::as_str));
        self.note_leaves(
            severed.into_iter().map(|id| FlowMutation::RemoveSynapse(RemoveSynapse { id })).chain(unplaced).chain([FlowMutation::RemoveWidget(RemoveWidget { id: cluster_id.to_string() })]).chain(restored_leaves).chain(placed).chain(reconnected),
        );
        self.rebuild_dag();
        self.checked_change()?;
        Ok(())
    }
}
