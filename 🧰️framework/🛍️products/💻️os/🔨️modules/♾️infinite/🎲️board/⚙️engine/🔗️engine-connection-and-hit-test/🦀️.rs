//! 🔗️ Engine connection and hit test of the board engine: drawing an edge from a handle, connection validity, type refusal, node drag proximity, cycle checks and hit testing. See [`super`].

use super::*;

// #region 🔖️Engine
impl<P: GraphPortModel, D: Directedness> GraphEngine<P, D> {
    pub(super) fn begin_draw_edge_from_handle(&mut self, handle_id: HandleId, cursor: Point) {
        let Some(handle) = self.handles.get(&handle_id).cloned() else {
            self.interaction = InteractionMode::Idle;
            return;
        };
        let incoming = self.incoming_edge_for_handle(handle_id);
        let reconnect_from_target = |edge_id: EdgeId, target_hid: HandleId| -> Option<(HandleId, bool, Option<HandleId>, Option<EdgeId>)> {
            let src = P::endpoint_as_u64(self.edges.get(&edge_id)?.source);
            Some((src, true, Some(target_hid), Some(edge_id)))
        };
        let (anchor_handle, anchor_is_source, fixed_target, reconnecting) = match handle.role {
            HandleRole::Target => incoming.and_then(|e| reconnect_from_target(e, handle_id)).unwrap_or((handle_id, false, None, None)),
            HandleRole::Source => (handle_id, true, None, None),
            HandleRole::Any => incoming.and_then(|e| reconnect_from_target(e, handle_id)).unwrap_or((handle_id, true, None, None)),
        };
        self.interaction = InteractionMode::DrawEdge { anchor_handle, anchor_is_source, fixed_target, cursor, reconnecting, snap_target: None };
    }

    fn incoming_edge_for_handle(&self, handle_id: HandleId) -> Option<EdgeId> {
        self.edges.values().find(|edge| P::endpoint_as_u64(edge.target) == handle_id).map(|edge| edge.id)
    }

    fn displaced_incoming_edge(&self, target_hid: HandleId, reconnecting: Option<EdgeId>) -> Option<EdgeId> {
        self.edges.values().find(|edge| Some(edge.id) != reconnecting && P::endpoint_as_u64(edge.target) == target_hid).map(|edge| edge.id)
    }

    /// 🔌️ Every connection rule EXCEPT the declared port types — role, self-node, duplicate edge,
    /// single-incoming and acyclicity. Split out so a drag can tell "this port could never take a
    /// wire" apart from "this port takes wires, but not one carrying THIS value", which is the only
    /// refusal a user can act on and therefore the only one worth naming on screen.
    fn is_structurally_valid_connection(&self, source_hid: HandleId, target_hid: HandleId, reconnecting: Option<EdgeId>, allow_target_replace: bool) -> bool {
        if source_hid == target_hid {
            return false;
        }
        let Some(source_handle) = self.handles.get(&source_hid) else {
            return false;
        };
        let Some(target_handle) = self.handles.get(&target_hid) else {
            return false;
        };
        if source_handle.node_id == target_handle.node_id {
            return false;
        }
        if !matches!(source_handle.role, HandleRole::Source | HandleRole::Any) {
            return false;
        }
        if !matches!(target_handle.role, HandleRole::Target | HandleRole::Any) {
            return false;
        }
        if self.edges.values().any(|e| Some(e.id) != reconnecting && P::endpoint_as_u64(e.source) == source_hid && P::endpoint_as_u64(e.target) == target_hid) {
            return false;
        }
        if !allow_target_replace && self.edges.values().any(|e| Some(e.id) != reconnecting && P::endpoint_as_u64(e.target) == target_hid) {
            return false;
        }
        if self.enforce_acyclic {
            let src_node = source_handle.node_id;
            let tgt_node = target_handle.node_id;
            let excluding = reconnecting.or_else(|| if allow_target_replace { self.displaced_incoming_edge(target_hid, reconnecting) } else { None });
            if self.would_create_cycle_between_nodes(src_node, tgt_node, excluding) {
                return false;
            }
        }
        true
    }

    pub(super) fn is_valid_connection(&self, source_hid: HandleId, target_hid: HandleId, reconnecting: Option<EdgeId>, allow_target_replace: bool) -> bool {
        self.is_structurally_valid_connection(source_hid, target_hid, reconnecting, allow_target_replace) && self.handles_type_compatible(source_hid, target_hid)
    }

    /// 🚫️ The port pair the live wire drag is hovering that ONLY the port-type rule refuses — the
    /// evidence a surface paints a snap target as not-droppable with, and names in the user's own
    /// language. `None` while the drag is over open canvas, over a port it can legally take, or over
    /// one refused for a structural reason a hint could not help with.
    pub fn wire_drag_type_refusal(&self) -> Option<(HandleId, HandleId)> {
        let InteractionMode::DrawEdge { anchor_handle, anchor_is_source, fixed_target, cursor, reconnecting, .. } = self.interaction else {
            return None;
        };
        self.nearest_type_refused_pair(anchor_handle, anchor_is_source, fixed_target, reconnecting, cursor)
    }

