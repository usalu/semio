//! 🎚️ Graph state interaction impl of the DAG graph: widget and node ids, hover and selection chrome, pointer plans, graph edit planning, selection domains and channel views. See [`super`].

use super::*;

impl DagGraphState {
    pub fn engine_node_id_for_index(&self, idx: usize) -> Option<NodeId> {
        self.node_id_map.iter().find(|(_, &i)| i == idx).map(|(&nid, _)| nid)
    }

    pub fn node_id_for_widget_id(&self, widget_id: &str) -> Option<NodeId> {
        let idx = self.host_snapshot.nodes.iter().position(|n| n.id == widget_id)?;
        self.engine_node_id_for_index(idx)
    }

    pub fn widget_id_for_node_id(&self, node_id: NodeId) -> Option<String> {
        self.widget_id_for_node_id_ref(node_id).map(str::to_owned)
    }

    pub fn widget_id_for_node_id_ref(&self, node_id: NodeId) -> Option<&str> {
        let idx = *self.node_id_map.get(&node_id)?;
        self.host_snapshot.nodes.get(idx).map(|node| node.id.as_str())
    }

    pub fn is_node_hovered(&self, node_id: NodeId) -> bool {
        matches!(self.engine.hover, Some(hover) if hover == node_id)
    }

    pub(super) fn resolve_minimap_hover_node_id(&self, world_x: f64, world_y: f64) -> Option<NodeId> {
        for idx in (0..self.host_snapshot.nodes.len()).rev() {
            let node = &self.host_snapshot.nodes[idx];
            let hw = node.width * 0.5;
            let hh = node.height * 0.5;
            if world_x >= node.x - hw && world_x <= node.x + hw && world_y >= node.y - hh && world_y <= node.y + hh {
                return self.engine_node_id_for_index(idx);
            }
        }
        None
    }

    fn is_node_selected(&self, node_id: NodeId) -> bool {
        self.engine.selection.node_ids.contains(&node_id)
    }

    fn is_node_preselected(&self, node_id: NodeId) -> bool {
        self.engine.preselect.node_ids.contains(&node_id)
    }

    fn is_node_preselect_removed(&self, node_id: NodeId) -> bool {
        self.engine.preselect_removed.node_ids.contains(&node_id)
    }

    pub fn is_preselect_active(&self) -> bool {
        matches!(self.engine.interaction, InteractionMode::SelectionPending { .. } | InteractionMode::AreaSelect { .. })
            || !self.engine.preselect.node_ids.is_empty()
            || !self.engine.preselect.handle_ids.is_empty()
            || !self.engine.preselect.edge_ids.is_empty()
    }

    pub fn node_interaction_chrome(&self, node_id: NodeId) -> (bool, bool, bool) {
        if self.is_preselect_active() {
            return (self.is_node_preselected(node_id), self.is_node_preselect_removed(node_id), false);
        }
        (self.is_node_selected(node_id), false, self.is_node_hovered(node_id))
    }

    pub fn handle_interaction_chrome(&self, handle_id: HandleId) -> (bool, bool, bool) {
        if self.is_preselect_active() {
            return (self.engine.preselect.handle_ids.contains(&handle_id), self.engine.preselect_removed.handle_ids.contains(&handle_id), false);
        }
        (self.engine.selection.handle_ids.contains(&handle_id), false, self.engine.hover == Some(handle_id))
    }

    pub fn edge_interaction_chrome(&self, edge_id: EdgeId) -> (bool, bool, bool) {
        if self.is_preselect_active() {
            return (self.engine.preselect.edge_ids.contains(&edge_id), self.engine.preselect_removed.edge_ids.contains(&edge_id), false);
        }
        (self.engine.selection.edge_ids.contains(&edge_id), false, self.engine.hover == Some(edge_id))
    }

    pub fn sync_camera_from_engine(&mut self) {
        let cam = self.engine.camera;
        self.host_snapshot.camera = DagCamera { x: cam.x, y: cam.y, zoom: cam.zoom };
    }

