//! 🧭️ DAG host constructors, selection and hover projection, pointer plans and screen hit queries.

use super::*;

// #region 🔖️DagHost
impl DagHost {
    pub fn default_demo() -> Self {
        Self::from_host_snapshot(DagHostSnapshot::default())
    }

    pub fn from_host_snapshot(snapshot: DagHostSnapshot) -> Self {
        Self::from_host_snapshot_with_layout(snapshot, false)
    }

    /// 🌳️ Builds a host without running auto-layout (preserves node positions).
    pub fn from_host_snapshot_without_layout(snapshot: DagHostSnapshot) -> Self {
        Self::from_host_snapshot(snapshot)
    }

    /// ♻️ Rebuilds transient DAG state while retaining the exact owner of admitted icon paints.
    pub fn replace_host_snapshot_without_layout(&mut self, snapshot: DagHostSnapshot) {
        let mut next = Self::from_host_snapshot_without_layout(snapshot);
        std::mem::swap(&mut self.icon_paint_cache, &mut next.icon_paint_cache);
        *self = next;
    }

    fn from_host_snapshot_with_layout(snapshot: DagHostSnapshot, apply_layout: bool) -> Self {
        let mut host = Self {
            host_snapshot: snapshot,
            engine: DagBoardEngine::new(),
            canvas_theme: CanvasPalette::default(),
            width: 1,
            height: 1,
            dpr: 1.0,
            last_screen_x: 0.0,
            last_screen_y: 0.0,
            node_id_map: HistoryFoldIndex::new(),
            handle_key_map: HistoryFoldIndex::new(),
            handle_port_shape: HistoryFoldIndex::new(),
            handle_port_visible: HistoryFoldIndex::new(),
            edge_id_map: HistoryFoldIndex::new(),
            edge_engine_ids: Vec::new(),
            edge_route_style: HistoryFoldIndex::new(),
            widget_drag: None,
            pending_port_insert: None,
            dimmed: HistoryFoldSet::new(),
            wheel_zoom_active: false,
            wheel_zoom_render_lod: None,
            automatic_lod: true,
            forced_draw_lod: None,
            grid_visible: true,
            grid_snap_enabled: false,
            grid_factor: GRID_FACTOR_DEFAULT,
            icon_paint_cache: graph::IconPaintCache::new(),
            ghost_node: None,
            pending_cluster_explode: None,
            pending_export_click: None,
            pending_open_instance_id: None,
            last_pointer_down_at_ms: 0.0,
            last_pointer_down_world: (0.0, 0.0),
            last_pointer_down_node_id: None,
            computing_active: None,
            computing_stale: NumericSet::new(),
            computing_active_anim_phase: Cell::new(0.0),
            computing_stale_anim_phase: Cell::new(0.0),
            node_eval_status: NumericIndex::new(),
            unresolved_input_ports: NumericSet::new(),
            editing_note: None,
            caret_visible: true,
            pan_anchor: None,
            minimap_widget_visible: false,
            minimap_widget_hovered: false,
            minimap_widget_drag: None,
            pending_graph_edits: Vec::new(),
            journal_refusal: None,
        };
        host.rebuild_engine_with_layout(apply_layout);
        host
    }

    pub(super) fn engine_node_id_for_index(&self, idx: usize) -> Option<NodeId> {
        self.node_id_map.iter().find(|(_, &i)| i == idx).map(|(&nid, _)| nid)
    }

    pub(super) fn node_id_for_widget_id(&self, widget_id: &str) -> Option<NodeId> {
        let idx = self.host_snapshot.nodes.iter().position(|n| n.id == widget_id)?;
        self.engine_node_id_for_index(idx)
    }

    pub(super) fn widget_id_for_node_id(&self, node_id: NodeId) -> Option<String> {
        self.widget_id_for_node_id_ref(node_id).map(str::to_owned)
    }

    pub(super) fn widget_id_for_node_id_ref(&self, node_id: NodeId) -> Option<&str> {
        let idx = *self.node_id_map.get(&node_id)?;
        self.host_snapshot.nodes.get(idx).map(|node| node.id.as_str())
    }

