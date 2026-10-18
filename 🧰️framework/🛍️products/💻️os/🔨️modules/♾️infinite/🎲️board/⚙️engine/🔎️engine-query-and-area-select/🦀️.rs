//! 🔎️ Engine query and area select of the board engine: the render snapshot, edge curves, wire snapping, pick application and the area selection and preselect helpers. See [`super`].

use super::*;

// #region 🔖️Engine
impl<P: GraphPortModel, D: Directedness> GraphEngine<P, D> {
    pub fn render_snapshot(&self) -> RenderSnapshot {
        let mut snapshot = RenderSnapshot::default();
        for node in self.nodes.values() {
            snapshot.nodes.push((node.id, node.center, node.radius));
        }
        if P::HAS_PORTS {
            for handle in self.handles.values() {
                if let Some(node) = self.nodes.get(&handle.node_id) {
                    snapshot.handles.push((handle.id, handle_position(node, handle), handle.radius));
                }
            }
        }
        for edge in self.edges.values() {
            if let Some(curve) = self.edge_curve(edge.id) {
                snapshot.edges.push(curve);
            }
        }
        if let InteractionMode::DrawEdge { anchor_handle, anchor_is_source, fixed_target, cursor, snap_target, .. } = self.interaction {
            snapshot.pending_edge = self.draw_edge_preview_curve(anchor_handle, anchor_is_source, fixed_target, cursor, snap_target);
        } else if let Some(conn) = self.proximity_connection {
            snapshot.pending_edge = self.proximity_preview_curve(conn);
        }
        let _stroke_scene = encode_board_stroke_scene(&snapshot.edges, 2.0);
        let _ = _stroke_scene.path_count();
        snapshot
    }

    pub fn drain_events(&mut self) -> Vec<BoardEvent> {
        std::mem::take(&mut self.events)
    }

    pub fn edge_curve(&self, edge_id: EdgeId) -> Option<CubicBez> {
        let edge = self.edges.get(&edge_id)?;
        let (source_position, target_position, source_node, target_node, source_cap_radius, target_cap_radius) = self.endpoint_wire_nodes(edge)?;
        Some(self.wire_bezier_between(source_position, target_position, source_node, target_node, source_cap_radius, target_cap_radius))
    }

    pub(super) fn wire_bezier_between(&self, source_point: Point, target_point: Point, source_node: Option<&Node>, target_node: Option<&Node>, source_cap_radius: f64, target_cap_radius: f64) -> CubicBez {
        let chord = normalize_or_zero(target_point - source_point);
        let source_out = source_node.and_then(|node| handle_outward_at_node_rim(source_point, node.center, node.shape, node.radius, node.width, node.height)).filter(|outward| outward.hypot() > f64::EPSILON).unwrap_or(chord);
        let target_out = target_node.and_then(|node| handle_outward_at_node_rim(target_point, node.center, node.shape, node.radius, node.width, node.height)).filter(|outward| outward.hypot() > f64::EPSILON).unwrap_or(-chord);
        let source_wire = if source_node.is_some() { handle_exterior_cap_peak(source_point, source_out, source_cap_radius) } else { source_point };
        let target_wire = if target_node.is_some() { handle_exterior_cap_peak(target_point, target_out, target_cap_radius) } else { target_point };
        compute_edge_bezier_outward(source_wire, target_wire, source_out, target_out)
    }