    /// 🎯️ Selected snapshot node ids from the engine selection snapshot.
    pub fn selected_node_ids(&self) -> Vec<String> {
        self.selected_node_id_refs().map(str::to_owned).collect()
    }

    pub fn selected_node_id_refs(&self) -> impl Iterator<Item = &str> {
        self.engine.selection.node_ids.iter().filter_map(|&node_id| self.widget_id_for_node_id_ref(node_id))
    }

    pub fn selected_node_count(&self) -> usize {
        self.engine.selection.node_ids.len()
    }

    pub fn bounded_interaction_projection(&self, revision: u64) -> Result<DagInteractionProjection, DagInteractionPlanFault> {
        if self.host_snapshot.nodes.len() > DAG_INTERACTION_NODE_CAPACITY {
            return Err(DagInteractionPlanFault::NodeCredits);
        }
        let mut bytes = 0usize;
        for node in &self.host_snapshot.nodes {
            if node.id.len() > 256 {
                return Err(DagInteractionPlanFault::StringCredits);
            }
            bytes = bytes.checked_add(node.id.len()).ok_or(DagInteractionPlanFault::StringCredits)?;
            if bytes > 16 * 1024 {
                return Err(DagInteractionPlanFault::StringCredits);
            }
        }
        let mut selected = [0; DAG_INTERACTION_WORD_CAPACITY];
        for node_id in &self.engine.selection.node_ids {
            if let Some(index) = self.node_id_map.get(node_id).copied() {
                dag_bit_set(&mut selected, index, true);
            }
        }
        let hover = self.engine.hover.and_then(|node_id| self.node_id_map.get(&node_id).copied()).and_then(|index| u16::try_from(index).ok());
        let camera = &self.host_snapshot.camera;
        Ok(DagInteractionProjection { revision, camera: [camera.x, camera.y, camera.zoom], selected, hover, gesture: DagProjectionGesture::Idle })
    }

    pub fn apply_pointer_plan(&mut self, plan: &DagPointerPlan) {
        self.set_camera(plan.next.camera[0], plan.next.camera[1], plan.next.camera[2]);
        for index in 0..usize::from(plan.move_len) {
            let Some(delta) = plan.moves[index] else {
                continue;
            };
            let index = usize::from(delta.index);
            if let Some(node) = self.host_snapshot.nodes.get_mut(index) {
                node.x = delta.x;
                node.y = delta.y;
            }
            self.sync_snapshot_node_center_to_engine(index);
        }
        self.engine.selection = Selection::default();
        for index in 0..self.host_snapshot.nodes.len() {
            if dag_bit_contains(&plan.next.selected, index) {
                if let Some(node_id) = self.engine_node_id_for_index(index) {
                    self.engine.selection.node_ids.insert(node_id);
                }
            }
        }
        self.engine.preselect = Selection::default();
        self.engine.preselect_removed = Selection::default();
        self.engine.hover = plan.next.hover.and_then(|index| self.engine_node_id_for_index(usize::from(index)));
    }