    fn nearest_type_refused_pair(&self, anchor_handle: HandleId, anchor_is_source: bool, fixed_target: Option<HandleId>, reconnecting: Option<EdgeId>, cursor: Point) -> Option<(HandleId, HandleId)> {
        if !P::HAS_PORTS {
            return None;
        }
        let mut best: Option<(f64, HandleId, HandleId)> = None;
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
            if !self.is_structurally_valid_connection(source_hid, target_hid, reconnecting, true) || self.handles_type_compatible(source_hid, target_hid) {
                continue;
            }
            let Some(node) = self.nodes.get(&handle.node_id) else {
                continue;
            };
            let pos = handle_position(node, handle);
            let d = distance(cursor, pos);
            let tol = self.active_proximity_distance_world().max(DRAW_EDGE_TYPE_REFUSAL_WORLD_TOLERANCE) + handle.radius;
            if d <= tol && best.as_ref().is_none_or(|(best_d, _, _)| d < *best_d) {
                best = Some((d, source_hid, target_hid));
            }
        }
        best.map(|(_, source, target)| (source, target))
    }

    pub(super) fn try_connect_handles(&mut self, source_hid: HandleId, target_hid: HandleId, reconnecting: Option<EdgeId>) -> bool {
        if !self.is_valid_connection(source_hid, target_hid, reconnecting, true) {
            return false;
        }
        let new_id = reconnecting.unwrap_or_else(|| {
            let id = self.next_edge_id;
            self.next_edge_id += 1;
            id
        });
        if let Some(old_id) = reconnecting.or_else(|| self.incoming_edge_for_handle(target_hid)) {
            self.remove_edge(old_id);
        }
        let (Some(src_ep), Some(tgt_ep)) = (P::try_handle_endpoint(source_hid), P::try_handle_endpoint(target_hid)) else {
            return false;
        };
        self.create_edge(new_id, src_ep, tgt_ep);
        self.events.push(BoardEvent::EdgeConnected { id: new_id, source: source_hid, target: target_hid });
        true
    }

    fn connection_pairs_for_handles(&self, a: &Handle, b: &Handle) -> Vec<(HandleId, HandleId)> {
        let mut pairs = Vec::new();
        match (a.role, b.role) {
            (HandleRole::Source, HandleRole::Target) => pairs.push((a.id, b.id)),
            (HandleRole::Target, HandleRole::Source) => pairs.push((b.id, a.id)),
            (HandleRole::Source, HandleRole::Any) => pairs.push((a.id, b.id)),
            (HandleRole::Any, HandleRole::Source) => pairs.push((b.id, a.id)),
            (HandleRole::Target, HandleRole::Any) => pairs.push((b.id, a.id)),
            (HandleRole::Any, HandleRole::Target) => pairs.push((a.id, b.id)),
            (HandleRole::Any, HandleRole::Any) => {
                pairs.push((a.id, b.id));
                pairs.push((b.id, a.id));
            }
            _ => {}
        }
        pairs
    }

    pub fn handle_has_incident_edge(&self, handle_id: HandleId) -> bool {
        self.edges.values().any(|edge| P::endpoint_as_u64(edge.source) == handle_id || P::endpoint_as_u64(edge.target) == handle_id)
    }

    pub(super) fn update_node_drag_proximity(&mut self, dragged_ids: &[NodeId]) {
        self.proximity_connection = None;
        if !self.proximity_enabled() || !P::HAS_PORTS {
            return;
        }
        let dragged: BTreeSet<NodeId> = dragged_ids.iter().copied().collect();
        let mut best: Option<(f64, ProximityConnection)> = None;
        for dragged_handle in self.handles.values().filter(|h| dragged.contains(&h.node_id)) {
            if self.handle_has_incident_edge(dragged_handle.id) {
                continue;
            }
            for other_handle in self.handles.values().filter(|h| !dragged.contains(&h.node_id)) {
                for (source_hid, target_hid) in self.connection_pairs_for_handles(dragged_handle, other_handle) {
                    if !self.is_valid_connection(source_hid, target_hid, None, false) {
                        continue;
                    }
                    let Some(source_handle) = self.handles.get(&source_hid) else {
                        continue;
                    };
                    let Some(target_handle) = self.handles.get(&target_hid) else {
                        continue;
                    };
                    let Some(source_node) = self.nodes.get(&source_handle.node_id) else {
                        continue;
                    };
                    let Some(target_node) = self.nodes.get(&target_handle.node_id) else {
                        continue;
                    };
                    let src_pos = handle_position(source_node, source_handle);
                    let tgt_pos = handle_position(target_node, target_handle);
                    let d = distance(src_pos, tgt_pos);
                    let tol = self.active_proximity_distance_world() + source_handle.radius + target_handle.radius;
                    if d <= tol && best.as_ref().is_none_or(|(best_d, _)| d < *best_d) {
                        best = Some((d, ProximityConnection { source: source_hid, target: target_hid }));
                    }
                }
            }
        }
        if let Some((_, conn)) = best {
            self.proximity_connection = Some(conn);
        }
    }

    pub(super) fn proximity_preview_curve(&self, conn: ProximityConnection) -> Option<CubicBez> {
        let source_handle = self.handles.get(&conn.source)?;
        let target_handle = self.handles.get(&conn.target)?;
        let source_node = self.nodes.get(&source_handle.node_id)?;
        let target_node = self.nodes.get(&target_handle.node_id)?;
        let source_position = handle_position(source_node, source_handle);
        let target_position = handle_position(target_node, target_handle);
        Some(self.wire_bezier_between(source_position, target_position, Some(source_node), Some(target_node), source_handle.radius, target_handle.radius))
    }

    fn would_create_cycle_between_nodes(&self, source: NodeId, target: NodeId, excluding: Option<EdgeId>) -> bool {
        if source == target {
            return true;
        }
        let mut adj: std::collections::HashMap<NodeId, Vec<NodeId>> = std::collections::HashMap::new();
        for edge in self.edges.values() {
            if excluding == Some(edge.id) {
                continue;
            }
            let Some(src_h) = self.handles.get(&P::endpoint_as_u64(edge.source)) else {
                continue;
            };
            let Some(tgt_h) = self.handles.get(&P::endpoint_as_u64(edge.target)) else {
                continue;
            };
            adj.entry(src_h.node_id).or_default().push(tgt_h.node_id);
        }
        adj.entry(source).or_default().push(target);
        Self::has_path_nodes(&adj, target, source)
    }

    fn has_path_nodes(adj: &std::collections::HashMap<NodeId, Vec<NodeId>>, from: NodeId, to: NodeId) -> bool {
        let mut seen = std::collections::HashSet::new();
        let mut stack = vec![from];
        while let Some(n) = stack.pop() {
            if n == to {
                return true;
            }
            if !seen.insert(n) {
                continue;
            }
            if let Some(next) = adj.get(&n) {
                for m in next {
                    stack.push(*m);
                }
            }
        }
        false
    }

    pub(super) fn hit_test(&self, point: Point) -> Option<HitObject<P::Endpoint>> {
        if P::HAS_PORTS && self.handle_pointer_picking {
            for handle in self.handles.values().rev() {
                let node = self.nodes.get(&handle.node_id)?;
                if distance(point, handle_position(node, handle)) <= handle.radius + 6.0 {
                    if let Some(ep) = P::try_handle_endpoint(handle.id) {
                        return Some(HitObject::Endpoint(ep));
                    }
                }
            }
        }
        for node in self.nodes.values().rev() {
            if node_contains_point(node, point) {
                return Some(HitObject::Node(node.id));
            }
        }
        for edge in self.edges.values().rev() {
            if let Some(curve) = self.edge_curve(edge.id) {
                if distance_point_to_cubic_bezier(point, curve, 18) <= 8.0 {
                    return Some(HitObject::Edge(edge.id));
                }
            }
        }
        None
    }

    /// 🎯️ Returns every graph entity under a world point for pick disambiguation menus.
    pub fn hit_test_pick_targets(&self, point: Point) -> Vec<GraphPickTarget> {
        let mut out = Vec::new();
        for node in self.nodes.values().rev() {
            if node_contains_point(node, point) {
                out.push(GraphPickTarget { domain: "node".into(), id: node.id, generality: 0 });
            }
        }
        for edge in self.edges.values().rev() {
            if let Some(curve) = self.edge_curve(edge.id) {
                if distance_point_to_cubic_bezier(point, curve, 18) <= 8.0 {
                    out.push(GraphPickTarget { domain: "edge".into(), id: edge.id, generality: 1 });
                }
            }
        }
        if P::HAS_PORTS && self.handle_pointer_picking {
            for handle in self.handles.values().rev() {
                let node = match self.nodes.get(&handle.node_id) {
                    Some(node) => node,
                    None => continue,
                };
                if distance(point, handle_position(node, handle)) <= handle.radius + 6.0 {
                    if let Some(ep) = P::try_handle_endpoint(handle.id) {
                        out.push(GraphPickTarget { domain: "handle".into(), id: P::endpoint_as_u64(ep), generality: 2 });
                    }
                }
            }
        }
        out
    }
}
// #endregion 🔖️Engine