    fn draw_edge_preview_curve(&self, anchor_handle: HandleId, anchor_is_source: bool, fixed_target: Option<HandleId>, cursor: Point, snap_target: Option<HandleId>) -> Option<CubicBez> {
        if let Some(fixed_tgt) = fixed_target {
            let tgt_h = self.handles.get(&fixed_tgt)?;
            let tgt_node = self.nodes.get(&tgt_h.node_id)?;
            let target_point = handle_position(tgt_node, tgt_h);
            if let Some(snap) = snap_target {
                let snap_h = self.handles.get(&snap)?;
                let snap_node = self.nodes.get(&snap_h.node_id)?;
                let source_point = handle_position(snap_node, snap_h);
                return Some(self.wire_bezier_between(source_point, target_point, Some(snap_node), Some(tgt_node), snap_h.radius, tgt_h.radius));
            }
            return Some(self.wire_bezier_between(cursor, target_point, None, Some(tgt_node), 0.0, tgt_h.radius));
        }
        let anchor = self.handles.get(&anchor_handle)?;
        let anchor_node = self.nodes.get(&anchor.node_id)?;
        let anchor_point = handle_position(anchor_node, anchor);
        if anchor_is_source {
            if let Some(snap) = snap_target {
                let snap_h = self.handles.get(&snap)?;
                let snap_node = self.nodes.get(&snap_h.node_id)?;
                let snap_point = handle_position(snap_node, snap_h);
                return Some(self.wire_bezier_between(anchor_point, snap_point, Some(anchor_node), Some(snap_node), anchor.radius, snap_h.radius));
            }
            Some(self.wire_bezier_between(anchor_point, cursor, Some(anchor_node), None, anchor.radius, 0.0))
        } else if let Some(snap) = snap_target {
            let snap_h = self.handles.get(&snap)?;
            let snap_node = self.nodes.get(&snap_h.node_id)?;
            let snap_point = handle_position(snap_node, snap_h);
            Some(self.wire_bezier_between(snap_point, anchor_point, Some(snap_node), Some(anchor_node), snap_h.radius, anchor.radius))
        } else {
            Some(self.wire_bezier_between(cursor, anchor_point, None, Some(anchor_node), 0.0, anchor.radius))
        }
    }

    pub(super) fn active_proximity_distance_world(&self) -> f64 {
        self.proximity_distance_override.unwrap_or(self.proximity_distance_world)
    }

    pub(super) fn proximity_enabled(&self) -> bool {
        self.active_proximity_distance_world() > 0.0
    }

    fn wire_snap_drag_tolerance_world(&self, handle_radius: f64) -> f64 {
        if !self.proximity_enabled() {
            return 0.0;
        }
        self.active_proximity_distance_world() + handle_radius
    }

    pub(super) fn wire_snap_still_active(&self, handle_id: HandleId, cursor: Point) -> bool {
        if !self.proximity_enabled() {
            return false;
        }
        let Some(handle) = self.handles.get(&handle_id) else {
            return false;
        };
        let Some(node) = self.nodes.get(&handle.node_id) else {
            return false;
        };
        let pos = handle_position(node, handle);
        distance(cursor, pos) <= self.wire_snap_drag_tolerance_world(handle.radius)
    }

    pub(super) fn nearest_wire_snap_handle(&self, anchor_handle: HandleId, anchor_is_source: bool, fixed_target: Option<HandleId>, reconnecting: Option<EdgeId>, cursor: Point) -> Option<HandleId> {
        if !P::HAS_PORTS || !self.proximity_enabled() {
            return None;
        }
        let mut best: Option<(f64, HandleId)> = None;
        for handle in self.handles.values() {
            let candidate = handle.id;
            if candidate == anchor_handle {
                continue;
            }
            let (source_hid, target_hid) = if let Some(fixed) = fixed_target {
                if anchor_is_source {
                    (candidate, fixed)
                } else {
                    (fixed, candidate)
                }
            } else if anchor_is_source {
                (anchor_handle, candidate)
            } else {
                (candidate, anchor_handle)
            };
            if !self.is_valid_connection(source_hid, target_hid, reconnecting, true) {
                continue;
            }
            let Some(node) = self.nodes.get(&handle.node_id) else {
                continue;
            };
            let pos = handle_position(node, handle);
            let d = distance(cursor, pos);
            let tol = self.wire_snap_drag_tolerance_world(handle.radius);
            if d <= tol && best.as_ref().is_none_or(|(best_d, _)| d < *best_d) {
                best = Some((d, candidate));
            }
        }
        best.map(|(_, id)| id)
    }

