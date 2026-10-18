//! 🪛️ Engine mutation of the board engine: constructing, camera, node, handle and edge edits, selection options, area cancel, select all and delete selection. See [`super`].

use super::*;

// #region 🔖️Engine
impl<P: GraphPortModel, D: Directedness> GraphEngine<P, D> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_camera(&mut self, x: f64, y: f64, zoom: f64) {
        self.camera = Camera { x, y, zoom };
    }

    pub fn create_node(&mut self, id: NodeId, x: f64, y: f64, radius: f64, draggable: bool) {
        self.nodes.insert(id, Node { center: Point::new(x, y), draggable, height: radius * 2.0, id, radius, shape: NodeShape::Circle, width: radius * 2.0, kind: None, label: None, properties: PropertyBag::new() });
    }

    pub fn create_rect_node(&mut self, id: NodeId, x: f64, y: f64, width: f64, height: f64, draggable: bool) {
        let hw = width * 0.5;
        let hh = height * 0.5;
        self.nodes.insert(id, Node { center: Point::new(x, y), draggable, height, id, radius: hw.max(hh).max(ui_styling::radii::NODE_MIN), shape: NodeShape::Rectangle, width, kind: None, label: None, properties: PropertyBag::new() });
    }

    pub fn update_node(&mut self, id: NodeId, x: f64, y: f64, radius: f64) {
        if let Some(node) = self.nodes.get_mut(&id) {
            node.center = Point::new(x, y);
            node.radius = radius;
            if node.shape == NodeShape::Circle {
                node.width = radius * 2.0;
                node.height = radius * 2.0;
            }
        }
    }

    pub fn set_next_edge_id(&mut self, id: u64) {
        self.next_edge_id = id;
    }

    pub fn remove_node(&mut self, id: NodeId) {
        self.nodes.remove(&id);
        if P::HAS_PORTS {
            let removed_handles: Vec<HandleId> = self.handles.values().filter(|handle| handle.node_id == id).map(|handle| handle.id).collect();
            for handle_id in removed_handles {
                self.remove_handle(handle_id);
            }
        } else {
            let removed_edges: Vec<EdgeId> = self.edges.values().filter(|edge| P::endpoint_as_u64(edge.source) == id || P::endpoint_as_u64(edge.target) == id).map(|edge| edge.id).collect();
            for edge_id in removed_edges {
                self.edges.remove(&edge_id);
                self.selection.edge_ids.remove(&edge_id);
            }
        }
        self.selection.node_ids.remove(&id);
        self.push_selection_event();
    }

    pub fn create_handle(&mut self, id: HandleId, node_id: NodeId, angle: f64) {
        if P::HAS_PORTS {
            self.handles.insert(id, Handle { angle, id, node_id, radius: ui_styling::radii::HANDLE_DEFAULT, role: HandleRole::Any, kind: None, value_types: Vec::new(), properties: PropertyBag::new() });
        }
    }

    pub fn set_handle_role(&mut self, id: HandleId, role: HandleRole) {
        if let Some(handle) = self.handles.get_mut(&id) {
            handle.role = role;
        }
    }

    /// 🔤️ Declares the value schemas a port carries, so `is_valid_connection` can refuse a wire the
    /// target port could never read. The board never invents these — they arrive with the port.
    ///
    /// @see `🧫️fixtures/🔌️port-types/🔣️.json` — the law both node-graph renderers answer
    pub fn set_handle_value_types(&mut self, id: HandleId, value_types: Vec<String>) {
        if let Some(handle) = self.handles.get_mut(&id) {
            handle.value_types = value_types;
        }
    }

    /// 🔌️ The ONE port-compatibility rule the board enforces: refuse only when both ends declare and
    /// the declared sets are disjoint.
    pub fn port_value_types_compatible(source: &[String], target: &[String]) -> bool {
        if source.is_empty() || target.is_empty() {
            return true;
        }
        source.iter().any(|provided| target.iter().any(|accepted| accepted == provided))
    }

    /// 🔌️ Whether a wire drawn from `source_hid` may land on `target_hid` given only the declared
    /// port types — the predicate a drag preview asks to paint a snap target as not-droppable.
    pub fn handles_type_compatible(&self, source_hid: HandleId, target_hid: HandleId) -> bool {
        let (Some(source), Some(target)) = (self.handles.get(&source_hid), self.handles.get(&target_hid)) else {
            return true;
        };
        Self::port_value_types_compatible(&source.value_types, &target.value_types)
    }

    pub fn create_edge(&mut self, id: EdgeId, source: P::Endpoint, target: P::Endpoint) {
        let (source, target) = orient_endpoints::<P::Endpoint, D>(source, target);
        self.edges.insert(id, GraphEdge { id, source, target });
        if id >= self.next_edge_id {
            self.next_edge_id = id + 1;
        }
    }

    pub fn remove_edge(&mut self, id: EdgeId) {
        if self.edges.remove(&id).is_some() {
            self.edge_semantics.remove(&id);
            self.selection.edge_ids.remove(&id);
            self.events.push(BoardEvent::EdgeRemoved { id });
        }
    }

    pub fn set_edge_semantics(&mut self, id: EdgeId, kind: Option<String>, properties: PropertyBag) {
        self.edge_semantics.insert(id, ElementSemantics { kind, properties });
    }

    pub fn set_selection_options(&mut self, method: &str, mode: &str, select_nodes: bool, select_handles: bool, select_edges: bool) {
        self.selection_options.method = method.to_string();
        self.selection_options.mode = normalize_selection_mode(mode);
        self.selection_options.select_nodes = select_nodes;
        self.selection_options.select_handles = select_handles;
        self.selection_options.select_edges = select_edges;
    }

    pub fn selection_preview_points(&self) -> &[Point] {
        &self.selection_preview_points
    }

    pub fn selection_preview_crossing(&self) -> bool {
        self.selection_preview_crossing
    }

    pub fn selection_preview_method(&self) -> &str {
        self.selection_options.method.as_str()
    }

    pub fn cancel_area_select(&mut self) -> bool {
        let prev = std::mem::replace(&mut self.interaction, InteractionMode::Idle);
        let cancelled = matches!(prev, InteractionMode::SelectionPending { .. } | InteractionMode::AreaSelect { .. });
        if cancelled {
            self.selection = self.area_initial.clone();
            self.clear_preselect();
            self.selection_preview_points.clear();
            self.selection_preview_crossing = false;
            self.push_selection_event();
        }
        cancelled
    }

    pub fn pointer_cancel_screen(&mut self) -> bool {
        let interaction = std::mem::replace(&mut self.interaction, InteractionMode::Idle);
        let cancelled = !matches!(interaction, InteractionMode::Idle);
        match interaction {
            InteractionMode::DragNode { .. } | InteractionMode::DragNodes { .. } => {
                for (id, start) in &self.drag_start_positions {
                    if let Some(node) = self.nodes.get_mut(id) {
                        node.center = start.0;
                    }
                }
            }
            InteractionMode::SelectionPending { .. } | InteractionMode::AreaSelect { .. } => {
                self.selection = self.area_initial.clone();
            }
            InteractionMode::DrawEdge { .. } | InteractionMode::Pan { .. } | InteractionMode::Idle => {}
        }
        self.proximity_connection = None;
        self.drag_start_positions.clear();
        self.clear_preselect();
        self.area_points.clear();
        self.area_screen_points.clear();
        self.selection_preview_points.clear();
        self.selection_preview_crossing = false;
        self.events.clear();
        cancelled
    }

    pub fn select_all(&mut self) {
        self.selection = Selection::default();
        if self.selection_options.select_nodes {
            self.selection.node_ids = self.nodes.keys().copied().collect();
        }
        if self.selection_options.select_handles && P::HAS_PORTS {
            self.selection.handle_ids = self.handles.keys().copied().collect();
        }
        if self.selection_options.select_edges {
            self.selection.edge_ids = self.edges.keys().copied().collect();
        }
        self.clear_preselect();
        self.push_selection_event();
    }

    pub fn delete_selection(&mut self) {
        let node_ids: Vec<_> = self.selection.node_ids.iter().copied().collect();
        for id in node_ids {
            self.remove_node(id);
        }
        let edge_ids: Vec<_> = self.selection.edge_ids.iter().copied().collect();
        for id in edge_ids {
            self.remove_edge(id);
        }
        if P::HAS_PORTS {
            let handle_ids: Vec<_> = self.selection.handle_ids.iter().copied().collect();
            for id in handle_ids {
                self.remove_handle(id);
            }
        }
        self.clear_preselect();
    }
}
// #endregion 🔖️Engine
