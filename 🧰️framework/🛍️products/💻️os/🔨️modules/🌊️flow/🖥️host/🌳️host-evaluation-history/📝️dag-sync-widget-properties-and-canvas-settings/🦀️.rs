//! 📝️ DAG sync, widget properties and canvas settings of the flow host: syncing from the DAG, the host snapshot, slider, note, variable and image edits and the canvas theme, level of detail and grid settings. See [`super`].

use super::*;

impl FlowHost {
    pub(in super::super) fn sync_from_dag(&mut self) {
        self.evaluation_revision=self.evaluation_revision.wrapping_add(1);
        let dag_ids: BTreeSet<String> = self.dag.graph.host_snapshot.nodes.iter().map(|node| node.id.clone()).collect();
        self.host_snapshot.widgets.retain(|widget| dag_ids.contains(widget_id_for(widget)));
        for node in &self.dag.graph.host_snapshot.nodes {
            self.host_snapshot.layout.insert(node.id.clone(), WidgetLayout { x: node.x, y: node.y });
        }
        for widget in &mut self.host_snapshot.widgets {
            let id = widget_id_for(widget);
            let Some(node) = self.dag.graph.host_snapshot.nodes.iter().find(|n| n.id == *id) else {
                continue;
            };
            match (widget, &node.kind) {
                (Widget::InputSlider { value, .. }, DagNodeKind::Slider { value: dag_value, .. }) => {
                    *value = *dag_value;
                }
                (Widget::InputNote { text, .. }, DagNodeKind::Note { text: dag_text, .. }) => {
                    *text = dag_text.clone();
                }
                (Widget::InputImage { src, .. }, DagNodeKind::Image { src: dag_src, .. }) => {
                    *src = dag_src.clone();
                }
                (Widget::OutputPreview { expanded, .. }, DagNodeKind::Preview { expanded: dag_expanded, .. }) => {
                    std::mem::replace(expanded, dag_expanded.iter().cloned().collect()).retire_cold();
                }
                (Widget::OutputAction { action, .. }, DagNodeKind::Action { label, .. }) => {
                    *action = label.clone();
                }
                (Widget::OutputExport { format, .. }, DagNodeKind::Export { format: dag_format, .. }) => {
                    *format = dag_format.clone();
                }
                _ => {}
            }
        }
        self.host_snapshot.synapses = self
            .dag.graph
            .host_snapshot
            .edges
            .iter()
            .map(|edge| {
                let (from, from_port) = parse_port_endpoint(&edge.source, "");
                let (to, to_port) = parse_port_endpoint(&edge.target, "");
                SynapseSpec { id: edge.id.clone(), from, to, from_port, to_port }
            })
            .collect();
        self.host_snapshot.camera = CameraJson { x: self.dag.graph.host_snapshot.camera.x, y: self.dag.graph.host_snapshot.camera.y, zoom: self.dag.graph.host_snapshot.camera.zoom };
    }

    pub(in super::super) fn build_dag_host_snapshot_v1(&self) -> DagHostSnapshot {
        let mut seen = BTreeSet::new();
        let nodes: Vec<DagNodeSpec> =
            self.host_snapshot.widgets.iter().enumerate().filter(|(_, widget)| seen.insert(widget_id_for(widget).to_string())).map(|(i, w)| widget_to_dag_node(w, i, &self.host_snapshot.layout, &self.host_snapshot.synapses, &self.kind_infos, widget_node_size(w, &self.host_snapshot.synapses, &self.kind_infos))).collect();
        let existing: Vec<(String, String)> = self.host_snapshot.synapses.iter().map(|s| (s.from.clone(), s.to.clone())).collect();
        let edges: Vec<DagHostSnapshotEdge> = self
            .host_snapshot
            .synapses
            .iter()
            .filter_map(|syn| {
                if would_create_cycle(&existing.iter().filter(|(a, b)| !(a == &syn.from && b == &syn.to)).cloned().collect::<Vec<_>>(), &syn.from, &syn.to) {
                    return None;
                }
                let from_port = resolve_synapse_port(&syn.from, &syn.from_port, PortSide::Output, &self.host_snapshot.widgets, &self.host_snapshot.synapses, &self.kind_infos).unwrap_or_else(|| syn.from_port.clone());
                let to_port = resolve_synapse_port(&syn.to, &syn.to_port, PortSide::Input, &self.host_snapshot.widgets, &self.host_snapshot.synapses, &self.kind_infos).unwrap_or_else(|| syn.to_port.clone());
                Some(DagHostSnapshotEdge { id: syn.id.clone(), source: format!("{}@{}", syn.from, from_port), target: format!("{}@{}", syn.to, to_port), route_style: EdgeRouteStyle::default(), properties: PropertyBag::new() })
            })
            .collect();
        DagHostSnapshot { schema: "dag.hostDocument".into(), camera: semio_framework_os_infinite_directed_presentation::DagCamera { x: self.host_snapshot.camera.x, y: self.host_snapshot.camera.y, zoom: self.host_snapshot.camera.zoom }, nodes, edges }
    }

