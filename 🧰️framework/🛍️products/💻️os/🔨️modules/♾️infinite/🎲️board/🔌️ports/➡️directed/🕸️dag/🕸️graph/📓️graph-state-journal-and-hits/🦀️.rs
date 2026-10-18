//! 📓️ Graph state journal and hits of the DAG graph: the connect and disconnect journal, graph edits, moves, port handle lookups and the widget, node and port hit tests. See [`super`].

use super::*;

impl DagGraphState {
    /// 🔗️ Records one completed connection in the guest's own vocabulary. `handle_key_map` already
    /// keys every handle as `"{nodeId}@{portId}"` — the same grammar the pick-target/hover channel
    /// uses — so this is a lookup, never a second derivation of what a port is.
    pub(super) fn journal_connect(&mut self, source: HandleId, target: HandleId) {
        let Some((source_node_id, source_port_id)) = self.handle_key_map.get(&source).and_then(|key| key.split_once('@')) else { return };
        let Some((target_node_id, target_port_id)) = self.handle_key_map.get(&target).and_then(|key| key.split_once('@')) else { return };
        let edit = DagGraphEdit::Connect { source_node_id: source_node_id.to_string(), source_port_id: source_port_id.to_string(), target_node_id: target_node_id.to_string(), target_port_id: target_port_id.to_string() };
        self.push_graph_edit(edit);
    }

    /// 🔗️ Records one removed wire by the SYNAPSE id the guest knows it as. An edge the engine
    /// created in this same gesture has no synapse id yet and is simply not journalled — the guest
    /// never learned about it, so it has nothing to remove.
    pub(super) fn journal_disconnect(&mut self, edge: EdgeId) {
        let Some(synapse_id) = self.edge_id_map.get(&edge).cloned() else { return };
        self.push_graph_edit(DagGraphEdit::Disconnect { synapse_id });
    }

    /// 🔗️ Journals one edit a press performed (a wire, a slider value, a port); an edit already pending is the same edit
    /// and is not journalled twice. A press journals a bounded handful, so these never compete with
    /// [`DAG_GRAPH_EDIT_CAPACITY`]; only moves grow with the graph and are reserved whole ([`Self::journal_moves`]).
    pub fn push_graph_edit(&mut self, edit: DagGraphEdit) {
        if !self.pending_graph_edits.contains(&edit) {
            self.pending_graph_edits.push(edit);
        }
    }

    /// 🔗️ Drains the wire edits this host performed since the last call — the renderer's one read
    /// point after a completed pointer gesture.
    pub fn take_graph_edits(&mut self) -> Vec<DagGraphEdit> {
        std::mem::take(&mut self.pending_graph_edits)
    }

    /// 👁️ The journalled graph edits not yet drained, in gesture order — read without draining so the embedding host can record
    /// the gesture's own concrete edits as history leaves before the renderer drains them ([`Self::take_graph_edits`]).
    pub fn graph_edits(&self) -> &[DagGraphEdit] {
        &self.pending_graph_edits
    }

    /// 🚫️ Drains why the last gesture was refused whole, if it was.
    pub fn take_journal_refusal(&mut self) -> Option<DagJournalRefusal> {
        self.journal_refusal.take()
    }

    /// 🚫️ Keeps a refusal the embedding host carried across rebuilding this host from the gesture's baseline.
    pub fn carry_journal_refusal(&mut self, refusal: DagJournalRefusal) {
        self.journal_refusal = Some(refusal);
    }