    fn endpoint_wire_nodes(&self, edge: &GraphEdge<P::Endpoint>) -> Option<WireEndpoints<'_>> {
        if P::HAS_PORTS {
            let source_handle = self.handles.get(&P::endpoint_as_handle(edge.source)?)?;
            let target_handle = self.handles.get(&P::endpoint_as_handle(edge.target)?)?;
            let source_node = self.nodes.get(&source_handle.node_id)?;
            let target_node = self.nodes.get(&target_handle.node_id)?;
            let source_position = handle_position(source_node, source_handle);
            let target_position = handle_position(target_node, target_handle);
            return Some((source_position, target_position, Some(source_node), Some(target_node), source_handle.radius, target_handle.radius));
        }
        let source_node = self.nodes.get(&P::endpoint_as_u64(edge.source))?;
        let target_node = self.nodes.get(&P::endpoint_as_u64(edge.target))?;
        Some((source_node.center, target_node.center, Some(source_node), Some(target_node), 0.0, 0.0))
    }

    pub(super) fn remove_handle(&mut self, id: HandleId) {
        self.handles.remove(&id);
        let removed_edges: Vec<EdgeId> = self.edges.values().filter(|edge| P::endpoint_as_u64(edge.source) == id || P::endpoint_as_u64(edge.target) == id).map(|edge| edge.id).collect();
        for edge_id in removed_edges {
            self.edges.remove(&edge_id);
            self.selection.edge_ids.remove(&edge_id);
        }
        self.selection.handle_ids.remove(&id);
    }

    pub(super) fn apply_pick_with_mode(&mut self, hit: HitObject<P::Endpoint>, mode: &str) {
        match hit {
            HitObject::Node(id) => {
                let current: BTreeSet<String> = self.selection.node_ids.iter().map(|nid| nid.to_string()).collect();
                let next = merge_pick_into_selection(&current, &id.to_string(), mode);
                self.selection.node_ids = next.iter().filter_map(|s| s.parse().ok()).collect();
            }
            HitObject::Endpoint(ep) => {
                let id = P::endpoint_as_u64(ep);
                if P::HAS_PORTS {
                    let current: BTreeSet<String> = self.selection.handle_ids.iter().map(|hid| hid.to_string()).collect();
                    let next = merge_pick_into_selection(&current, &id.to_string(), mode);
                    self.selection.handle_ids = next.iter().filter_map(|s| s.parse().ok()).collect();
                } else {
                    let current: BTreeSet<String> = self.selection.node_ids.iter().map(|nid| nid.to_string()).collect();
                    let next = merge_pick_into_selection(&current, &id.to_string(), mode);
                    self.selection.node_ids = next.iter().filter_map(|s| s.parse().ok()).collect();
                }
            }
            HitObject::Edge(id) => {
                let current: BTreeSet<String> = self.selection.edge_ids.iter().map(|eid| eid.to_string()).collect();
                let next = merge_pick_into_selection(&current, &id.to_string(), mode);
                self.selection.edge_ids = next.iter().filter_map(|s| s.parse().ok()).collect();
            }
        }
        self.clear_preselect();
        self.push_selection_event();
    }

    pub(super) fn clear_preselect(&mut self) {
        self.preselect = Selection::default();
        self.preselect_removed = Selection::default();
    }

    pub fn sync_selection_screen_overlay(&mut self, start_screen: Point, screen_points: &[Point]) {
        if screen_points.len() < 2 {
            self.selection_preview_points.clear();
            self.selection_preview_crossing = false;
            return;
        }
        self.selection_preview_crossing = !selection_drag_enclosing(self.selection_options.method.as_str(), start_screen, screen_points);
        self.selection_preview_points = selection_screen_overlay_points(self.selection_options.method.as_str(), start_screen, screen_points).unwrap_or_default();
    }

    pub(super) fn area_initial_string_set(&self) -> BTreeSet<String> {
        let mut set = BTreeSet::new();
        for id in &self.area_initial.node_ids {
            set.insert(id.to_string());
        }
        for id in &self.area_initial.handle_ids {
            set.insert(id.to_string());
        }
        for id in &self.area_initial.edge_ids {
            set.insert(id.to_string());
        }
        set
    }

    pub(super) fn resolve_area_hits(&self, initial: &BTreeSet<String>, start: Point, points: &[Point], merge_mode: &str) -> BTreeSet<String> {
        let Some((box_, enclosing, ref polygon)) = selection_drag_shape(self.selection_options.method.as_str(), start, points) else {
            return initial.clone();
        };
        let lasso = self.selection_options.method == "lasso";
        let mut hits = BTreeSet::new();
        if self.selection_options.select_nodes {
            for node in self.nodes.values() {
                if selection_contains_node_bounds(node, box_, enclosing, polygon, lasso) {
                    hits.insert(node.id.to_string());
                }
            }
        }
        if self.selection_options.select_handles && P::HAS_PORTS {
            for handle in self.handles.values() {
                if let Some(node) = self.nodes.get(&handle.node_id) {
                    let pos = handle_position(node, handle);
                    if selection_contains_handle_point(pos, handle.radius.max(6.0), box_, enclosing, polygon, lasso) {
                        hits.insert(handle.id.to_string());
                    }
                }
            }
        }
        if self.selection_options.select_edges {
            for edge in self.edges.keys() {
                if let Some(curve) = self.edge_curve(*edge) {
                    if selection_contains_edge_curve(curve, box_, enclosing, polygon, lasso) {
                        hits.insert(edge.to_string());
                    }
                }
            }
        }
        merge_ids_into_selection(initial, &hits, merge_mode)
    }

    fn selection_from_string_set(&self, ids: &BTreeSet<String>) -> Selection {
        let mut selection = Selection::default();
        for id in ids {
            if let Ok(nid) = id.parse::<NodeId>() {
                if self.nodes.contains_key(&nid) {
                    selection.node_ids.insert(nid);
                    continue;
                }
            }
            if let Ok(hid) = id.parse::<HandleId>() {
                if self.handles.contains_key(&hid) {
                    selection.handle_ids.insert(hid);
                    continue;
                }
            }
            if let Ok(eid) = id.parse::<EdgeId>() {
                if self.edges.contains_key(&eid) {
                    selection.edge_ids.insert(eid);
                }
            }
        }
        selection
    }

    pub(super) fn apply_area_preselect(&mut self, start: Point, points: &[Point], shift: bool, ctrl_or_meta: bool) {
        let merge_mode = pick_merge_mode_for_modifiers(ctrl_or_meta, shift, self.selection_options.mode.as_str());
        let anchor = self.area_initial_string_set();
        let next_ids: Vec<String> = self.resolve_area_hits(&anchor, start, points, merge_mode.as_str()).into_iter().collect();
        let (sorted, removed) = area_preselect_ids(&anchor, &next_ids);
        let next = self.selection_from_string_set(&next_ids.iter().cloned().collect());
        let removed_sel = self.selection_from_string_set(&removed.iter().cloned().collect());
        if self.preselect == next && self.preselect_removed == removed_sel {
            return;
        }
        self.preselect = next;
        self.preselect_removed = removed_sel;
        let _ = sorted;
        self.push_preselect_event();
    }

    pub(super) fn commit_selection_from_hits(&mut self, hits: &BTreeSet<String>) {
        self.selection = self.selection_from_string_set(hits);
        self.clear_preselect();
        self.push_selection_event();
    }

    fn push_preselect_event(&mut self) {
        self.events.push(BoardEvent::PreselectChanged {
            node_ids: self.preselect.node_ids.iter().copied().collect(),
            handle_ids: self.preselect.handle_ids.iter().copied().collect(),
            edge_ids: self.preselect.edge_ids.iter().copied().collect(),
            removed_node_ids: self.preselect_removed.node_ids.iter().copied().collect(),
            removed_handle_ids: self.preselect_removed.handle_ids.iter().copied().collect(),
            removed_edge_ids: self.preselect_removed.edge_ids.iter().copied().collect(),
        });
    }

    pub(super) fn update_hover(&mut self, hover: Option<u64>) {
        if self.hover == hover {
            return;
        }
        self.hover = hover;
        self.events.push(BoardEvent::HoverChanged { id: hover });
    }

    pub(super) fn push_selection_event(&mut self) {
        self.events.push(BoardEvent::SelectionChanged { edge_ids: self.selection.edge_ids.iter().copied().collect(), handle_ids: self.selection.handle_ids.iter().copied().collect(), node_ids: self.selection.node_ids.iter().copied().collect() });
    }
}
// #endregion 🔖️Engine
