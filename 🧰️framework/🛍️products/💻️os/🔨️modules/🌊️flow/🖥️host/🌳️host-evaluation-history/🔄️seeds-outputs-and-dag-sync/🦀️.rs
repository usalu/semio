//! 🔄️ Seeds, outputs and DAG sync of the flow host: the compiled wire literal, the evaluation seeds, preview and export outputs and the DAG display, ghost and rebuild synchronisation. See [`super`].

use super::*;

impl FlowHost {
    /// 📝️ Renders the compiled DAG fixture as wire-literal text.
    pub fn compiled_wire_literal(&self) -> String {
        dag_host_snapshot_to_wire_literal(&self.build_dag_host_snapshot_v1())
    }

    pub fn build_seeds(&self) -> HashMap<String, Dictionary> {
        let mut seeds = HashMap::new();
        for widget in &self.host_snapshot.widgets {
            match widget {
                Widget::InputSlider { id, value, .. } => {
                    seeds.insert(id.clone(), channel_output("number", Dictionary::with_schema("number").insert("value", NeuralValue::Atom(Atom::Decimal(*value)))));
                }
                Widget::InputNote { id, text } => {
                    seeds.insert(id.clone(), channel_output("text", Dictionary::with_schema("text").insert("value", NeuralValue::Atom(Atom::String(text.clone())))));
                }
                Widget::InputImage { id, src } => {
                    seeds.insert(id.clone(), channel_output("image", Dictionary::with_schema("image").insert("dataUrl", NeuralValue::Atom(Atom::String(src.clone())))));
                }
                _ => {}
            }
        }
        seeds
    }

    pub(in super::super) fn apply_preview_outputs(&mut self, outputs: &HistoryFoldIndex<String, Dictionary>) {
        for widget in &mut self.host_snapshot.widgets {
            if let Widget::OutputPreview { id, preview, .. } = widget {
                if let Some(out) = outputs.get(id) {
                    std::mem::replace(preview, out.clone()).retire_cold();
                } else if let Some(syn) = self.host_snapshot.synapses.iter().find(|s| s.to == *id) {
                    if let Some(src) = outputs.get(&syn.from) {
                        std::mem::replace(preview, preview_dict_from_connection(src, &syn.from_port, &syn.to_port)).retire_cold();
                    }
                }
            }
        }
        self.sync_dag_display_from_widgets();
        self.dag.graph.fit_preview_sizes();
    }

    pub(in super::super) fn apply_export_outputs(&mut self, outputs: &HistoryFoldIndex<String, Dictionary>) {
        for widget in &self.host_snapshot.widgets {
            if let Widget::OutputExport { id, .. } = widget {
                if let Some(out) = outputs.get(id) {
                    self.export_payloads.insert(id.clone(), out.clone());
                } else if let Some(syn) = self.host_snapshot.synapses.iter().find(|s| s.to == *id) {
                    if let Some(src) = outputs.get(&syn.from) {
                        let payload = preview_dict_from_connection(src, &syn.from_port, &syn.to_port);
                        self.export_payloads.insert(id.clone(), payload);
                    }
                }
            }
        }
    }

    pub fn export_payload_json(&self, widget_id: &str) -> Result<String, FlowCoreError> {
        let payload = self.export_payloads.get(widget_id).cloned().unwrap_or_default();
        let json = semio_framework_pack_json::to_json_string(&payload);
        payload.retire_cold();
        Ok(json)
    }

    /// 📤️ Returns and clears a pending export control click from the last pointer hit.
    pub fn take_pending_export_click(&mut self) -> Option<String> {
        self.dag.pointer.take_pending_export_click()
    }

    pub(in super::super) fn sync_dag_display_from_widgets(&mut self) {
        for widget in &self.host_snapshot.widgets {
            let id = widget_id_for(widget);
            let Some(node) = self.dag.graph.host_snapshot.nodes.iter_mut().find(|n| n.id == *id) else {
                continue;
            };
            match (widget, &mut node.kind) {
                (Widget::InputSlider { value, .. }, DagNodeKind::Slider { value: dag_value, .. }) => {
                    *dag_value = *value;
                }
                (Widget::InputNote { text, .. }, DagNodeKind::Note { text: dag_text, .. }) => {
                    *dag_text = text.clone();
                }
                (Widget::InputImage { src, .. }, DagNodeKind::Image { src: dag_src, .. }) => {
                    *dag_src = src.clone();
                }
                (Widget::OutputPreview { preview, expanded, .. }, DagNodeKind::Preview { content, expanded: dag_expanded, .. }) => {
                    *content = dag_preview_content_from_dict(preview);
                    *dag_expanded = expanded.iter().cloned().collect();
                }
                (Widget::OutputAction { action, .. }, DagNodeKind::Action { label, .. }) => {
                    *label = action.clone();
                }
                (Widget::OutputExport { format, .. }, DagNodeKind::Export { label, format: dag_format, .. }) => {
                    *label = format.to_uppercase();
                    *dag_format = format.clone();
                }
                _ => {}
            }
        }
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    pub fn sync_dag_ghost(&mut self) {
        self.dag.pointer.set_ghost_node(self.ghost_node.clone());
    }

    pub fn rebuild_dag(&mut self) {
        let fixture = self.build_dag_host_snapshot_v1();
        if self.pending_change || self.gesture_active {
            let kept: BTreeSet<&str> = fixture.edges.iter().map(|edge| edge.id.as_str()).collect();
            let dropped: Vec<FlowMutation> = self.host_snapshot.synapses.iter().filter(|synapse| !kept.contains(synapse.id.as_str())).map(|synapse| FlowMutation::RemoveSynapse(RemoveSynapse { id: synapse.id.clone() })).collect();
            self.pending_leaves.extend(dropped);
        }
        let theme = self.dag.viewport.canvas_theme;
        let automatic_lod = self.dag.viewport.automatic_lod();
        let forced_draw_lod = self.dag.viewport.forced_draw_lod_label().map(str::to_string);
        let ghost = self.ghost_node.clone();
        self.dag.replace_host_snapshot_without_layout(fixture);
        self.dag.viewport.canvas_theme = theme;
        self.dag.viewport.set_viewport(self.viewport_w, self.viewport_h, self.viewport_dpr);
        self.dag.viewport.set_automatic_lod(automatic_lod);
        if let Some(label) = forced_draw_lod {
            self.dag.viewport.set_forced_draw_lod_label(&label);
        }
        self.dag.pointer.set_ghost_node(ghost);
        self.dag.minimap.set_minimap_widget_visible(true);
        self.sync_preview_dimmed();
        self.sync_from_dag();
    }

    pub(super) fn sync_preview_dimmed(&mut self) {
        let off = self.preview_off_widget_ids();
        set_dimmed(&mut self.dag.graph, &mut self.dag.paint, &off);
    }
}