    /// ✋️ Journals what a gesture moved — `displacements` of `(node id, dx, dy)` in node order — as node-graph gesture records
    /// of `gesture_id` (design §13.3): one record per distinct offset, the nodes of each in order; a zero offset moved
    /// nothing. All or nothing: when the records do not fit the journal ([`DAG_GRAPH_EDIT_CAPACITY`]) the whole gesture is
    /// refused — the journal is emptied, the refusal kept for the renderer — and the embedding host restores the gesture's
    /// baseline, so neither the document nor the host keeps any of it.
    pub fn journal_moves(&mut self, gesture_id: &str, displacements: impl IntoIterator<Item = (String, f64, f64)>) -> Result<(), DagJournalRefusal> {
        let mut drags: Vec<(f64, f64, Vec<String>)> = Vec::new();
        for (id, dx, dy) in displacements {
            if (dx, dy) == (0.0, 0.0) {
                continue;
            }
            match drags.iter_mut().find(|(known_x, known_y, _)| (*known_x, *known_y) == (dx, dy)) {
                Some((_, _, ids)) => ids.push(id),
                None => drags.push((dx, dy, vec![id])),
            }
        }
        let rows = self.pending_graph_edits.len() + drags.len();
        if rows > DAG_GRAPH_EDIT_CAPACITY {
            let refusal = DagJournalRefusal { rows, limit: DAG_GRAPH_EDIT_CAPACITY };
            self.pending_graph_edits.clear();
            self.journal_refusal = Some(refusal);
            return Err(refusal);
        }
        self.pending_graph_edits.extend(drags.into_iter().map(|(dx, dy, node_ids)| DagGraphEdit::Move { gesture_id: gesture_id.to_string(), node_ids, dx, dy }));
        Ok(())
    }

    /// 📍️ Every node's id and centre, in node order — the baseline an embedding host records at a press so
    /// [`Self::journal_moves_since`] can narrate what the gesture moved.
    pub fn node_positions(&self) -> Vec<(String, f64, f64)> {
        self.host_snapshot.nodes.iter().map(|node| (node.id.clone(), node.x, node.y)).collect()
    }

    /// ✋️ Journals every node that moved since `baseline` ([`Self::node_positions`] at the press) — a drag's release or an
    /// align — through [`Self::journal_moves`]; a node the baseline does not know is not narrated.
    pub fn journal_moves_since(&mut self, gesture_id: &str, baseline: &[(String, f64, f64)]) -> Result<(), DagJournalRefusal> {
        let displacements: Vec<(String, f64, f64)> = self.host_snapshot.nodes.iter().filter_map(|node| baseline.iter().find(|(id, _, _)| *id == node.id).map(|(_, x, y)| (node.id.clone(), node.x - x, node.y - y))).collect();
        self.journal_moves(gesture_id, displacements)
    }

    /// ↩️ Puts every node `baseline` names back where it stood — the zero trace of a gesture refused whole.
    pub fn restore_node_positions(&mut self, baseline: &[(String, f64, f64)]) {
        for (id, x, y) in baseline {
            let Some(index) = self.host_snapshot.nodes.iter().position(|node| node.id == *id) else { continue };
            (self.host_snapshot.nodes[index].x, self.host_snapshot.nodes[index].y) = (*x, *y);
            self.sync_snapshot_node_center_to_engine(index);
        }
    }

    /// 🔌️ Journals a variadic port the embedding host inserted on node `node_id` at `index` of `side`.
    pub fn journal_port_insert(&mut self, node_id: String, side: DagPortSide, index: usize) {
        self.push_graph_edit(DagGraphEdit::InsertPort { node_id, side, index });
    }

    pub(crate) fn handle_id_for_port(&self, node_id: &str, port_id: &str) -> Option<HandleId> {
        let key = format!("{node_id}@{port_id}");
        self.handle_key_map.iter().filter(|(_, k)| k.as_str() == key).map(|(&hid, _)| hid).min()
    }