    pub(super) fn is_node_hovered(&self, node_id: NodeId) -> bool {
        matches!(self.engine.hover, Some(hover) if hover == node_id)
    }

    fn resolve_minimap_hover_node_id(&self, world_x: f64, world_y: f64) -> Option<NodeId> {
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

    pub(super) fn sync_minimap_pointer_hover(&mut self, world_x: f64, world_y: f64) {
        if !matches!(self.draw_lod_for_frame(), DagDrawLod::Minimap) {
            return;
        }
        if !matches!(self.engine.interaction, InteractionMode::Idle) {
            return;
        }
        self.engine.hover = self.resolve_minimap_hover_node_id(world_x, world_y);
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

    fn is_preselect_active(&self) -> bool {
        matches!(self.engine.interaction, InteractionMode::SelectionPending { .. } | InteractionMode::AreaSelect { .. })
            || !self.engine.preselect.node_ids.is_empty()
            || !self.engine.preselect.handle_ids.is_empty()
            || !self.engine.preselect.edge_ids.is_empty()
    }

    pub(super) fn node_interaction_chrome(&self, node_id: NodeId) -> (bool, bool, bool) {
        if self.is_preselect_active() {
            return (self.is_node_preselected(node_id), self.is_node_preselect_removed(node_id), false);
        }
        (self.is_node_selected(node_id), false, self.is_node_hovered(node_id))
    }

    pub(super) fn handle_interaction_chrome(&self, handle_id: HandleId) -> (bool, bool, bool) {
        if self.is_preselect_active() {
            return (self.engine.preselect.handle_ids.contains(&handle_id), self.engine.preselect_removed.handle_ids.contains(&handle_id), false);
        }
        (self.engine.selection.handle_ids.contains(&handle_id), false, self.engine.hover == Some(handle_id))
    }

    pub(super) fn edge_interaction_chrome(&self, edge_id: EdgeId) -> (bool, bool, bool) {
        if self.is_preselect_active() {
            return (self.engine.preselect.edge_ids.contains(&edge_id), self.engine.preselect_removed.edge_ids.contains(&edge_id), false);
        }
        (self.engine.selection.edge_ids.contains(&edge_id), false, self.engine.hover == Some(edge_id))
    }

    pub(super) fn sync_camera_from_engine(&mut self) {
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

    pub fn derive_pointer_plan(&self, projection: DagInteractionProjection, intent: DagPointerIntent) -> Result<DagPointerPlan, DagInteractionPlanFault> {
        if projection.revision == u64::MAX || self.host_snapshot.nodes.len() > DAG_INTERACTION_NODE_CAPACITY {
            return Err(DagInteractionPlanFault::NodeCredits);
        }
        let mut next = projection;
        next.revision = projection.revision + 1;
        let mut moves = [None; DAG_INTERACTION_NODE_CAPACITY];
        let mut move_len = 0u16;
        let mut drag = None;
        match intent.phase {
            DagPointerPhase::Down => self.derive_projection_down(&mut next, intent)?,
            DagPointerPhase::Move => self.derive_projection_move(&mut next, &mut moves, &mut move_len, &mut drag, intent)?,
            DagPointerPhase::Up => {
                self.derive_projection_move(&mut next, &mut moves, &mut move_len, &mut drag, intent)?;
                next.gesture = DagProjectionGesture::Idle;
            }
            DagPointerPhase::Leave => {
                next.gesture = DagProjectionGesture::Idle;
                next.hover = None;
            }
        }
        Ok(DagPointerPlan { expected_revision: projection.revision, previous_active: !matches!(projection.gesture, DagProjectionGesture::Idle), next, moves, move_len, drag })
    }

    fn bounded_node_hit_index(&self, sx: f64, sy: f64) -> Result<Option<usize>, DagInteractionPlanFault> {
        if self.minimap_widget_pointer_hit(sx, sy).is_some() {
            return Err(DagInteractionPlanFault::Unsupported);
        }
        let world = self.screen_to_world_point(sx, sy);
        if self.port_insert_hit(world.x, world.y, self.host_snapshot.camera.zoom).is_some() || self.world_hits_handle(world.x, world.y) || self.widget_hit_at(world.x, world.y).is_some() {
            return Err(DagInteractionPlanFault::Unsupported);
        }
        Ok(self.snapshot_draggable_node_hit(world.x, world.y).and_then(|node_id| self.node_id_map.get(&node_id).copied()))
    }

    fn derive_projection_down(&self, next: &mut DagInteractionProjection, intent: DagPointerIntent) -> Result<(), DagInteractionPlanFault> {
        if intent.pan || intent.button == 1 {
            next.gesture = DagProjectionGesture::Pan { start_x: intent.x, start_y: intent.y, camera: next.camera };
            return Ok(());
        }
        if intent.button != 0 || intent.alt {
            return Err(DagInteractionPlanFault::Unsupported);
        }
        let Some(index) = self.bounded_node_hit_index(intent.x, intent.y)? else {
            next.gesture = DagProjectionGesture::Select { start_x: intent.x, start_y: intent.y, initial: next.selected };
            return Ok(());
        };
        if intent.ctrl_or_meta {
            let selected = !dag_bit_contains(&next.selected, index);
            dag_bit_set(&mut next.selected, index, selected);
            if !selected {
                next.gesture = DagProjectionGesture::Idle;
                return Ok(());
            }
        } else if intent.shift {
            dag_bit_set(&mut next.selected, index, true);
        } else if !dag_bit_contains(&next.selected, index) {
            next.selected = [0; DAG_INTERACTION_WORD_CAPACITY];
            dag_bit_set(&mut next.selected, index, true);
        }
        next.hover = u16::try_from(index).ok();
        let mut starts = [None; DAG_INTERACTION_NODE_CAPACITY];
        let mut len = 0usize;
        for node_index in 0..self.host_snapshot.nodes.len() {
            if !dag_bit_contains(&next.selected, node_index) {
                continue;
            }
            let node = &self.host_snapshot.nodes[node_index];
            starts[len] = Some(DagNodeMove { index: node_index as u16, x: node.x, y: node.y });
            len += 1;
        }
        next.gesture = DagProjectionGesture::Drag { press: next.revision, start_x: intent.x, start_y: intent.y, starts, len: len as u16 };
        Ok(())
    }

    fn derive_projection_move(&self, next: &mut DagInteractionProjection, moves: &mut [Option<DagNodeMove>; DAG_INTERACTION_NODE_CAPACITY], move_len: &mut u16, drag: &mut Option<DagDragOffset>, intent: DagPointerIntent) -> Result<(), DagInteractionPlanFault> {
        match next.gesture {
            DagProjectionGesture::Pan { start_x, start_y, camera } => {
                let zoom = camera[2].max(1e-9);
                next.camera = [camera[0] - (intent.x - start_x) / zoom, camera[1] - (intent.y - start_y) / zoom, camera[2]];
            }
            DagProjectionGesture::Drag { press, start_x, start_y, starts, len } => {
                let zoom = next.camera[2].max(1e-9);
                let dx = (intent.x - start_x) / zoom;
                let dy = (intent.y - start_y) / zoom;
                *drag = (dx != 0.0 || dy != 0.0).then_some(DagDragOffset { press, dx, dy });
                for index in 0..usize::from(len) {
                    let Some(start) = starts[index] else {
                        continue;
                    };
                    moves[index] = Some(DagNodeMove { index: start.index, x: start.x + dx, y: start.y + dy });
                }
                *move_len = len;
            }
            DagProjectionGesture::Select { start_x, start_y, initial } => {
                let min_x = start_x.min(intent.x);
                let max_x = start_x.max(intent.x);
                let min_y = start_y.min(intent.y);
                let max_y = start_y.max(intent.y);
                let mut selected = if intent.shift || intent.ctrl_or_meta { initial } else { [0; DAG_INTERACTION_WORD_CAPACITY] };
                for index in 0..self.host_snapshot.nodes.len() {
                    let node = &self.host_snapshot.nodes[index];
                    let screen = self.world_to_screen_point(node.x, node.y);
                    if screen.0 >= min_x && screen.0 <= max_x && screen.1 >= min_y && screen.1 <= max_y {
                        dag_bit_set(&mut selected, index, true);
                    }
                }
                next.selected = selected;
            }
            DagProjectionGesture::Idle => {
                next.hover = self.bounded_node_hit_index(intent.x, intent.y)?.and_then(|index| u16::try_from(index).ok());
            }
        }
        Ok(())
    }

    pub(super) fn world_to_screen_point(&self, x: f64, y: f64) -> (f64, f64) {
        use canvas::camera::{world_to_screen, Camera, Viewport};
        let camera = Camera { x: self.host_snapshot.camera.x, y: self.host_snapshot.camera.y, zoom: self.host_snapshot.camera.zoom };
        let viewport = Viewport { width: self.width, height: self.height, dpr: self.dpr };
        let point = world_to_screen(&camera, &viewport, canvas::Point::new(x, y));
        (point.x, point.y)
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

    /// 🖱️ Hovered snapshot widget id for node body hover, or parent widget when a channel handle is hovered at detail LOD.
    pub fn hovered_node_id(&self) -> Option<String> {
        self.hovered_node_id_ref().map(str::to_owned)
    }

    pub fn hovered_node_id_ref(&self) -> Option<&str> {
        let hover = self.engine.hover?;
        if self.node_id_map.contains_key(&hover) {
            return self.widget_id_for_node_id_ref(hover);
        }
        if self.draw_lod_for_frame().uses_channel_row_pick() {
            if let Some(handle) = self.engine.handles.get(&hover) {
                return self.widget_id_for_node_id_ref(handle.node_id);
            }
        }
        None
    }

    pub(super) fn channel_view(&self, target:u64)->Option<infinite::board::schema::dag_input::output::DagChannelView<'_>> {
        if self.node_id_map.contains_key(&target) {
            return None;
        }
        let key = self.handle_key_map.get(&target)?;
        let handle=self.engine.handles.get(&target)?;
        let node=self.host_snapshot.nodes.get(*self.node_id_map.get(&handle.node_id)?)?;
        let port=key.get(node.id.len().checked_add(1)?..)?;
        let direction=match handle.role{HandleRole::Target=>DagChannelDirection::In,HandleRole::Source=>DagChannelDirection::Out,_=>return None};
        Some(infinite::board::schema::dag_input::output::DagChannelView {widget_id:&node.id,port,direction})
    }

    pub(super) fn decode_channel_ref(&self,target:u64)->Option<DagChannelRef>{self.channel_view(target).map(|value|DagChannelRef{widget_id:value.widget_id.to_owned(),port:value.port.to_owned(),direction:value.direction})}

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

    /// 🗺️ Every node and port this graph paints, each with the SCREEN geometry
    /// [`Self::entity_screen_json`] publishes for it — one census of where everything IS, for a
    /// renderer diagnostic and for a browser probe that would otherwise have to FIND the graph by
    /// sweeping the pointer across it. A sweep is not a cheap way to ask: measured on the wgpu serve,
    /// a 224-point hover sweep spends the host's whole attention budget and leaves nothing for the
    /// gesture it was sweeping to find (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    ///
    /// Rects are SURFACE-local, in the same coordinates `pointer_*_screen` takes.
    pub fn screen_geometry_census_json(&self) -> String {
        let mut rows: Vec<String> = Vec::new();
        for node in &self.host_snapshot.nodes {
            let body = match self.entity_screen_rect("node", &node.id) {
                Some(rect) => self.first_screen_point_in(rect, |hit| hit.is_draggable_body() && hit.node_id.as_deref() == Some(node.id.as_str())),
                None => None,
            };
            let body_json = body.map_or_else(|| "null".to_string(), |(x, y)| format!("[{x:.1},{y:.1}]"));
            rows.push(format!("{{\"kind\":\"node\",\"id\":{},\"body\":{body_json},\"geometry\":{}}}", semio_framework_pack_json::to_json_string(&node.id), self.entity_screen_json("node", &node.id)));
            for (port, direction) in node.inputs().iter().map(|port| (port, "in")).chain(node.outputs().iter().map(|port| (port, "out"))) {
                let channel = format!("{}@{}", node.id, port.id);
                rows.push(format!("{{\"kind\":\"handle\",\"direction\":\"{direction}\",\"id\":{},\"geometry\":{}}}", semio_framework_pack_json::to_json_string(&channel), self.entity_screen_json("handle", &channel)));
            }
        }
        let panel = [0.0, 0.0, self.width as f64, self.height as f64];
        let navigate = self.first_screen_point_in(panel, |hit| hit.minimap && !hit.minimap_viewport);
        rows.push(format!("{{\"kind\":\"minimap\",\"navigate\":{}}}", navigate.map_or_else(|| "null".to_string(), |(x, y)| format!("[{x:.1},{y:.1}]"))));
        format!("[{}]", rows.join(","))
    }

    /// 🗺️ The screen rect [`Self::entity_screen_json`] reports for one entity, or `None` when it is
    /// off screen — the typed read the census needs before it can scan inside it.
    fn entity_screen_rect(&self, domain: &str, id: &str) -> Option<[f64; 4]> {
        let value = semio_framework_pack_json::parse(&self.entity_screen_json(domain, id), semio_framework_pack_json::JsonMemberPolicy::Reject).ok()?;
        if value.get("visible").and_then(Value::as_bool) != Some(true) {
            return None;
        }
        let rect = value.get("rect")?.as_array()?;
        if rect.len() < 4 {
            return None;
        }
        let read = |index: usize| rect.get(index).and_then(Value::as_f64);
        Some([read(0)?, read(1)?, read(2)?, read(3)?])
    }

    /// 🔎️ The first point of a screen rect whose classification satisfies `wanted`, scanned on a fixed
    /// grid inset from the rim. Which part of a node is draggable body and which is inline widget is
    /// the ENGINE's answer, and this is how it is asked without a pointer ever moving.
    fn first_screen_point_in(&self, rect: [f64; 4], wanted: impl Fn(&DagScreenHit) -> bool) -> Option<(f64, f64)> {
        const STEPS: usize = 12;
        for row in 0..STEPS {
            for column in 0..STEPS {
                let x = rect[0] + rect[2] * (0.06 + 0.88 * column as f64 / (STEPS - 1) as f64);
                let y = rect[1] + rect[3] * (0.06 + 0.88 * row as f64 / (STEPS - 1) as f64);
                if x < 0.0 || y < 0.0 || x > self.width as f64 || y > self.height as f64 {
                    continue;
                }
                if wanted(&self.screen_hit(x, y)) {
                    return Some((x, y));
                }
            }
        }
        None
    }

    /// 🎯️ All pick targets under a screen point as JSON (`domain`, `id`, `generality`).
    pub fn pick_targets_at_screen_json(&self, sx: f64, sy: f64) -> String {
        #[derive(ToValue, FromValue)]
        struct Row {
            domain: String,
            id: String,
            generality: u32,
            #[value(default, skip_serializing_if = "Option::is_none")]
            label: Option<String>,
        }
        let world = self.screen_to_world_point(sx, sy);
        let targets = self.engine.hit_test_pick_targets(world);
        let rows: Vec<Row> = targets
            .into_iter()
            .filter_map(|target| {
                let row = match target.domain.as_str() {
                    "node" => self.widget_id_for_node_id(target.id).map(|id| Row { domain: target.domain, id, generality: target.generality, label: None }),
                    "edge" => self.edge_id_map.get(&target.id).cloned().map(|id| Row { domain: target.domain, id, generality: target.generality, label: None }),
                    "handle" => self.decode_channel_ref(target.id).map(|channel| Row {
                        domain: "handle".into(),
                        id: format!("{}@{}", channel.widget_id, channel.port),
                        generality: target.generality,
                        label: Some(format!("{} · {}", channel.widget_id, channel.port)),
                    }),
                    _ => None,
                };
                row
            })
            .collect();
        semio_framework_pack_json::to_json_string(&rows)
    }

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
// #endregion 🔖️DagHost