    pub(in super::super) fn screen_to_world_point(&self, sx: f64, sy: f64) -> canvas::Point {
        use canvas::camera::{screen_to_world, Camera, Viewport};
        use canvas::Point;
        let cam = Camera { x: self.host_snapshot.camera.x, y: self.host_snapshot.camera.y, zoom: self.host_snapshot.camera.zoom };
        let viewport = Viewport { width: self.viewport_w, height: self.viewport_h, dpr: self.viewport_dpr };
        screen_to_world(&cam, &viewport, Point::new(sx, sy))
    }

    pub(in super::super) fn next_widget_id(&mut self, descriptor: &WidgetDescriptor) -> String {
        let (id, serial) = generated_widget_id(descriptor, self.host_snapshot.widgets.iter().map(widget_id_for));
        self.next_widget_serial = serial;
        id
    }

    pub fn set_slider_value(&mut self, widget_id: &str, value: f64) {
        self.begin_change();
        for widget in &mut self.host_snapshot.widgets {
            if let Widget::InputSlider { id, .. } = widget {
                if id == widget_id {
                    set_widget_slider_value(widget, value);
                }
            }
        }
        let changed = self.changed_widget_leaf(widget_id);
        self.note_leaves(changed);
        self.sync_dag_display_from_widgets();
        self.refresh_computing_chrome_from_pending();
        self.finish_change();
    }

    pub fn slider_overlay_state_json(&self) -> Result<String, FlowCoreError> {
        Ok(slider_overlay_state_json(&self.dag.viewport, &self.dag.graph)?)
    }

    pub fn set_note_text(&mut self, widget_id: &str, text: &str) {
        self.begin_change();
        for widget in &mut self.host_snapshot.widgets {
            if let Widget::InputNote { id, text: note } = widget {
                if id == widget_id {
                    *note = text.to_string();
                }
            }
        }
        let changed = self.changed_widget_leaf(widget_id);
        self.note_leaves(changed);
        self.sync_dag_display_from_widgets();
        self.dag.graph.fit_note_sizes();
        self.refresh_computing_chrome_from_pending();
        self.finish_change();
    }

    /// ✏️ Begins inline note editing for a widget at a world-space click.
    pub fn begin_note_edit(&mut self, widget_id: &str, world_x: f64, world_y: f64) {
        self.begin_gesture();
        self.edited_note = Some(widget_id.to_string());
        begin_note_edit(&mut self.dag.graph, &mut self.dag.notes, widget_id, world_x, world_y);
    }

    /// ✏️ Inserts text into the active note editor.
    pub fn note_insert_text(&mut self, chunk: &str) {
        if !note_insert_text(&mut self.dag.graph, &mut self.dag.notes, chunk) {
            return;
        }
        self.sync_from_dag();
    }

    /// ✏️ Backspaces in the active note editor.
    pub fn note_backspace(&mut self) {
        if !note_backspace(&mut self.dag.graph, &mut self.dag.notes) {
            return;
        }
        self.sync_from_dag();
    }

    /// ✏️ Deletes forward in the active note editor.
    pub fn note_delete_forward(&mut self) {
        if !note_delete_forward(&mut self.dag.graph, &mut self.dag.notes) {
            return;
        }
        self.sync_from_dag();
    }

    /// ✏️ Moves the active note caret.
    pub fn note_move_caret(&mut self, direction: &str, extend: bool) {
        if !note_move_caret(&mut self.dag.graph, &mut self.dag.notes, direction, extend) {
            return;
        }
        self.sync_from_dag();
    }

    /// ✏️ Commits inline note editing into fixture history.
    pub fn note_commit_edit(&mut self) {
        self.dag.notes.note_commit_edit();
        self.sync_from_dag();
        self.commit_gesture_history();
    }

    /// ✏️ Toggles native caret visibility while editing a note.
    pub fn set_note_caret_visible(&mut self, visible: bool) {
        self.dag.notes.set_note_caret_visible(visible);
    }

    pub fn schemas_json(&self) -> Result<String, FlowCoreError> {
        let refs = self.operator_registry().schema_refs();
        Ok(semio_framework_pack_json::to_json_string(&refs))
    }