    /// 🔌️ The handle on ONE side of a port. `handle_key_map` keys every handle as `"{nodeId}@{portId}"`,
    /// and a node may carry an input AND an output under the same port id (`extrusion-axis` carries
    /// `vector`, `x`, `y`, `z` on both sides): a name-only lookup then answers whichever handle came
    /// first in the map, so a press on an OUTPUT could resolve to the input handle 40 world units away
    /// on the other side of the node, snap there, and start a wire from the wrong end. The engine
    /// already records the side as [`HandleRole`]; this reads it rather than keeping a second map.
    pub(crate) fn handle_id_for_port_side(&self, node_id: &str, port_id: &str, input: bool) -> Option<HandleId> {
        let key = format!("{node_id}@{port_id}");
        let wanted = if input { HandleRole::Target } else { HandleRole::Source };
        self.handle_key_map
            .iter()
            .filter(|(_, candidate)| candidate.as_str() == key)
            .map(|(&hid, _)| hid)
            .find(|hid| self.engine.handles.get(hid).is_some_and(|handle| handle.role == wanted))
            .or_else(|| self.handle_id_for_port(node_id, port_id))
    }

    pub(super) fn port_row_handle_hit(&self, world_x: f64, world_y: f64, inputs: bool, outputs: bool) -> Option<HandleId> {
        for node in self.host_snapshot.nodes.iter().rev() {
            if inputs {
                for (port_idx, port) in node.inputs().iter().enumerate() {
                    let Some((x0, y0, x1, y1)) = input_port_row_hit_bounds(node, port_idx) else {
                        continue;
                    };
                    if !point_in_rect(world_x, world_y, x0, y0, x1, y1) {
                        continue;
                    }
                    if let Some(hid) = self.handle_id_for_port_side(&node.id, &port.id, true) {
                        return Some(hid);
                    }
                }
            }
            if outputs {
                for (port_idx, port) in node.outputs().iter().enumerate() {
                    let Some((x0, y0, x1, y1)) = output_port_row_hit_bounds(node, port_idx) else {
                        continue;
                    };
                    if !point_in_rect(world_x, world_y, x0, y0, x1, y1) {
                        continue;
                    }
                    if let Some(hid) = self.handle_id_for_port_side(&node.id, &port.id, false) {
                        return Some(hid);
                    }
                }
            }
        }
        None
    }

    /// 🔌️ The port whose CONNECTOR rect — the published one — contains this world point.
    pub(super) fn port_connector_handle_hit(&self, world_x: f64, world_y: f64) -> Option<HandleId> {
        for node in self.host_snapshot.nodes.iter().rev() {
            for (port_idx, port) in node.inputs().iter().enumerate() {
                let Some((x0, y0, x1, y1)) = input_port_connector_bounds(node, port_idx) else { continue };
                if point_in_rect(world_x, world_y, x0, y0, x1, y1) {
                    if let Some(hid) = self.handle_id_for_port_side(&node.id, &port.id, true) {
                        return Some(hid);
                    }
                }
            }
            for (port_idx, port) in node.outputs().iter().enumerate() {
                let Some((x0, y0, x1, y1)) = output_port_connector_bounds(node, port_idx) else { continue };
                if point_in_rect(world_x, world_y, x0, y0, x1, y1) {
                    if let Some(hid) = self.handle_id_for_port_side(&node.id, &port.id, false) {
                        return Some(hid);
                    }
                }
            }
        }
        None
    }

    pub fn snapshot_draggable_node_hit(&self, world_x: f64, world_y: f64) -> Option<NodeId> {
        for idx in (0..self.host_snapshot.nodes.len()).rev() {
            let node = &self.host_snapshot.nodes[idx];
            let hw = node.width * 0.5;
            let hh = node.height * 0.5;
            if world_x < node.x - hw || world_x > node.x + hw || world_y < node.y - hh || world_y > node.y + hh {
                continue;
            }
            let Some(nid) = self.engine_node_id_for_index(idx) else {
                continue;
            };
            if self.engine.nodes.get(&nid).is_some_and(|n| n.draggable) {
                return Some(nid);
            }
        }
        None
    }