    pub fn projection_selected_id_refs<'a>(&'a self, projection: &'a DagInteractionProjection) -> impl Iterator<Item = &'a str> + 'a {
        self.host_snapshot.nodes.iter().enumerate().filter(move |(index, _)| dag_bit_contains(&projection.selected, *index)).map(|(_, node)| node.id.as_str())
    }

    pub fn projection_hovered_id_ref<'a>(&'a self, projection: &DagInteractionProjection) -> Option<&'a str> {
        projection.hover.and_then(|index| self.host_snapshot.nodes.get(usize::from(index))).map(|node| node.id.as_str())
    }

    pub fn pointer_plan_move(&self, plan: &DagPointerPlan, index: usize) -> Option<(&str, f64, f64)> {
        let delta = plan.moves.get(index).and_then(|delta| *delta)?;
        self.host_snapshot.nodes.get(usize::from(delta.index)).map(|node| (node.id.as_str(), delta.x, delta.y))
    }

    /// 🫳️ The node drag a plan COMMITS when it ends a drag: ONE gesture record (design §13.3) — the press, every moved
    /// node and the drag's ONE relative offset — in the guest's own edit vocabulary.
    ///
    /// 🩸️ A bounded drag writes the new positions into `snapshot.layout` and into the engine — and
    /// told the guest NOTHING, so every node the user dragged snapped back on the next snapshot push.
    /// Only the TERMINAL application is journalled (a drag publishes one record, not one per pointer
    /// sample), and a gesture that ended where it started is not an edit at all — a plain click on a
    /// node body derives a zero-delta drag, which must not spend a guest mutation.
    ///
    /// @see `🕸️NodeGraph/🟦️.tsx` — the SSR `Diagram` fallback's `onNodeDragStop`
    pub fn plan_graph_edits(&self, plan: &DagPointerPlan) -> Vec<DagGraphEdit> {
        let Some(drag) = plan.drag.filter(|_| plan.previous_active && matches!(plan.next.gesture, DagProjectionGesture::Idle)) else {
            return Vec::new();
        };
        let node_ids: Vec<String> = plan.moves[..usize::from(plan.move_len)].iter().flatten().filter_map(|delta| self.host_snapshot.nodes.get(usize::from(delta.index))).map(|node| node.id.clone()).collect();
        if node_ids.is_empty() {
            return Vec::new();
        }
        vec![DagGraphEdit::Move { gesture_id: dag_drag_gesture_id(drag.press), node_ids, dx: drag.dx, dy: drag.dy }]
    }

    /// 🔗️ Selected snapshot edge ids (synapse ids) from the engine selection snapshot.
    pub fn selected_edge_ids(&self) -> Vec<String> {
        self.engine.selection.edge_ids.iter().filter_map(|&eid| self.edge_id_map.get(&eid).cloned()).collect()
    }

    /// 🎯️ Projects typed node, edge and handle selection facts.
    pub fn selection_domains(&self)->DagSelectionDomains {
        let handles=self.selected_channels().into_iter().map(|channel|format!("{}@{}",channel.widget_id,channel.port)).collect();
        DagSelectionDomains {nodes:self.selected_node_ids(),edges:self.selected_edge_ids(),handles}
    }

    fn apply_selection_domains(&mut self, nodes: &[String], edges: &[String], handles: &[String]) {
        self.engine.selection = Selection::default();
        for widget_id in nodes {
            if let Some(nid) = self.node_id_for_widget_id(widget_id) {
                self.engine.selection.node_ids.insert(nid);
            }
        }
        for edge_id in edges {
            for (&eid, synapse_id) in &self.edge_id_map {
                if synapse_id == edge_id {
                    self.engine.selection.edge_ids.insert(eid);
                    break;
                }
            }
        }
        for handle_key in handles {
            if let Some((node_id, port_id)) = handle_key.split_once('@') {
                if let Some(hid) = self.handle_id_for_port(node_id, port_id) {
                    self.engine.selection.handle_ids.insert(hid);
                }
            }
        }
        self.engine.preselect = Selection::default();
        self.engine.preselect_removed = Selection::default();
    }

    /// ✅️ Replaces selection from fully admitted semantic domains.
    pub fn set_selection_domains(&mut self,domains:&DagSelectionDomains){self.apply_selection_domains(&domains.nodes,&domains.edges,&domains.handles);}

    /// ✅️ Whether the engine has any committed node, edge, or handle selection.
    pub fn has_selection(&self) -> bool {
        !self.engine.selection.node_ids.is_empty() || !self.engine.selection.edge_ids.is_empty() || !self.engine.selection.handle_ids.is_empty()
    }

    pub fn channel_view(&self, target:u64)->Option<DagChannelView<'_>> {
        if self.node_id_map.contains_key(&target) {
            return None;
        }
        let key = self.handle_key_map.get(&target)?;
        let handle=self.engine.handles.get(&target)?;
        let node=self.host_snapshot.nodes.get(*self.node_id_map.get(&handle.node_id)?)?;
        let port=key.get(node.id.len().checked_add(1)?..)?;
        let direction=match handle.role{HandleRole::Target=>DagChannelDirection::In,HandleRole::Source=>DagChannelDirection::Out,_=>return None};
        Some(DagChannelView {widget_id:&node.id,port,direction})
    }

    pub fn decode_channel_ref(&self,target:u64)->Option<DagChannelRef>{self.channel_view(target).map(|value|DagChannelRef{widget_id:value.widget_id.to_owned(),port:value.port.to_owned(),direction:value.direction})}

    /// 🔌️ Hovered snapshot channel when the pointer is over a port row or handle.
    pub fn hovered_channel(&self) -> Option<DagChannelRef> {
        let hover = self.engine.hover?;
        self.decode_channel_ref(hover)
    }

    /// 🔌️ Selected snapshot channels from handle picks in the current selection snapshot.
    pub fn selected_channels(&self) -> Vec<DagChannelRef> {
        self.engine.selection.handle_ids.iter().filter_map(|&handle_id| self.decode_channel_ref(handle_id)).collect()
    }

    /// 🚫️ Projects a live wire refusal with both declared semantic value type lists.
    pub fn wire_type_refusal(&self)->Option<DagWireTypeRefusal>{
        let (source_hid,target_hid)=self.engine.wire_drag_type_refusal()?;
        let source=self.handle_key_map.get(&source_hid)?.clone();let target=self.handle_key_map.get(&target_hid)?.clone();
        let types=|id|self.engine.handles.get(&id).map(|handle|handle.value_types.clone()).unwrap_or_default();
        Some(DagWireTypeRefusal{source,source_types:types(source_hid),target,target_types:types(target_hid)})
    }

    /// 🖱️ Projects channel hover and live wire refusal from one accepted graph state.
    pub fn hover_facts(&self)->DagHoverFacts{DagHoverFacts{channel:self.hovered_channel(),refusal:self.wire_type_refusal()}}

    /// ✅️ Replaces node selection from snapshot widget ids.
    pub fn set_selection(&mut self, widget_ids: &[String]) {
        self.engine.selection = Selection::default();
        for widget_id in widget_ids {
            if let Some(nid) = self.node_id_for_widget_id(widget_id) {
                self.engine.selection.node_ids.insert(nid);
            }
        }
        self.engine.preselect = Selection::default();
        self.engine.preselect_removed = Selection::default();
    }

    /// 🎯️ Configures rectangle/lasso area-select behavior.
    pub fn set_selection_options(&mut self, method: &str, mode: &str, select_nodes: bool, select_handles: bool, select_edges: bool) {
        self.engine.set_selection_options(method, mode, select_nodes, select_handles, select_edges);
    }

    /// 🧿️ Screen-space marquee overlay points for the shared selection overlay.
    pub fn selection_preview_points_json(&self) -> String {
        let points: Vec<[f64; 2]> = self.engine.selection_preview_points().iter().map(|p| [p.x, p.y]).collect();
        semio_framework_pack_json::to_json_string(&points)
    }

    pub fn selection_preview_crossing(&self) -> bool {
        self.engine.selection_preview_crossing()
    }

    pub fn selection_preview_method(&self) -> &str {
        self.engine.selection_preview_method()
    }

    /// 👁️ Preselected widget ids during an in-flight marquee gesture.
    pub fn preselect_widget_ids(&self) -> Vec<String> {
        self.engine.preselect.node_ids.iter().filter_map(|&nid| self.widget_id_for_node_id(nid)).collect()
    }

    /// 👁️ Widget ids highlighted as marquee-exit candidates.
    pub fn preselect_removed_widget_ids(&self) -> Vec<String> {
        self.engine.preselect_removed.node_ids.iter().filter_map(|&nid| self.widget_id_for_node_id(nid)).collect()
    }

    /// ↩️ Cancels an in-flight area select and restores the pre-drag selection.
    pub fn cancel_area_select(&mut self) -> bool {
        self.engine.cancel_area_select()
    }
}