    pub fn set_variable_name(&mut self, widget_id: &str, name: &str) {
        let trimmed = name.trim();
        if trimmed.is_empty() {
            return;
        }
        self.begin_change();
        for widget in &mut self.host_snapshot.widgets {
            if let Widget::Variable { id, name: variable_name, .. } = widget {
                if id == widget_id {
                    *variable_name = trimmed.to_string();
                }
            }
        }
        let changed = self.changed_widget_leaf(widget_id);
        self.note_leaves(changed);
        self.rebuild_dag();
        self.finish_change();
    }

    pub fn set_variable_schema(&mut self, widget_id: &str, schema: &str) {
        let trimmed = schema.trim();
        if trimmed.is_empty() {
            return;
        }
        self.begin_change();
        for widget in &mut self.host_snapshot.widgets {
            if let Widget::Variable { id, schema: variable_schema, .. } = widget {
                if id == widget_id {
                    *variable_schema = trimmed.to_string();
                }
            }
        }
        let changed = self.changed_widget_leaf(widget_id);
        self.note_leaves(changed);
        self.rebuild_dag();
        self.finish_change();
    }

    pub fn set_image_src(&mut self, widget_id: &str, src: &str) {
        self.begin_change();
        for widget in &mut self.host_snapshot.widgets {
            if let Widget::InputImage { id, src: image } = widget {
                if id == widget_id {
                    *image = src.to_string();
                }
            }
        }
        let changed = self.changed_widget_leaf(widget_id);
        self.note_leaves(changed);
        self.sync_dag_display_from_widgets();
        self.dag.graph.fit_preview_sizes();
        self.refresh_computing_chrome_from_pending();
        self.finish_change();
    }

    pub fn preview_text(&self) -> String {
        self.host_snapshot
            .widgets
            .iter()
            .find_map(|w| match w {
                Widget::OutputPreview { preview, .. } => Some(preview_content_summary(&dag_preview_content_from_dict(preview))),
                _ => None,
            })
            .unwrap_or_else(|| "—".into())
    }

    pub fn set_canvas_theme_from_json(&mut self, json: &str) -> Result<(), FlowCoreError> {
        let mut accepted = |_| true;
        let mut control = semio_framework_value::NativeDecodeControl::new(64 * 1024, &mut accepted);
        let overlay = semio_framework_os_infinite::board::io::text::palette::decode_board_palette_overlay_json(json, &mut control).map_err(|error| FlowCoreError::Json(error.into_message()))?;
        dag_set_canvas_palette(&mut self.dag.viewport, &mut self.dag.paint, &overlay);
        Ok(())
    }

    pub fn set_canvas_theme_dark(&mut self, dark: bool) {
        self.dag.viewport.canvas_theme = dag::CanvasPalette::from_board_palette(if dark { &ui_styling::BOARD_DARK } else { &ui_styling::BOARD_LIGHT });
    }

    pub fn paint_scene(&self, scene: &mut canvas::Scene, width: u32, height: u32, dpr: f64) {
        self.dag.paint_scene(scene, width, height, dpr);
    }

    pub fn set_automatic_lod(&mut self, enabled: bool) {
        self.dag.viewport.set_automatic_lod(enabled);
    }

    pub fn set_proximity_distance(&mut self, world: f64) {
        self.dag.graph.set_proximity_distance(world);
    }

    pub fn set_forced_draw_lod_label(&mut self, label: &str) {
        self.dag.viewport.set_forced_draw_lod_label(label);
    }

    pub fn set_grid_visible(&mut self, visible: bool) {
        self.dag.viewport.set_grid_visible(visible);
    }

    pub fn set_grid_snap_enabled(&mut self, enabled: bool) {
        self.dag.viewport.set_grid_snap_enabled(enabled);
    }

    pub fn set_grid_factor(&mut self, factor: f64) -> Result<(), FlowCoreError> {
        self.dag.viewport.set_grid_factor(factor)?;
        Ok(())
    }

    pub fn focus_selection_camera(&self, pad: f64) -> Option<CameraJson> {
        focus_selection_camera(&self.dag.viewport, &self.dag.graph, pad).map(|camera| CameraJson { x: camera.x, y: camera.y, zoom: camera.zoom })
    }

    pub fn draw_lod_label(&self) -> &'static str {
        dag_draw_lod_label(&self.dag.viewport, &self.dag.graph)
    }

    pub fn label_overlay_paint_state_json(&self) -> Result<String, FlowCoreError> {
        Ok(self.dag.label_overlay_paint_state_json()?)
    }

    /// 💥️ Returns and clears a pending cluster explode target from the last pointer hit.
    pub fn take_pending_cluster_explode(&mut self) -> Option<String> {
        self.dag.pointer.take_pending_cluster_explode()
    }
}