    /// 🔌️ A press on a port ROW never arrives here any more — `port_pointer_handle_hit` claims it
    /// first, and the engine's own `HitObject::Endpoint` arm already selects that channel before it
    /// begins the wire. The channel-row pick this branch used to duplicate is that selection.
    pub fn try_node_rectangle_pointer_down(&mut self, world_x: f64, world_y: f64, button: u8, shift: bool, ctrl_or_meta: bool, alt: bool) -> bool {
        if button != 0 || alt {
            return false;
        }
        let Some(node_id) = self.snapshot_draggable_node_hit(world_x, world_y) else {
            return false;
        };
        use canvas::Point;

        let point = Point::new(world_x, world_y);
        self.engine.pointer_down_on_draggable_node_at(node_id, point, shift, ctrl_or_meta);
        true
    }

    pub fn widget_hit_at(&self, world_x: f64, world_y: f64) -> Option<(usize, WidgetPointerKind)> {
        for idx in (0..self.host_snapshot.nodes.len()).rev() {
            let node = &self.host_snapshot.nodes[idx];
            match &node.kind {
                DagNodeKind::Slider { .. } => {
                    let (x0, y0, x1, y1) = slider_track_bounds(node);
                    if point_in_rect(world_x, world_y, x0, y0, x1, y1) {
                        return Some((idx, WidgetPointerKind::SliderDrag));
                    }
                }
                DagNodeKind::Select { .. } => {
                    let (x0, y0, x1, y1) = select_control_bounds(node);
                    if point_in_rect(world_x, world_y, x0, y0, x1, y1) {
                        return Some((idx, WidgetPointerKind::SelectClick));
                    }
                }
                DagNodeKind::Preview { content: DagPreviewContent::Tree { json }, expanded, .. } => {
                    for row in preview_tree_row_layouts(node, json, expanded) {
                        let (x0, y0, x1, y1) = row.row_rect;
                        if x1 > x0 && point_in_rect(world_x, world_y, x0, y0, x1, y1) {
                            return Some((idx, WidgetPointerKind::PreviewToggle(row.path)));
                        }
                    }
                }
                DagNodeKind::Cluster { .. } => {
                    if cluster_explode_hit(node, world_x, world_y) {
                        return Some((idx, WidgetPointerKind::ClusterExplode));
                    }
                }
                DagNodeKind::Export { .. } => {
                    let (x0, y0, x1, y1) = action_control_bounds(node);
                    if point_in_rect(world_x, world_y, x0, y0, x1, y1) {
                        return Some((idx, WidgetPointerKind::ExportClick));
                    }
                }
                _ => {}
            }
        }
        None
    }

    pub fn sync_snapshot_node_size_to_engine(&mut self, idx: usize) {
        let node = &self.host_snapshot.nodes[idx];
        let Some(nid) = self.engine_node_id_for_index(idx) else {
            return;
        };
        let Some(engine_node) = self.engine.nodes.get_mut(&nid) else {
            return;
        };
        engine_node.width = node.width;
        engine_node.height = node.height;
    }

    /// 📐️ Recomputes preview and image node sizes after content changes.
    pub fn fit_preview_sizes(&mut self) {
        for idx in 0..self.host_snapshot.nodes.len() {
            let kind = &self.host_snapshot.nodes[idx].kind;
            if matches!(kind, DagNodeKind::Preview { .. } | DagNodeKind::Image { .. }) {
                fit_node_size(&mut self.host_snapshot.nodes[idx]);
                self.sync_snapshot_node_size_to_engine(idx);
            }
        }
    }

    /// 📐️ Recomputes note node sizes after text changes.
    pub fn fit_note_sizes(&mut self) {
        for idx in 0..self.host_snapshot.nodes.len() {
            if matches!(self.host_snapshot.nodes[idx].kind, DagNodeKind::Note { .. }) {
                fit_node_size(&mut self.host_snapshot.nodes[idx]);
                self.sync_snapshot_node_size_to_engine(idx);
            }
        }
    }
}
