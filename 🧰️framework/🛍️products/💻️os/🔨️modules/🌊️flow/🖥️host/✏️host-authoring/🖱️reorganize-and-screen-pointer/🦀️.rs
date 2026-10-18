//! 🖱️ Reorganize and screen pointer of the flow host: the tree layout reorganize, the screen pointer down, move, up and cancel, the graph edits drain and the selection. See [`super`].

use super::*;

impl FlowHost {
    /// 🌳️ Recomputes widget positions from the current graph using layered tree layout.
    pub fn reorganize(&mut self,opts:&DagLayoutOptions,control:&mut semio_framework_os_infinite::board::schema::layout::LayoutControl<'_>)->Result<(),FlowCoreError>{
        let mut dag=DagHost::from_host_snapshot_without_layout(self.build_dag_host_snapshot_v1());dag.viewport.canvas_theme=self.dag.viewport.canvas_theme;
        dag.graph.reorganize(opts,control)?;self.begin_change();self.dag=dag;self.sync_from_dag();
        let placed: Vec<String> = self.host_snapshot.layout.keys().cloned().collect();
        let moved = self.layout_leaf(placed.iter().map(String::as_str));
        self.note_leaves(moved);
        self.checked_change()?;
        Ok(())
    }

    #[allow(
        clippy::too_many_arguments,
        reason = "pointer-event handler mirroring this file's other screen-space input methods (pointer_move_screen/pointer_up_screen/wheel_screen) — position + button + modifier-key flags is the natural shape for this UI event, not a bundling candidate on its own without also restructuring its siblings"
    )]
    pub fn pointer_down_screen(&mut self, sx: f64, sy: f64, button: u8, shift: bool, ctrl_or_meta: bool, alt: bool, pan: bool) {
        self.interaction_revision = self.interaction_revision.wrapping_add(1);
        if pan {
            self.pan_anchor = Some((sx, sy, self.host_snapshot.camera.x, self.host_snapshot.camera.y));
            return;
        }
        self.clear_ghost_widget();
        self.dag.viewport.set_viewport(self.viewport_w, self.viewport_h, self.viewport_dpr);
        self.begin_gesture();
        self.dag.pointer_down_screen(sx, sy, button, shift, ctrl_or_meta, alt, false);
        if let Some((side, widget_id, index)) = self.dag.pointer.take_pending_port_insert() {
            let inserted = match side {
                dag::DagPortSide::Input => self.add_input_port(&widget_id, index),
                dag::DagPortSide::Output => self.add_output_port(&widget_id, index),
            };
            if inserted.is_ok() {
                self.dag.graph.journal_port_insert(widget_id, side, index);
            }
            return;
        }
        self.sync_from_dag();
    }

    pub fn pointer_move_screen(&mut self, sx: f64, sy: f64, shift: bool, ctrl_or_meta: bool, alt: bool) {
        self.interaction_revision = self.interaction_revision.wrapping_add(1);
        if let Some((start_sx, start_sy, cam_x, cam_y)) = self.pan_anchor {
            let zoom = self.host_snapshot.camera.zoom;
            let dx = (sx - start_sx) / zoom;
            let dy = (sy - start_sy) / zoom;
            self.set_camera(cam_x - dx, cam_y - dy, zoom);
            return;
        }
        self.dag.viewport.set_viewport(self.viewport_w, self.viewport_h, self.viewport_dpr);
        self.dag.pointer_move_screen(sx, sy, shift, ctrl_or_meta, alt);
        self.sync_from_dag();
    }

    pub fn widget_drag_active(&self) -> bool {
        self.dag.pointer.widget_drag_active()
    }

    pub fn pointer_up_screen(&mut self, sx: f64, sy: f64, shift: bool, ctrl_or_meta: bool, alt: bool) {
        self.interaction_revision = self.interaction_revision.wrapping_add(1);
        self.pan_anchor = None;
        self.dag.viewport.set_viewport(self.viewport_w, self.viewport_h, self.viewport_dpr);
        self.dag.pointer_up_screen(sx, sy, shift, ctrl_or_meta, alt);
        self.sync_from_dag();
        self.commit_gesture_history();
    }

    pub fn pointer_cancel_screen(&mut self) {
        self.interaction_revision = self.interaction_revision.wrapping_add(1);
        self.pan_anchor = None;
        self.dag.pointer_cancel_screen();
        if !self.gesture_active {
            return;
        }
        self.gesture_active = false;
        self.discard_pending_leaves();
        self.edited_note = None;
        let Some(mut baseline) = self.pending_history_baseline.take() else {
            return;
        };
        baseline.camera = self.host_snapshot.camera.clone();
        let stale = std::mem::replace(&mut self.host_snapshot, baseline);
        stale.retire_cold();
        self.rebuild_dag();
    }

    /// 🔗️ Drains the graph edits the last gesture performed as the `nodeGraphEdit` arguments the renderer dispatches
    /// (`dag::dag_graph_edit_rows_json`: `connect`, `disconnect`, `move` — the node-graph gesture record of design §13.3 —
    /// `setSlider`, `insertPort`). The journal narrates EVERY content change a gesture makes, so the renderer never
    /// re-publishes the whole fixture; a plain click, a marquee, a pan and a press that grabbed nothing journal nothing
    /// and are owed no dispatch at all.
    pub fn take_graph_edits_json(&mut self) -> String {
        let edits = self.dag.graph.take_graph_edits();
        let refusal = self.dag.graph.take_journal_refusal();
        dag::dag_graph_edit_rows_json(&edits, refusal)
    }

    pub fn set_selection_options(&mut self, method: &str, mode: &str) {
        self.dag.graph.set_selection_options(method, mode, true, true, true);
    }

    pub fn selection_preview_points_json(&self) -> String {
        self.dag.graph.selection_preview_points_json()
    }

    pub fn selection_preview_crossing(&self) -> bool {
        self.dag.graph.selection_preview_crossing()
    }

    pub fn selection_preview_method(&self) -> &str {
        self.dag.graph.selection_preview_method()
    }

    pub fn preselect_widget_ids_json(&self) -> String {
        semio_framework_pack_json::to_string(&semio_framework_pack_json::object([
            ("ids".to_string(), semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&self.dag.graph.preselect_widget_ids()))),
            ("removedIds".to_string(), semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&self.dag.graph.preselect_removed_widget_ids()))),
        ]))
    }

    pub fn cancel_area_select(&mut self) -> bool {
        let cancelled = self.dag.graph.cancel_area_select();
        if cancelled {
            self.sync_from_dag();
        }
        cancelled
    }

    pub fn delete_selection(&mut self) -> Result<(), FlowCoreError> {
        if !self.dag.graph.has_selection() {
            return Ok(());
        }
        self.begin_change();
        let nodes: Vec<String> = self.dag.graph.selected_node_ids().into_iter().filter(|id| self.host_snapshot.widgets.iter().any(|widget| widget_id_for(widget) == id)).collect();
        let edges = self.dag.graph.selected_edge_ids();
        let severed: Vec<String> = self.host_snapshot.synapses.iter().filter(|synapse| edges.contains(&synapse.id) || nodes.contains(&synapse.from) || nodes.contains(&synapse.to)).map(|synapse| synapse.id.clone()).collect();
        let unplaced: Vec<FlowLayoutEntry> = nodes.iter().filter(|id| self.host_snapshot.layout.contains_key(id.as_str())).map(|id| FlowLayoutEntry { id: id.clone(), layout: None }).collect();
        self.dag.graph.delete_selected();
        self.sync_from_dag();
        let unplaced = (!unplaced.is_empty()).then(|| FlowMutation::ChangeLayout(ChangeLayout { entries: unplaced }));
        self.note_leaves(severed.into_iter().map(|id| FlowMutation::RemoveSynapse(RemoveSynapse { id })).chain(unplaced).chain(nodes.into_iter().map(|id| FlowMutation::RemoveWidget(RemoveWidget { id }))));
        self.checked_change()?;
        Ok(())
    }

    /// ✅️ Whether the canvas has any committed node, edge, or handle selection.
    pub fn has_selection(&self) -> bool {
        self.dag.graph.has_selection()
    }

    pub fn select_all(&mut self) {
        self.dag.graph.select_all();
        self.sync_from_dag();
    }
}
